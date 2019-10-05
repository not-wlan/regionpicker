#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use winapi::ctypes::{c_char, c_void};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EGCResults {
    k_EGCResultOK = 0,
    k_EGCResultNoMessage = 1,      // There is no message in the queue
    k_EGCResultBufferTooSmall = 2, // The buffer is too small for the requested message
    k_EGCResultNotLoggedOn = 3,    // The client is not logged onto Steam
    k_EGCResultInvalidMessage = 4, // Something was wrong with the message being sent with SendMessage
}

#[repr(C)]
#[derive(Debug)]
pub struct IGameCoordinator {
    pub vtable: &'static [usize; 3],
}

#[repr(C)]
pub struct ISteamClient {
    _private: [u8; 0],
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CSteamID(pub u64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HSteamPipe(pub i32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HSteamUser(pub i32);
#[no_mangle]
extern "C" {
    pub fn SteamClient() -> *mut ISteamClient;
    pub fn SteamAPI_ISteamClient_CreateSteamPipe(instance: *mut ISteamClient) -> HSteamPipe;
    pub fn SteamAPI_ISteamClient_BReleaseSteamPipe(
        instance: *mut ISteamClient,
        pipe: HSteamPipe,
    ) -> u8;
    pub fn SteamAPI_ISteamClient_ConnectToGlobalUser(
        instance: *mut ISteamClient,
        pipe: HSteamPipe,
    ) -> HSteamUser;
    pub fn SteamAPI_ISteamClient_GetISteamGenericInterface(
        instance: *mut ISteamClient,
        user: HSteamUser,
        pipe: HSteamPipe,
        name: *const c_char,
    ) -> *mut c_void;

    pub fn Msg(message: *const c_char, ...);
    pub fn Warning(message: *const c_char, ...);
}

macro_rules! msg {
    ($x:expr) => {
        unsafe {
            crate::steam::Msg(concat!(concat!("[RegionPicker] ", $x), "\n\0").as_ptr()
                as *const winapi::ctypes::c_char)
        }
    };
}

impl ISteamClient {
    pub fn connect() -> *mut ISteamClient {
        unsafe { SteamClient() }
    }

    pub fn CreateSteamPipe(&mut self) -> HSteamPipe {
        unsafe { SteamAPI_ISteamClient_CreateSteamPipe(self as *mut Self) }
    }

    pub fn BReleaseSteamPipe(&mut self, pipe: HSteamPipe) -> u8 {
        unsafe { SteamAPI_ISteamClient_BReleaseSteamPipe(self as *mut Self, pipe) }
    }

    pub fn ConnectToGlobalUser(&mut self, pipe: HSteamPipe) -> HSteamUser {
        unsafe { SteamAPI_ISteamClient_ConnectToGlobalUser(self as *mut Self, pipe) }
    }

    pub fn GetISteamGenericInterface(
        &mut self,
        user: HSteamUser,
        pipe: HSteamPipe,
        name: &str,
    ) -> *mut c_void {
        unsafe {
            SteamAPI_ISteamClient_GetISteamGenericInterface(
                self as *mut Self,
                user,
                pipe,
                name.as_ptr() as *const c_char,
            )
        }
    }
}
