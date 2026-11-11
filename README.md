# rust-paint

A pixel-perfect drawing and painting application built in Rust with egui.

## Features

### Current Status
| Category | Status | Notes |
|----------|--------|-------|
| **Canvas Visibility** | Done | Canvas renders correctly using egui ColorImage texture |
| **Canvas Resizing** | Done | Scales to fit available window space while maintaining 4:3 aspect ratio |
| **Coordinate Mapping** | Done | Screen→canvas coordinate transformation for accurate mouse interaction |
| **Pencil Tool** | Done | Freehand drawing with live brush preview as user drags |
| **Eraser Tool** | Done | Erases pixels (white color) with live preview |
| **Line Tool** | Done | Dashed preview during drag, finalized on release |
| **Rectangle Tool** | Done | Dashed preview during drag, filled/outline on release |
| **Ellipse Tool** | Done | Dashed preview during drag, filled/outline on release |
| **Fill Tool** | Planned | Flood fill logic exists in shapes.rs but UI not wired up yet |
| **Eyedropper Tool** | Planned | Color picker logic exists but canvas sampling not implemented |
| **Save/Load** | In Progress | Save as PNG implemented; open image loading needs testing |
| **Undo/Redo** | In Progress | Basic undo stack in place; needs verification with new drawing logic |
| **Menu Bar** | Done | File and Edit menus with proper UI layout |
| **Dark Theme** | Done | Custom dark-themed interface throughout |

### Planned Features
- **Fill Tool**: Complete flood fill with UI integration and tolerance slider
- **Eyedropper Tool**: Sample colors from canvas display (requires pixel readback)
- **Redo Support**: Extend undo stack to support redo (currently only undo)
- **Zoom/Pan**: Add zoom controls and pan navigation for large canvases
- **Layers**: Multi-layer support with layer panel
- **More Tools**: Text tool, shape fill, gradient fill, selection tools
- **Preferences**: Configurable settings (default canvas size, undo steps, etc.)

### Known Limitations
- Fill tool not yet fully integrated with UI
- Eyedropper cannot sample from displayed canvas (texture readback limitation)
- No redo support (only single-stack undo)
- No zoom or pan controls (canvas scales to fit but cannot zoom in/out)

## Bugs Fixed (v1.0.1)

| Bug | Root Cause | Fix |
|-----|-----------|-----|
| **Menu buttons flashing** | `ui.button("File").clicked()` logic was flawed; `ui.menu_button` requires a different pattern | Replaced `if ui.button(...).clicked() { ui.menu_button(...) }` with standard `ui.menu_button(...)` calls |
| **No drawing with shape tools** | Usage of deprecated `hover_pos()` for pointer coordinates | Use `self.pointer_pos` (set in `update()`) as the single source of truth for pointer position |
| **Fill toggle created multiple checkboxes** | Checkbox was inside the `Tool::all()` loop, creating one checkbox per tool type | Moved "Fill" checkbox outside the tool loop; shown once when a shape tool is selected |
| **Shape preview + final shape rendered together** | Dashed preview drawn to canvas buffer during drag, then final shape drawn on top on release | Save canvas state on drag start; restore it on release before drawing the final shape |

## Features

- **Multiple Drawing Tools**: Pencil, Line, Rectangle, Ellipse, Fill (flood fill), Eyedropper (color picker), Eraser
- **Shape Filling**: Toggle fill mode for Rectangle and Ellipse tools
- **Color Palette**: 12 preset colors with custom color picker (full RGBA control)
- **Brush Size**: Adjustable brush size from 1px to 50px
- **Undo Support**: 20-step undo history
- **Save/Load**: Save paintings as PNG files, open existing images (PNG, JPG)
- **Dark Theme UI**: Custom-styled dark-themed interface

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- Cargo package manager

## Building

```bash
git clone https://github.com/yourusername/rust-paint.git
cd rust-paint
cargo build --release
```

## Running

```bash
# Debug build (with debug assertions)
cargo run

# Release build (optimized)
cargo run --release
```

## Usage

### Toolbar (Left Panel)

| Tool | Description |
|------|-------------|
| Pencil | Freehand drawing with adjustable brush size |
| Line | Draw straight lines (click and drag) |
| Rectangle | Draw rectangles (click and drag) |
| Ellipse | Draw ellipses (click and drag) |
| Fill | Flood fill areas with color (single click) |
| Eyedropper | Pick colors from the canvas (single click) |
| Eraser | Erase pixels (uses white color) |

### Drawing Shapes

- **Rectangle** and **Ellipse** tools have a "Fill" checkbox that appears when selected
- When Fill is checked: the shape is filled with the selected color
- When Fill is unchecked: only the shape outline is drawn

### Color Selection

- Click any of the 12 preset color swatches in the toolbar
- Click the custom color picker at the bottom of the palette for full RGBA control
- Use the Eyedropper tool to sample colors directly from the canvas

### File Operations

- **File > New Canvas**: Start with a blank 1024x768 canvas
- **File > Clear Canvas**: Clear the current canvas
- **File > Save as PNG**: Save your work as a PNG file
- **File > Open**: Load an image from disk (PNG, JPG, JPEG)
- **Edit > Undo**: Undo the last action (20 steps)

### Keyboard Shortcuts

Press `1-7` on the keyboard to quickly select tools:
- `1` - Pencil
- `2` - Line
- `3` - Rectangle
- `4` - Ellipse
- `5` - Fill
- `6` - Eyedropper
- `7` - Eraser

## Project Structure

```
src/
  main.rs       # Application entry point
  lib.rs        # Library root and re-exports
  app.rs        # Main PaintApp struct and UI logic
  canvas.rs     # Canvas buffer management (1024x768 RGBA)
  tool.rs       # Tool enum and tool selection logic
  draw.rs       # Drawing state tracking
  shapes.rs     # Drawing primitives (Bresenham line, rect, ellipse, flood fill)
  history.rs    # Undo/redo stack
  ui/
    mod.rs      # UI module exports
    toolbar.rs  # Toolbar palette constants
```

## Technical Details

- **Canvas Resolution**: 1024 x 768 pixels (RGBA8888)
- **Rendering**: egui texture system with automatic scaling to fit the window
- **Drawing Primitives**: Bresenham line algorithm for crisp pixel-perfect lines
- **Flood Fill**: Stack-based BFS algorithm with configurable color tolerance
- **UI Framework**: egui (Immediate Mode GUI) via eframe
- **Theme**: Custom dark theme with styled widgets, rounded corners, and consistent color palette

## Dependencies

| Crate | Version | Purpose |
|-------|---------|--------|
| eframe | 0.29 | egui application framework |
| image | 0.25 | Image loading and saving |
| serde | 1.0 | Serialization (for future features) |
| rfd | 0.15 | Native file dialog |

## License

MIT License
