# I4ArchSettings

A modern, comprehensive settings application for Arch Linux (Hyprland environment), built with Tauri and Vue 3.

> **Note:** This project was developed with significant assistance from Artificial Intelligence (~60% of the code was generated or refactored by AI). BECAUSE I AM A NOOB AND I DONT KNOW ABOUT RUST :)))))

## Overview

i4archsettings provides a user-friendly graphical interface to manage various system configurations that usually require editing configuration files or using terminal commands in a Hyprland environment.

## Features

- **Wi-Fi Management**: Scan, connect, and manage wireless networks.
- **Bluetooth**: Toggle power, scan for devices, and connect/disconnect.
- **Display Settings**: Drag-and-drop monitor arrangement, resolution, and positioning.
- **Appearance**: 
  - Change Wallpapers
  - Switch Light/Dark themes
  - Customize Hyprland window decorations (gaps, borders, opacity, blur)
  - Taskbar (Waybar) positioning
- **Keybinds**: View, add, and manage Hyprland keyboard shortcuts.
- **Startup Applications**: Manage applications that run on boot (exec-once).
- **System Information**: Overview of hardware and OS details.

## Technology Stack

- **Frontend**: Vue 3 (Composition API), TypeScript, PrimeVue, Tailwind (styles).
- **Backend**: Rust (Tauri).
- **Architecture**: MVVM pattern (Model-View-ViewModel) for Separation of Concerns.

## Getting Started

### Prerequisites

- Rust & Cargo
- Node.js & npm/pnpm
- Arch Linux with Hyprland (recommended target environment)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/i4archsettings.git
   cd i4archsettings
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## Project Structure

- `src/views`: UI Pages (Vue components).
- `src/viewmodels`: Business logic and state management.
- `src-tauri/src/modules`: Rust backend modules for system interaction.
