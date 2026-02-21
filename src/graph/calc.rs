#[must_use]
pub fn calculate_zoom_delta(delta: f32, current_zoom: f32) -> f32 {
    let new_zoom = current_zoom * (1.0 + delta);
    new_zoom.clamp(0.1, 5.0)
}

#[must_use]
pub fn calculate_pan_offset(
    viewport_x: f32,
    viewport_y: f32,
    center_x: f32,
    center_y: f32,
    old_zoom: f32,
    new_zoom: f32,
) -> (f32, f32) {
    let factor = new_zoom / old_zoom;
    let new_x = (center_x - viewport_x).mul_add(-factor, center_x);
    let new_y = (center_y - viewport_y).mul_add(-factor, center_y);
    (new_x, new_y)
}

#[must_use]
pub fn calculate_fit_view(
    nodes: &[(f32, f32)],
    viewport_width: f32,
    viewport_height: f32,
    padding: f32,
) -> Option<(f32, f32, f32)> {
    if nodes.is_empty() {
        return None;
    }

    let (min_x, min_y, max_x, max_y) = nodes.iter().fold(
        (
            f32::INFINITY,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NEG_INFINITY,
        ),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            let right = x + 220.0;
            let bottom = y + 68.0;
            (
                min_x.min(x),
                min_y.min(y),
                max_x.max(right),
                max_y.max(bottom),
            )
        },
    );

    let width = max_x - min_x;
    let height = max_y - min_y;

    let scale_x = (viewport_width - padding) / width.max(1.0);
    let scale_y = (viewport_height - padding) / height.max(1.0);
    let zoom = scale_x.min(scale_y).clamp(0.15, 1.5);

    let center_x = f32::midpoint(min_x, max_x);
    let center_y = f32::midpoint(min_y, max_y);

    let viewport_x = center_x.mul_add(-zoom, viewport_width / 2.0);
    let viewport_y = center_y.mul_add(-zoom, viewport_height / 2.0);

    Some((viewport_x, viewport_y, zoom))
}

#[must_use]
pub fn find_safe_position(
    existing_positions: &[(f32, f32)],
    desired_x: f32,
    desired_y: f32,
    step: f32,
) -> (f32, f32) {
    let mut current_x = desired_x;
    let mut current_y = desired_y;

    while existing_positions
        .iter()
        .any(|&(ex, ey)| (ex - current_x).abs() < 10.0 && (ey - current_y).abs() < 10.0)
    {
        current_x += step;
        current_y += step;
    }

    (current_x, current_y)
}

#[must_use]
pub fn update_node_position(current_x: f32, current_y: f32, dx: f32, dy: f32) -> (f32, f32) {
    // Safety check: if any value is NaN or infinite, don't update
    if !dx.is_finite() || !dy.is_finite() || !current_x.is_finite() || !current_y.is_finite() {
        return (current_x, current_y);
    }

    let new_x = (current_x + dx / 10.0).round() * 10.0;
    let new_y = (current_y + dy / 10.0).round() * 10.0;

    // Additional safety: clamp to reasonable bounds
    let new_x = new_x.clamp(-100_000.0, 100_000.0);
    let new_y = new_y.clamp(-100_000.0, 100_000.0);

    (new_x, new_y)
}

#[must_use]
pub const fn calculate_rect_center(rect: (f32, f32, f32, f32)) -> (f32, f32) {
    let (min_x, min_y, max_x, max_y) = rect;
    (f32::midpoint(min_x, max_x), f32::midpoint(min_y, max_y))
}

#[must_use]
pub fn calculate_rect_size(rect: (f32, f32, f32, f32)) -> (f32, f32) {
    let (min_x, min_y, max_x, max_y) = rect;
    (max_x - min_x, max_y - min_y)
}
