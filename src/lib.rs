#![allow(non_snake_case)]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod config;
mod link;
mod ui;
mod walker;

use config::Configuration;
use libc::{strncpy, wcslen};
use link::{Link, LinkType};
use msgbox::IconType;
use nwg::NativeUi;
use std::{
    ffi::{CStr, CString},
    mem::size_of,
    os::{
        raw::{c_char, c_void},
        windows::raw::HANDLE,
    },
    path::{Path, PathBuf},
};
use ui::NewLinkUi;
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

    if let Some(w) = w {
        let e = w.current();

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

    if let Some(e) = w.next() {
        e.apply_to(find_data);
        true
    } else {
        false
    }
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

#[no_mangle]
pub unsafe extern "stdcall" fn FsMkDirW(path: *const WideChar) -> bool {
    let s = std::slice::from_raw_parts(path, wcslen(path));
    let mut ps = String::from_utf16_lossy(s);
    ps.remove(0);
    let p = Path::new(&ps);

    if p.ancestors().count() == 2 {
        let config = Configuration::get();
        let dir_name = p.file_name().unwrap().to_str().unwrap();

        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

        let app = NewLinkUi::build_ui(Default::default()).expect("Failed to build UI");

        nwg::dispatch_thread_events();

        if app.get_link_type() == LinkType::None {
            false
        } else {
            let l = Link::new(dir_name, app.get_link_type());
            config.write(&l);
            true
        }
    } else {
        false
    }
}

pub unsafe extern "stdcall" fn FsMkDir(_path: *const c_char) -> bool {
    false
}
