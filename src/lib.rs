#![allow(non_snake_case)]

use libc::strncpy;
use std::{
    ffi::CString,
    os::{raw::c_char, windows::raw::HANDLE},
};
use widestring::U16CString;
use windows_sys::{
    //core::w,
    Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::{WIN32_FIND_DATAA, WIN32_FIND_DATAW},
        //UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
};

#[no_mangle]
pub unsafe extern "stdcall" fn FsInitW(
    _plugin_number: i32,
    _progress_proc: extern "C" fn(i32, *const c_char, *const c_char, i32) -> i32,
    _log_proc: extern "C" fn(i32, i32, *const c_char),
    _request_proc: extern "C" fn(
        i32,
        i32,
        *const c_char,
        *const c_char,
        *const c_char,
        i32,
    ) -> bool,
) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsInit(
    _plugin_number: i32,
    _progress_proc: extern "C" fn(i32, *const c_char, *const c_char, i32) -> i32,
    _log_proc: extern "C" fn(i32, i32, *const c_char),
    _request_proc: extern "C" fn(
        i32,
        i32,
        *const c_char,
        *const c_char,
        *const c_char,
        i32,
    ) -> bool,
) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsGetDefRootName(name: *mut c_char, max_length: usize) {
    let yt = CString::new("youtube").expect("cannot allocate def root name");
    strncpy(name, yt.as_ptr(), max_length);
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindFirstW(
    _path: *const U16CString,
    _find_data: *const WIN32_FIND_DATAW,
) -> HANDLE {
    INVALID_HANDLE_VALUE as HANDLE
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindFirst(
    _path: *const c_char,
    _find_data: *const WIN32_FIND_DATAA,
) -> HANDLE {
    INVALID_HANDLE_VALUE as HANDLE
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindNextW(
    _handle: HANDLE,
    _find_data: *const WIN32_FIND_DATAW,
) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindNext(
    _handle: HANDLE,
    _find_data: *const WIN32_FIND_DATAA,
) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindClose(_handle: HANDLE) -> i32 {
    0
}
