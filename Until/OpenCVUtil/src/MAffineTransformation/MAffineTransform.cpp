#include"MAffineTransform.h"

namespace MUZI
{
	MMatIndex_t MAffineTransform::flip(MMatIndex_t index, int flip_code)
	{
		if (flip_code > 1 || flip_code < -1)
		{
			return MERROR::MAFFINETRANSFORM_FLIP_CODE_IS_NOT_MATCH;
		}
		auto error_code = 0;

		auto& manager = MMatManger::getManager();
		Mat dst_mat;
		cv::flip(manager.getMat(index), dst_mat, flip_code);
		return manager.setMat(dst_mat);
	}
}