//! 会话提示音：内置音效池 + rodio 即发即忘播放（见 docs/openspec/changes/audio-feedback-sounds/design.md D1/D2/D5）
//!
//! 播放不依赖任何前端窗口；输出流懒初始化并常驻（菜单栏应用生命周期内复用），
//! 每次播放新建 Sink 存入注册表并顺带清理已结束项。任何失败只打日志、静默跳过，
//! 绝不影响录音与识别主流程。

use crate::settings::Settings;
use crate::AppState;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
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
];

/// 各事件默认音效 id（音色挑选见 resources/sounds/LICENSE.md）
pub const DEFAULT_START: &str = "switch";
pub const DEFAULT_PROCESSING: &str = "minimize";
pub const DEFAULT_DONE: &str = "confirmation";
pub const DEFAULT_ERROR: &str = "error";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEvent {
    Start,
    Processing,
    Done,
    Error,
}

/// 事件对应的音效 id（不考虑总开关）
pub fn event_id<'a>(event: SoundEvent, s: &'a Settings) -> &'a str {
    match event {
        SoundEvent::Start => &s.sound_start,
        SoundEvent::Processing => &s.sound_processing,
        SoundEvent::Done => &s.sound_done,
        SoundEvent::Error => &s.sound_error,
    }
}

/// 事件→音效的取值门控：总开关关闭、或配置的 id 不在内置池中时返回 None（不发声）
pub fn gated_id<'a>(event: SoundEvent, s: &'a Settings) -> Option<&'a str> {
    if !s.sound_enabled {
        return None;
    }
    let id = event_id(event, s);
    is_known(id).then_some(id)
}

/// 音效 id 是否在内置池中
pub fn is_known(id: &str) -> bool {
    SOUNDS.iter().any(|(known, _)| *known == id)
}

/// 会话事件播放：遵循总开关与逐事件音效设置
pub fn play<R: Runtime>(app: &AppHandle<R>, event: SoundEvent) {
    let st = app.state::<AppState>();
    let s = st.settings.read().unwrap();
    let Some(id) = gated_id(event, &s).map(str::to_string) else { return };
    drop(s);
    play_quietly(app, &id);
}

/// 试听播放：不受总开关约束；未知 id 静默忽略
pub fn play_id<R: Runtime>(app: &AppHandle<R>, id: &str) {
    if !is_known(id) {
        return;
    }
    play_quietly(app, id);
}

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
    fn master_switch_off_gates_all_events() {
        let mut s = Settings::default();
        s.sound_enabled = false;
        for event in [SoundEvent::Start, SoundEvent::Processing, SoundEvent::Done, SoundEvent::Error] {
            assert_eq!(gated_id(event, &s), None, "总开关关闭时 {event:?} 不应发声");
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
}
