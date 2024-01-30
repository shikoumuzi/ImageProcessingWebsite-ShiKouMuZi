#pragma once
#ifndef __MUZI_NUMERICCALCULATIONINF_H__
#define __MUZI_NUMERICCALCULATIONINF_H__
#include"MNumericCalculation.h"
#include"../MMat/MMatInf.h"
extern "C"
{
	MMat_t mat_add(MMat_t img_a, MMat_t img_b);
	MMat_t mat_sub(MMat_t img_a, MMat_t img_b);
	MMat_t mat_multiply(MMat_t img_a, MMat_t img_b);
	MMat_t mat_divide(MMat_t img_a, MMat_t img_b);
}

#endif // !__MUZI_NUMERICCALCULATIONINF_H__
