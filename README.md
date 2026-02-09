# BDO Boss Timer Desktop Widget

A lightweight, native desktop widget for the Garmoth BDO Boss Timer with system tray integration.

## Prerequisites

### Windows
Install Rust from: https://www.rust-lang.org/tools/install
That's it! The Windows WebView2 is already installed on Windows 10/11.

### Linux
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

Then install Rust: https://www.rust-lang.org/tools/install

## Setup

1. Install dependencies:
```bash
npm install
```

2. Run in development mode:
```bash
npm run dev
```

## Building Executable

To create a standalone Windows executable:

```bash
npm run build
```

The executable will be in:
- Windows: `src-tauri/target/release/bdo-timer-widget.exe`
- Also creates an installer in: `src-tauri/target/release/bundle/`

### Build Notes

The built executable is:
- **Portable** - Just copy the .exe anywhere and run it
- **No extraction** - Runs immediately with no setup
- **Small** - Only 3-5MB
- **Fast** - Native binary, not JavaScript

## Usage

### Controls

- **Drag**: Click and drag anywhere on the window to move it
- **System Tray Icon**: Click the tray icon to show/hide
  - **Right-click** for menu:
    - **Always on Top**: Keep widget above all windows
    - Show/Hide window
    - Quit application

### Customization

Edit `index.html` to change:
- The Garmoth URL (currently set to NA server)
- Window appearance (colors, opacity, etc.)
- Button styles

Edit `src-tauri/tauri.conf.json` to change:
- Default window size
- Window behaviors

Edit `src-tauri/src/main.rs` for advanced features

## Replacing the Tray Icon

1. Replace files in `src-tauri/icons/`:
   - `icon.png` - Main icon (any size, will be scaled)
   - `icon.ico` - Windows tray icon (16x16 or 32x32 recommended)
   - `32x32.png`, `128x128.png`, `128x128@2x.png` - App icons

2. Rebuild the app

## Notes

- Window position is automatically saved in `settings.json`
- Always-on-top setting persists between sessions
- The window stays in the system tray when closed
- Click the tray icon to show/hide the window
- Settings stored in platform-specific location:
  - Windows: `%APPDATA%/com.bdotimer.widget/`
  - Linux: `~/.config/com.bdotimer.widget/`

## Troubleshooting

**"Rust not found" error:**
- Install Rust from https://www.rust-lang.org/tools/install
- Restart your terminal after installation

**Build errors:**
- Make sure you have all prerequisites installed
- Try `cargo clean` in the `src-tauri` folder
- Then run `npm run build` again

**Window doesn't show iframe:**
- Check your internet connection
- Verify the Garmoth URL is accessible
- Open DevTools (F12 in dev mode) to check for errors

**Reset saved position:**
- Delete the settings file:
  - Windows: `%APPDATA%/com.bdotimer.widget/settings.json`
  - Linux: `~/.config/com.bdotimer.widget/settings.json`

## Development vs Production

**Development mode** (`npm run dev`):
- Hot reload enabled
- DevTools available (F12)
- Faster iteration

**Production build** (`npm run build`):
- Optimized binary
- Smaller file size
- No dev dependencies
- Ready for distribution
