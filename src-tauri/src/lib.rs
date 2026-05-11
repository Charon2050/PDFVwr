use tauri::{Emitter, Manager};

mod file_assoc;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();

            if let Err(e) = file_assoc::register_pdf_handler() {
                eprintln!("[PDFVwr] Registry registration failed: {e}");
            }

            let args: Vec<String> = std::env::args().collect();
            let pdf_path = args
                .iter()
                .skip(1)
                .find(|a| a.to_lowercase().ends_with(".pdf"))
                .cloned();

            if let Some(path) = pdf_path {
                let handle2 = handle.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(600));
                    if let Some(win) = handle2.get_webview_window("main") {
                        let _ = win.emit("open-file", path);
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running PDFVwr");
}
