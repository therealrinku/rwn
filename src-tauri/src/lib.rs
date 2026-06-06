use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, State};
use tauri::tray::TrayIconBuilder;
use tokio::sync::Mutex;

struct TimerData {
    seconds_remaining: u32,
    is_paused: bool,
    task_name: String,
}

pub struct TimerState(Arc<Mutex<TimerData>>);

#[tauri::command]
async fn start_timer(
    initial_seconds: u32,
    task: String,
    state: State<'_, TimerState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let inner_state = state.0.clone();
    
    {
        let mut data = inner_state.lock().await;
        
        // 1. DUPLICATE LOOP PROTECTION: Prevent spinning up overlapping loops
        if data.seconds_remaining > 0 && !data.is_paused {
            return Ok(());
        }

        data.seconds_remaining = initial_seconds;
        data.task_name = task;
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

            if data.seconds_remaining == 0 {
                app_handle.emit("timer-finished", ()).unwrap();
                tray.set_title(Some(format!("{} · Completed!", data.task_name))).unwrap();
                break;
            }

            data.seconds_remaining -= 1;

            let minutes = data.seconds_remaining / 60;
            let remaining_seconds = data.seconds_remaining % 60;
            let display_text = format!("{} · {:02}:{:02}", data.task_name, minutes, remaining_seconds);

            tray.set_title(Some(display_text.clone())).unwrap();
            app_handle.emit("timer-tick", data.seconds_remaining).unwrap();
        }
    });

    Ok(())
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
            seconds_remaining: 0,
            is_paused: true,
            task_name: String::from("Focusing"),
        }))))
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![start_timer, toggle_pause])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
