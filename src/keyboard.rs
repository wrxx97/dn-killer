use std::cell::RefCell;
use std::collections::{HashMap, HashSet, hash_map::Entry};
use std::io;
use std::mem::size_of;
use std::ptr::{null, null_mut};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use async_channel::Sender;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    VK_CONTROL, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RCONTROL, VK_RMENU,
    VK_RSHIFT, VK_RWIN, VK_SHIFT,
};
use windows_sys::Win32::UI::Input::{
    GetRawInputData, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RAWKEYBOARD, RID_INPUT,
    RIDEV_DEVNOTIFY, RIDEV_INPUTSINK, RIM_TYPEKEYBOARD, RegisterRawInputDevices,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GIDC_REMOVAL, GetMessageW,
    HWND_MESSAGE, MSG, PostMessageW, PostQuitMessage, RI_KEY_BREAK, RI_KEY_E0, RIM_INPUTSINK,
    RegisterClassW, WM_CLOSE, WM_DESTROY, WM_INPUT, WM_INPUT_DEVICE_CHANGE, WNDCLASSW,
};

use crate::hotkey::{Hotkey, Modifiers};

#[derive(Clone, Copy)]
pub struct KeyEvent {
    pub shortcut: Hotkey,
    pub pressed: bool,
    pub background: bool,
    pub received_at: Instant,
}

pub struct KeyboardListener {
    hwnd: isize,
    worker: Option<JoinHandle<()>>,
}

struct InputState {
    sender: Sender<KeyEvent>,
    keys: KeyTracker,
}

#[derive(Default)]
struct KeyTracker {
    modifiers: HashSet<(isize, u16)>,
    held: HashMap<(isize, u16), Option<Hotkey>>,
}

impl KeyTracker {
    fn update(&mut self, device: isize, key: RAWKEYBOARD, pressed: bool) -> Option<Hotkey> {
        if let Some(modifier) = modifier_key(key) {
            if pressed {
                self.modifiers.insert((device, modifier));
            } else {
                self.modifiers.remove(&(device, modifier));
            }
            return None;
        }
        if !Hotkey::supports_key(key.VKey) {
            return None;
        }

        let physical_key = (device, key.VKey);
        if !pressed {
            return self.held.remove(&physical_key).flatten();
        }
        let Entry::Vacant(entry) = self.held.entry(physical_key) else {
            return None;
        };
        let mut modifiers = Modifiers::default();
        for &(_, key) in &self.modifiers {
            match key {
                VK_LCONTROL | VK_RCONTROL => modifiers.control = true,
                VK_LMENU | VK_RMENU => modifiers.alt = true,
                VK_LSHIFT | VK_RSHIFT => modifiers.shift = true,
                VK_LWIN | VK_RWIN => modifiers.win = true,
                _ => {}
            }
        }
        // Remember the chord from key-down so release order and auto-repeat cannot change it.
        let shortcut = Hotkey::from_virtual_key(key.VKey, modifiers);
        entry.insert(shortcut);
        shortcut
    }

    fn remove_device(&mut self, device: isize) {
        self.modifiers.retain(|&(id, _)| id != device);
        self.held.retain(|&(id, _), _| id != device);
    }
}

fn modifier_key(key: RAWKEYBOARD) -> Option<u16> {
    let right = key.Flags & RI_KEY_E0 as u16 != 0;
    match key.VKey {
        VK_SHIFT => Some(if key.MakeCode == 0x36 {
            VK_RSHIFT
        } else {
            VK_LSHIFT
        }),
        VK_CONTROL => Some(if right { VK_RCONTROL } else { VK_LCONTROL }),
        VK_MENU => Some(if right { VK_RMENU } else { VK_LMENU }),
        VK_LCONTROL | VK_RCONTROL | VK_LSHIFT | VK_RSHIFT | VK_LMENU | VK_RMENU | VK_LWIN
        | VK_RWIN => Some(key.VKey),
        _ => None,
    }
}

thread_local! {
    static INPUT: RefCell<Option<InputState>> = const { RefCell::new(None) };
}

impl KeyboardListener {
    pub fn start(sender: Sender<KeyEvent>) -> Result<Self> {
        let (ready, startup) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .name("dn-killer-raw-input".into())
            .spawn(move || run(sender, ready))
            .context("创建 Raw Input 线程失败")?;

        match startup
            .recv()
            .context("Raw Input 线程启动失败")
            .and_then(|result| result)
        {
            Ok(hwnd) => Ok(Self {
                hwnd,
                worker: Some(worker),
            }),
            Err(error) => {
                let _ = worker.join();
                Err(error)
            }
        }
    }
}

