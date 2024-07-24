#![allow(non_snake_case)]

mod config;
mod walker;

use config::Configuration;
use libc::{strncpy, wcslen};
use msgbox::IconType;
use std::{
    ffi::{CStr, CString},
    mem::size_of,
    os::{
        raw::{c_char, c_void},
        windows::raw::HANDLE,
    },
    path::{Path, PathBuf},
};
use walker::DirectoryWalker;
use widestring::WideChar;
use windows_sys::Win32::{
    Foundation::{SetLastError, ERROR_NO_MORE_FILES, INVALID_HANDLE_VALUE, MAX_PATH},
    Storage::FileSystem::{WIN32_FIND_DATAA, WIN32_FIND_DATAW},
};

#[repr(C)]
pub struct FsDefaultParamStruct {
    size: i32,
    PluginInterfaceVersionLow: u32,
    PluginInterfaceVersionHi: u32,
    DefaultIniName: [c_char; MAX_PATH as usize],
}

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
    path: *const WideChar,
    find_data: *mut WIN32_FIND_DATAW,
) -> HANDLE {
    libc::memset(find_data as *mut c_void, 0, size_of::<WIN32_FIND_DATAW>());

    let s = std::slice::from_raw_parts(path, wcslen(path));
    let mut ps = String::from_utf16_lossy(s);
    ps.remove(0);
    let p = Path::new(&ps);
    let w = DirectoryWalker::try_new(p);

    if let Some(mut w) = w {
        let e = w.next();

        if let Some(e) = e {
            e.apply_to(find_data);

            let bw = Box::new(w);

            return Box::into_raw(bw) as HANDLE;
        } else {
            SetLastError(ERROR_NO_MORE_FILES);
        }
    }

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
    handle: HANDLE,
    find_data: *mut WIN32_FIND_DATAW,
) -> bool {
    let w: &mut DirectoryWalker = &mut *(handle as *mut DirectoryWalker);

    let success = if let Some(e) = w.next() {
        e.apply_to(find_data);
        true
    } else {
        false
    };

    success
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindNext(
    _handle: HANDLE,
    _find_data: *const WIN32_FIND_DATAA,
) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsFindClose(handle: HANDLE) -> i32 {
    drop(Box::from_raw(handle as *mut DirectoryWalker));
    0
}

#[no_mangle]
pub unsafe extern "stdcall" fn FsSetDefaultParams(dps: *const FsDefaultParamStruct) {
    let c = Configuration::get();
    let s = CStr::from_ptr((&(*dps).DefaultIniName[0]) as *const i8);
    let mut p = PathBuf::from(s.to_str().unwrap());

    p.pop();

    p.push("tc_yt.ini");

    c.load_from_file(p.as_path());
}
