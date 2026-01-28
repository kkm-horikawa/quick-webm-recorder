mod platform;
mod recorder;

use platform::{Region, ScreenInfo};
use recorder::RecorderState;
use tauri::Manager;

#[tauri::command]
fn start_recording(
    state: tauri::State<'_, RecorderState>,
    region: Option<Region>,
    output_path: String,
) -> Result<(), String> {
    state.start(region, output_path)
}

#[tauri::command]
fn stop_recording(state: tauri::State<'_, RecorderState>) -> Result<String, String> {
    state.stop()
}

#[tauri::command]
fn convert_to_gif(
    state: tauri::State<'_, RecorderState>,
    video_path: String,
    fps: u32,
    width: u32,
) -> Result<String, String> {
    state.convert_to_gif(&video_path, fps, width)
}

#[tauri::command]
fn list_screens() -> Result<Vec<ScreenInfo>, String> {
    Ok(platform::list_screens())
}

#[tauri::command]
fn open_region_selector(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::WebviewWindowBuilder;

    // Close existing selector if any
    if let Some(win) = app.get_webview_window("region-selector") {
        let _ = win.close();
    }

    WebviewWindowBuilder::new(&app, "region-selector", tauri::WebviewUrl::App("/selection.html".into()))
        .title("範囲選択")
        .fullscreen(true)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .build()
        .map_err(|e| format!("Failed to open region selector: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(RecorderState::new())
        .setup(|app| {
            let resource_dir = app
                .path()
                .resource_dir()
                .expect("failed to resolve resource dir");

            let ffmpeg_name = if cfg!(target_os = "windows") {
                "ffmpeg.exe"
            } else {
                "ffmpeg"
            };

            let ffmpeg_path = resource_dir.join(ffmpeg_name);
            let ffmpeg_path = if ffmpeg_path.is_file() {
                ffmpeg_path
            } else {
                std::path::PathBuf::from("ffmpeg")
            };

            let state = app.state::<RecorderState>();
            state.set_ffmpeg_path(ffmpeg_path);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            convert_to_gif,
            list_screens,
            open_region_selector,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
