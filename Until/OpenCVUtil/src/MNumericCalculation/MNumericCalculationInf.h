#pragma once
#ifndef __MUZI_NUMERICCALCULATIONINF_H__
#define __MUZI_NUMERICCALCULATIONINF_H__
#include"MNumericCalculation.h"
#include"../MMat/MMatInf.h"
extern "C"
{
	struct Scalar
	{
		uint8_t r, g, b;
	};

	MMat_t mat_addBetweenMats(MMat_t img_a, MMat_t img_b);
	MMat_t mat_addBetweenMatAndValue(MMat_t img_a, uint8_t value);
	MMat_t mat_addBetweenMatAndScalar(MMat_t img_a, const struct Scalar* scalar);
	MMat_t mat_subBetweenMats(MMat_t img_a, MMat_t img_b);
	MMat_t mat_subBetweenMatAndValue(MMat_t img_a, uint8_t value);
	MMat_t mat_subBetweenMatAndScalar(MMat_t img_a, const struct Scalar* scalar);
	MMat_t mat_multiply(MMat_t img_a, MMat_t img_b);
	MMat_t mat_divide(MMat_t img_a, MMat_t img_b);
}

#endif // !__MUZI_NUMERICCALCULATIONINF_H__
