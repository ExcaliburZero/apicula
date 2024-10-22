use super::config::Config;

/// Tracks frames per second.
pub struct FpsCounter<'a> {
    fps: f64,
    time_acc: f64,
    frames_acc: f64,
    config: &'a Config,
}

impl<'a> FpsCounter<'a> {
    pub fn new(config: &'a Config) -> FpsCounter<'a> {
        FpsCounter {
            fps: 0.0,
            time_acc: 0.0,
            frames_acc: 0.0,
            config: config,
        }
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn update(&mut self, dt: f64) {
        self.time_acc += dt;
        self.frames_acc += 1.0;

        if self.time_acc > self.config.fps_interval {
            self.fps = self.frames_acc / self.time_acc;
            self.time_acc = 0.0;
            self.frames_acc = 0.0;
        }
    }
}
