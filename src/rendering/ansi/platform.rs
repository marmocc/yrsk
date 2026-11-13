use crate::common::size::Size;
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

#[cfg(windows)]
pub fn terminal_size(output: &File) -> Option<Size> {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;

    #[repr(C)]
    struct Coord {
        _x: i16,
        _y: i16,
    }

    #[repr(C)]
    struct SmallRect {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }

    #[repr(C)]
    struct ConsoleScreenBufferInfo {
        _size: Coord,
        _cursor_position: Coord,
        _attributes: u16,
        window: SmallRect,
        _maximum_window_size: Coord,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "GetConsoleScreenBufferInfo"]
        fn get_console_screen_buffer_info(
            handle: *mut c_void,
            info: *mut ConsoleScreenBufferInfo,
        ) -> i32;
    }

    let mut info = ConsoleScreenBufferInfo {
        _size: Coord { _x: 0, _y: 0 },
        _cursor_position: Coord { _x: 0, _y: 0 },
        _attributes: 0,
        window: SmallRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        _maximum_window_size: Coord { _x: 0, _y: 0 },
    };

    let success = unsafe { get_console_screen_buffer_info(output.as_raw_handle(), &mut info) };

    if success == 0 {
        return None;
    }

    let width = i32::from(info.window.right) - i32::from(info.window.left) + 1;
    let height = i32::from(info.window.bottom) - i32::from(info.window.top) + 1;

    Size::try_new(u16::try_from(width).ok()?, u16::try_from(height).ok()?)
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
fn unix_terminal_size(output: &File, request: std::ffi::c_ulong) -> Option<Size> {
    use std::ffi::c_int;
    use std::os::fd::AsRawFd;

    #[repr(C)]
    struct WinSize {
        ws_row: u16,
        ws_col: u16,
        _ws_xpixel: u16,
        _ws_ypixel: u16,
    }

    unsafe extern "C" {
        #[link_name = "ioctl"]
        fn ioctl(fd: c_int, request: std::ffi::c_ulong, ...) -> c_int;
    }

    let mut window = WinSize {
        ws_row: 0,
        ws_col: 0,
        _ws_xpixel: 0,
        _ws_ypixel: 0,
    };

    let result = unsafe { ioctl(output.as_raw_fd(), request, &mut window as *mut WinSize) };

    if result != 0 {
        return None;
    }

    Size::try_new(window.ws_col, window.ws_row)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn terminal_size(output: &File) -> Option<Size> {
    unix_terminal_size(output, 0x5413)
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
pub fn terminal_size(output: &File) -> Option<Size> {
    unix_terminal_size(output, 0x4008_7468)
}

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly"
)))]
pub fn terminal_size(_output: &File) -> Option<Size> {
    None
}
