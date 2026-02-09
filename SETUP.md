# Setup Instructions for BDO Timer Widget (Tauri)

## Windows Setup

### 1. Install Rust

1. Download Rust installer: https://www.rust-lang.org/tools/install
2. Run the installer
3. Choose option 1 (default installation)
4. Restart your terminal/command prompt

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. Install Node.js Dependencies

```bash
cd bdo-timer-tauri
npm install
```

### 3. Run the App

**Development mode** (with hot reload):
```bash
npm run dev
```

**Build production executable**:
```bash
npm run build
```

The executable will be at:
```
src-tauri\target\release\bdo-timer-widget.exe
```

Just copy this .exe file anywhere and run it - no installation needed!

## Linux Setup

### 1. Install System Dependencies

**Ubuntu/Debian:**
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

**Fedora:**
```bash
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Install Node.js Dependencies

```bash
cd bdo-timer-tauri
npm install
```

### 4. Run the App

```bash
npm run dev  # Development
npm run build  # Production
```

## File Sizes Comparison

**Electron version:**
- Development: ~200MB (node_modules)
- Built installer: ~150MB
- Installed: ~250MB
- First run: Extracts to temp folder

**Tauri version:**
- Development: ~500MB (includes Rust compiler cache)
- Built executable: **3-5MB** ✨
- No installation needed: **Just run the .exe**
- First run: **Instant** - no extraction

## Common Issues

### "rustc not found"
Solution: Install Rust from https://www.rust-lang.org/tools/install and restart terminal

### "WebView2 not found" (Windows)
Solution: Windows 10/11 includes WebView2 by default. If missing, download from:
https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### Build errors on Windows
Solution: Make sure you have Visual Studio Build Tools or Visual Studio installed with C++ development tools

### Linux: "webkit2gtk not found"
Solution: Install the system dependencies listed above for your distro

## Directory Structure

```
bdo-timer-tauri/
├── index.html          # Main HTML file
├── main.js             # Frontend JavaScript
├── package.json        # Node.js dependencies
├── src-tauri/          # Rust backend
│   ├── src/
│   │   └── main.rs     # Main Rust code
│   ├── icons/          # App icons
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
└── README.md
```

## Next Steps

After building:
1. Find your executable in `src-tauri/target/release/`
2. Copy `bdo-timer-widget.exe` anywhere you want
3. Run it - no installation needed!
4. The app will remember its position and settings

That's it! You now have a tiny, portable desktop widget.
