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
	// 加法运算
	DLLEXPORT MMat_t mat_addBetweenMats(MMat_t img_a, MMat_t img_b);
	DLLEXPORT MMat_t mat_addBetweenMatAndValue(MMat_t img_a, uint8_t value);
	DLLEXPORT MMat_t mat_addBetweenMatAndScalar(MMat_t img_a, const struct Scalar* scalar);
	//DLLEXPORT MMat_t mat_addBetweenMatsWithMask(MMat_t img_a, MMat_t img_b, MMat_t mask);
	//DLLEXPORT MMat_t mat_addBetweenMatAndValueWithMask(MMat_t img_a, uint8_t value, MMat_t mask);
	//DLLEXPORT MMat_t mat_addBetweenMatAndScalarWithMask(MMat_t img_a, const struct Scalar* scalar, MMat_t mask);
	DLLEXPORT MMat_t mat_addWeighted(MMat_t mat_a, float alpha, MMat_t mat_b, float beta, float gamma);
	// 减法运算
	DLLEXPORT MMat_t mat_subBetweenMats(MMat_t img_a, MMat_t img_b);
	DLLEXPORT MMat_t mat_subBetweenMatAndValue(MMat_t img_a, uint8_t value);
	DLLEXPORT MMat_t mat_subBetweenMatAndScalar(MMat_t img_a, const struct Scalar* scalar);
	// DLLEXPORT MMat_t mat_subBetweenMatsWithMask(MMat_t img_a, MMat_t img_b, MMat_t mask);
	// DLLEXPORT MMat_t mat_subBetweenMatAndValueWithMask(MMat_t img_a, uint8_t value, MMat_t mask);
	// DLLEXPORT MMat_t mat_subBetweenMatAndScalarWithMask(MMat_t img_a, const struct Scalar* scalar, MMat_t mask);
	// 乘除
	//DLLEXPORT MMat_t mat_multiply(MMat_t img_a, MMat_t img_b);
	//DLLEXPORT MMat_t mat_divide(MMat_t img_a, MMat_t img_b);
	// 位运算
	DLLEXPORT MMat_t mat_BitwiseAnd(MMat_t img_a, MMat_t img_b);
	DLLEXPORT MMat_t mat_BitwiseOr(MMat_t img_a, MMat_t img_b);
	DLLEXPORT MMat_t mat_BitwiseNot(MMat_t img_a, MMat_t img_b);
	DLLEXPORT MMat_t mat_BitwiseXor(MMat_t img_a, MMat_t img_b);
}

#endif // !__MUZI_NUMERICCALCULATIONINF_H__
