#pragma once
#ifndef __MUZI_MAFFINETRANSFORM_H__
#define __MUZI_MAFFINETRANSFORM_H__
#include"../MMat/MMat.h"
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

		static void translation();

		static void rotate();
		static void leftRotate90();
		static void rightRotate90();

		static MMatIndex_t flip(MMatIndex_t index, int flip_code);

		static void pyrDown();
	};
}

#endif // !__MUZI_MAFFINETRANSFORM_H__
