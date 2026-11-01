<script lang="ts">
  import { onMount } from "svelte";
  import { app, save, refreshMics } from "../lib/store.svelte";
  import { call, listSounds, previewSound, simpleScheme, type SimpleSchemeRow } from "../lib/api";
  import Meter from "../lib/Meter.svelte";

  const s = $derived(app.s!);
  const missing = $derived(s.mic && !app.mics.includes(s.mic));
  let sounds = $state<{ id: string; name: string }[]>([]);
  let schemeRows = $state<SimpleSchemeRow[]>([]);
  // 设置值和 UI 都直接表示“降低比例”；100% 只改变展示为“静音”，数值本身不做反转。
  const duckReductionLabel = $derived(s.lowerSystemVolumePercent === 100 ? "静音" : `−${s.lowerSystemVolumePercent}%`);

  onMount(() => {
    listSounds().then((v) => (sounds = v));
    simpleScheme().then((v) => (schemeRows = v));
    // 电平表只在设置窗处于前台时占用麦克风，切到其他应用立即释放。
    const onFocus = () => {
      refreshMics();
      call("meter", { on: true });
    };
    const onBlur = () => {
      call("meter", { on: false });
    };
    window.addEventListener("focus", onFocus);
    window.addEventListener("blur", onBlur);
    call("meter", { on: true });
    return () => {
      window.removeEventListener("focus", onFocus);
      window.removeEventListener("blur", onBlur);
      call("meter", { on: false });
    };
  });

  function pick(v: string) {
    save({ mic: v });
  }

  const EVENTS: { key: "soundStart" | "soundProcessing" | "soundDone" | "soundError"; event: SimpleSchemeRow["event"]; label: string; desc: string }[] = [
    { key: "soundStart", event: "start", label: "录音开始", desc: "开始录入语音时" },
    { key: "soundProcessing", event: "processing", label: "识别处理", desc: "录音结束进入识别时" },
    { key: "soundDone", event: "done", label: "识别完成", desc: "结果就绪或上屏时" },
    { key: "soundError", event: "error", label: "识别失败", desc: "出错、超时或没有内容时" },
  ];
  const evInfo = (event: SimpleSchemeRow["event"]) => EVENTS.find((e) => e.event === event)!;
</script>

<h2>音频</h2>
<div class="sub" aria-hidden="true">&nbsp;</div>
<h3>输入设备</h3>
<div class="group">
  <div class="row">
    <div class="lbl">麦克风<small>用于识别的麦克风，设备断开时自动切回系统默认</small></div>
    <select class="sel" value={s.mic} onchange={(e) => pick(e.currentTarget.value)}>
      <option value="">系统默认</option>
      {#each app.mics as m}<option value={m}>{m}</option>{/each}
      {#if missing}<option value={s.mic}>已断开（暂用系统默认）</option>{/if}
    </select>
  </div>
  <div class="row">
    <div class="lbl">输入电平<small>对着麦克风说话测试</small></div>
    <Meter />
  </div>
</div>

<h3>系统音频</h3>
<div class="group">
  <div class="row">
    <div class="lbl">录音时降低音量<small>减少背景声音的干扰，结束后自动恢复</small></div>
    {#if s.lowerSystemVolume}
      <!-- 录音中拖动比例会即时生效；worker 从当前实际音量平滑重定向到新目标，不等待当前过渡完成。 -->
      <input class="duck-range" type="range" min="20" max="100" step="5" value={s.lowerSystemVolumePercent} oninput={(e) => save({ lowerSystemVolumePercent: +e.currentTarget.value })} aria-label="录音时降低音量比例" aria-valuetext={s.lowerSystemVolumePercent === 100 ? "静音" : `降低 ${s.lowerSystemVolumePercent}%`} />
      <b class="duck-value">{duckReductionLabel}</b>
    {/if}
    <button class="sw" class:on={s.lowerSystemVolume} onclick={() => save({ lowerSystemVolume: !s.lowerSystemVolume })} aria-label="录音时降低音量"></button>
  </div>
</div>

<h3>提示音</h3>
<div class="group">
  <div class="row">
    <div class="lbl">提示音<small>{s.soundEnabled ? "录音各阶段播放提示音" : "已关闭，录音各阶段不播放提示音"}</small></div>
    <button class="sw" class:on={s.soundEnabled} onclick={() => save({ soundEnabled: !s.soundEnabled })} aria-label="提示音"></button>
  </div>
  <div class="row">
    <div class="lbl">音效方案<small>{s.soundScheme === "simple" ? "极简风格，各阶段使用固定音效" : "四个事件各自挑选音效"}</small></div>
    <div class="seg" role="radiogroup" aria-label="音效方案">
      <button class="seg-btn" class:on={s.soundScheme === "simple"} role="radio" aria-checked={s.soundScheme === "simple"} onclick={() => save({ soundScheme: "simple" })}>简洁</button>
      <button class="seg-btn" class:on={s.soundScheme === "custom"} role="radio" aria-checked={s.soundScheme === "custom"} onclick={() => save({ soundScheme: "custom" })}>自定义</button>
    </div>
  </div>
  {#if s.soundScheme === "simple"}
    {#each schemeRows as row (row.event)}
      {@const ev = evInfo(row.event)}
      <div class="row">
        <div class="lbl">{ev.label}<small>{ev.desc}</small></div>
        <b class="sound-name">{row.soundName}</b>
        <button class="btn" onclick={() => previewSound(row.soundId)} aria-label={`试听${ev.label}`}>试听</button>
      </div>
    {/each}
    <div class="row">
      <div class="lbl">识别完成 / 识别失败<small>纯双音极简风格，这两个阶段不发声</small></div>
      <b class="sound-name muted">不发声</b>
    </div>
  {:else}
    {#each EVENTS as ev}
      <div class="row">
        <div class="lbl">{ev.label}<small>{ev.desc}</small></div>
        <select class="sel" value={s[ev.key]} onchange={(e) => save({ [ev.key]: e.currentTarget.value })} aria-label={ev.label}>
          <option value="none">无</option>
          {#each sounds as snd}<option value={snd.id}>{snd.name}</option>{/each}
        </select>
        {#if s[ev.key] !== "none"}<button class="btn" onclick={() => previewSound(s[ev.key])} aria-label={`试听${ev.label}`}>试听</button>{/if}
      </div>
    {/each}
  {/if}
</div>

<style>
  input[type="range"] { accent-color: var(--accent); width: 180px; }
  .val { width: 44px; }
  .seg { display: flex; border: 1px solid var(--line); border-radius: var(--rs); overflow: hidden; flex: none; }
  .seg-btn { border: 0; background: var(--card2); padding: 6px 14px; cursor: pointer; color: var(--fg2); }
  .seg-btn + .seg-btn { border-left: 1px solid var(--line); }
  .seg-btn.on { background: var(--accent); color: #fff; }
  .sound-name { font-weight: 600; flex: none; }
  .sound-name.muted { color: var(--fg2); font-weight: 400; }
  .duck-range { width: 160px; accent-color: var(--accent); }
  .duck-value { width: 42px; text-align: right; font-variant-numeric: tabular-nums; }
</style>
