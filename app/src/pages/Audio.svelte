<script lang="ts">
  import { onMount } from "svelte";
  import { app, save, refreshMics } from "../lib/store.svelte";
  import { call, listSounds, previewSound } from "../lib/api";
  import Meter from "../lib/Meter.svelte";

  const s = $derived(app.s!);
  const missing = $derived(s.mic && !app.mics.includes(s.mic));
  const mac = $derived(app.info?.platform === "mac");
  let sounds = $state<{ id: string; name: string }[]>([]);

  onMount(() => {
    const sync = () => call("meter", { on: document.hasFocus() });
    refreshMics();
    listSounds().then((v) => (sounds = v));
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

  const EVENTS: { key: "soundStart" | "soundProcessing" | "soundDone" | "soundError"; label: string; desc: string }[] = [
    { key: "soundStart", label: "录音开始", desc: "开始录入语音时" },
    { key: "soundProcessing", label: "识别处理", desc: "录音结束进入识别时" },
    { key: "soundDone", label: "识别完成", desc: "结果就绪或上屏时" },
    { key: "soundError", label: "识别失败", desc: "出错、超时或没有内容时" },
  ];
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
  {#each EVENTS as ev}
    <div class="row">
      <div class="lbl">{ev.label}<small>{ev.desc}</small></div>
      <select class="sel" value={s[ev.key]} onchange={(e) => save({ [ev.key]: e.currentTarget.value })} aria-label={ev.label}>
        {#each sounds as snd}<option value={snd.id}>{snd.name}</option>{/each}
      </select>
      <button class="btn" onclick={() => previewSound(s[ev.key])} aria-label={`试听${ev.label}`}>试听</button>
    </div>
  {/each}
</div>

<style>
  input[type="range"] { accent-color: var(--accent); width: 180px; }
  .val { width: 44px; }
</style>
