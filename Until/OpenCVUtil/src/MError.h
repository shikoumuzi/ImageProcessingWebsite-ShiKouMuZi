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
			MATMANAGER_MAT_COUNT_REACH_MAX = -101, // mat�ﵽ���������������������
			MATMANAGER_READ_IMG_FAILED = -102, // ��ȡͼƬʧ��
			MATMANAGER_NOT_MATCH_SIZE = -103,

			//MNumericCalculation
			MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL = -202,
			MNUMBERICCALCULATION__UNKONW_FLAG = -203,
			MNUMBERICCALCULATION_UNKONU_CHANNELS = -204,

		};
	}
}

#endif // !__MUZI_ERROR__
