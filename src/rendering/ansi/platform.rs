use std::{fs::File, io::Result};

pub fn prepare_and_get_stdout() -> Result<File> {
    let output = get_raw_stdout()?;

    #[cfg(windows)]
    enable_virtual_terminal(&output)?;

    Ok(output)
}

#[cfg(unix)]
fn get_raw_stdout() -> Result<File> {
    use std::os::fd::AsFd;
    let fd = std::io::stdout().as_fd().try_clone_to_owned()?;
    Ok(std::fs::File::from(fd))
}

#[cfg(windows)]
fn get_raw_stdout() -> Result<File> {
    use std::os::windows::io::AsHandle;
    let handle = std::io::stdout().as_handle().try_clone_to_owned()?;
    Ok(std::fs::File::from(handle))
}

#[cfg(windows)]
fn enable_virtual_terminal(output: &File) -> Result<()> {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;

    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "GetConsoleMode"]
        fn get_console_mode(handle: *mut c_void, mode: *mut u32) -> i32;
        #[link_name = "SetConsoleMode"]
        fn set_console_mode(handle: *mut c_void, mode: u32) -> i32;
    }

    let handle = output.as_raw_handle();
    let mut mode = 0u32;

    unsafe {
        if get_console_mode(handle, &mut mode) == 0 {
            return Ok(());
        }

        if mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0
            && set_console_mode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) == 0
        {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}
