#pragma once
#ifndef __MUZI_ERROR__
#define __MUZI_ERROR__

namespace MUZI
{
	extern "C"
	{
		enum MERROR
		{
			// MatManger
			MATMANAGER_MAT_COUNT_REACH_MAX = -101, // mat达到最大数量，不能再申请了
			MATMANAGER_READ_IMG_FAILED = -102, // 读取图片失败
			MATMANAGER_NOT_MATCH_SIZE = -103,
			MATMANAGER_NOT_MATCH_SHAPE = -104, // hstack 或 vstack 输入图长宽不一致

			// NumericCalculation
			MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL = -202, // 通道数不同
			MNUMBERICCALCULATION__UNKONW_FLAG = -203, // 位置的flag
			MNUMBERICCALCULATION_UNKONW_CHANNELS = -204, // 未知的通道数
			MNUMBERICCALCULATION_NOT_MATCH_SHAPE = -205, // 不匹配的形状

			// AffineTransform
			MAFFINETRANSFORM_FLIP_CODE_IS_NOT_MATCH = -301,
		};
	}
}

#endif // !__MUZI_ERROR__
