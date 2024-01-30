#include "MNumericCalculation.h"

namespace MUZI
{
	int MNumericCalculation::checkMat(Mat a, Mat b)
	{
		return 0;
	}
	std::tuple<Mat, Mat> MNumericCalculation::getMat(MMatManger& manager, MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto mat_1 = manager.getMat(mat_a);
		auto mat_2 = manager.getMat(mat_b);
		return std::make_tuple(mat_1, mat_2);
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
	MMatIndex_t MNumericCalculation::multiply(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat_1 = manager.getMat(mat_a);
		auto mat_2 = manager.getMat(mat_b);
		int erroc_code = MNumericCalculation::checkMat(mat_1, mat_2);
		if (erroc_code < 0)
		{
			return erroc_code;
		}
	}
}