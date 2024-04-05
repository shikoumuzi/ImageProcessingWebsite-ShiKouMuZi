use libc;
use std::ffi::{c_void, CStr, CString};
use libc::{c_int};
#[link(name = "F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Until\\OpenCVUtil\\out\\build\\x64-debug\\OpenCVUtil")]
extern {
    fn mat_readImg(path: CString, flag: c_int) -> i32;
    fn mat_saveImg(mat_index: i32, path: CString) -> c_void;
    fn mat_freeImg(mat_index: i32) -> c_void;
    fn mat_createImg(mat_index: i32) -> i32;
    fn copy(mat_index_src: i32, mat_index_dest: i32) -> c_void;
    fn hstack(mat_indexs_array: * mut i32, mats_count: u32) -> i32;
    fn vstack(mat_indexs_array: * mut i32, mats_count: u32) -> i32;
    fn  resize(mat_index: i32, width: u32, height: u32) -> i32;
}

pub struct Mat{
}

impl Mat{
    pub fn mat_readImg(path: CString, flag: c_int) -> i32{
        unsafe {
            mat_readImg(path, flag)
        }
    }
    pub fn mat_saveImg(mat_index: i32, path: CString) -> c_void{

    }
    pub fn mat_freeImg(mat_index: i32) -> c_void{

    }
    pub fn mat_createImg(mat_index: i32) -> i32{

    }
    pub fn copy(mat_index_src: i32, mat_index_dest: i32) -> c_void{

    }
    pub fn hstack(mat_indexs_array: * mut i32, mats_count: u32) -> i32{

    }
    pub fn vstack(mat_indexs_array: * mut i32, mats_count: u32) -> i32{

    }
    pub fn  resize(mat_index: i32, width: u32, height: u32) -> i32{

    }
}