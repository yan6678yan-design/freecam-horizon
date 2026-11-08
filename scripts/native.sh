#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
revision=32abdd651c045ca2a6aab40c4560152507c07d67
cache="${FREECAM_BUILD_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/freecam-horizon}"
tools="$cache/horizon-mods"
if [ ! -d "$tools/.git" ]; then
  mkdir -p "$cache"
  git clone https://github.com/horizon-rust/horizon-mods.git "$tools"
fi
git -C "$tools" fetch origin "$revision"
git -C "$tools" checkout --detach "$revision"
python3 "$tools/native/tools/native.py" "${1:-check}" "$root" "${@:2}"
if [ "${1:-check}" = package ]; then
  mkdir -p "$root/dist"
  cp "$tools/native/target/freecam_yannis-1.0.1.hnmod" "$root/dist/"
fi
