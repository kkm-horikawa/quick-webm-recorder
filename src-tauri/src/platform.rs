use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenInfo {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

/// Detect primary screen resolution
pub fn detect_screen_size() -> (u32, u32) {
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("xdpyinfo").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("dimensions:") {
                    // "dimensions:    1920x1080 pixels ..."
                    if let Some(dims) = trimmed.split_whitespace().nth(1) {
                        let parts: Vec<&str> = dims.split('x').collect();
                        if parts.len() == 2 {
                            if let (Ok(w), Ok(h)) = (parts[0].parse(), parts[1].parse()) {
                                return (w, h);
                            }
                        }
                    }
                }
            }
        }
        // fallback: try xrandr
        if let Ok(output) = Command::new("xrandr").arg("--current").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains(" connected") && line.contains('+') {
                    // e.g. "eDP-1 connected primary 1920x1080+0+0 ..."
                    for word in line.split_whitespace() {
                        if word.contains('x') && word.contains('+') {
                            let res = word.split('+').next().unwrap_or("");
                            let parts: Vec<&str> = res.split('x').collect();
                            if parts.len() == 2 {
                                if let (Ok(w), Ok(h)) = (parts[0].parse(), parts[1].parse()) {
                                    return (w, h);
                                }
                            }
                        }
                    }
                }
            }
        }
        (1920, 1080)
    }

    #[cfg(not(target_os = "linux"))]
    {
        (1920, 1080) // macOS/Windows: ffmpeg handles fullscreen automatically
    }
}

/// List available screens (simplified)
pub fn list_screens() -> Vec<ScreenInfo> {
    let (w, h) = detect_screen_size();
    vec![ScreenInfo {
        id: "0".into(),
        name: "Main Display".into(),
        width: w,
        height: h,
    }]
}

/// Build ffmpeg arguments for screen recording based on OS
pub fn build_record_args(
    ffmpeg_path: &Path,
    region: Option<&Region>,
    output_path: &str,
) -> (String, Vec<String>) {
    let ffmpeg = ffmpeg_path.to_string_lossy().to_string();
    let mut args: Vec<String> = vec!["-y".into()];

    #[cfg(target_os = "macos")]
    {
        args.extend_from_slice(&["-f".into(), "avfoundation".into()]);
        args.extend_from_slice(&["-i".into(), "1:none".into()]);
        if let Some(r) = region {
            args.extend_from_slice(&[
                "-vf".into(),
                format!("crop={}:{}:{}:{}", r.width, r.height, r.x, r.y),
            ]);
        }
    }

    #[cfg(target_os = "windows")]
    {
        args.extend_from_slice(&["-f".into(), "gdigrab".into()]);
        if let Some(r) = region {
            args.extend_from_slice(&[
                "-offset_x".into(), r.x.to_string(),
                "-offset_y".into(), r.y.to_string(),
                "-video_size".into(), format!("{}x{}", r.width, r.height),
            ]);
        }
        args.extend_from_slice(&["-i".into(), "desktop".into()]);
    }

    #[cfg(target_os = "linux")]
    {
        args.extend_from_slice(&["-f".into(), "x11grab".into()]);
        if let Some(r) = region {
            args.extend_from_slice(&[
                "-video_size".into(), format!("{}x{}", r.width, r.height),
                "-i".into(), format!(":0.0+{},{}", r.x, r.y),
            ]);
        } else {
            let (w, h) = detect_screen_size();
            args.extend_from_slice(&[
                "-video_size".into(), format!("{}x{}", w, h),
                "-i".into(), ":0.0".into(),
            ]);
        }
    }

    args.extend_from_slice(&[
        "-c:v".into(), "libvpx".into(),
        "-crf".into(), "8".into(),
        "-b:v".into(), "1M".into(),
        "-deadline".into(), "realtime".into(),
        output_path.into(),
    ]);

    (ffmpeg, args)
}

/// Build ffmpeg arguments for GIF conversion
pub fn build_gif_args(
    ffmpeg_path: &Path,
    input_path: &str,
    output_path: &str,
    fps: u32,
    width: u32,
) -> (String, Vec<String>) {
    let ffmpeg = ffmpeg_path.to_string_lossy().to_string();
    let filter = format!(
        "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        fps, width
    );
    let args = vec![
        "-y".into(),
        "-i".into(), input_path.into(),
        "-vf".into(), filter,
        "-loop".into(), "0".into(),
        output_path.into(),
    ];
    (ffmpeg, args)
}
