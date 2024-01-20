#pragma once
#ifndef __MUZI_BASICOPERATOR_H__
#define __MUZI_BASICOPERATOR_H__
#include"../ObjectBase.h"

namespace MUZI
{
	class BasicOperation
	{
		enum IMREAD_OPTION
		{
			COLOR = 1,
			GRAY_SCALE = 0,
			UNCHANGED = -1
		};
		static Mat read(const Path& picture_path, int flag)
		{
			return cv::imread(picture_path.string(), flag);
		}
	};
}

extern "C"
{
}

#endif // !__MUZI_BASICOPERATOR_H__
