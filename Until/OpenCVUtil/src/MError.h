#pragma once
#ifndef __MUZI_ERROR__
#define __MUZI_ERROR__

namespace MUZI
{
	enum MERROR
	{
		// MatManger
		MATMANAGER_MAT_COUNT_REACH_MAX = -101, // mat�ﵽ���������������������
		MATMANAGER_READ_IMG_FAILED = -102, // ��ȡͼƬʧ��

		//MNumericCalculation
		MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL = -202,
	};
}

#endif // !__MUZI_ERROR__
