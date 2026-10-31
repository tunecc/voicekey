//! 会话提示音：内置音效池 + rodio 即发即忘播放（见 docs/openspec/changes/audio-feedback-sounds/design.md D1/D2/D5）
//!
//! 播放不依赖任何前端窗口；输出流懒初始化并常驻（菜单栏应用生命周期内复用），
//! 每次播放新建 Sink 存入注册表并顺带清理已结束项。任何失败只打日志、静默跳过，
//! 绝不影响录音与识别主流程。

use crate::settings::Settings;
use crate::AppState;
#[cfg(target_os = "macos")]
use objc2_app_kit::NSSound;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::OnceLock;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, Runtime};

/// 内置音效池：(id, 设置页显示名)。id 同时是 `resources/sounds/<id>.wav` 的文件名，是设置的单一来源。
pub const SOUNDS: &[(&str, &str)] = &[
    ("switch", "咔哒"),
    ("minimize", "下沉"),
    ("confirmation", "叮咚"),
    ("error", "警示"),
    ("pluck", "弹拨"),
    ("glass", "清脆"),
    ("tink", "轻叮"),
    ("pop", "轻啵"),
    ("ding", "轻铃"),
    ("soft-alert", "低鸣"),
];

/// 各事件默认音效 id（音色挑选见 resources/sounds/LICENSE.md）
pub const DEFAULT_START: &str = "switch";
pub const DEFAULT_PROCESSING: &str = "minimize";
pub const DEFAULT_DONE: &str = "confirmation";
pub const DEFAULT_ERROR: &str = "error";

/// 音效方案：simple=简洁（固定映射，复刻极简提示音风格）；custom=自定义（逐事件自选）
pub const SCHEME_SIMPLE: &str = "simple";
pub const SCHEME_CUSTOM: &str = "custom";

/// 方案值是否合法
pub fn is_known_scheme(scheme: &str) -> bool {
    matches!(scheme, SCHEME_SIMPLE | SCHEME_CUSTOM)
}

/// 简洁方案固定映射（纯双音复刻 cometix-dictation）：开始=轻叮、处理=轻啵；完成与失败不发声。
/// macOS 播系统音 Tink/Pop（见 `simple_scheme_system_name`），Windows 及系统音不可用时播池内 CC0 兜底音。
pub const SIMPLE_SCHEME: &[(SoundEvent, &str)] = &[
    (SoundEvent::Start, "tink"),
    (SoundEvent::Processing, "pop"),
];

/// 简洁方案在 macOS 上使用的系统音名（NSSound(named:)，与 cometix-dictation 同源同声）；
/// 非 macOS 平台无对应系统音，回退池内兜底音。完成与失败在简洁方案下不发声。
pub fn simple_scheme_system_name(event: SoundEvent) -> Option<&'static str> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    match event {
        SoundEvent::Start => Some("Tink"),
        SoundEvent::Processing => Some("Pop"),
        SoundEvent::Done | SoundEvent::Error => None,
    }
}

/// 事件对外键名（设置页事件行与固定映射行的关联键）
pub fn event_key(event: SoundEvent) -> &'static str {
    match event {
        SoundEvent::Start => "start",
        SoundEvent::Processing => "processing",
        SoundEvent::Done => "done",
        SoundEvent::Error => "error",
    }
}

/// 简洁方案只读映射行（设置页展示用；事件标签由前端事件表提供，避免双源）
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSchemeRow {
    pub event: &'static str,
    pub sound_id: &'static str,
    pub sound_name: &'static str,
}

