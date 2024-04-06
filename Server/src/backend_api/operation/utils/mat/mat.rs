use libc;
use std::ffi::{c_char, c_void, CStr, CString};
use libc::{c_int};
#[link(name = "F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Until\\OpenCVUtil\\out\\build\\x64-debug\\OpenCVUtil")]
extern {
    fn mat_readImg(path: *const c_char, flag: c_int) -> i32;
    fn mat_saveImg(mat_index: i32, path: *const c_char) -> c_void;
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

    pub fn readImg(&mut self, path: &str, flag: c_int) -> i32{
        unsafe {
            mat_readImg(CString::new(path).unwrap().as_ptr(), flag)
        }
    }
    pub fn saveImg(&mut self, mat_index: i32, path: &str) -> c_void{
        unsafe {
            mat_saveImg(mat_index, CString::new(path).unwrap().as_ptr())
        }
    }
    pub fn freeImg(&mut self, mat_index: i32) -> c_void{
        unsafe {
            mat_freeImg(mat_index)
        }
    }
    pub fn createImg(&mut self, mat_index: i32) -> i32{
        unsafe {
            mat_createImg(mat_index)
        }
    }
    pub fn copy(&mut self, mat_index_src: i32, mat_index_dest: i32) -> c_void{
        unsafe {
            copy(mat_index_src, mat_index_dest)
        }
    }
    pub fn hstack(&mut self, mat_indexs_array: * mut i32, mats_count: u32) -> i32{
        unsafe {
            hstack(mat_indexs_array, mats_count)
        }
    }
    pub fn vstack(&mut self, mat_indexs_array: * mut i32, mats_count: u32) -> i32{
        unsafe {
            vstack(mat_indexs_array, mats_count)
        }
    }
    pub fn  resize(&mut self, mat_index: i32, width: u32, height: u32) -> i32{
        unsafe {
            resize(mat_index, width, height)
        }
    }
}