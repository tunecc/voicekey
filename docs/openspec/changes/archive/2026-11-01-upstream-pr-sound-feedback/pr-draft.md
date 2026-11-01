# PR 草稿 — feat/session-sound-feedback → J3n5en/voicekey

**标题**：`feat: session sound feedback — per-event system chimes with preview`

**基线**：upstream/main @ 25b3f50 (v1.9.0)　**提交**：单个 feat 提交

---

## Summary

Adds audible session feedback: a chime when recording starts and when recognition processing begins (plus optional error chimes), with a new "提示音" (Sound) section in the audio settings page offering a master switch, per-event sound pickers, and instant preview ("试听即所得") buttons.

- macOS plays the user-selected named sound via `NSSound` on the main thread (no bundled audio needed); if the system sound is unavailable it falls back to a small bundled CC0 WAV set.
- Windows (and the fallback path) plays bundled CC0 WAVs via `rodio` in a dedicated audio thread; picks without a same-name fallback WAV stay silent (documented limitation).
- Sounds never block or delay recognition/paste: playback is fire-and-forget on background threads, and any failure is only logged.
- Volume ducking interplay: the start chime plays **before** ducking engages (ducking is delayed until the chime finishes, reusing the existing `Msg::VolumeDucking` handler and its `enabled && self.recording` gate); post-recording chimes (processing/error) wait for the volume-restore window (~450 ms, the ducker's max fade) so they aren't masked by the fade-in. A per-session generation counter drops stale queued chimes when sessions restart quickly.
- Defaults: start = `Tink`, processing = `Pop`, done/error = silent (matches cometix-dictation's visual-only error style). The "识别完成" (done) row exists for parity but is intentionally not wired to any trigger — recognition is fast and the processing chime already marks completion.
- New `settings.json` keys (all optional, serde defaults): `soundEnabled`, `soundStart`, `soundProcessing`, `soundDone`, `soundError`. Unknown ids are sanitized back to per-event defaults; `none` is an explicit valid value.

## Files

- **New**: `app/src-tauri/src/sound.rs` (module: gating, NSSound main-thread playback with per-name caching, rodio WAV fallback thread, duration estimation, tests), `app/src-tauri/resources/sounds/` (10 CC0 WAVs + `LICENSE.md`)
- **Modified**: `Cargo.toml` (+rodio, +objc2-app-kit/objc2-foundation on macOS), `Cargo.lock`, `tauri.conf.json` (bundle `resources/sounds/*` only), `main.rs` (`mod sound`, `preview_sound`/`list_sounds` commands), `settings.rs` (fields/defaults/sanitize/tests), `controller.rs` (trigger points + restore-wait + generation latch), `app/src/lib/api.ts`, `app/src/pages/Audio.svelte` (settings section)

## Testing

- `cargo check -p voicekey` — 0 warnings
- `cargo test --workspace` — all pass (voicekey 23, voicekey-core 51, voicekey-platform 14, voicekey-wtlocal 1)
- `npm run build` (vite) — pass
- Diff reviewed hunk-by-hunk against `upstream/main` — no unrelated changes

## Note (pre-existing, unrelated)

On rustc ≥ 1.97 stable, `crates/wtlocal/src/kernels.rs` fails to build (`vdotq_s32` is still gated behind unstable `stdarch_neon_dotprod`, tracking #117224; local `cargo test` hits E0658). This PR does not touch `crates/`; verification above was run on nightly. Happy to split that fix into a separate PR if useful.
