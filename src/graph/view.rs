use super::layout::DagLayout;
use super::Workflow;
use crate::graph::calc;

impl Workflow {
    pub fn apply_layout(&mut self) {
        let layout = DagLayout::default();
        layout.apply(self);
    }

    pub fn zoom(&mut self, delta: f32, cx: f32, cy: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = calc::calculate_zoom_delta(delta, old_zoom);
        let (new_x, new_y) = calc::calculate_pan_offset(
            self.viewport.x,
            self.viewport.y,
            cx,
            cy,
            old_zoom,
            new_zoom,
        );
        self.viewport.x = new_x;
        self.viewport.y = new_y;
        self.viewport.zoom = new_zoom;
    }

    pub fn fit_view(&mut self, viewport_width: f32, viewport_height: f32, padding: f32) {
        let node_positions: Vec<(f32, f32)> = self.nodes.iter().map(|n| (n.x, n.y)).collect();

        if let Some((viewport_x, viewport_y, zoom)) =
            calc::calculate_fit_view(&node_positions, viewport_width, viewport_height, padding)
        {
            self.viewport.x = viewport_x;
            self.viewport.y = viewport_y;
            self.viewport.zoom = zoom;
        }
    }
}
