#include"MAffineTransform.h"


namespace MUZI
{
	int MAffineTransform::checkMat(Mat& img)
	{
		if (img.empty()) {
			return MERROR::MAFFINETRANSFORM_IMAGE_IS_EMPTY;
		}

		return 0;
	}
	void MAffineTransform::translation()
	{
	
	}
	/// @brief this function is not implemented now
	/// @param index 
	/// @return 
	MMatIndex_t MAffineTransform::rotate(MMatIndex_t index)
	{
		auto& manager = MMatManger::getManager();

		Mat dst_mat;
		cv::rotate(manager.getMat(index), dst_mat, cv::ROTATE_90_COUNTERCLOCKWISE);
		return manager.setMat(dst_mat);
	}
	MMatIndex_t MAffineTransform::leftRotate90(MMatIndex_t index)
	{

		auto& manager = MMatManger::getManager();
		auto& mat = manager.getMat(index);
		auto error_code = MAffineTransform::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		Mat dst_mat;
		cv::rotate(mat, dst_mat, cv::ROTATE_90_COUNTERCLOCKWISE);
		return manager.setMat(dst_mat);
	}
	MMatIndex_t MAffineTransform::rightRotate90(MMatIndex_t index)
	{

		auto& manager = MMatManger::getManager();
		auto& mat = manager.getMat(index);
		auto error_code = MAffineTransform::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		Mat dst_mat;
		cv::rotate(mat, dst_mat, cv::ROTATE_90_CLOCKWISE);
		return manager.setMat(dst_mat);
	}
	MMatIndex_t MAffineTransform::flip(MMatIndex_t index, int8_t flip_code)
	{
		if (flip_code > 1 || flip_code < -1)
		{
			return MERROR::MAFFINETRANSFORM_FLIP_CODE_IS_NOT_MATCH;
		}
		auto& manager = MMatManger::getManager();
		auto& mat = manager.getMat(index);
		auto error_code = MAffineTransform::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		Mat dst_mat;
		cv::flip(mat, dst_mat, flip_code);
		return manager.setMat(dst_mat);
	}
	/// @brief this function is not implemented now
	void MAffineTransform::pyrDown()
	{

		//cv::pyrDown()
	}
}