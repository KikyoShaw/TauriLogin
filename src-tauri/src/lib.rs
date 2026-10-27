mod login;

use login::LoginParams;

#[tauri::command]
fn open_login(app: tauri::AppHandle, params: LoginParams) -> Result<(), String> {
    login::open_login(&app, params)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_login])
        .run(tauri::generate_context!())
        .expect("无法启动登录演示");
}
