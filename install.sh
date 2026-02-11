#!/bin/bash
npm run tauri build

# Install I4ArchSettings
sudo install -Dm755 \
src-tauri/target/release/I4ArchSettings \
/usr/bin/i4archsettings

# Copy to config directory
cp src-tauri/target/release/I4ArchSettings ~/.config/IArchDE/apps/i4archsettings