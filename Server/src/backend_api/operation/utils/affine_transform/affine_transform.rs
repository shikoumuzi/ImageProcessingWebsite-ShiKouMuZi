use libc;

#[link(name = "F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Until\\OpenCVUtil\\out\\build\\x64-debug\\OpenCVUtil")]
extern {
    fn mat_leftRotate90(mat: i32) -> i32;
    fn mat_rightRotate90(mat: i32) ->i32;
    fn mat_flip(mat: i32, flip_code: i8) -> i32;
}
#[repr(C)]
pub enum FLIP_CODE {
    X_AXIS = 0,
    Y_AXIS = 1,
    X_Y_AXIS = -1
}

pub struct AffineTransForm{
}

impl AffineTransForm{
    pub fn leftRotate90(&mut self, mat: i32) -> i32 {
        unsafe {
            mat_leftRotate90(mat)
        }
    }
    pub fn rightRotate90(&mut self, mat: i32) ->i32{
      unsafe {
          mat_rightRotate90(mat)
      }
    }
    pub fn flip(&mut self, mat: i32, flip_code: i8) -> i32{
        unsafe {
            mat_flip(mat, flip_code)
        }
    }
}