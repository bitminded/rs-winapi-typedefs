use std::os::raw::{c_uint, c_int};
use crate::winnt::Handle;
use crate::windef::{Instance, Icon, Cursor, Brush};

pub mod cdef;

pub type WindowProcedure = cdef::WNDPROC;

/// Contains Window class information. It is used with the [winapi_user32::register_class_ex]
/// and [winapi_user32::get_class_info_ex] functions.
///
/// The WindowClassExW structure is similar to the [winapi_typedefs::winuser::WindowClassW]
/// structure. The difference is that WindowClassExW includes icon_small, which
/// contains a handle to a small icon associated with the window class.
///
/// # Members
/// ## style
/// The class style(s). This member can be any combination of the [WindowClassStyles].
/// 
/// ## window_procedure
/// A pointer to the window procedure. You must use the [CallWindowProcedure] function
/// to call the window procedure. For more information, see [WindowProcedure].
///
/// ## class_extra_bytes
/// The number of extra bytes to allocate following the window-class structure.
/// The system initializes the bytes to zero.
///
/// ## window_extra_bytes
/// The number of extra bytes to allocate following the window instance. The system
/// initializes the bytes to zero. If an application uses WindowClassExW to register
/// a dialog box created by using the class directory in the resource file, it must
/// set this member to DLGWINDOWEXTRA.
///
/// ## instance
/// A handle to the instance that contains the window procedure for the class.
///
/// ## icon
/// A handle to the class icon. This member must be a handle to an icon resource.
/// if this member is None, the system provides a default icon.
///
/// ## cursor
/// A handle to the class cursor. This member must be a handle to a cursor resource.
/// If this member is None, an application must explicitly set the cursor shape
/// whenever the mouse moves into the application's window.
///
/// ## background_brush
/// TODO: Document this member. The documentation on this is a bit tricky to
/// understand.
///
/// ## menu_name
/// A unicode compatible string that specified the resource name of the class menu,
/// as the name appears in the resource file. If this member is None, windows belonging
/// to this class have no default menu.
///
/// ## class_name
/// A unicode compatible string that specifies the window class name. The class
/// name can be a new name, any name registered with register_class_w or
/// register_class_ex_w, or any of the predefined control-class names. The name's
/// maximum length is 256. If the name exceeds the maximum length, the register_class_ex_w
/// function will fail.
///
/// Note: Atoms as described in WNDCLASSEXW are not supported for the time being
/// as that feature feels like a bit of a hack and there doesn't seem to be any
/// use-case other than compatibility with legacy code that we don't have to worry
/// about.
///
/// ## icon_small
/// A handle to a small icon that is associated with the window class. If this
/// member is None, the system searches the icon resource specified by the icon
/// member for an icon of the appropriate size to use as the small icon.
pub struct WindowClassExW
{
    style: c_uint,
    window_procedure: WindowProcedure,
    class_extra_bytes: c_int,
    window_extra_bytes: c_int,
    instance: Handle<Instance>,
    icon: Option<Handle<Icon>>,
    cursor: Option<Handle<Cursor>>,
    background_brush: Option<Handle<Brush>>,
    menu_name: Option<String>,
    class_name: String,
    icon_small: Option<Handle<Icon>> 
}

impl WindowClassExW
{
    pub fn add_style(&mut self, value: WindowClassStyle)
    {
        self.style |= value as c_uint;
    } 

    pub fn set_window_procedure(&mut self, value: WindowProcedure)
    {
        self.window_procedure = value;
    }

    pub fn set_class_extra_bytes(&mut self, value: c_int)
    {
        self.class_extra_bytes = value;
    }

    pub fn set_window_extra_bytes(&mut self, value: c_int)
    {
        self.window_extra_bytes = value;
    }

    pub fn set_instance(&mut self, value: Handle<Instance>)
    {
        self.instance = value;
    }

    pub fn set_icon(&mut self, value: Option<Handle<Icon>>)
    {
        self.icon = value;
    }

    pub fn set_cursor(&mut self, value: Option<Handle<Cursor>>)
    {
        self.cursor = value;
    }

    pub fn set_background_brush(&mut self, value: Option<Handle<Brush>>) 
    {
        self.background_brush = value;
    }

    pub fn set_menu_name(&mut self, value: Option<String>)
    {
        self.menu_name = value;
    }

    pub fn set_class_name(&mut self, value: String)
    {
        self.class_name = value;
    }

    pub fn set_icon_small(&mut self, value: Option<Handle<Icon>>)
    {
        self.icon_small = value;
    }

    pub fn create_cdef(&self) -> cdef::WNDCLASSEXW
    {
        cdef::WNDCLASSEXW
        {
            cbSize: std::mem::size_of::<cdef::WNDCLASSEXW>() as c_uint,
            style: self.style,
            lpfnWndProc: self.window_procedure, 
            cbClsExtra: self.class_extra_bytes,
            cbWndExtra: self.window_extra_bytes,
            hInstance: unsafe {
                std::mem::transmute(self.instance.ptr)
            },
            hIcon: match &self.icon {
                None => std::ptr::null_mut(),
                Some(icon) => unsafe {
                    std::mem::transmute(icon.ptr)
                }
            },
            hCursor: match &self.cursor {
                None => std::ptr::null_mut(),
                Some(cursor) => unsafe {
                    std::mem::transmute(cursor.ptr)
                }
            },
            hbrBackground: match &self.background_brush {
                None => std::ptr::null_mut(),
                Some(background_brush) => unsafe {
                    std::mem::transmute(background_brush.ptr)
                }
            },
            lpszMenuName: match &self.menu_name {
                None => std::ptr::null_mut(),
                Some(menu_name) => {
                    let mut menu_name: Vec<u16> = menu_name.encode_utf16().collect();
                    menu_name.push(0);
                    let ptr = menu_name.as_ptr() as *const i16;
                    std::mem::forget(menu_name);
                    ptr
                }
            },
            lpszClassName: {
                let mut class_name: Vec<u16> = self.class_name.encode_utf16().collect();
                class_name.push(0);
                let ptr = class_name.as_ptr() as *const i16;
                std::mem::forget(class_name);
                ptr
            },
            hIconSm: match &self.icon_small {
                None => std::ptr::null_mut(),
                Some(icon_small) => unsafe {
                    std::mem::transmute(icon_small.ptr)
                }
            }
        }
    }
}

pub enum WindowClassStyle
{
    OwnDeviceContext = 0x0020
}
