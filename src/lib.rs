#![cfg(windows)]
extern crate lazy_static;
extern crate protobuf;
extern crate winapi;

mod protos {
    pub mod cstrike15_gcmessages;
    pub mod engine_gcmessages;
}

#[macro_use]
mod steam;
mod regionpicker;

use winapi::{
    ctypes::{c_char, c_void},
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPARAM, LPVOID, LRESULT, TRUE, UINT, WPARAM},
    shared::windef::HWND,
    um::{
        memoryapi::VirtualProtect,
        winnt::{DLL_PROCESS_ATTACH, LONG, PAGE_EXECUTE_READWRITE},
        winuser::{
            CallWindowProcA, FindWindowA, GetWindowLongA, SetWindowLongA, GWL_WNDPROC, WM_SYSKEYUP,
            WNDPROC,
        },
    },
};

use crate::protos::cstrike15_gcmessages::ECsgoGCMsg;
use crate::steam::{EGCResults, IGameCoordinator, ISteamClient};

use lazy_static::lazy_static;
use protobuf::ProtobufEnum;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use regionpicker::RegionPicker;

lazy_static! {
    static ref REGION_PICKER: RegionPicker =
        { RegionPicker::new().expect("Couldn't initialize RegionPicker") };
}

extern "stdcall" fn hooked_wnd_proc(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if u_msg == WM_SYSKEYUP {
        // TODO: make this less verbose
        if [
            '1' as usize,
            '2' as usize,
            '3' as usize,
            '4' as usize,
            '0' as usize,
        ]
        .contains(&w_param)
        {
            REGION_PICKER.apply_mode(w_param as u8 as char);
            // Don't forward the WndProc message
            return 0;
        }
    }
    let original: WNDPROC = unsafe { std::mem::transmute(REGION_PICKER.wndproc) };
    unsafe { CallWindowProcA(original, hwnd, u_msg, w_param, l_param) }
}

extern "fastcall" fn hooked_send_message(
    _: *const IGameCoordinator,
    _: *mut c_void,
    msg_type: u32,
    msg_data: *const c_void,
    msg_len: u32,
) -> EGCResults {
    let decoded_type = ECsgoGCMsg::from_i32((msg_type & 0x7FFF_FFFF) as i32);

    if let Some(decoded_type) = decoded_type {
        let slice = unsafe {
            std::slice::from_raw_parts((msg_data as *const u8).add(8), (msg_len - 8) as usize)
        };

        if REGION_PICKER.handle_message(decoded_type, slice) {
            // We assume that our message succeeded here.
            return EGCResults::k_EGCResultOK;
        }
    }

    REGION_PICKER.send_message(msg_type, msg_data, msg_len)
}

//TODO: this sucks
fn get_window() -> HWND {
    unsafe { FindWindowA("Valve001\0".as_ptr() as *const c_char, std::ptr::null()) }
}

impl RegionPicker {
    unsafe fn patch(&self, value: usize) -> bool {
        const SIZE: usize = std::mem::size_of::<usize>();
        let mut old: DWORD = 0;
        let address = &self.gc.vtable[0] as *const _ as *mut usize;
        // We're patching the vtable directly instead of shadowing it and editing the copy
        // This *should* be fine with VAC but we could instead grab CS:GOs interface
        if VirtualProtect(address as LPVOID, SIZE, PAGE_EXECUTE_READWRITE, &mut old) != 0 {
            address.write(value);
            VirtualProtect(address as LPVOID, SIZE, old, &mut old);
            return true;
        }

        false
    }

    pub fn enable(&self) {
        unsafe {
            self.patch(hooked_send_message as usize);
            let window = get_window();
            SetWindowLongA(window, GWL_WNDPROC, hooked_wnd_proc as usize as LONG);
        }
    }

    // TODO: Fix unhooking
    pub fn disable(&self) {
        unsafe {
            self.patch(self.original_send_message);
            let window = get_window();
            SetWindowLongA(window, GWL_WNDPROC, REGION_PICKER.wndproc);
        }
    }

    pub fn new() -> Option<RegionPicker> {
        // TODO: Error handling!(?)
        let client = unsafe { ISteamClient::connect().as_mut() }?;
        let pipe = client.CreateSteamPipe();
        let user = client.ConnectToGlobalUser(pipe);
        let gc = unsafe {
            (client.GetISteamGenericInterface(user, pipe, "SteamGameCoordinator001\0")
                as *mut IGameCoordinator)
                .as_ref()?
        };

        let window = get_window();
        let original = unsafe { GetWindowLongA(window, GWL_WNDPROC) };

        Some(RegionPicker {
            original_send_message: gc.vtable[0],
            gc,
            datacenters: Arc::new(Mutex::new(HashMap::new())),
            pipe,
            wndproc: original,
        })
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        REGION_PICKER.enable();
    }
    TRUE
}
