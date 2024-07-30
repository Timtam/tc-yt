use msgbox::IconType;
use std::{collections::VecDeque, ffi::c_void, mem::size_of, path::Path};
use widestring::U16String;
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_READONLY, WIN32_FIND_DATAW,
};

#[derive(Clone)]
pub struct Entry {
    read_only: bool,
    file: bool,
    name: String,
}

impl Entry {
    pub unsafe fn apply_to(&self, fd: *mut WIN32_FIND_DATAW) {
        libc::memset(fd as *mut c_void, 0, size_of::<WIN32_FIND_DATAW>());

        if !self.file {
            (*fd).dwFileAttributes = FILE_ATTRIBUTE_DIRECTORY;
        } else if self.read_only {
            (*fd).dwFileAttributes = FILE_ATTRIBUTE_READONLY;
        } else {
            (*fd).dwFileAttributes = FILE_ATTRIBUTE_NORMAL;
        }

        let uname = U16String::from_str(&self.name);

        (*fd).cFileName[..uname.len()].copy_from_slice(uname.as_slice());
        (*fd).ftLastWriteTime.dwHighDateTime = 0xFFFFFFFF;
        (*fd).ftLastWriteTime.dwLowDateTime = 0xFFFFFFFE;
    }
}

pub struct DirectoryWalker {
    entries: VecDeque<Entry>,
}

impl DirectoryWalker {
    pub fn try_new(p: &Path) -> Option<Self> {
        let entries: VecDeque<Entry> = if p.ancestors().count() == 1 {
            DirectoryWalker::get_root_entries()
        } else {
            return None;
        };

        Some(Self { entries })
    }

    pub fn get_root_entries() -> VecDeque<Entry> {
        VecDeque::from([Entry {
            read_only: true,
            file: true,
            name: "F7=Create new link.txt".to_string(),
        }])
    }

    pub fn next(&mut self) -> Option<Entry> {
        self.entries.pop_front();
        self.entries.front().cloned()
    }

    pub fn current(&self) -> Option<Entry> {
        self.entries.front().cloned()
    }
}
