use std::time::{Duration, Instant};

pub struct FpsCounter {
    last_instant: Instant,
    frame_count: u32,  // frames in current second window
    fps: u32,          // last measured FPS
    total_frames: u64, // total frames since start
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame_count: 0,
            fps: 0,
            total_frames: 0,
        }
    }

    /// Call once per frame
    pub fn tick(&mut self) {
        self.frame_count += 1;
        self.total_frames += 1;

        if self.last_instant.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }

    /// Render FPS + total frame count at top-right
    pub fn render(&self, buffer: &mut ratatui::buffer::Buffer) {
        let text = format!("FPS: {} | Frames: {}", self.fps, self.total_frames);

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
