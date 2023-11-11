// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem, SystemTraySubmenu, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(handle: tauri::AppHandle,name: &str) -> String {
    let simple=handle.get_window("simple").unwrap();
    simple.show().unwrap();
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn create_menu()->SystemTray{
    let new=CustomMenuItem::new("new".to_string(),"New note      - Ctrl+N");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit            - Ctrl+E");
    let sub_menu_1=CustomMenuItem::new("menu1".to_string(),"menu1");
    let sub_menu_final=CustomMenuItem::new("panel".to_string(),"Open panel");
    let sub_menus=SystemTrayMenu::new().add_item(sub_menu_1).add_native_item(SystemTrayMenuItem::Separator).add_item(sub_menu_final);
    let submenu=SystemTraySubmenu::new("Latest notes ", sub_menus);
    let tray_menu = SystemTrayMenu::new()
        .add_item(new)
        .add_submenu(submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit); 
  SystemTray::new()
    .with_menu(tray_menu)
}
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    let init_tray=create_menu();
    
    tauri::Builder::default()
        .setup(|app|{
            let simple=app.get_window("simple").unwrap();
            let id=simple.listen("close", |event|{
                
            });
            Ok(())
        })
        .system_tray(init_tray)
        .on_system_tray_event(|app,event|match event {

            SystemTrayEvent::MenuItemClick { id, .. }=>{
                
                match id.as_str() {
                "quit"=>std::process::exit(0),
                "new"=>{
                    let simple=app.get_window("simple").unwrap();
                    simple.show().unwrap();
                },
                "panel"=>{
                    let main=app.get_window("main").unwrap();
                    main.show().unwrap();
                }
                _=>{}
            }
        },
        SystemTrayEvent::LeftClick { tray_id, .. }=>{
            let main=app.get_window("main").unwrap();
          
          
          
                    main.show().unwrap();
        },
            _=>{}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
