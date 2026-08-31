use std::path::Path;

use crate::error::{AppError, AppResult};

pub fn relaunch_self() -> AppResult<()> {
    let exe = std::env::current_exe()?;
    #[cfg(windows)]
    {
        shell_execute_runas(&exe)
    }
    #[cfg(not(windows))]
    {
        let _ = exe;
        Err(AppError::Message("Elevation is only available on Windows.".into()))
    }
}

#[cfg(windows)]
fn shell_execute_runas(exe: &Path) -> AppResult<()> {
    use std::os::windows::ffi::OsStrExt;

    const SW_SHOWNORMAL: i32 = 1;

    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteW(
            hwnd: *mut std::ffi::c_void,
            lp_operation: *const u16,
            lp_file: *const u16,
            lp_parameters: *const u16,
            lp_directory: *const u16,
            n_show_cmd: i32,
        ) -> isize;
    }

    fn wide(value: &str) -> Vec<u16> {
        std::ffi::OsStr::new(value)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    let file = wide(&exe.to_string_lossy());
    let operation = wide("runas");
    let directory = exe.parent().map(|path| wide(&path.to_string_lossy()));
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            directory
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null()),
            SW_SHOWNORMAL,
        )
    };

    if result <= 32 {
        return Err(AppError::Message(
            "Windows non ha autorizzato il riavvio. Puoi continuare senza questo permesso: alcune ottimizzazioni resteranno in attesa.".into(),
        ));
    }
    Ok(())
}
