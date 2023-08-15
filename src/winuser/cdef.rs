//! This module aims to provide definitions that are as close to the ones that
//! can be found in the winuser.h header file of the winapi.

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::*;
use std::ptr::null_mut;
use crate::windef::cdef::*;
use crate::intsafe::cdef::*;
use crate::winnt::cdef::*;

// Extended Windows Styles
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;

// Window Styles
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: DWORD = 
    WS_OVERLAPPED |
    WS_CAPTION |
    WS_SYSMENU |
    WS_THICKFRAME |
    WS_MINIMIZEBOX |
    WS_MAXIMIZEBOX;

pub const CW_USEDEFAULT: c_uint = 0x80000000;

// Window Class Styles
pub const CS_OWNDC: UINT = 0x0020;

// System Colors
pub const COLOR_BACKGROUND: c_int = 1;

// Icons
pub const IDI_APPLICATION: isize = 32512;

// Cursors
pub const IDC_ARROW: isize = 32512;

// ShowWindow
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_SHOW: c_int = 5;

// Windows Messages
pub const WM_CREATE: c_uint = 1;
pub const WM_DESTROY: c_uint = 2;
pub const WM_SIZE: c_uint = 5;
pub const WM_PAINT: c_uint = 15;
pub const WM_GETMINMAXINFO: c_uint = 36;
pub const WM_NCCREATE: c_uint = 129;
pub const WM_NCDESTROY: c_uint = 130;
pub const WM_WINDOWPOSCHANGING: c_uint = 70;
pub const WM_WINDOWPOSCHANGED: c_uint = 71;

// Window Extra Data Offsets
pub const GWLP_USERDATA: c_int =  -21;

// SetWindowPos flags
pub const SWP_SHOWWINDOW: c_uint = 0x0040;

// MessageBox flags
pub const MB_OK: UINT = 0x00000000;

#[repr(C)]
pub struct CREATESTRUCTA {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCSTR,
    pub lpszClass: LPCSTR,
    pub dwExStyle: DWORD
}

#[repr(C)]
pub struct WINDOWPOS
{
    pub hwnd: HWND,
    pub hwndInsertAfter: HWND,
    pub x: c_int,
    pub y: c_int,
    pub cx: c_int,
    pub cy: c_int,
    pub flags: UINT
}

#[repr(C)]
pub struct MSG
{
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT
}

impl MSG
{
    pub fn new() -> Self
    {
        MSG
        {
            hwnd: 0 as HWND,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: Default::default()
        }
    }
}

impl Default for MSG
{
    fn default() -> MSG
    {
        MSG::new()
    }
}

pub type LPMSG = *mut MSG;

#[repr(C)]
pub struct WNDCLASSEXA
{
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
    pub hIconSm: HICON
}

#[repr(C)]
pub struct WNDCLASSEXW
{
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON
}

#[repr(C)]
pub struct PAINTSTRUCT
{
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32]
}

impl PAINTSTRUCT
{
    pub fn new() -> Self
    {
        PAINTSTRUCT {
            hdc: null_mut::<c_void>(),
            fErase: 0,
            rcPaint: RECT::new(),
            fRestore: 0,
            fIncUpdate: 0,
            rgbReserved: [0; 32]
        }
    }

    pub fn get_hdc(&self) -> HDC
    {
        self.hdc
    }
}

pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

pub type WNDPROC = extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;
