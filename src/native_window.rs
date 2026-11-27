use std::io;
use std::ptr::null_mut;

use anyhow::{Context, Result, anyhow, bail};
use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, SetLastError, WPARAM};
use windows_sys::Win32::UI::Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
use windows_sys::Win32::UI::WindowsAndMessaging::{GWLP_HWNDPARENT, SetWindowLongPtrW};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    HCURSOR, HTCAPTION, HWND_NOTOPMOST, HWND_TOPMOST, IDC_SIZEALL, IsIconic, LoadCursorW,
    SW_RESTORE, SW_SHOW, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetCursor, SetWindowPos,
    ShowWindow, WM_NCDESTROY, WM_SETCURSOR,
};

fn hwnd(window: &Window) -> Result<HWND> {
    let handle = HasWindowHandle::window_handle(window)
        .map_err(|error| anyhow!("获取 GPUI 窗口句柄失败: {error}"))?;
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        bail!("GPUI 未返回 Win32 窗口句柄");
    };
    Ok(handle.hwnd.get() as HWND)
}

pub fn set_owner(window: &Window, owner: &Window) -> Result<()> {
    let window = hwnd(window)?;
    let owner = hwnd(owner)?;
    // SAFETY: Both handles are live top-level windows on this UI thread.
    unsafe {
        SetLastError(0);
        if SetWindowLongPtrW(window, GWLP_HWNDPARENT, owner as isize) == 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error().is_some_and(|code| code != 0) {
                return Err(error).context("关联弹窗失败");
            }
        }
    }
    Ok(())
}

pub fn install_drag_cursor(window: &Window) -> Result<()> {
    let hwnd = hwnd(window)?;
    // SAFETY: The stock cursor is shared by Windows and the window belongs to this UI thread.
    unsafe {
        let cursor = LoadCursorW(null_mut(), IDC_SIZEALL);
        if cursor.is_null() {
            return Err(io::Error::last_os_error()).context("加载窗口拖动光标失败");
        }
        if SetWindowSubclass(hwnd, Some(drag_cursor_proc), 1, cursor as usize) == 0 {
            bail!("安装窗口拖动光标处理失败");
        }
    }
    Ok(())
}

unsafe extern "system" fn drag_cursor_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    subclass: usize,
    cursor: usize,
) -> LRESULT {
    // GPUI marks the drag strip as HTCAPTION in WM_NCHITTEST.
    if message == WM_SETCURSOR && (lparam as u16) == HTCAPTION as u16 {
        unsafe { SetCursor(cursor as HCURSOR) };
        return 1;
    }
    if message == WM_NCDESTROY {
        unsafe { RemoveWindowSubclass(hwnd, Some(drag_cursor_proc), subclass) };
    }
    unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}

pub fn show(window: &Window) -> Result<()> {
    let hwnd = hwnd(window)?;
    // SAFETY: GPUI owns this live HWND and the calls are made on the UI thread.
    unsafe {
        let command = if IsIconic(hwnd) != 0 {
            SW_RESTORE
        } else {
            SW_SHOW
        };
        ShowWindow(hwnd, command);
    }
    window.activate_window();
    Ok(())
}

pub fn set_topmost(window: &Window, enabled: bool) -> Result<()> {
    let hwnd = hwnd(window)?;
    let position = if enabled {
        HWND_TOPMOST
    } else {
        HWND_NOTOPMOST
    };
    // SAFETY: GPUI owns this live HWND and the call is made on the UI thread.
    if unsafe {
        SetWindowPos(
            hwnd,
            position,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        )
    } == 0
    {
        return Err(io::Error::last_os_error()).context("切换窗口置顶失败");
    }
    Ok(())
}
