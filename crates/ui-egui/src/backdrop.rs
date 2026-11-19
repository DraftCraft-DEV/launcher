//! The blueprint behind the app list: a blue gradient with a fine drafting grid and a heavier
//! line every [`MAJOR_EVERY`] cells, in DraftCraft's blueprint colours (its app icon's tile:
//! `#0e3463` to `#081d38`, lines `#9cc3ff`). The colour pans from light to dark blue in a soft
//! diagonal wave, and the grid drifts slowly, unless the OS asks for fewer animations; then both
//! stand still.

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
/// How long the light-to-dark colour wave takes to pan once across the window, in seconds.
const PAN_SECONDS: f64 = 20.0;
/// The colour wave's mesh: this many columns and rows of vertices (a smooth gradient, cheaply).
const MESH_COLS: u32 = 24;
const MESH_ROWS: u32 = 16;
/// Never draw more lines than this per direction, whatever size the window claims.
const MAX_LINES: u32 = 400;

struct Palette {
    light: Color32,
    dark: Color32,
    minor: Color32,
    major: Color32,
}

fn palette(t: &Tokens) -> Palette {
    if t.dark {
        Palette {
            light: Color32::from_rgb(0x16, 0x4a, 0x86),
            dark: Color32::from_rgb(0x08, 0x1d, 0x38),
            minor: Color32::from_rgba_unmultiplied(0x9c, 0xc3, 0xff, 26),
            major: Color32::from_rgba_unmultiplied(0x9c, 0xc3, 0xff, 56),
        }
    } else {
        // Blueprint paper for the light theme: pale blue, with blue lines.
        Palette {
            light: Color32::from_rgb(0xf4, 0xf8, 0xfd),
            dark: Color32::from_rgb(0xcf, 0xdd, 0xef),
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

    // The colour: a diagonal wave from light to dark and back, panning across the window.
    let time = ctx.input(|i| i.time);
    let phase = if still { 0.0 } else { (time / PAN_SECONDS).rem_euclid(1.0) as f32 };
    let mut mesh = Mesh::default();
    for row in 0..=MESH_ROWS {
        for col in 0..=MESH_COLS {
            let (u, v) = (col as f32 / MESH_COLS as f32, row as f32 / MESH_ROWS as f32);
            // 0 at one corner, 1 at the opposite one; one full light-dark-light cycle across.
            let along = (u + v) / 2.0 - phase;
            // Cubed, so the light part is a narrow band and the window stays mostly dark.
            let k = (0.5 + 0.5 * (along * std::f32::consts::TAU).cos()).powi(3);
            let pos = pos2(rect.left() + u * rect.width(), rect.top() + v * rect.height());
            mesh.colored_vertex(pos, p.dark.lerp_to_gamma(p.light, k));
        }
    }
    let stride = MESH_COLS + 1;
    for row in 0..MESH_ROWS {
        for col in 0..MESH_COLS {
            let i = row * stride + col;
            mesh.add_triangle(i, i + 1, i + stride + 1);
            mesh.add_triangle(i, i + stride + 1, i + stride);
        }
    }
    painter.add(Shape::mesh(mesh));

    // The drift wraps every major cell, so the pattern is seamless.
    let period = CELL * MAJOR_EVERY as f32;
    let offset = if still {
        0.0
    } else {
        ctx.request_repaint_after(FRAME);
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
