//! The blueprint behind the app list: a blue gradient with a fine drafting grid and a heavier
//! line every [`MAJOR_EVERY`] cells, in DraftCraft's blueprint colours (its app icon's tile:
//! `#0e3463` to `#081d38`, lines `#9cc3ff`). The grid drifts slowly unless the OS asks for
//! fewer animations; then it stands still.

use std::time::Duration;

use egui::{Color32, Mesh, Painter, Pos2, Rect, Shape, Stroke, pos2};

use crate::theme::Tokens;

/// One grid cell, in points.
const CELL: f32 = 24.0;
/// A major line every this many cells.
const MAJOR_EVERY: u32 = 4;
/// How fast the grid drifts, in points a second (down and to the right).
const DRIFT: f32 = 6.0;
/// The drift's frame interval: smooth enough for 6 pt/s, and a scheduled (not immediate)
/// repaint, so the window still counts as idle between frames.
const FRAME: Duration = Duration::from_millis(33);
/// Never draw more lines than this per direction, whatever size the window claims.
const MAX_LINES: u32 = 400;

struct Palette {
    top: Color32,
    bottom: Color32,
    minor: Color32,
    major: Color32,
}

fn palette(t: &Tokens) -> Palette {
    if t.dark {
        Palette {
            top: Color32::from_rgb(0x0e, 0x34, 0x63),
            bottom: Color32::from_rgb(0x08, 0x1d, 0x38),
            minor: Color32::from_rgba_unmultiplied(0x9c, 0xc3, 0xff, 26),
            major: Color32::from_rgba_unmultiplied(0x9c, 0xc3, 0xff, 56),
        }
    } else {
        // Blueprint paper for the light theme: pale blue, with blue lines.
        Palette {
            top: Color32::from_rgb(0xee, 0xf3, 0xfa),
            bottom: Color32::from_rgb(0xdc, 0xe6, 0xf3),
            minor: Color32::from_rgba_unmultiplied(0x2b, 0x5f, 0xa8, 22),
            major: Color32::from_rgba_unmultiplied(0x2b, 0x5f, 0xa8, 48),
        }
    }
}

/// Paints the blueprint into `rect`. `still` stops the drift (the OS's reduce-motion setting).
pub(crate) fn paint(ctx: &egui::Context, painter: &Painter, rect: Rect, t: &Tokens, still: bool) {
    if !rect.is_positive() || !rect.is_finite() {
        return;
    }
    let p = palette(t);

    // The gradient, top to bottom.
    let mut mesh = Mesh::default();
    for (pos, color) in [(rect.left_top(), p.top), (rect.right_top(), p.top), (rect.right_bottom(), p.bottom), (rect.left_bottom(), p.bottom)] {
        mesh.colored_vertex(pos, color);
    }
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(Shape::mesh(mesh));

    // The drift wraps every major cell, so the pattern is seamless.
    let period = CELL * MAJOR_EVERY as f32;
    let offset = if still {
        0.0
    } else {
        ctx.request_repaint_after(FRAME);
        let time = ctx.input(|i| i.time);
        (time as f32 * DRIFT).rem_euclid(period)
    };
    let painter = painter.with_clip_rect(rect);
    let mut shapes = Vec::new();
    // Start one major cell before the rect, so the first line is a major one and lines enter
    // from the edge as the grid drifts.
    for (along_x, start, end) in [(true, rect.left(), rect.right()), (false, rect.top(), rect.bottom())] {
        let first = start - period + offset;
        for i in 0..MAX_LINES {
            let at = first + i as f32 * CELL;
            if at > end {
                break;
            }
            let color = if i % MAJOR_EVERY == 0 { p.major } else { p.minor };
            let stroke = Stroke::new(1.0, color);
            let (a, b): (Pos2, Pos2) =
                if along_x { (pos2(at, rect.top()), pos2(at, rect.bottom())) } else { (pos2(rect.left(), at), pos2(rect.right(), at)) };
            shapes.push(Shape::line_segment([a, b], stroke));
        }
    }
    painter.extend(shapes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_or_infinite_rect_paints_nothing_and_does_not_panic() {
        let ctx = egui::Context::default();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            let t = Tokens::get(ui.ctx());
            let painter = ui.painter().clone();
            paint(ui.ctx(), &painter, Rect::NOTHING, &t, false);
            paint(ui.ctx(), &painter, Rect::EVERYTHING, &t, false);
            paint(ui.ctx(), &painter, Rect::from_min_size(Pos2::ZERO, egui::vec2(1e9, 1e9)), &t, false);
        });
        out.textures_delta.clear();
    }
}