/// 简洁方案固定映射的展示数据，行序与 SIMPLE_SCHEME 一致
pub fn simple_scheme_rows() -> Vec<SimpleSchemeRow> {
    SIMPLE_SCHEME
        .iter()
        .map(|(event, id)| SimpleSchemeRow {
            event: event_key(*event),
            sound_id: id,
            sound_name: SOUNDS.iter().find(|(known, _)| known == id).map(|(_, name)| *name).unwrap_or(""),
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEvent {
    Start,
    Processing,
    Done,
    Error,
}

/// 事件对应的逐事件音效 id（自定义方案取值；不考虑总开关与方案门控）
pub fn event_id<'a>(event: SoundEvent, s: &'a Settings) -> &'a str {
    match event {
        SoundEvent::Start => &s.sound_start,
        SoundEvent::Processing => &s.sound_processing,
        SoundEvent::Done => &s.sound_done,
        SoundEvent::Error => &s.sound_error,
    }
}

/// 事件→音效的取值门控：总开关关闭时返回 None（不发声）；
/// 简洁方案按固定映射取音效（忽略逐事件设置），自定义方案按逐事件设置；
/// 配置的 id 不在内置池中时返回 None
pub fn gated_id<'a>(event: SoundEvent, s: &'a Settings) -> Option<&'a str> {
    if !s.sound_enabled {
        return None;
    }
    let id = if s.sound_scheme == SCHEME_SIMPLE {
        SIMPLE_SCHEME.iter().find(|(e, _)| *e == event)?.1
    } else {
        event_id(event, s)
    };
    is_known(id).then_some(id)
}

/// 音效 id 是否在内置池中
pub fn is_known(id: &str) -> bool {
    SOUNDS.iter().any(|(known, _)| *known == id)
}

/// 会话事件播放：遵循总开关与音效方案。简洁方案在 macOS 播系统音 Tink/Pop（cometix-dictation 同声），
/// 系统音不可用回退池内兜底音；自定义方案按逐事件设置。
pub fn play<R: Runtime>(app: &AppHandle<R>, event: SoundEvent) {
    let st = app.state::<AppState>();
    let s = st.settings.read().unwrap();
    if !s.sound_enabled {
        return;
    }
    if s.sound_scheme == SCHEME_SIMPLE {
        let system_name = simple_scheme_system_name(event);
        let fallback_id = gated_id(event, &s).map(str::to_string);
        drop(s);
        if let Some(name) = system_name {
            play_system_quietly(app, name, fallback_id);
            return;
        }
        if let Some(id) = fallback_id {
            play_quietly(app, &id);
        }
        return;
    }
    let Some(id) = gated_id(event, &s).map(str::to_string) else { return };
    drop(s);
    play_quietly(app, &id);
}

/// 试听播放：不受总开关约束；未知 id 静默忽略。
/// 简洁方案下试听映射音效 id 时，播放该事件当前平台的实际音源（macOS 系统音，试听即所得）。
pub fn play_id<R: Runtime>(app: &AppHandle<R>, id: &str) {
    if !is_known(id) {
        return;
    }
    let st = app.state::<AppState>();
    let s = st.settings.read().unwrap();
    if s.sound_scheme == SCHEME_SIMPLE {
        let event = SIMPLE_SCHEME.iter().find(|(_, sid)| *sid == id).map(|(e, _)| *e);
        if let Some(event) = event {
            if let Some(name) = simple_scheme_system_name(event) {
                drop(s);
                play_system_quietly(app, name, Some(id.to_string()));
                return;
            }
        }
    }
    drop(s);
    play_quietly(app, id);
}

/// macOS 系统音：投递到主线程用 NSSound 播放（cometix-dictation 同策略）；系统音不可用时
/// 在主线程回退内置近似音。投递失败（应用退出中）直接走兜底，不影响主流程。
#[cfg(target_os = "macos")]
fn play_system_quietly<R: Runtime>(app: &AppHandle<R>, name: &str, fallback_id: Option<String>) {
    let inner_app = app.clone();
    let name = name.to_string();
    let inner_fallback = fallback_id.clone();
    let posted = app.run_on_main_thread(move || {
        if !system::play_named(&name) {
            if let Some(id) = inner_fallback {
                play_quietly(&inner_app, &id);
            }
        }
    });
    if posted.is_err() {
        if let Some(id) = fallback_id {
            play_quietly(app, &id);
        }
    }
}

#[cfg(target_os = "macos")]
mod system {
    use objc2_app_kit::NSSound;

    /// 主线程调用：播放系统命名提示音；返回是否成功发声
    pub fn play_named(name: &str) -> bool {
        use objc2_foundation::NSString;
        let Some(sound) = (unsafe { NSSound::soundNamed(&NSString::from_str(name)) }) else {
            return false;
        };
        unsafe { sound.play() }
    }
}

