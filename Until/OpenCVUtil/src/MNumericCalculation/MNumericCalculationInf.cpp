#include"MNumericCalculationInf.h"

extern "C"
{
	MMat_t mat_add(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::add(img_a, img_b);
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