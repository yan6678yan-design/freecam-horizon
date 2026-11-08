<div align="center">
  <img src="assets/icon-original.png" alt="FreeCam icon" width="128" height="128">
  <h1>FreeCam for Horizon</h1>
  <p><strong>Leave the camera behind. Keep the world in view.</strong></p>
  <p>Free flight · F6 follow view · Nine tripod positions</p>
  <p>
    <a href="https://github.com/yan6678yan-design/freecam-horizon/releases">Download</a> ·
    <a href="https://modrinth.com/mod/freecam_yannis_">Original Java mod</a> ·
    <a href="https://github.com/yan6678yan-design/freecam-horizon/issues">Report an issue</a>
  </p>
  <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-6d90db">
  <img alt="Minecraft 26.3" src="https://img.shields.io/badge/Minecraft-26.3-79a861">
  <img alt="Native Rust mod" src="https://img.shields.io/badge/Horizon-native_Rust-ce986b">
</div>

## Your world, another perspective

The Horizon edition of **FreeCam by Yannis_**, rewritten in Rust for Horizon's
native mod system. Fly away from your player for a better view, or switch to the
high, wide **F6 follow camera** while continuing to move and play normally.

- **Free flight:** independent mouse look, horizontal movement, vertical flight
  and sprint boost. Movement and block interactions are suppressed while you
  control the free camera; your player's normal physics continue.
- **Follow view:** a smooth offset behind and above your player. Player controls
  remain active. Default distance **28 blocks**, height **18 blocks**.
- **Return where you left off:** F6 restores your previous free camera, or returns
  to the player if you entered follow view directly.
- **Nine tripods:** save and revisit camera positions for the current world.
- **Native settings:** rebind keys and adjust speed, follow distance and height
  in **Options → Mods → FreeCam**. Settings persist between launches.
- **Clean transitions:** camera state and tripods reset when the world unloads
  or the connection ends. Disabling the mod returns control to Horizon.

No obstruction hiding, block scanning or chunk rebuilding is performed. Camera
math uses bounded frame time and fixed-size state. Settings are read on activation
or in the settings screen, rather than from disk on every frame.

## Controls

| Control | Action |
| --- | --- |
| **F4** | Toggle free camera (on key release) |
| **F6** | Toggle follow view |
| **Movement keys** | Move the free camera horizontally |
| **Jump / Sneak** | Fly up / down |
| **Sprint** | 3× camera speed |
| **Hold F4 + 1–9** | First use saves that tripod; later uses return to it |
| **Options → Mods → FreeCam** | Rebind keys, change settings, clear tripods |

The camera uses your rebound movement keys. F4 is the default tripod modifier;
rebinding free camera changes that modifier too. Tripod positions are session-only.

## Install

Download **`freecam_yannis-1.0.0.hnmod`** from
[Releases](https://github.com/yan6678yan-design/freecam-horizon/releases).
This is a **Horizon native source bundle**, not a Fabric JAR. The Horizon launcher
compiles it into the game. It does not require Fabric, Sodium or Java.

This release targets Minecraft **26.3** and Horizon Nightly source commit
[`2ebf00a`](https://github.com/horizon-rust/horizon/commit/2ebf00a0281c592f79191e76fce0e7d522c6f425).
It uses camera API 402 and two internal hooks for independent mouse look and
rendering your own player. Those hooks require the pinned Horizon revision;
compatibility with other revisions is not promised.

Before registry approval, developers can build the source bundle locally using
`horizon-launcher native-build ./mixins/freecam_yannis --trust-native-code`.
After approval, players can install it from Horizon's Mods page.
See [Horizon's native-mod instructions](https://playhorizon.dev/docs/test-a-mod).

## Scope of this edition

The free camera moves through blocks and only shows chunks already loaded around
the player. It does not move the server-side player or request distant chunks.
Follow view keeps the ordinary player interactions; free flight blocks them.
Use it where the server permits camera mods.

This first Horizon edition ports the main F4/F6/tripod workflow. The Java edition's
optional collision, full-brightness, fog and hand-display settings are not included.
Horizon is a different renderer, so Fabric/Sodium compatibility does not apply.
The broken obstruction-hiding feature is deliberately absent.

For current test results and the remaining in-game checks, see
[VALIDATION.md](VALIDATION.md). A successful compile is not a claim of a complete
in-game or cross-platform performance test.

## Build and verify

Requires stable Rust, Git, Python 3.11+, rsync and Horizon's platform build tools.
The initial Horizon dependency build can take a while.

```sh
cargo test
cargo fmt --check
./scripts/native.sh check
./scripts/native.sh package
# Optional: check a newer Nightly before porting the two internal hooks.
./scripts/native.sh check --horizon latest
```

The script obtains the official build tool at a fixed commit and stores build
artifacts outside the repository. `package` writes the reproducible bundle into
`dist/`. `compatibility.json` and the mixin manifest pin matching client revisions.
Do not change those pins without checking the resulting game build.

## Publishing to Horizon

The public repository, MIT license, original-author credits, source bundle and
submission text are included. See [SUBMISSION.md](SUBMISSION.md) for the fields to
use at [Horizon submission](https://playhorizon.dev/mods/submit). Native mods need
Horizon review; a GitHub release alone does not create a registry listing.

## Credits

Created for **Yannis_**, using the existing FreeCam artwork. Based on the feature
set of [FreeCam by Yannis_](https://modrinth.com/mod/freecam_yannis_), whose Java
edition originated from [MinecraftFreecam/Freecam](https://github.com/MinecraftFreecam/Freecam)
and its contributors. This Rust port is independently maintained.

[MIT license](LICENSE) · [Original license](LICENSE-UPSTREAM) · [Full credits](NOTICE.md)

<sub>Not an official Minecraft product. Not approved by or associated with Mojang,
Microsoft, MinecraftFreecam or the Horizon project.</sub>
