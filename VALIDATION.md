# Validation — 2026-11-08

## Environment

- macOS arm64, Rust 1.97.1, Minecraft target 26.3.
- Latest Nightly resolved using Horizon's official `latest_nightly()` tool:
  `2ebf00a0281c592f79191e76fce0e7d522c6f425` (native hooks API 411).
- Required camera API: 402. Internal hooks: `turn_player`, `own_body_in_view`.
- horizon-mods build tool: `32abdd651c045ca2a6aab40c4560152507c07d67`.
- horizon-mixins: `e8cd0aa38cffef49cb2e088ef716baa107148c02`.

## Passed — 1.0.1

- Crosshair cancellation is restricted to F6 follow mode. Tests cover exit to
  player/free camera, disabled mod, missing player, other HUD layers, and preserving
  another mod's cancellation.

- `cargo test`: eight camera/settings tests and five adapter scenarios.
  The eight core tests also run in the adapter binary: 21 executions, 13 distinct tests.
- 1,000 F6 round trips preserve the previous free camera, both at core level and
  through the actual injected keyboard helpers.
- Free mode intercepts look, movement and interactions; follow mode leaves normal
  look and movement in place. Disabled mods leave the camera/render hooks alone.
- Menu/server guards, disconnect/unload resets, tripod chords, invalid settings,
  frame-time bounds, normalized diagonal movement and follow geometry.
- `cargo fmt --check`.
- Official `native.py check` applies all 15 injections and compiles the full
  Horizon client with `RUSTFLAGS=-D warnings` on the pinned Nightly.

The adapter tests execute the delivered helper files against a minimal host;
the full build check separately validates their signatures and types against
actual Horizon source. Neither substitutes for rendering a live world.

- Official `cargo-horizon-mod check`: native bundle accepted; unsigned as documented.
- Packaged Rust/TOML sources and license match the repository byte for byte.
- Bundle SHA-256: `97b727405c8aeef6d7865701fc7fda4f52fe61549a47125e6b6244c7872e161c`.

## Remaining manual checks

This environment has not run the mod inside an interactive Horizon world.
Before promoting the first release from preview, test:

1. F4 flight, mouse look, rebound controls and sprint at low/high frame rates.
2. F6 while walking, sprinting, riding and swimming; repeated F4/F6 transitions.
3. All tripod slots, menus, pause/focus loss and disabling the mod while active.
4. Death/respawn, Nether/End transitions, disconnect/rejoin and multiplayer.
5. Player body visibility, camera clipping and compatibility with other camera mods.
6. Frame-time comparison against unmodified Horizon on the same scene/settings.

No measured FPS claim or universal crash-free guarantee is made. Other native
mods targeting the same camera functions may conflict. Windows/Linux builds
have not been run locally. The Java mod's earlier Sodium tests do not establish
Horizon compatibility.