/// 非 macOS 平台没有系统提示音路径：`simple_scheme_system_name` 恒为 None，
/// 该分支运行期不可达，兜底由 `fallback_id` 分支承担；空实现仅保证跨平台编译。
#[cfg(not(target_os = "macos"))]
fn play_system_quietly<R: Runtime>(_: &AppHandle<R>, _: &str, _: Option<String>) {}

fn play_quietly<R: Runtime>(app: &AppHandle<R>, id: &str) {
    let path = match sound_path(app, id) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("提示音播放失败（已跳过）：{e:#}");
            return;
        }
    };
    match audio_tx() {
        Some(tx) => {
            let _ = tx.send(path);
        }
        None => eprintln!("提示音播放失败（已跳过）：音频线程不可用"),
    }
}

fn sound_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> anyhow::Result<PathBuf> {
    Ok(app.path().resolve(format!("resources/sounds/{id}.wav"), BaseDirectory::Resource)?)
}

/// 事件提示音的大致时长：「先播音、后压音量」的延迟依据。
/// 简洁方案 macOS 用系统音已知时长，其余读池内 WAV 头估算；不发声（总开关关/简洁完成失败）为 0。
/// 附投递余量：主线程队列与音量生效各需几毫秒。
pub fn play_duration<R: Runtime>(app: &AppHandle<R>, event: SoundEvent, s: &Settings) -> Duration {
    const MARGIN: Duration = Duration::from_millis(40);
    if !s.sound_enabled {
        return Duration::ZERO;
    }
    let base = if s.sound_scheme == SCHEME_SIMPLE {
        match simple_scheme_system_name(event) {
            Some("Tink") => Some(Duration::from_millis(60)),
            Some("Pop") => Some(Duration::from_millis(75)),
            Some(_) => None,
            None => gated_id(event, s).and_then(|id| pool_wav_duration(app, id)),
        }
    } else {
        gated_id(event, s).and_then(|id| pool_wav_duration(app, id))
    };
    base.map(|d| d + MARGIN).unwrap_or_default()
}

/// 解析池内 WAV 的时长：标准 PCM 头 byte_rate@28..32、data 大小@40..44；解析失败返回 None
fn pool_wav_duration<R: Runtime>(app: &AppHandle<R>, id: &str) -> Option<Duration> {
    let path = sound_path(app, id).ok()?;
    let bytes = std::fs::read(path).ok()?;
    wav_duration_bytes(&bytes)
}

/// 解析 PCM WAV 时长：从偏移 12 遍历 RIFF 块链（fourcc + LE u32 size，偶对齐步进），
/// 取 `fmt ` 的 byte_rate 与 `data` 的大小；兼容 ffmpeg 写出的 LIST/INFO 元数据块。
/// 非 RIFF/WAVE、缺块或字节率为 0 时返回 None（调用方按 0 时长立即压音量，即 6.3 前行为）。
fn wav_duration_bytes(b: &[u8]) -> Option<Duration> {
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return None;
    }
    let mut off = 12usize;
    let mut byte_rate = None;
    let mut data_len = None;
    while off + 8 <= b.len() {
        let fourcc = &b[off..off + 4];
        let size = u32::from_le_bytes(b[off + 4..off + 8].try_into().ok()?) as usize;
        let body = off + 8;
        match fourcc {
            // fmt：wFormatTag(2) nChannels(2) nSamplesPerSec(4) nAvgBytesPerSec(4)…
            b"fmt " if body + 16 <= b.len() => {
                byte_rate = Some(u32::from_le_bytes(b[body + 8..body + 12].try_into().ok()?));
            }
            b"data" => {
                data_len = Some(size.min(b.len().saturating_sub(body)) as u64);
            }
            _ => {}
        }
        if byte_rate.is_some() && data_len.is_some() {
            break;
        }
        off = body.saturating_add(size + (size & 1));
    }
    let byte_rate = byte_rate? as u64;
    if byte_rate == 0 {
        return None;
    }
    Some(Duration::from_millis(data_len?.saturating_mul(1000) / byte_rate))
}

/// 音频工作线程的请求通道；线程首次播放请求时懒启动
static AUDIO_TX: OnceLock<Option<mpsc::Sender<PathBuf>>> = OnceLock::new();

