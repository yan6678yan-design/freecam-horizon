# Credits and provenance

FreeCam for Horizon is maintained by **Yannis_**. It is a new Rust implementation
of the free-camera, F6 follow-view and tripod behaviour of
[FreeCam by Yannis_](https://modrinth.com/mod/freecam_yannis_).
The icon is the existing artwork from that mod, supplied by its owner.

The Java mod originated from
[MinecraftFreecam/Freecam](https://github.com/MinecraftFreecam/Freecam), by its
[contributors](https://github.com/MinecraftFreecam/Freecam/graphs/contributors).
Thank you to those authors for the original Freecam project. Their MIT notice is
preserved in `LICENSE-UPSTREAM`. This is an independent port, not an official
release of MinecraftFreecam or Horizon.

The Rust camera code is newly written for this port. Horizon's public native-mod
API and documented bundle format are used to integrate it; Horizon is MIT licensed
and developed by [horizon-rust/horizon](https://github.com/horizon-rust/horizon).
Build tooling is obtained from `horizon-rust/horizon-mods` at a pinned commit,
not redistributed as our own work.

No Minecraft code or game assets are included. Minecraft is a trademark of Mojang
Synergies AB. Not an official Minecraft product; not approved by or associated
with Mojang or Microsoft.
