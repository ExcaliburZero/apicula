use regex::Regex;

use crate::cli::Args;
use crate::errors::Result;

type RgbaColor = (f32, f32, f32, f32);

#[derive(Clone)]
pub struct Config {
    /// Initial window width.
    pub window_width: u32,
    /// Initial window height.
    pub window_height: u32,
    /// Window background color.
    pub bg_color: RgbaColor,
    /// Near-plane distance for perspective.
    pub z_near: f32,
    /// Far-plane distance for perspective.
    pub z_far: f32,
    /// Vertical field-of-view for perspective (radians).
    pub fov_y: f32,
    /// Animation framerate (seconds/frame)
    pub animation_framerate: f64,
    /// Limits the maximum FPS the viewer is rendered at in order to reduce CPU usage (seconds/frame).
    pub max_render_framerate: Option<f64>,
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
            max_render_framerate: None,
            fps_interval: 2.0,
        }
    }
}

impl Config {
    pub fn from_cli_args(args: &Args) -> Result<Config> {
        let mut config = Config::default();

        for (flag, value) in args.opt_args.iter() {
            match *flag {
                "window-width" => {config.window_width = value.clone().into_string().unwrap().parse::<u32>()?},
                "window-height" => {config.window_height = value.clone().into_string().unwrap().parse::<u32>()?},
                "animation-fps" => {config.animation_framerate = 1.0 / value.clone().into_string().unwrap().parse::<f64>()?},
                "max-render-fps" => {config.max_render_framerate = Some(1.0 / value.clone().into_string().unwrap().parse::<f64>()?)},
                "bg-color" => {config.bg_color = hex_code_to_color(&value.clone().into_string().unwrap())?},
                _ => {},
            }
        }

        Ok(config)
    }
}

fn hex_code_to_color(hex_code: &str) -> Result<RgbaColor> {
    let hex_code_regex = Regex::new(r"#(?<red>[0-9a-fA-F][0-9a-fA-F])(?<green>[0-9a-fA-F][0-9a-fA-F])(?<blue>[0-9a-fA-F][0-9a-fA-F])(?<alpha>[0-9a-fA-F][0-9a-fA-F])?")?;

    if let Some(caps) = hex_code_regex.captures(hex_code) {
        let red = i64::from_str_radix(&caps["red"], 16)?;
        let green = i64::from_str_radix(&caps["green"], 16)?;
        let blue = i64::from_str_radix(&caps["blue"], 16)?;
        let alpha = if let Some(a) = caps.name("alpha") {
            i64::from_str_radix(a.as_str(), 16)?
        } else { 255 };

        Ok((red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha as f32 / 255.0))
    } else {
        bail!("{} is not a valid hex code of the format #RRGGBB or #RRGGBBAA", hex_code)
    }
}