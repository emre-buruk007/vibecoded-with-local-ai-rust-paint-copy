# Paint App - Action Plan

## Current Status: v1.0.3 - Stability Fixes Complete

### Recent Fixes (v1.0.3)
- [x] **Root Cause**: `set_pixel_unchecked` allows negative/overflow coordinates → panic on edge clicks
- [x] **Fix 1**: Brush circle clamping — replaced `set_pixel_unchecked` with safe `set_pixel` (clamps to canvas bounds) in Pencil/Eraser PRESS handler
- [x] **Fix 2**: Rapid-click cancellation — cancel existing `DrawState` before starting a new one (prevents undo/history corruption)
- [x] **Fix 3**: All shape functions use safe `set_pixel` — replaced all 22 `set_pixel_unchecked` calls in shapes.rs (draw_line, draw_dashed_line, draw_dashed_ellipse, draw_ellipse, fill_ellipse, fill_rect, flood_fill)
- [x] **Fix 4**: RELEASE handler guard — skip shape finalization if canvas dimensions changed (e.g., after resize)

### Next Up (v1.1)
- [ ] Complete Fill tool integration (tolerance slider in toolbar)
- [ ] Eyedropper canvas sampling (requires texture readback)
- [ ] Redo support (double-stack undo system)
- [ ] Zoom/Pan controls
- [ ] Multi-layer support

## Plan & Act Workflow

## Phase 1: When in PLAN MODE (Read-Only)
- Read `CURRENT_AI_ACTION_PLAN.md` if it exists in the project root.
- Present a complete plan in your chat response covering:
  1. **Review of Past Plan:** What worked well, what didn't work, and how this plan remedies past issues.
  2. **Next Steps & Feature Scope:** Exact features or fixes to build.
  3. **Implementation Plan:** Files to create/modify and technical approach.
  4. **Testing & Expected Output:** Exact test commands to run and expected results.

## Phase 2: When in ACT MODE (Execution)
Follow these steps in strict order:

1. **Write Initial Plan First:** Before editing any code files, write or overwrite `CURRENT_AI_ACTION_PLAN.md` in the root directory using the approved plan from Phase 1. Include a placeholder section at the bottom titled `## 5. Execution Results (To be filled before commit)`.
2. **Implement & Test:** Build the feature and run the tests listed in Section 4.
3. **Update Plan Before Commit:** Before running `git commit`, update `CURRENT_AI_ACTION_PLAN.md`—fill in Section 5 with what actually worked, any errors encountered, and whether actual outputs matched expected outputs. Make it easy to read, you are allowed to use tickboxes for working tests and red crosses or other appropriate emojis for failed stuff for visual engagement.
4. **Git Commit:** Follow the Git Commit rule to stage and commit all code changes along with `CURRENT_AI_ACTION_PLAN.md`.
