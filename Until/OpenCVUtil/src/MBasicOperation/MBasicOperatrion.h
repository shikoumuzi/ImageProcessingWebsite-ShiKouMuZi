#pragma once
#ifndef __MUZI_BASICOPERATOR_H__
#define __MUZI_BASICOPERATOR_H__
#include"../MObjectBase.h"

namespace MUZI
{
	class BasicOperation
	{
		static Mat read(const Path& picture_path, int flag)
		{
			return cv::imread(picture_path.string(), flag);
		}
	};
}

#endif // !__MUZI_BASICOPERATOR_H__
