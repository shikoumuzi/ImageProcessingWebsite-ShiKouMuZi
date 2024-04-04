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
			MATMANAGER_NOT_MATCH_SHAPE = -104, // hstack �� vstack ����ͼ����һ��

			// NumericCalculation
			MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL = -202, // ͨ������ͬ
			MNUMBERICCALCULATION__UNKONW_FLAG = -203, // λ�õ�flag
			MNUMBERICCALCULATION_UNKONW_CHANNELS = -204, // δ֪��ͨ����
			MNUMBERICCALCULATION_NOT_MATCH_SHAPE = -205, // ��ƥ�����״

			// AffineTransform
			MAFFINETRANSFORM_FLIP_CODE_IS_NOT_MATCH = -301,
		};
	}
}

#endif // !__MUZI_ERROR__
