#include"MAffineTransform.h"


namespace MUZI
{
	int MAffineTransform::checkMat(MMatIndex_t index)
	{
	}
	void MAffineTransform::translation()
	{
	}
	void MAffineTransform::rotate()
	{
	}
	MMatIndex_t MAffineTransform::leftRotate90(MMatIndex_t index)
	{

		auto& manager = MMatManger::getManager();

		Mat dst_mat;
		cv::rotate(manager.getMat(index), dst_mat, cv::ROTATE_90_COUNTERCLOCKWISE);
		return manager.setMat(dst_mat);
	}
	MMatIndex_t MAffineTransform::rightRotate90(MMatIndex_t index)
	{

		auto& manager = MMatManger::getManager();
		Mat dst_mat;
		cv::rotate(manager.getMat(index), dst_mat, cv::ROTATE_90_CLOCKWISE);
		return manager.setMat(dst_mat);
	}
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
	void MAffineTransform::pyrDown()
	{
	}
}