use std::os::raw::*;

pub type PVOID = *mut c_void;
pub type HANDLE = PVOID;
pub type CHAR = c_char;
pub type WCHAR = c_short;
pub type LONG = c_long;
pub type LPWSTR = *mut WCHAR;
pub type LPSTR = *mut CHAR;
#[cfg(unicode)]
pub type LPTSTR = LPWSTR;
#[cfg(not(unicode))]
pub type LPTSTR = LPSTR;
pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;