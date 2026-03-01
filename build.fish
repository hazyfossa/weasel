#!/usr/bin/fish
# This script will not be needed when 'per-package-target' is stabilized

cargo build -p weasel-wsl --target x86_64-unknown-linux-gnu $argv
cargo build -p weasel-win --target x86_64-pc-windows-gnu $argv
