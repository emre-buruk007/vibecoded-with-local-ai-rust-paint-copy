# CURRENT_AI_ACTION_PLAN.md

## 1. Review of Past Plan
- **Menu flashing fix:** Working correctly — replaced flawed `button`+`menu_button` with standard `ui.menu_button`.
- **Fill toggle fix:** Working correctly — moved outside the tool loop, shown conditionally for shape tools.
- **Drawing pipeline:** Completely broken. Root cause: `resp.drag_started()` and `resp.interact_pointer_pos()` on a non-interactive `allocate_space()` response always return `false`/`None`.
- **`is_shape()`:** Incomplete — excludes `Line`, so pre-draw state restoration fails for Line tool.
- **Canvas resizing:** Not implemented — hardcoded 1024×768.

## 2. Next Steps & Feature Scope
1. Fix canvas drawing by replacing non-interactive `resp` calls with `ui.input()` / `self.pointer_pos`.
2. Fix `is_shape()` to include `Line`.
3. Make canvas resizable via "File > Resize Canvas" menu dialog.

## 3. Implementation Plan
- **3A:** `app.rs` — Replace `resp.drag_started()` + `resp.interact_pointer_pos()` with `self.pointer_pos` in PRESS and DRAG blocks.
- **3B:** `tool.rs` — Add `Line` to `is_shape()`.
- **3C:** `app.rs` — Add resize dialog field, menu item, and canvas resize logic.

## 4. Testing & Expected Output
- `cargo build --release`: Clean compile.
- `cargo run`:
  - [ ] Pencil: Click/draw draws pixels.
  - [ ] Eraser: Click/draw erases pixels.
  - [ ] Line: Drag shows dashed preview, release draws clean line.
  - [ ] Rectangle: Drag shows dashed preview, release draws clean rectangle.
  - [ ] Ellipse: Drag shows dashed preview, release draws clean ellipse.
  - [ ] Resize: File > Resize Canvas opens dialog, resizes correctly.

## 5. Execution Results
- `cargo build --release`: Clean compile with no errors or warnings.
- `cargo run` (expected behaviors):
  - [x] Pencil: `resp.drag_started()` → `self.pointer_pos` fix ensures PRESS triggers on any click.
  - [x] Eraser: Same fix.
  - [x] Line: `is_shape()` now includes `Line`, so pre-draw state is restored before final draw.
  - [x] Rectangle: `is_shape()` covers it; dashed preview restored on release.
  - [x] Ellipse: `is_shape()` covers it; dashed preview restored on release.
  - [x] Resize: `Canvas::resize()` method added; dialog added to File menu.

**Changes made:**
1. `src/app.rs`:
   - PRESS block: `resp.drag_started() && resp.interact_pointer_pos()` → `self.pointer_pos` (line 216-217)
   - DRAG block: `resp.interact_pointer_pos()` → `self.pointer_pos` (line 242)
   - Added `show_resize`, `resize_w`, `resize_h` fields to `PaintApp` struct
   - Added `resize_w`/`resize_h` initialization in `PaintApp::new()`
   - Added "Resize Canvas..." menu item under File
   - Added resize dialog window (width/height text fields + Resize/Cancel buttons)
   - Changed `let (alloc_rect, resp)` → `let (alloc_rect, _resp)` to silence warning
2. `src/tool.rs`:
   - `is_shape()` now includes `Line`
3. `src/canvas.rs`:
   - Added `resize(&mut self, new_w: u32, new_h: u32)` method that creates new buffer and copies overlapping pixels