fn audio_tx() -> Option<mpsc::Sender<PathBuf>> {
    if let Some(tx) = AUDIO_TX.get() {
        return tx.clone();
    }
    let (tx, rx) = mpsc::channel();
    let ok = std::thread::Builder::new()
        .name("voicekey-sound".into())
        .spawn(move || audio_loop(rx))
        .is_ok();
    let result = ok.then_some(tx);
    if AUDIO_TX.set(result.clone()).is_err() {
        // 并发首播：另一线程已登记通道；弃用本次新建（其发送端无人持有，线程会自行退出），
        // 改用已登记的通道，避免把请求发进即将断开的管道
        return AUDIO_TX.get().cloned().flatten();
    }
    result
}

/// 常驻音频线程：OutputStream/Sink 全部在本线程内创建与持有（OutputStream 不满足 Send，
/// 不能跨线程移动）。设备打开失败时按请求重试，稍后设备就绪即可恢复出声。
fn audio_loop(rx: mpsc::Receiver<PathBuf>) {
    struct Output {
        _stream: OutputStream,
        handle: OutputStreamHandle,
        sinks: Vec<Sink>,
    }
    fn open() -> anyhow::Result<Output> {
        let (stream, handle) = OutputStream::try_default().map_err(|_| anyhow::anyhow!("无法打开系统音频输出设备"))?;
        Ok(Output { _stream: stream, handle, sinks: Vec::new() })
    }
    fn play_once(out: &mut Output, path: &PathBuf) -> anyhow::Result<()> {
        let sink = Sink::try_new(&out.handle)?;
        sink.append(Decoder::new_wav(BufReader::new(std::fs::File::open(path)?))?);
        out.sinks.retain(|s| !s.empty());
        out.sinks.push(sink);
        Ok(())
    }

    let mut output: Option<Output> = None;
    while let Ok(path) = rx.recv() {
        if output.is_none() {
            output = open().map_err(|e| eprintln!("提示音播放失败（已跳过）：{e:#}")).ok();
        }
        if let Some(out) = output.as_mut() {
            if let Err(e) = play_once(out, &path) {
                eprintln!("提示音播放失败（已跳过）：{e:#}");
                // 设备可能已失效（拔出/切换）：置空以便下次请求时重新打开
                output = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_pool_is_not_empty_and_ids_match_wav_files() {
        assert!(!SOUNDS.is_empty(), "内置音效池不能为空");
        let mut seen = std::collections::HashSet::new();
        for (id, label) in SOUNDS {
            assert!(!id.is_empty() && !label.is_empty());
            assert!(seen.insert(*id), "重复音效 id：{id}");
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("resources/sounds")
                .join(format!("{id}.wav"));
            assert!(path.is_file(), "缺少音效文件：{}", path.display());
        }
    }

    #[test]
    fn is_known_only_accepts_pool_ids() {
        assert!(!is_known(""));
        assert!(!is_known("nonexistent-sound"));
    }

    #[test]
    fn simple_scheme_sound_ids_are_registered_in_the_pool() {
        for id in ["tink", "pop", "ding", "soft-alert"] {
            assert!(is_known(id), "简洁方案音效 {id} 必须登记在内置池中");
        }
    }

    #[test]
    fn master_switch_off_gates_all_events() {
        let mut s = Settings::default();
        s.sound_enabled = false;
        for event in [SoundEvent::Start, SoundEvent::Processing, SoundEvent::Done, SoundEvent::Error] {
            assert_eq!(gated_id(event, &s), None, "总开关关闭时 {event:?} 不应发声");
        }
        // 无论音效方案为何：简洁方案同样全静音
        s.sound_scheme = SCHEME_SIMPLE.into();
        for event in [SoundEvent::Start, SoundEvent::Processing, SoundEvent::Done, SoundEvent::Error] {
            assert_eq!(gated_id(event, &s), None, "总开关关闭时简洁方案 {event:?} 不应发声");
        }
    }

    #[test]
    fn master_switch_on_maps_each_event_to_its_own_setting() {
        let mut s = Settings::default();
        s.sound_enabled = true;
        // 默认映射存在
        for event in [SoundEvent::Start, SoundEvent::Processing, SoundEvent::Done, SoundEvent::Error] {
            assert!(!event_id(event, &s).is_empty(), "{event:?} 必须有默认音效");
        }
        // 改一个事件的选择只影响该事件
        let done_before = event_id(SoundEvent::Done, &s).to_string();
        s.sound_processing = done_before.clone();
        assert_eq!(event_id(SoundEvent::Processing, &s), done_before);
        s.sound_processing = String::new();
        assert_eq!(event_id(SoundEvent::Done, &s), done_before, "其他事件的选择不受影响");
    }

    #[test]
    fn every_default_setting_id_is_in_the_pool() {
        let s = Settings::default();
        for id in [&s.sound_start, &s.sound_processing, &s.sound_done, &s.sound_error] {
            assert!(is_known(id), "默认音效 {id} 必须在内置池中");
        }
    }

    #[test]
    fn gated_id_rejects_unknown_configured_ids() {
        let mut s = Settings::default();
        s.sound_enabled = true;
        s.sound_start = "removed-sound".into();
        assert_eq!(gated_id(SoundEvent::Start, &s), None, "未知音效 id 不应发声");
    }

    #[test]
    fn simple_scheme_maps_start_and_processing_only() {
        let mut s = Settings::default();
        s.sound_enabled = true;
        s.sound_scheme = SCHEME_SIMPLE.into();
        // 逐事件字段与映射不同：简洁方案应忽略它们，按固定映射发声
        s.sound_start = "confirmation".into();
        s.sound_done = "switch".into();
        assert_eq!(gated_id(SoundEvent::Start, &s), Some("tink"));
        assert_eq!(gated_id(SoundEvent::Processing, &s), Some("pop"));
        // 纯双音复刻：完成与失败不发声
        assert_eq!(gated_id(SoundEvent::Done, &s), None, "简洁方案完成不发声");
        assert_eq!(gated_id(SoundEvent::Error, &s), None, "简洁方案失败不发声");
    }

    #[test]
    fn custom_scheme_keeps_per_event_selection() {
        let mut s = Settings::default();
        s.sound_enabled = true;
        s.sound_scheme = SCHEME_CUSTOM.into();
        assert_eq!(gated_id(SoundEvent::Start, &s), Some(s.sound_start.as_str()));
        assert_eq!(gated_id(SoundEvent::Processing, &s), Some(s.sound_processing.as_str()));
        assert_eq!(gated_id(SoundEvent::Done, &s), Some(s.sound_done.as_str()));
        assert_eq!(gated_id(SoundEvent::Error, &s), Some(s.sound_error.as_str()));
    }

    #[test]
    fn simple_scheme_mapping_is_pure_two_sound_replica() {
        assert_eq!(SIMPLE_SCHEME.len(), 2, "纯双音：仅开始与处理两个映射");
        let mut seen = std::collections::HashSet::new();
        for (event, id) in SIMPLE_SCHEME {
            assert!(is_known(id), "映射音效 {id} 必须在内置池中（Windows 兜底）");
            assert!(seen.insert(*id), "映射音效必须互不相同：{id} 重复");
            assert!(matches!(event, SoundEvent::Start | SoundEvent::Processing), "只允许开始/处理映射");
        }
    }

    #[test]
    fn simple_scheme_system_names_follow_platform() {
        if cfg!(target_os = "macos") {
            assert_eq!(simple_scheme_system_name(SoundEvent::Start), Some("Tink"));
            assert_eq!(simple_scheme_system_name(SoundEvent::Processing), Some("Pop"));
        } else {
            assert_eq!(simple_scheme_system_name(SoundEvent::Start), None);
            assert_eq!(simple_scheme_system_name(SoundEvent::Processing), None);
        }
        assert_eq!(simple_scheme_system_name(SoundEvent::Done), None, "完成不发声");
        assert_eq!(simple_scheme_system_name(SoundEvent::Error), None, "失败不发声");
    }

    /// 方案切换不得动逐事件设置：简洁门控只读，切回自定义后原选择仍生效
    #[test]
    fn simple_scheme_gating_never_touches_per_event_fields() {
        let mut s = Settings::default();
        s.sound_enabled = true;
        s.sound_start = "glass".into();
        s.sound_processing = "pluck".into();
        s.sound_done = "switch".into();
        s.sound_error = "error".into();
        s.sound_scheme = SCHEME_SIMPLE.into();
        let before = (s.sound_start.clone(), s.sound_processing.clone(), s.sound_done.clone(), s.sound_error.clone());
        for event in [SoundEvent::Start, SoundEvent::Processing, SoundEvent::Done, SoundEvent::Error] {
            let _ = gated_id(event, &s);
        }
        assert_eq!(
            (s.sound_start.clone(), s.sound_processing.clone(), s.sound_done.clone(), s.sound_error.clone()),
            before,
            "简洁方案门控必须只读，不改写逐事件字段"
        );
        s.sound_scheme = SCHEME_CUSTOM.into();
        assert_eq!(gated_id(SoundEvent::Start, &s), Some("glass"), "切回自定义后保留原选择");
    }

    #[test]
    fn simple_scheme_rows_cover_sounding_events_with_pool_labels() {
        let rows = simple_scheme_rows();
        assert_eq!(rows.len(), 2, "纯双音：仅开始与处理两行");
        for row in &rows {
            assert!(is_known(row.sound_id), "soundId 必须在内置池中");
            assert!(!row.sound_name.is_empty(), "soundName 必须来自音效池中文名");
        }
        let ids: Vec<_> = rows.iter().map(|r| r.sound_id).collect();
        assert_eq!(ids, vec!["tink", "pop"], "行序与固定映射一致");
        assert_eq!(rows[0].event, "start");
        assert_eq!(rows[1].event, "processing");
    }

    /// 前端按 camelCase 读取：序列化键名必须是 soundId/soundName，否则设置页映射行为空
    #[test]
    fn simple_scheme_rows_serialize_camel_case_keys() {
        let value = serde_json::to_value(simple_scheme_rows()).unwrap();
        let first = value.as_array().unwrap().first().unwrap();
        assert!(first.get("soundId").is_some(), "缺少 camelCase 键 soundId：{first}");
        assert!(first.get("soundName").is_some(), "缺少 camelCase 键 soundName：{first}");
        assert!(first.get("event").is_some());
        assert!(first.get("sound_id").is_none(), "不允许混入 snake_case 键");
    }

    /// RIFF 块链解析：标准头、ffmpeg 实际布局（fmt → LIST 元数据 → data）、坏头
    #[test]
    fn wav_duration_parses_riff_chunk_chain() {
        let mut b = Vec::new();
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(b"WAVE");
        b.extend_from_slice(b"fmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&[1, 0, 1, 0]);
        b.extend_from_slice(&44_100u32.to_le_bytes());
        b.extend_from_slice(&88_200u32.to_le_bytes());
        b.extend_from_slice(&[2, 0, 16, 0]);
        b.extend_from_slice(b"LIST");
        b.extend_from_slice(&26u32.to_le_bytes());
        b.extend_from_slice(&vec![0u8; 26]);
        b.extend_from_slice(b"data");
        b.extend_from_slice(&8_820u32.to_le_bytes());
        b.extend_from_slice(&vec![0u8; 8_820]);
        assert_eq!(wav_duration_bytes(&b), Some(Duration::from_millis(100)));
        assert_eq!(wav_duration_bytes(&[]), None);
        let mut notwav = b.clone();
        notwav[0..4].copy_from_slice(b"RIFX");
        assert_eq!(wav_duration_bytes(&notwav), None);
    }

    /// 回归 6.3：固定偏移解析曾对真实池内文件（含 LIST 元数据块）返回 0ms。
    /// 真实文件必须解析出合理时长，且不为 0。
    #[test]
    fn wav_duration_matches_real_pool_files() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/sounds");
        for (id, expected_ms) in [("tink", 55), ("pop", 72), ("ding", 300), ("switch", 124)] {
            let bytes = std::fs::read(dir.join(format!("{id}.wav"))).expect("池内文件存在");
            let d = wav_duration_bytes(&bytes).expect("真实 WAV 应能解析出时长");
            let got = d.as_millis() as i64;
            assert!(got > 0, "{id} 时长不应为 0（固定偏移回归）");
            assert!(
                (got - expected_ms).abs() <= 10,
                "{id} 时长 {got}ms 与 ffprobe 基准 {expected_ms}ms 偏差过大"
            );
        }
    }
}
