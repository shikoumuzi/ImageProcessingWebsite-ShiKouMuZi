#pragma once
#ifndef __MUZI_NUMERICCALCULATION_H__
#define __MUZI_NUMERICCALCULATION_H__
#include"../MMat/MMat.h"
#include<tuple>
namespace MUZI
{
	class MNumericCalculation
	{
	private:
		static int checkMat(Mat a);
		static int checkMat(Mat a, Mat b);
		static std::tuple<Mat, Mat> getMat(MMatManger& manager, MMatIndex_t mat_a, MMatIndex_t mat_b);
		static Mat& getMat(MMatManger& manager, MMatIndex_t mat_a);
	public:

		static MMatIndex_t add(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t sub(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t multiply(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t divide(MMatIndex_t mat_a, MMatIndex_t mat_b);
	};
}

#endif // !__MUZI_NUMERICCALCULATION_H__
