# Installation and running

New Bearings is a desktop application. There is no installer and no published
release: you run it from source. Bundling is disabled in `tauri.conf.json`
(`bundle.active: false`), so no `.msi`, `.dmg`, `.deb` or AppImage is produced.

## 1. Prerequisites

| Requirement | Why |
| --- | --- |
| Rust stable (1.85 or newer) | The project uses the 2024 edition. |
| Node.js 20 or newer with npm | Vite, TypeScript and the Tauri CLI run on Node. |
| Git | To obtain the source. |
| Tauri 2 platform prerequisites | The window uses the platform WebView. |

Platform notes:

- **Windows 10/11** — WebView2 is normally already installed; otherwise install
  the Evergreen WebView2 Runtime. Compiling requires the "Desktop development
  with C++" workload from the Visual Studio Build Tools.
- **macOS** — install the Xcode command line tools (`xcode-select --install`).
- **Debian/Ubuntu**:

  ```bash
  sudo apt-get install build-essential pkg-config libssl-dev libgtk-3-dev \
    libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
  ```

On Windows, `./scripts/setup-dev.ps1` reports which of these tools are missing.

## 2. Get the source and dependencies

```bash
git clone https://github.com/Pandagamerteen08/New-Bearings.git
cd New-Bearings
npm --prefix frontend install
```

`npm install` also installs `@tauri-apps/cli`, which is how the application is
started. Cargo dependencies (`serde`, `serde_json`, `tauri`, `tauri-build`) are
downloaded on the first build.

## 3. Run the application

```bash
./frontend/node_modules/.bin/tauri.cmd dev      # Windows
./frontend/node_modules/.bin/tauri dev          # macOS / Linux
```

Or, on Windows, `./scripts/run.ps1`.

This starts Vite on `http://localhost:5173`, compiles the Rust binary, and opens
the window. Editing Vue files reloads the interface; editing Rust files rebuilds
and restarts the application.

Do not run `cargo run` on its own: in development the window loads from the Vite
dev server, which only the Tauri CLI starts.

## 4. Build a standalone executable

```bash
npm --prefix frontend run build     # writes frontend/dist
cargo build                         # embeds the built interface
```

The result is `target/debug/new-bearings.exe` on Windows (or
`target/debug/new-bearings` elsewhere). It starts without a dev server.

On Windows, `./scripts/build.ps1` runs both steps. For an optimised build, use
`cargo build --release` after building the frontend; it is not required and is
not packaged for distribution.

## 5. Your data

The application writes one file to the platform application data directory:

| Platform | Path |
| --- | --- |
| Windows | `%APPDATA%\com.newbearings.desktop\app-state.json` |
| macOS | `~/Library/Application Support/com.newbearings.desktop/app-state.json` |
| Linux | `~/.local/share/com.newbearings.desktop/app-state.json` |

Close the application before deleting that file if you want a clean start. A file
that cannot be read is kept as `app-state.recovered-<timestamp>.json` in the same
directory, so nothing is destroyed silently.

## 6. Removing the application

Delete the checkout and, if you want the learner data gone too, the application
data directory above. Nothing is installed elsewhere, no services are registered,
and no registry keys are written by this build.

## Troubleshooting

**The build cannot find an icon.** The icon files in `icons/` are required by
`tauri-build`, which embeds a window icon and needs a PNG on Linux and macOS. If
they are missing, regenerate them from the source artwork:

```bash
./frontend/node_modules/.bin/tauri.cmd icon resources/icons/app-icon.svg
```

**`error while loading shared libraries: libwebkit2gtk-4.1.so`.** Install the
Linux packages listed above.

**The window opens blank in development.** Check that Vite is running on port
5173; the Tauri CLI starts it, but a second copy of the application can conflict
on the port. Stop the other instance first.

**`cargo build` fails with a missing `frontend/dist`.** Run
`npm --prefix frontend run build` first; the configuration points the Rust build
at that directory.

**Progress is not saved.** The shared path is shown in the warning banner. The
application data directory must be writable by your user.
