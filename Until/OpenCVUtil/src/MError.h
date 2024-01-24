#pragma once
#ifndef __MUZI_ERROR__
#define __MUZI_ERROR__

namespace MUZI
{
	enum MERROR
	{
		// MatManger
		MAT_COUNT_REACH_MAX = -101, // mat达到最大数量，不能再申请了
		READ_IMG_FAILED = -102, // 读取图片失败
	};
}

#endif // !__MUZI_ERROR__
