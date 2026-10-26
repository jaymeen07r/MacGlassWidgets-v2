use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Serialize)]
struct SystemStats { cpu: f32, memory: f32, gpu: Option<f32>, disk: f32 }
#[derive(Serialize)]
struct BatteryInfo { percent: u8, charging: bool }

#[tauri::command]
fn get_system_stats() -> SystemStats {
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    std::thread::sleep(std::time::Duration::from_millis(150));
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    let cpu = sys.global_cpu_usage().clamp(0.0, 100.0);
    let total = sys.total_memory();
    let memory = if total > 0 { (sys.used_memory() as f64 / total as f64 * 100.0) as f32 } else { 0.0 };
    let disks = Disks::new_with_refreshed_list();
    let (mut total_space, mut used_space) = (0u64, 0u64);
    for disk in disks.list() {
        total_space = total_space.saturating_add(disk.total_space());
        used_space = used_space.saturating_add(disk.total_space().saturating_sub(disk.available_space()));
    }
    let disk = if total_space > 0 { (used_space as f64 / total_space as f64 * 100.0) as f32 } else { 0.0 };
    // GPU utilization needs a Windows/vendor-specific provider; do not fabricate a value.
    SystemStats { cpu, memory, gpu: None, disk }
}

#[tauri::command]
fn get_battery() -> Option<BatteryInfo> {
    // TODO: implement Windows GetSystemPowerStatus for real battery data.
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri_plugin_autostart::MacosLauncher;
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_system_stats, get_battery])
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::ManagerExt;
                if let Err(error) = app.handle().autolaunch().enable() {
                    eprintln!("Could not enable autostart: {error}");
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running MacGlass Widgets");
}
