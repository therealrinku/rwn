use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, State};
use tauri::tray::TrayIconBuilder;
use tokio::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

struct TimerData {
    finish_timestamp: u64,
    is_paused: bool,
    task_title: String
}

pub struct TimerState(Arc<Mutex<TimerData>>);

#[tauri::command]
async fn start_timer(
    finishTimestamp: u64,
    taskTitle: String,
    state: State<'_, TimerState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let inner_state = state.0.clone();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?;
    let now_sec = now.as_secs() as u64;
    let mut remaining_seconds = if finishTimestamp / 1000 > now_sec {
      finishTimestamp / 1000 - now_sec
    } else {
      0
    };
    
    {
        let mut data = inner_state.lock().await;

        // prevent multiple loops spawing
        if remaining_seconds > 0 && !data.is_paused {
            return Ok(());
        }

        data.task_title = taskTitle;
        data.is_paused = false;
    }

    if app_handle.tray_by_id("main-tray").is_none() {
        let tray_builder = TrayIconBuilder::with_id("main-tray").title("Ready to Focus · 00:00");
        #[cfg(not(target_os = "macos"))]
        {
            if let Some(icon) = app_handle.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
        }
        let _tray = tray_builder.build(&app_handle).map_err(|e| e.to_string())?;
    }

    tauri::async_runtime::spawn(async move {
        let tray = app_handle.tray_by_id("main-tray").unwrap();

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let mut data = inner_state.lock().await;

            if data.is_paused {
                continue;
            }

            if remaining_seconds == 0 {
                app_handle.emit("timer-finished", remaining_seconds);
                break;
            }

            remaining_seconds -= 1;

            let display_time = format!("{:02}:{:02}", remaining_seconds / 60, remaining_seconds % 60);
            let display_text = format!("{} · {}", data.task_title, display_time);
            tray.set_title(Some(display_text.clone())).unwrap();

            app_handle.emit("timer-tick", display_time);
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_timer(state: State<'_, TimerState>, app_handle: tauri::AppHandle) -> Result<bool, bool> {
    let mut data = state.0.lock().await;
    data.is_paused = false;

    // remove the tray info
    let tray = app_handle.tray_by_id("main-tray").unwrap();
    let _ = tray.set_title(Some(String::new()));
    
    Ok(true)
}

#[tauri::command]
async fn toggle_pause(state: State<'_, TimerState>, app_handle: tauri::AppHandle) -> Result<bool, String> {
    let mut data = state.0.lock().await;
    data.is_paused = !data.is_paused;

    // remove the tray info when it's paused
    if data.is_paused {
    let tray = app_handle.tray_by_id("main-tray").unwrap();
      let _ = tray.set_title(Some(String::new()));
    }
    
    Ok(data.is_paused)
}

pub fn run() {
    tauri::Builder::default()
        .manage(TimerState(Arc::new(Mutex::new(TimerData {
            finish_timestamp: 0,
            is_paused: true,
            task_title: String::from("rwn"),
        }))))
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![start_timer, toggle_pause, stop_timer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
