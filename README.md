# MacGlass Widgets v2

Updated Tauri 2 + React + TypeScript starter.

## Features
- Removes the bottom-right Customize button; widget gallery is opened with right-click on empty desktop or the small plus button.
- Ahmedabad is the default weather location.
- Cleaner macOS-inspired frosted glass; no repeated widget title/icon header.
- Clock, calendar, weather, photos, system monitor and battery widget catalog.
- Right-click a widget for sizes (2x2, 2x4, 4x2, 4x4), swap position, remove, or gallery.
- Drag and drop one widget onto another to swap their order. Grid auto-packs widgets to avoid overlap.
- Photo widget opens native file picker; multiple photo widget instances can be added.
- CPU, memory and aggregate disk usage use `sysinfo`; refresh every 2.5 seconds.
- Weather is fetched from Open-Meteo; internet required.
- Autostart is enabled at sign-in.

## Run
Requirements: Node.js LTS, Rust stable, Microsoft C++ Build Tools (MSVC), WebView2.

```powershell
npm install
npm run tauri dev
npm run tauri build
```

## Honest limitations
- This is a transparent app window prototype, not yet a true wallpaper-level Windows desktop child. It may cover desktop icons or other windows depending on focus. Implement and test native Explorer WorkerW/desktop integration before using it as a permanent desktop layer.
- GPU utilization is `N/A` because portable sysinfo does not provide reliable GPU load. A Windows performance counter/vendor-specific provider is still required.
- Battery is currently unavailable; implement Win32 `GetSystemPowerStatus` for real status.
- Native photo paths use Tauri's asset URL conversion; verify asset protocol permissions on the target Windows build.
- Grid swaps reorder and auto-pack widgets; they don't exchange exact cells when sizes differ. The layout avoids overlap.
- Not compiled or tested on the target PC in this environment.
