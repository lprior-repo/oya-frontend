pub const NODE_WIDTH: f32 = 220.0;
pub const NODE_HEIGHT: f32 = 68.0;
pub type SelectionRect = (f32, f32, f32, f32);

#[must_use]
pub fn normalize_rect(start: (f32, f32), end: (f32, f32)) -> SelectionRect {
    let min_x = start.0.min(end.0);
    let min_y = start.1.min(end.1);
    let max_x = start.0.max(end.0);
    let max_y = start.1.max(end.1);
    (min_x, min_y, max_x, max_y)
}

#[must_use]
pub fn rect_contains(rect: SelectionRect, point: (f32, f32)) -> bool {
    point.0 >= rect.0 && point.0 <= rect.2 && point.1 >= rect.1 && point.1 <= rect.3
}

#[must_use]
pub fn node_intersects_rect(node_x: f32, node_y: f32, rect: SelectionRect) -> bool {
    let node_left = node_x;
    let node_top = node_y;
    let node_right = node_x + NODE_WIDTH;
    let node_bottom = node_y + NODE_HEIGHT;

    !(node_right < rect.0 || node_left > rect.2 || node_bottom < rect.1 || node_top > rect.3)
}

#[must_use]
pub fn snap_handle(
    nodes: &[oya_frontend::graph::Node],
    mx: f32,
    my: f32,
    viewport: &oya_frontend::graph::Viewport,
) -> Option<(
    oya_frontend::graph::NodeId,
    String,
    crate::ui::edges::Position,
)> {
    if !viewport.zoom.is_finite() || viewport.zoom.abs() <= f32::EPSILON {
        return None;
    }

    let canvas_x = (mx - viewport.x) / viewport.zoom;
    let canvas_y = (my - viewport.y) / viewport.zoom;

    const SNAP_RADIUS: f32 = 24.0;
    let radius_sq = SNAP_RADIUS * SNAP_RADIUS;

    let mut best: Option<(
        oya_frontend::graph::NodeId,
        String,
        crate::ui::edges::Position,
        f32,
    )> = None;

    for node in nodes {
        let handle_x = node.x + NODE_WIDTH / 2.0;
        let candidates = [
            (
                "target",
                crate::ui::edges::Position {
                    x: handle_x,
                    y: node.y,
                },
            ),
            (
                "source",
                crate::ui::edges::Position {
                    x: handle_x,
                    y: node.y + NODE_HEIGHT,
                },
            ),
        ];

        for (kind, pos) in candidates {
            let dx = canvas_x - pos.x;
            let dy = canvas_y - pos.y;
            let dist_sq = dx.mul_add(dx, dy * dy);

            if dist_sq > radius_sq {
                continue;
            }

            let should_replace = match &best {
                Some((_, _, _, best_dist)) => dist_sq < *best_dist,
                None => true,
            };
            if should_replace {
                best = Some((node.id, kind.to_string(), pos, dist_sq));
            }
        }
    }

    best.map(|(node_id, handle_kind, position, _)| (node_id, handle_kind, position))
}
