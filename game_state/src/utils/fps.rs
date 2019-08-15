use std::time::Instant;

pub struct FPS {
    num_samples: u32,
    frame_count: u32,
    last_frame: Instant,
}

impl FPS {
    pub fn new() -> Self {
        FPS {
            num_samples: 10000,
            frame_count: 0,
            last_frame: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.frame_count = if self.frame_count > self.num_samples {
            self.last_frame = Instant::now();
            0
        } else {
            self.frame_count + 1
        }
    }

    pub fn get(&self) -> f32 {
        let diff = self.last_frame.elapsed();
        let millis = diff.as_millis();
        self.frame_count as f32 / millis as f32 * 1000 as f32
    }

    pub fn count(&self) -> u32 {
        self.frame_count
    }
}
