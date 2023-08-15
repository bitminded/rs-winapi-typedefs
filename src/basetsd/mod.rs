pub mod cdef
{
    #![allow(non_camel_case_types)]
    use std::os::raw::*;

    pub type UINT_PTR = c_uint;
    pub type LONG_PTR = isize;
}
