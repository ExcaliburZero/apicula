use crate::cli::Args;
use crate::errors::Result;

#[derive(Clone)]
pub struct Config {
    /// Initial window width.
    pub window_width: u32,
    /// Initial window height.
    pub window_height: u32,
    /// Window background color.
    pub bg_color: (f32, f32, f32, f32),
    /// Near-plane distance for perspective.
    pub z_near: f32,
    /// Far-plane distance for perspective.
    pub z_far: f32,
    /// Vertical field-of-view for perspective (radians).
    pub fov_y: f32,
    /// Animation framerate (seconds/frame)
    pub animation_framerate: f64,
    /// Calculate FPS over intervals of this length (seconds).
    pub fps_interval: f64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            window_width: 640,
            window_height: 480,
            bg_color: (0.3, 0.3, 0.3, 1.0),
            z_near: 0.01,
            z_far: 4000.0,
            fov_y: 1.1,
            animation_framerate: 1.0 / 60.0,
            fps_interval: 2.0,
        }
    }
}

impl Config {
    pub fn from_cli_args(args: &Args) -> Result<Config> {
        let mut config = Config::default();
        Ok(config)
    }
}