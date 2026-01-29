# I4ArchSettings

**A powerful, native settings manager for Arch Linux (Hyprland), built with Tauri and Vue 3.**

I4ArchSettings bridges the gap between the terminal-heavy configuration of Hyprland and the convenience of a modern GUI. It allows users to manage hardware, networks, and system aesthetics effortlessly without manually editing `.conf` files.

> **Note:** This project was developed with significant assistance from Artificial Intelligence (~60% of the code was generated or refactored by AI). BECAUSE I AM A NOOB AND I DONT KNOW ABOUT RUST :)))))

## Key Features

### Network & Connectivity
- **Wi-Fi**: Scan, connect to known networks, and manage static IP/DNS configurations.
- **VPN**: Import and manage OpenVPN/WireGuard connections via `nmcli`.
- **Bluetooth**: Full device lifecycle management—scan, pair, connect, and toggle power.

### Display & Graphics
- **Monitor Management**: Drag-and-drop interface to position monitors.
- **Resolution & Scaling**: Adjust resolution, refresh rate, and UI scaling per display.
- **Workspace Binding**: Assign active workspaces to specific monitors.

### Personalization
- **Theming**: One-click switch between Light/Dark system modes (GTK).
- **Appearance**: Select GTK themes, Cursor themes, and customize Hyprland aesthetics (borders, gaps, blur, opacity).
- **Wallpaper**: Integrated wallpaper picker utilizing `swww` for smooth transitions.
- **Waybar**: Customize the position (top/bottom/left/right) of your status bar.

### System Configuration
- **Keybindings**: View and add Hyprland keyboard shortcuts.
- **Startup Apps**: Manage `exec-once` startup scripts easily.
- **Environment Variables**: Graphical editor for `env.conf`.
- **System Info**: Real-time overview of CPU, Memory, GPU, and OS version.

## Technology Stack

- **Frontend**: 
  - **Framework**: Vue 3 (Composition API)
  - **Language**: TypeScript
  - **Styling**: Tailwind CSS & PrimeVue Components
  - **Architecture**: MVVM (Model-View-ViewModel)

- **Backend**: 
  - **Core**: Rust (Tauri v2)
  - **System Interaction**: `process::Command` bindings for `nmcli`, `hyprctl`, `bluetoothctl`, etc.

## Installation & Setup

### Prerequisites
- **OS**: Arch Linux (or distro running Hyprland)
- **Dependencies**: 
  - `npm` or `pnpm`
  - `rust` & `cargo`
  - System tools: `nmcli`, `hyprctl`, `swww`, `gsettings`

### Build from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/i4archsettings.git
   cd i4archsettings
   ```

2. **Install dependencies**
   ```bash
   npm install
   # or
   pnpm install
   ```

3. **Run in Development Mode**
   ```bash
   npm run tauri dev
   ```

4. **Build Release Bundle**
   ```bash
   npm run tauri build
   ```

## Project Structure

```
├── src
│   ├── components   # Reusable UI components
│   ├── viewmodels   # State management & business logic (TypeScript)
│   └── views        # Vue Page components
├── src-tauri
│   └── src
│       └── modules  # Rust backend implementations (wifi, vpn, display, etc.)
└── public           # Static assets
```

## Contribution

Contributions are welcome! Please ensure you follow the existing MVVM pattern on the frontend and keep Rust modules atomic and focused on specific system domains.

## License

This project is open-source and available under the MIT License.
