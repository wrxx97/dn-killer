use std::io;
use std::os::windows::io::{FromRawHandle, OwnedHandle};
use std::ptr::null;

use windows_sys::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError};
use windows_sys::Win32::System::Threading::CreateMutexW;
use windows_sys::w;

pub fn acquire() -> io::Result<Option<OwnedHandle>> {
    // The named object exists for as long as the running app keeps its handle open.
    // SAFETY: The name is NUL-terminated and null selects default security attributes.
    let (handle, status) = unsafe {
        let handle = CreateMutexW(
            null(),
            0,
            w!("Local\\DN-Killer.GPUI.2709d68a-8f42-4d65-bb95-2cd4eac77f1c"),
        );
        (handle, GetLastError())
    };
    if handle.is_null() {
        return Err(io::Error::from_raw_os_error(status as i32));
    }
    // SAFETY: A successful CreateMutexW returns a handle owned by this caller.
    let handle = unsafe { OwnedHandle::from_raw_handle(handle) };
    Ok((status != ERROR_ALREADY_EXISTS).then_some(handle))
}
