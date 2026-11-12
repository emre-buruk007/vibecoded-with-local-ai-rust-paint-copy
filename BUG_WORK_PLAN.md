# Rust Paint — Bug Fix Work Plan

## Bugs Found

### Bug 1: Line & Rectangle Outline Draws on Canvas But Does Not Render
**Symptom:** Line and Rectangle outline tools draw on the canvas (pixels are written), but the result does not appear on screen.
**Root Cause:** `shape_event` in `src/app_events.rs` only calls `draw_shape_preview` (which renders to egui Painter overlay) and never calls `draw_shape_on_canvas`. The canvas pixels are written but never synced to the display texture — `canvas_dirty` flag is never set to `true` after shape drawing.
**Fix:** After `draw_shape_on_canvas` succeeds, set `self.canvas_dirty = true`.

### Bug 2: Ellipse Breaks the Program
**Symptom:** Ellipse tool crashes/hangs the application.
**Root Cause:** The midpoint ellipse algorithm in `draw_ellipse_preview` (`src/app_ui.rs` lines 214-239) has multiple bugs:
- **Wrong initial decision parameter** (line 224): uses `ry^3` instead of `ry^2`
- **Second loop doesn't update `px`/`py`** (line 233-238): increments `y` but never updates `px`, `py`, or `p` correctly — causes infinite loop or garbage rendering
- **Degenerate case**: `rx=0` or `ry=0` causes no drawing (no guard clause)
- **Algorithm structure** doesn't match the standard midpoint ellipse — the region transitions are incorrect

## Fix Plan

### Fix 1: Make Line/Rectangle Persist on Canvas
**File:** `src/app_events.rs`
- After `draw_shape_on_canvas` call in `shape_event`, set `self.canvas_dirty = true`
- Also update canvas texture if needed

### Fix 2: Rewrite Ellipse Drawing
**File:** `src/app_ui.rs` — `draw_ellipse_preview` method
- Rewrite using the correct midpoint ellipse algorithm
- Guard against `rx=0 || ry=0`
- Use proper decision parameter initialization
- Draw filled ellipse points with 8-way symmetry
- Also fix `draw_shape_on_canvas` ellipse case to use the same correct algorithm

### Fix 3: Make Ellipse Outline Render on Canvas
**File:** `src/app_events.rs` — `shape_event`
- Add ellipse case to call `draw_shape_on_canvas`
- Set `canvas_dirty = true`

## Files to Modify
1. `src/app_events.rs` — `shape_event` function: add canvas drawing + dirty flag
2. `src/app_ui.rs` — `draw_ellipse_preview`: rewrite ellipse algorithm
3. `src/shapes/ellipse.rs` (if it exists): fix `draw_ellipse` for canvas rendering
4. `src/shapes/shapes.rs` — `draw_shape_on_canvas`: ensure ellipse case calls correct function

## Implementation Order
1. Fix Bug 1 (line/rect canvas dirty flag) — simplest fix, quick win
2. Fix Bug 2 (rewrite ellipse algorithm in both preview and canvas drawing)
3. Verify all three shape tools: line, rectangle, ellipse
4. Test: draw shapes, verify they persist on canvas after mouse release
5. Test: verify canvas dirty flag propagation to texture upload

## Testing Checklist
- [ ] Line: draw → release → line persists on canvas
- [ ] Rectangle: draw → release → rectangle outline persists on canvas
- [ ] Ellipse: draw → release → ellipse persists on canvas (no crash/hang)
- [ ] Undo/redo works for shape operations
- [ ] Brush tool still works
- [ ] No memory leaks or performance regression
