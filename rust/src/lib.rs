use std::time::{Duration, Instant};

pub struct FpsCounter {
    last_instant: Instant,
    frame_count: u32,
    fps: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }

    /// Call once per frame
    pub fn tick(&mut self) {
        self.frame_count += 1;

        if self.last_instant.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }

    /// Render FPS at top-right corner
    pub fn render(&self, buffer: &mut ratatui::buffer::Buffer) {
        let text = format!("FPS: {}", self.fps);

        let area = buffer.area();
        let x_start = area.width.saturating_sub(text.len() as u16);
        let y = 0;

        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buffer.cell_mut((x_start + i as u16, y)) {
                cell.set_symbol(&ch.to_string());
            }
        }
    }
}
