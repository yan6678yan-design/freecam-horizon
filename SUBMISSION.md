# Horizon submission

Use https://playhorizon.dev/mods/submit and sign in with the owning GitHub account.
This repository is ready to link as the public source. Registry approval is a
separate step; this project does not claim to be listed or officially endorsed.

| Field | Value |
| --- | --- |
| Type | Native mod |
| ID | `freecam_yannis` |
| Name | FreeCam |
| Version | `1.0.0` |
| Minecraft version | `26.3` |
| Author | `Yannis_` |
| License | `MIT` |
| Source | `https://github.com/yan6678yan-design/freecam-horizon` |
| Issues | `https://github.com/yan6678yan-design/freecam-horizon/issues` |
| File | `freecam_yannis-1.0.0.hnmod` from this repository's release |
| Icon | `assets/icon.png` — square PNG, 256 × 256, under 256 KiB |
| Summary | Free camera, F6 follow view and nine tripod positions for Horizon. |

## Description to paste

**FreeCam by Yannis_ — now native to Horizon.**

Explore with a free camera, switch to a high, wide F6 follow view while controlling
your player, and save nine tripod positions for the current world.

- F4: toggle free camera. Use your movement keys, Jump/Sneak and Sprint to fly.
- F6: toggle follow view; toggle again to restore your previous camera.
- Hold F4 + 1–9: save or return to a tripod.
- Configure rebindable keys, flight speed, follow distance and height in Options → Mods.

This is a native Rust port of [FreeCam by Yannis_](https://modrinth.com/mod/freecam_yannis_).
The Java edition originated from [MinecraftFreecam/Freecam](https://github.com/MinecraftFreecam/Freecam)
and its contributors; original MIT notices and credits are retained in the source.
This port is independently maintained by Yannis_.

The bundle compiles into Horizon and targets Minecraft 26.3 and Horizon commit
`2ebf00a0281c592f79191e76fce0e7d522c6f425`. It uses two internal camera hooks and
therefore requires that pinned client revision. It only changes the client view
and free-camera input handling; it has no networking, telemetry, background threads,
block scans or obstruction hiding. Settings use Horizon's own local storage.
Chunks still load around the player. Free flight passes through blocks.

The Java edition's optional collision, full-brightness, fog and hand-display
settings are not part of this release. Unit/adapter tests and the full native
build check pass; in-game visual testing and performance measurements remain
pending. See the repository's validation report.

Not an official Minecraft product. Not approved by or associated with Mojang,
Microsoft, MinecraftFreecam or the Horizon project.

## Release notes

Initial Horizon Rust port: F4 free flight, F6 follow view with return-to-camera,
nine session tripod slots, persistent speed/distance/height settings and native
key rebinding. Clean world/connection resets. No obstruction-hiding feature.

## Maintainer notes

The existing icon is preserved as `assets/icon-original.png`; `assets/icon.png`
is a smaller rendition of that same artwork for Horizon's upload limit.
No signing key is created for this release. Optional minisign signing can be
introduced later with a key managed and backed up by the maintainer.
