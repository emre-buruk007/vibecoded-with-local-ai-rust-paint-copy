# rust-paint

> A pixel-perfect drawing and painting application built in Rust with egui.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Screenshots

> *(Add screenshots here once available)*

## Features

| Category | Status | Details |
|---|---|---|
| **Canvas** | Done | 1024×768 RGBA canvas, scales to fit window |
| **Pencil** | Done | Freehand drawing with adjustable brush (1–50px) |
| **Eraser** | Done | Erases pixels to white |
| **Line** | Done | Click-and-drag straight lines with live preview |
| **Rectangle** | Done | Outline or filled rectangles |
| **Ellipse** | Done | Outline or filled ellipses |
| **Fill** | Done | Flood fill with configurable color tolerance |
| **Eyedropper** | Done | Pick colors from the palette |
| **Undo** | Done | 20-step undo history |
| **Save / Open** | Done | Save as PNG; open PNG and JPG files |
| **Dark Theme** | Done | Custom-styled dark UI |

### Planned
- Redo (double-stack undo)
- Zoom / Pan
- Layers panel
- Text tool, gradient fill
- Preferences dialog

## Installation & Running

### Prerequisites

- **[Rust](https://www.rust-lang.org/tools/install)** 1.70 or later (installed via [rustup](https://rustup.rs/))
- A compatible OS: **Windows**, **macOS**, or **Linux** (Wayland / X11)

### Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/emre-buruk007/vibecoded-with-local-ai-rust-paint-copy.git
cd rust-paint

# 2. Run in debug mode (fast compilation, with assertions)
cargo run

# 3. Run in release mode (optimized, faster rendering)
cargo run --release
```

### Build Only (no run)

```bash
cargo build --release
```

The binary will be at:

| Platform | Path |
|---|---|
| Windows | `target\release\rust-paint.exe` |
| macOS / Linux | `target/release/rust-paint` |

### Troubleshooting

| Problem | Solution |
|---|---|
| `rustc: command not found` | Install Rust via [rustup.rs](https://rustup.rs/) and restart your terminal |
| `glutin` / windowing errors on Linux | Install OpenGL/Wayland libs: `sudo apt install libxcb-dev libx11-dev libwayland-dev libx11-xcb-dev` |
| Slow initial build | Be patient — the first `cargo build` compiles all dependencies (~2–5 min) |

## Usage Guide

### Toolbar (Left Panel)

| Key | Tool | How to Use |
|-----|------|------------|
| `1` | **Pencil** | Click and drag to draw freehand lines |
| `2` | **Line** | Click start point, drag, release at end point |
| `3` | **Rectangle** | Click and drag to define the rectangle |
| `4` | **Ellipse** | Click and drag to define the bounding box |
| `5` | **Fill** | Click once on a region to flood-fill it |
| `6` | **Eyedropper** | Click a color swatch to select it |
| `7` | **Eraser** | Click and drag to erase (paints white) |

### Shape Fill Mode
When **Rectangle** or **Ellipse** is selected, a **Fill** checkbox appears:
- **Checked** → shape is filled with the current color
- **Unchecked** → only the outline is drawn

### Color Palette
- 12 preset color swatches in the toolbar
- Custom color picker at the bottom for full **RGBA** control
- Eyedropper picks from the palette colors

### File Operations

| Action | Menu Path |
|--------|-----------|
| New Canvas | **File → New Canvas** |
| Clear Canvas | **File → Clear Canvas** |
| Save as PNG | **File → Save as PNG** |
| Open Image | **File → Open** (PNG, JPG) |
| Undo (20 steps) | **Edit → Undo** |

## Project Structure

```
rust-paint/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library root & re-exports
│   ├── app.rs           # Main app state, UI logic, event handling
│   ├── canvas.rs        # Canvas buffer (1024×768 RGBA8888)
│   ├── tool.rs          # Tool enum & selection
│   ├── draw.rs          # Drawing state tracking
│   ├── shapes.rs        # Drawing primitives (Bresenham line, rect, ellipse, flood fill)
│   ├── history.rs       # Undo stack (20 steps)
│   └── ui/
│       ├── mod.rs       # UI module
│       └── toolbar.rs   # Toolbar palette constants
├── Cargo.toml
├── README.md
└── .clinerules/
    └── action-plan.md   # Development plan & tracking
```

## Technical Details

| Aspect | Detail |
|--------|--------|
| Canvas resolution | 1024 × 768 px, RGBA8888 |
| Rendering | egui texture system, auto-scaled to window |
| Line algorithm | Bresenham (crisp pixel-perfect lines) |
| Fill algorithm | Stack-based BFS with configurable tolerance |
| UI framework | egui via eframe (immediate-mode GUI) |
| Theme | Custom dark theme with styled widgets |

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `eframe` | 0.29 | egui application framework |
| `image` | 0.25 | Image loading and saving |
| `serde` | 1.0 | Serialization (future features) |
| `rfd` | 0.15 | Native file dialogs |

## Version History

| Version | Date | Changes |
|---------|------|---------|
| **v1.0.3** | 2026-11-11 | **Stability fixes** — replaced all `set_pixel_unchecked` with safe `set_pixel`, added rapid-click cancellation, RELEASE guard for canvas resize safety |
| v1.0.1 | — | Fixed menu flashing, shape preview overlap, fill toggle duplication, deprecated hover_pos |
| v1.0.0 | — | Initial release with basic drawing tools |

## License

MIT License — see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.