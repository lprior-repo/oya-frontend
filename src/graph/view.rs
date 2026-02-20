use super::layout::DagLayout;
use super::Workflow;

impl Workflow {
    pub fn apply_layout(&mut self) {
        let layout = DagLayout::default();
        layout.apply(self);
    }

    pub fn zoom(&mut self, delta: f32, cx: f32, cy: f32) {
        let old_zoom = self.viewport.zoom;
        let new_zoom = (old_zoom * (1.0 + delta)).clamp(0.1, 5.0);
        let factor = new_zoom / old_zoom;
        self.viewport.x = (cx - self.viewport.x).mul_add(-factor, cx);
        self.viewport.y = (cy - self.viewport.y).mul_add(-factor, cy);
        self.viewport.zoom = new_zoom;
    }

    pub fn fit_view(&mut self, viewport_width: f32, viewport_height: f32, padding: f32) {
        let bounds = self
            .nodes
            .iter()
            .fold(None::<(f32, f32, f32, f32)>, |acc, node| {
                let right = node.x + 220.0;
                let bottom = node.y + 68.0;
                match acc {
                    Some((min_x, min_y, max_x, max_y)) => Some((
                        min_x.min(node.x),
                        min_y.min(node.y),
                        max_x.max(right),
                        max_y.max(bottom),
                    )),
                    None => Some((node.x, node.y, right, bottom)),
                }
            });

        if let Some((min_x, min_y, max_x, max_y)) = bounds {
            let width = max_x - min_x;
            let height = max_y - min_y;
            let scale_x = (viewport_width - padding) / width.max(1.0_f32);
            let scale_y = (viewport_height - padding) / height.max(1.0_f32);
            let zoom = scale_x.min(scale_y).clamp(0.15, 1.5);
            let center_x = f32::midpoint(min_x, max_x);
            let center_y = f32::midpoint(min_y, max_y);

            self.viewport.zoom = zoom;
            self.viewport.x = viewport_width / 2.0 - center_x * zoom;
            self.viewport.y = viewport_height / 2.0 - center_y * zoom;
        }
    }
}
