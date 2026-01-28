use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;

use crate::platform::{self, Region};

/// Manages the ffmpeg recording process
pub struct RecorderState {
    process: Mutex<Option<Child>>,
    output_path: Mutex<Option<String>>,
    ffmpeg_path: Mutex<Option<PathBuf>>,
}

impl RecorderState {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
            output_path: Mutex::new(None),
            ffmpeg_path: Mutex::new(None),
        }
    }

    pub fn set_ffmpeg_path(&self, path: PathBuf) {
        *self.ffmpeg_path.lock().unwrap() = Some(path);
    }

    fn get_ffmpeg_path(&self) -> Result<PathBuf, String> {
        self.ffmpeg_path
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| "ffmpeg path not set".into())
    }

    pub fn start(
        &self,
        region: Option<Region>,
        output_path: String,
    ) -> Result<(), String> {
        let mut proc_guard = self.process.lock().unwrap();
        if proc_guard.is_some() {
            return Err("Recording already in progress".into());
        }

        let ffmpeg_path = self.get_ffmpeg_path()?;
        let (cmd, args) =
            platform::build_record_args(&ffmpeg_path, region.as_ref(), &output_path);

        let child = Command::new(&cmd)
            .args(&args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start ffmpeg: {}", e))?;

        *proc_guard = Some(child);
        *self.output_path.lock().unwrap() = Some(output_path);

        Ok(())
    }

    pub fn stop(&self) -> Result<String, String> {
        let mut proc_guard = self.process.lock().unwrap();
        let child = proc_guard
            .as_mut()
            .ok_or("No recording in progress")?;

        // Send 'q' to ffmpeg stdin to gracefully stop
        if let Some(ref mut stdin) = child.stdin {
            use std::io::Write;
            let _ = stdin.write_all(b"q");
        }

        // Wait for process to exit
        let _ = child.wait().map_err(|e| format!("Failed to stop ffmpeg: {}", e))?;
        *proc_guard = None;

        let path = self
            .output_path
            .lock()
            .unwrap()
            .take()
            .unwrap_or_default();

        Ok(path)
    }

    pub fn convert_to_gif(
        &self,
        video_path: &str,
        fps: u32,
        width: u32,
    ) -> Result<String, String> {
        let ffmpeg_path = self.get_ffmpeg_path()?;
        let gif_path = video_path.replace(".webm", ".gif");

        let (cmd, args) =
            platform::build_gif_args(&ffmpeg_path, video_path, &gif_path, fps, width);

        let output = Command::new(&cmd)
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to run ffmpeg for GIF: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("GIF conversion failed: {}", stderr));
        }

        Ok(gif_path)
    }
}
