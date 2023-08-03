use std::os::raw::*;
use winnt::*;
use basetsd::*;

pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type UINT = c_uint;
pub type WORD = c_ushort;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type ATOM = WORD;
pub type HWND = HANDLE;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HANDLE;
pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type HDC = HANDLE;
pub type HMENU = HANDLE;
pub type HGLRC = HANDLE;

pub type PROC = extern fn();
pub type FARPROC = extern fn();

pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

#[derive(Default)]
#[repr(C)]
pub struct POINT
{
    pub x: LONG,
    pub y: LONG
}

#[repr(C)]
pub struct RECT
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG
}

impl RECT
{
    pub fn new() -> Self
    {
        RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        }
    }
}

