#[cfg(target_os = "windows")]
pub fn register_pdf_handler() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let exe_path = std::env::current_exe()?;
    let exe_str  = exe_path.to_string_lossy();

    let hkcu    = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;

    let (dot_pdf, _) = classes.create_subkey(".pdf")?;
    dot_pdf.set_value("", &"PDFVwr.Document")?;

    let (prog_root, _) = classes.create_subkey("PDFVwr.Document")?;
    prog_root.set_value("", &"PDF Document")?;

    let (icon_key, _) = prog_root.create_subkey("DefaultIcon")?;
    icon_key.set_value("", &format!("\"{exe_str}\",0"))?;

    let (cmd_key, _) = prog_root.create_subkey("shell\\open\\command")?;
    cmd_key.set_value("", &format!("\"{exe_str}\" \"%1\""))?;

    unsafe {
        windows_sys::Win32::UI::Shell::SHChangeNotify(
            0x0800_0000i32, // SHCNE_ASSOCCHANGED
            0x0000,         // SHCNF_IDLIST
            std::ptr::null(),
            std::ptr::null(),
        );
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn register_pdf_handler() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