impl Drop for KeyboardListener {
    fn drop(&mut self) {
        // Win32 windows belong to their creating thread. Let it destroy the window.
        if unsafe { PostMessageW(self.hwnd as HWND, WM_CLOSE, 0, 0) } != 0 {
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }
}

fn run(sender: Sender<KeyEvent>, ready: mpsc::SyncSender<Result<isize>>) {
    let hwnd = match create_input_window() {
        Ok(hwnd) => hwnd,
        Err(error) => {
            let _ = ready.send(Err(error));
            return;
        }
    };
    INPUT.with(|input| {
        *input.borrow_mut() = Some(InputState {
            sender,
            keys: KeyTracker::default(),
        })
    });

    if ready.send(Ok(hwnd as isize)).is_ok() {
        let mut message = MSG::default();
        // SAFETY: The message buffer is writable and this thread owns the input window.
        while unsafe { GetMessageW(&mut message, null_mut(), 0, 0) } > 0 {
            unsafe { DispatchMessageW(&message) };
        }
    }

    INPUT.with(|input| input.borrow_mut().take());
    // WM_CLOSE can already have destroyed the window.
    unsafe { DestroyWindow(hwnd) };
}

fn create_input_window() -> Result<HWND> {
    let class_name: Vec<u16> = "DnKillerPrototypeRawInput\0".encode_utf16().collect();
    // SAFETY: Class name is NUL-terminated and the window procedure has Win32's ABI.
    unsafe {
        let instance = GetModuleHandleW(null());
        if instance.is_null() {
            return Err(io::Error::last_os_error()).context("获取应用模块失败");
        }
        let class = WNDCLASSW {
            hInstance: instance,
            lpszClassName: class_name.as_ptr(),
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };
        if RegisterClassW(&class) == 0 {
            return Err(io::Error::last_os_error()).context("注册 Raw Input 窗口类失败");
        }
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            null(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            null_mut(),
            instance,
            null(),
        );
        if hwnd.is_null() {
            return Err(io::Error::last_os_error()).context("创建 Raw Input 消息窗口失败");
        }
        let device = RAWINPUTDEVICE {
            usUsagePage: 0x01, // Generic desktop controls
            usUsage: 0x06,     // Keyboard
            dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY,
            hwndTarget: hwnd,
        };
        if RegisterRawInputDevices(&device, 1, size_of::<RAWINPUTDEVICE>() as u32) == 0 {
            let error = io::Error::last_os_error();
            DestroyWindow(hwnd);
            bail!("订阅后台键盘事件失败: {error}");
        }
        Ok(hwnd)
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_INPUT => {
            // SAFETY: LPARAM is the Raw Input handle for this WM_INPUT message.
            read_key(lparam as HRAWINPUT, wparam & 0xff == RIM_INPUTSINK as usize);
        }
        WM_INPUT_DEVICE_CHANGE if wparam == GIDC_REMOVAL as usize => {
            INPUT.with_borrow_mut(|input| {
                if let Some(input) = input {
                    input.keys.remove_device(lparam);
                }
            });
        }
        WM_CLOSE => {
            unsafe { DestroyWindow(hwnd) };
            return 0;
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}

fn read_key(handle: HRAWINPUT, background: bool) {
    let mut input = RAWINPUT::default();
    let mut size = size_of::<RAWINPUT>() as u32;
    // SAFETY: RAWINPUT is a correctly aligned buffer of the advertised size.
    let bytes = unsafe {
        GetRawInputData(
            handle,
            RID_INPUT,
            (&mut input as *mut RAWINPUT).cast(),
            &mut size,
            size_of::<RAWINPUTHEADER>() as u32,
        )
    };
    if bytes == u32::MAX || input.header.dwType != RIM_TYPEKEYBOARD {
        return;
    }
    // SAFETY: The parsed header identifies the active union member as a keyboard.
    let key = unsafe { input.data.keyboard };
    let pressed = (key.Flags & RI_KEY_BREAK as u16) == 0;

    INPUT.with_borrow_mut(|state| {
        let Some(state) = state else { return };
        if let Some(shortcut) = state
            .keys
            .update(input.header.hDevice as isize, key, pressed)
        {
            let _ = state.sender.try_send(KeyEvent {
                shortcut,
                pressed,
                background,
                received_at: Instant::now(),
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{VK_F4, VK_R};

    use super::*;

    fn key(vkey: u16) -> RAWKEYBOARD {
        RAWKEYBOARD {
            VKey: vkey,
            ..Default::default()
        }
    }

    #[test]
    fn keeps_modifiers_from_key_down_until_the_main_key_is_released() {
        let mut tracker = KeyTracker::default();
        assert_eq!(tracker.update(1, key(VK_LMENU), true), None);
        assert_eq!(tracker.update(1, key(VK_F4), true), Some(Hotkey::ALT_F4));
        assert_eq!(tracker.update(1, key(VK_F4), true), None);
        assert_eq!(tracker.update(1, key(VK_LMENU), false), None);
        assert_eq!(tracker.update(1, key(VK_F4), false), Some(Hotkey::ALT_F4));
        assert_eq!(tracker.update(1, key(VK_F4), false), None);
    }

    #[test]
    fn handles_modifiers_across_keyboards_and_resets_a_disconnected_keyboard() -> Result<()> {
        let mut tracker = KeyTracker::default();
        tracker.update(1, key(VK_LCONTROL), true);
        tracker.update(2, key(VK_RSHIFT), true);
        assert_eq!(
            tracker.update(2, key(VK_R), true),
            Some("Ctrl+Shift+R".parse()?)
        );
        tracker.remove_device(1);
        assert_eq!(
            tracker.update(2, key(VK_R), false),
            Some("Ctrl+Shift+R".parse()?)
        );
        assert_eq!(tracker.update(2, key(VK_R), true), Some("Shift+R".parse()?));
        Ok(())
    }
}
