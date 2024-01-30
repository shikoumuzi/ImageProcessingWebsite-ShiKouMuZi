#include"MNumericCalculationInf.h"
#include<tuple>
extern "C"
{
	MMat_t mat_addInMats(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::add(img_a, img_b);
	}
	MMat_t mat_addBetweenMatAndValue(MMat_t img_a, uint8_t value)
	{
		return MUZI::MNumericCalculation::add(img_a, value);
	}
	MMat_t mat_addBetweenMatAndScalar(MMat_t img_a, const Scalar* scalar)
	{
		return MUZI::MNumericCalculation::add(img_a, std::make_tuple(scalar->r, scalar->g, scalar->b));
	}
	MMat_t mat_sub(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::sub(img_a, img_b);
	}
	MMat_t mat_multiply(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::multiply(img_a, img_b);
	}
	MMat_t mat_divide(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::divide(img_a, img_b);
	}
}