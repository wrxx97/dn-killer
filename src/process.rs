use std::ffi::{OsStr, OsString};
use std::io;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::io::{FromRawHandle, OwnedHandle};
use std::path::Path;

use anyhow::{Context, Result, bail, ensure};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use windows_sys::Win32::Foundation::{WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
    QueryFullProcessImageNameW, TerminateProcess, WaitForSingleObject,
};

pub fn terminate(name: &str) -> Result<Pid> {
    let mut system = System::new();
    system.refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new());
    let own = Pid::from_u32(std::process::id());
    let mut matches = system.processes().values().filter(|process| {
        process.pid() != own && process.name().eq_ignore_ascii_case(OsStr::new(name))
    });
    let Some(process) = matches.next() else {
        bail!("未找到进程 {name}");
    };
    ensure!(
        matches.next().is_none(),
        "找到多个 {name} 实例，无法确定要退出的进程"
    );
    let pid = process.pid();
    // SAFETY: Request only the rights needed to verify, terminate, and wait on this process.
    let handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_TERMINATE | PROCESS_SYNCHRONIZE,
            0,
            pid.as_u32(),
        )
    };
    if handle.is_null() {
        return Err(io::Error::last_os_error())
            .with_context(|| format!("无法打开 {name}（PID {pid}）"));
    }
    // SAFETY: OpenProcess returned a newly owned handle. Keep it alive for all calls below.
    let _owned = unsafe { OwnedHandle::from_raw_handle(handle) };

    // Check the same handle we will terminate in case the original PID was reused.
    let mut image = vec![0u16; 32768];
    let mut length = image.len() as u32;
    // SAFETY: The handle is live, and length describes the writable UTF-16 buffer.
    if unsafe { QueryFullProcessImageNameW(handle, 0, image.as_mut_ptr(), &mut length) } == 0 {
        return Err(io::Error::last_os_error())
            .with_context(|| format!("无法核对 {name}（PID {pid}）"));
    }
    let image_name = OsString::from_wide(&image[..length as usize]);
    ensure!(
        Path::new(&image_name)
            .file_name()
            .is_some_and(|image| image.eq_ignore_ascii_case(OsStr::new(name))),
        "PID {pid} 已变更，无法核对 {name}"
    );

    // SAFETY: The verified target is the process the caller selected by name.
    if unsafe { TerminateProcess(handle, 1) } == 0 {
        return Err(io::Error::last_os_error())
            .with_context(|| format!("无法结束 {name}（PID {pid}）"));
    }
    match unsafe { WaitForSingleObject(handle, 2500) } {
        WAIT_OBJECT_0 => Ok(pid),
        WAIT_TIMEOUT => bail!("已请求结束 {name}（PID {pid}），等待退出超时"),
        WAIT_FAILED => Err(io::Error::last_os_error()).context("等待进程退出失败"),
        status => bail!("等待进程退出返回未知状态 {status}"),
    }
}
