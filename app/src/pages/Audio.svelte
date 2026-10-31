<script lang="ts">
  import { onMount } from "svelte";
  import { app, save, refreshMics } from "../lib/store.svelte";
  import { call, listSounds, previewSound, simpleScheme, type SimpleSchemeRow } from "../lib/api";
  import Meter from "../lib/Meter.svelte";

  const s = $derived(app.s!);
  const missing = $derived(s.mic && !app.mics.includes(s.mic));
  const mac = $derived(app.info?.platform === "mac");
  let sounds = $state<{ id: string; name: string }[]>([]);
  let schemeRows = $state<SimpleSchemeRow[]>([]);

  onMount(() => {
    const sync = () => call("meter", { on: document.hasFocus() });
    refreshMics();
    listSounds().then((v) => (sounds = v));
    simpleScheme().then((v) => (schemeRows = v));
    window.addEventListener("focus", refreshMics);
    window.addEventListener("focus", sync);
    window.addEventListener("blur", sync);
    sync();
    return () => {
      window.removeEventListener("focus", refreshMics);
      window.removeEventListener("focus", sync);
      window.removeEventListener("blur", sync);
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
<div class="sub">选择用于识别的麦克风。</div>
<div class="group">
  <div class="row">
    <div class="lbl">麦克风<small>设备断开时自动回落到系统默认</small></div>
    <select class="sel" value={s.mic} onchange={(e) => pick(e.currentTarget.value)}>
      <option value="">系统默认</option>
      {#each app.mics as m}<option value={m}>{m}</option>{/each}
      {#if missing}<option value={s.mic}>已断开（暂用系统默认）</option>{/if}
    </select>
  </div>
  {#if mac}
    <div class="row">
      <div class="lbl">录音时调整音量<small>{s.volume > 0 ? `录音时把系统输出音量设为 ${s.volume}%，结束后还原` : "录音时不改变系统输出音量"}</small></div>
      {#if s.volume > 0}
        <input type="range" min="1" max="100" step="1" value={s.volume} oninput={(e) => save({ volume: +e.currentTarget.value })} aria-label="录音时调整音量" />
        <b class="val">{s.volume}%</b>
      {/if}
      <button class="sw" class:on={s.volume > 0} onclick={() => save({ volume: s.volume > 0 ? 0 : 15 })} aria-label="录音时调整音量"></button>
    </div>
  {/if}
  <div class="row">
    <div class="lbl">输入电平<small>对着麦克风说话测试</small></div>
    <Meter />
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
          {#each sounds as snd}<option value={snd.id}>{snd.name}</option>{/each}
        </select>
        <button class="btn" onclick={() => previewSound(s[ev.key])} aria-label={`试听${ev.label}`}>试听</button>
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
</style>
