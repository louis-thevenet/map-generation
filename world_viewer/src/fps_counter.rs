use std::time::Instant;

use color_eyre::Result;
use ratatui::{text::Span, widgets::Paragraph};

#[derive(Debug, Clone, PartialEq)]
pub struct FpsCounter {
    enabled: bool,
    last_tick_update: Instant,
    tick_count: u32,
    ticks_per_second: f64,

    last_frame_update: Instant,
    frame_count: u32,
    frames_per_second: f64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsCounter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_tick_update: Instant::now(),
            tick_count: 0,
            ticks_per_second: 0.0,
            last_frame_update: Instant::now(),
            frame_count: 0,
            frames_per_second: 0.0,
            enabled: true,
        }
    }

    pub fn app_tick(&mut self) -> Result<()> {
        self.tick_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_tick_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.ticks_per_second = f64::from(self.tick_count) / elapsed;
            self.last_tick_update = now;
            self.tick_count = 0;
        }
        Ok(())
    }

    pub fn render_tick(&mut self) -> Result<()> {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_frame_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.frames_per_second = f64::from(self.frame_count) / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
        Ok(())
    }
    #[must_use]
    pub fn to_paragraph(&self) -> Paragraph {
        if self.enabled {
            let message = format!(
                "{:.2} ticks/sec, {:.2} FPS",
                self.ticks_per_second, self.frames_per_second
            );
            Paragraph::new(Span::raw(message))
        } else {
            Paragraph::default()
        }
    }
}
