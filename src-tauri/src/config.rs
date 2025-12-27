use crate::{error::Error, APP};
use dirs::config_dir;
use log::{info, warn};
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

pub struct StoreWrapper(pub Mutex<Store<Wry>>);

pub fn init_config(app: &mut tauri::App) {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().tauri.bundle.identifier.clone());
    let config_path = config_path.join("config.json");
    info!("Load config from: {:?}", config_path);
    let mut store = StoreBuilder::new(app.handle(), config_path).build();

    match store.load() {
        Ok(_) => info!("Config loaded"),
        Err(e) => {
            warn!("Config load error: {:?}", e);
            info!("Config not found, creating new config");
        }
    }
    app.manage(StoreWrapper(Mutex::new(store)));
    let _ = check_service_available();
}

fn check_available(list: Vec<String>, builtin: Vec<&str>, plugin: Vec<String>, key: &str) {
    let origin_length = list.len();
    let mut new_list = list.clone();
    for service in list {
        let name = service.split("@").collect::<Vec<&str>>()[0];
        let mut is_available = true;
        if name.starts_with("plugin") {
            if !plugin.contains(&name.to_string()) {
                is_available = false;
            }
        } else {
            if !builtin.contains(&name) {
                is_available = false;
            }
        }
        if !is_available {
            new_list.retain(|x| x != &service);
        }
    }
    if new_list.len() != origin_length {
        set(key, new_list);
    }
}

pub fn check_service_available() -> Result<(), Error> {
    let builtin_recognize_list: Vec<&str> = vec!["system", "tesseract"];
    let builtin_translate_list: Vec<&str> = vec![
        "youdao",
        "google",
        "deepl",
    ];

    let plugin_recognize_list: Vec<String> = Vec::new();
    let plugin_translate_list: Vec<String> = Vec::new();
    if let Some(recognize_service_list) = get("recognize_service_list") {
        let recognize_service_list: Vec<String> = serde_json::from_value(recognize_service_list)?;
        check_available(
            recognize_service_list,
            builtin_recognize_list,
            plugin_recognize_list,
            "recognize_service_list",
        );
    }
    if let Some(translate_service_list) = get("translate_service_list") {
        let translate_service_list: Vec<String> = serde_json::from_value(translate_service_list)?;
        check_available(
            translate_service_list,
            builtin_translate_list,
            plugin_translate_list,
            "translate_service_list",
        );
    }
    Ok(())
}

pub fn get(key: &str) -> Option<Value> {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    match store.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn set<T: serde::ser::Serialize>(key: &str, value: T) {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let mut store = state.0.lock().unwrap();
    store.insert(key.to_string(), json!(value)).unwrap();
    store.save().unwrap();
}

pub fn is_first_run() -> bool {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    store.is_empty()
}
