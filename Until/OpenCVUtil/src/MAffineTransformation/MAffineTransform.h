#pragma once
#ifndef __MUZI_MAFFINETRANSFORM_H__
#define __MUZI_MAFFINETRANSFORM_H__
#include"../MMat/MMat.h"
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/core/core.hpp>
#include <opencv2/opencv.hpp>
#include <opencv2/imgproc/imgproc.hpp>

namespace MUZI
{
	class MAffineTransform
	{
	public:
		enum FLIP_CODE
		{
			VERTICAL = 0,
			HORIZINTAL = 1,
			VERTICAL_AND_HORIZINTAL = -1
		};
	public:
		static int checkMat(MMatIndex_t index);
	public:

		static void translation();

		static void rotate();
		static MMatIndex_t leftRotate90(MMatIndex_t index);
		static MMatIndex_t rightRotate90(MMatIndex_t index);

		static MMatIndex_t flip(MMatIndex_t index, int flip_code);

		static void pyrDown();
	};
}

#endif // !__MUZI_MAFFINETRANSFORM_H__
