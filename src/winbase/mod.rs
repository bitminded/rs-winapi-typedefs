pub mod cdef
{
    use crate::intsafe::cdef::DWORD;

    pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 256;
    pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 512;
    pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 4096;
}
