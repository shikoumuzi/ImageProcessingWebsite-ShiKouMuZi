#pragma once
#ifndef __MUZI_NUMERICCALCULATION_H__
#define __MUZI_NUMERICCALCULATION_H__
#include"../MMat/MMat.h"
#include<tuple>

namespace MUZI
{
	class MNumericCalculation
	{
	public:
		struct Location
		{
			uint32_t x, y;
		};
	private:
		static int checkMat(Mat& a);
		static int checkMat(Mat& a, Mat& b);
		static int checkMask(Mat& mask, Mat& a, Mat& b);
		static std::tuple<Mat, Mat> getMat(MMatManger& manager, MMatIndex_t mat_a, MMatIndex_t mat_b);
		static Mat& getMat(MMatManger& manager, MMatIndex_t mat_a);
	public:
		static MMatIndex_t add(MMatIndex_t mat_a, MMatIndex_t mat_b, MMatIndex_t mask = -1);
		static MMatIndex_t add(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar, MMatIndex_t mask = -1);
		static MMatIndex_t add(MMatIndex_t mat_a, uint8_t value, MMatIndex_t mask = -1);
		/// @brief this add function will increase mat_a * alpha by mat_b*beta, it required the size of mat_a and mat_b, channels should be equaled
		/// @param mat_a src picture A
		/// @param alpha the weighted number between 0 and 1 for mat_a
		/// @param mat_b src picture B
		/// @param beta the weighted number between 0 and 1 for mat_b
		/// @param gamma the number to adjust brightness
		/// @return an index of mat in manager if no error, negative if error
		static MMatIndex_t add(MMatIndex_t mat_a, double alpha, MMatIndex_t mat_b, double beta, double gamma = 0);
		/// @brief This function operates on a weighted overlay of two images of different sizes
		/// @param mat_a src picture A and bigger than mat_b
		/// @param alpha the weighted number between 0 and 1 for mat_a
		/// @param mat_b src picture A and smaller than mat_a
		/// @param location the location of mat_b in mat_a
		/// @param beta the weighted number between 0 and 1 for mat_b
		/// @param gamma the number to adjust brightness
		/// @return an index of mat in manager if no error, negative if error
		static MMatIndex_t add(MMatIndex_t mat_a, double alpha, MMatIndex_t mat_b, Location location, double beta, double gamma = 0);
		static MMatIndex_t sub(MMatIndex_t mat_a, MMatIndex_t mat_b, MMatIndex_t mask = -1);
		static MMatIndex_t sub(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar, MMatIndex_t mask = -1);
		static MMatIndex_t sub(MMatIndex_t mat_a, uint8_t value, MMatIndex_t mask = -1);
		static MMatIndex_t multiply(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t divide(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t bitwiseAnd(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t bitwiseOr(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t bitwiseXor(MMatIndex_t mat_a, MMatIndex_t mat_b);
		static MMatIndex_t bitwiseNot(MMatIndex_t mat_a, MMatIndex_t mat_b);
	};
}

#endif // !__MUZI_NUMERICCALCULATION_H__
