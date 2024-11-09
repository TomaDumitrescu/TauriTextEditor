// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use std::fs;

#[tauri::command]
fn open_file(app: AppHandle) -> Option<String> {
	let file_path = app.dialog().file().blocking_pick_file();

	let get_file_path = match file_path {
		Some(file_str) => {
			file_str.to_string()
		},
		None => {
			return None;
		}
	};

	let result_read = fs::read_to_string(get_file_path);
	match result_read {
		Ok(contents) => {
			Some(contents)
		},

		Err(_) => {
			None
		}
	}
}

#[tauri::command]
fn save_file(app: AppHandle, content: String) {
	let file_path = app.dialog()
									.file()
									.add_filter("My Filter", &["txt"])
									.blocking_save_file();

	let get_file_path = match file_path {
		Some(file_str) => {
			file_str.to_string()
		},
		None => {
			return ();
		}
	};

	let _ = fs::write(get_file_path, content.clone());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_shell::init())
		.invoke_handler(tauri::generate_handler![open_file, save_file])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
