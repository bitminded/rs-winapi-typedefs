pub mod cdef
{
    use crate::windef::cdef::*;
    use crate::intsafe::cdef::*;

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PIXELFORMATDESCRIPTOR
    {
        pub nSize: WORD,
        pub nVersion: WORD,
        pub dwFlags: DWORD,
        pub iPixelType: BYTE,
        pub cColorBits: BYTE,
        pub cRedBits: BYTE,
        pub cRedShift: BYTE,
        pub cGreenBits: BYTE,
        pub cGreenShift: BYTE,
        pub cBlueBits: BYTE,
        pub cBlueShift: BYTE,
        pub cAlphaBits: BYTE,
        pub cAccumRedBits: BYTE,
        pub cAccumGreenBits: BYTE,
        pub cAccumBlueBits: BYTE,
        pub cAccumAlphaBits: BYTE,
        pub cDepthBits: BYTE,
        pub cStencilBits: BYTE,
        pub cAuxBuffers: BYTE,
        pub iLayerType: BYTE,
        pub bReserved: BYTE,
        pub dwLayerMask: DWORD,
        pub dwVisibleMask: DWORD,
        pub dwDamageMask: DWORD
    }

    pub const PFD_DRAW_TO_WINDOW: DWORD = 4;
    pub const PFD_SUPPORT_OPENGL: DWORD = 32;
    pub const PFD_DOUBLEBUFFER: DWORD = 1;
    pub const PFD_TYPE_RGBA: BYTE = 0;
}
