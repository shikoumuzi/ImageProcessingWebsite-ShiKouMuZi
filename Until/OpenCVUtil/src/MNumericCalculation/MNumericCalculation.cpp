#include "MNumericCalculation.h"

namespace MUZI
{
	int MNumericCalculation::checkMat(Mat a)
	{
		return 0;
	}

	int MNumericCalculation::checkMat(Mat a, Mat b)
	{
		if (a.channels() != b.channels())
		{
			return MERROR::MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL;
		}
		return 0;
	}

	std::tuple<Mat, Mat> MNumericCalculation::getMat(MMatManger& manager, MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto mat_1 = manager.getMat(mat_a);
		auto mat_2 = manager.getMat(mat_b);
		return std::make_tuple(mat_1, mat_2);
	}

	Mat& MNumericCalculation::getMat(MMatManger& manager, MMatIndex_t mat_a)
	{
		return manager.getMat(mat_a);
	}

	MMatIndex_t MUZI::MNumericCalculation::add(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int erroc_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		auto res_mat = std::get<0>(mats) + std::get<1>(mats);
		return manager.setMat(cv::Mat(res_mat));
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int erroc_code = MNumericCalculation::checkMat(mat);
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		cv::Mat dst_mat;
		cv::add(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), dst_mat);

		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, uint8_t value)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int erroc_code = MNumericCalculation::checkMat(mat);
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		cv::Mat dst_mat;
		cv::add(manager.getMat(mat_a), value, dst_mat);

		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int erroc_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		auto res_mat = std::get<0>(mats) - std::get<1>(mats);
		return manager.setMat(cv::Mat(res_mat));
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a);
		int erroc_code = MNumericCalculation::checkMat(mats);
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		cv::Mat dst_mat;
		cv::subtract(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), dst_mat);
		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, uint8_t value)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int erroc_code = MNumericCalculation::checkMat(mat);
		if (erroc_code < 0)
		{
			return erroc_code;
		}
		cv::Mat dst_mat;
		cv::add(manager.getMat(mat_a), value, dst_mat);

		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::multiply(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int erroc_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (erroc_code < 0)
		{
			return erroc_code;
		}
	}

	MMatIndex_t MNumericCalculation::divide(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int erroc_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (erroc_code < 0)
		{
			return erroc_code;
		}
	}
}