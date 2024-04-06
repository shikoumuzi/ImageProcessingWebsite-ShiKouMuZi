use libc;
#[repr(C)]
pub struct Scalar
{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[link(name = "F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Until\\OpenCVUtil\\out\\build\\x64-debug\\OpenCVUtil")]
extern {
    fn mat_addBetweenMats(img_a: i32, img_b: i32) -> i32;
    fn mat_addBetweenMatAndValue(img_a: i32, value: u8) -> i32;
    fn mat_addBetweenMatAndScalar(img_a: i32, scalar: *const Scalar) -> i32;
    fn mat_subBetweenMats(img_a: i32, img_b: i32) -> i32;
    fn mat_subBetweenMatAndValue(img_a: i32, value: u8) -> i32;
    fn mat_subBetweenMatAndScalar(img_a: i32, scalar: *const Scalar) -> i32;

    // fn mat_multiply(img_a: i32, img_b: i32) -> i32;
    // fn mat_divide(img_a: i32, img_b: i32) -> i32;

    fn mat_BitwiseAnd(img_a: i32, img_b: i32) -> i32;
    fn mat_BitwiseOr(img_a: i32, img_b: i32) -> i32;
    fn mat_BitwiseNot(img_a: i32, img_b: i32) -> i32;
    fn mat_BitwiseXor(img_a: i32, img_b: i32) -> i32;

}

pub struct NumericCalculation{
}

impl NumericCalculation{
    pub fn addBetweenMats(&mut self, img_a: i32, img_b: i32)-> i32{
        unsafe {
            mat_addBetweenMats(img_a, img_b)
        }
    }

    pub fn addBetweenMatAndValue(&mut self, img_a: i32, value: u8)-> i32{
        unsafe {
            mat_addBetweenMatAndValue(img_a, value)
        }
    }
    pub fn addBetweenMatAndScalar(&mut self, img_a: i32, scalar: *const Scalar) -> i32{
        unsafe {
            mat_addBetweenMatAndScalar(img_a, scalar)
        }
    }
    pub fn subBetweenMats(&mut self, img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_subBetweenMats(img_a, img_b)
        }
    }
    pub fn subBetweenMatAndValue(&mut self, img_a: i32, value: u8) -> i32{
        unsafe {
            mat_addBetweenMatAndValue(img_a, value)
        }
    }
    pub fn subBetweenMatAndScalar(&mut self, img_a: i32, scalar: *const Scalar) -> i32{
        unsafe {
            mat_addBetweenMatAndScalar(img_a, scalar)
        }
    }

    pub fn bitwiseAnd(&mut self, img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_BitwiseAnd(img_a, img_b)
        }
    }
    pub fn bitwiseOr(&mut self, img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_BitwiseOr(img_a, img_b)
        }
    }
    pub fn bitwiseNot(&mut self, img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_BitwiseNot(img_a, img_b)
        }
    }
    pub fn bitwiseXor(&mut self, img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_BitwiseXor(img_a, img_b)
        }
    }

}