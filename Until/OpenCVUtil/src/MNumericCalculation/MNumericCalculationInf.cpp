#include"MNumericCalculationInf.h"
#include"MNumericCalculation.h"
#include<tuple>
extern "C"
{
	MMat_t mat_addBetweenMats(MMat_t img_a, MMat_t img_b)
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
	MMat_t mat_addWeighted(MMat_t mat_a, float alpha, MMat_t mat_b, float beta, float gamma)
	{
		return MUZI::MNumericCalculation::add(mat_a, alpha, mat_b, beta, gamma);
	}
	MMat_t mat_subBetweenMats(MMat_t img_a, MMat_t img_b)
	{
		return MUZI::MNumericCalculation::sub(img_a, img_b);
	}
	MMat_t mat_subBetweenMatAndValue(MMat_t img_a, uint8_t value)
	{
		return MUZI::MNumericCalculation::sub(img_a, value);
	}
	MMat_t mat_subBetweenMatAndScalar(MMat_t img_a, const struct Scalar* scalar)
	{
		return MUZI::MNumericCalculation::sub(img_a, std::make_tuple(scalar->r, scalar->g, scalar->b));
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