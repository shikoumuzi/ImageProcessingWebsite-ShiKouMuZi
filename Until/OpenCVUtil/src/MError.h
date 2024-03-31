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
			MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL = -202,
			MNUMBERICCALCULATION__UNKONW_FLAG = -203,
			MNUMBERICCALCULATION_UNKONU_CHANNELS = -204,

			// AffineTransform
			MAFFINETRANSFORM_FLIP_CODE_IS_NOT_MATCH = -301,
		};
	}
}

#endif // !__MUZI_ERROR__
