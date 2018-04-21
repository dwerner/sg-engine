extern crate time;

pub struct FPS {
    num_samples: u32,
    frame_count: u32,
    last_frame: time::PreciseTime 
}

impl FPS {
    pub fn new() -> Self {
        FPS { num_samples:10000, frame_count: 0, last_frame: time::PreciseTime::now() }
    }

    #[inline]
    pub fn update(&mut self) {
        self.frame_count = if self.frame_count > self.num_samples {
            self.last_frame = time::PreciseTime::now();
            0
        } else {
            self.frame_count + 1
        }
    }

    #[inline]
    pub fn get(&self) -> f32 {
        let diff = self.last_frame.to(time::PreciseTime::now());
        let millis = diff.num_milliseconds();
        self.frame_count as f32 / millis as f32 * 1000 as f32
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.frame_count
    }
}

//TODO tests??
#[test] fn fps_counter() { }

