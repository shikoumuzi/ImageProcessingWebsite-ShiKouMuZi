use libc;
#[repr(C)]
struct Scalar
{
    r: u8,
    g: u8,
    b: u8
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
    pub fn addBetweenMats(img_a: i32, img_b: i32)-> i32{
        unsafe {
            mat_addBetweenMats(img_a, img_b)
        }
    }

    pub fn addBetweenMatAndValue(img_a: i32, value: u8)-> i32{
        unsafe {
            mat_addBetweenMatAndValue(img_a, value)
        }
    }
    fn mat_addBetweenMatAndScalar(img_a: i32, scalar: *const Scalar) -> i32{
        unsafe {
            mat_addBetweenMatAndScalar(img_a, scalar)
        }
    }
    fn mat_subBetweenMats(img_a: i32, img_b: i32) -> i32{
        unsafe {
            mat_subBetweenMats(img_a, img_b)
        }
    }
    fn mat_subBetweenMatAndValue(img_a: i32, value: u8) -> i32{
        unsafe {
            mat_addBetweenMatAndValue(img_a, value)
        }
    }
    fn mat_subBetweenMatAndScalar(img_a: i32, scalar: *const Scalar) -> i32{
        unsafe {
            mat_addBetweenMatAndScalar(img_a, scalar)
        }
    }

}