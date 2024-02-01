#include "MNumericCalculation.h"
#include "../MLog.h"
namespace MUZI
{
	int MNumericCalculation::checkMat(Mat& a)
	{
		return 0;
	}

	int MNumericCalculation::checkMat(Mat& a, Mat& b)
	{
		if (a.channels() != b.channels())
		{
			return MERROR::MNUMBERICCALCULATION__MATS_CHANNELS_IS_NOT_EQUAL;
		}
		return 0;
	}

	int MNumericCalculation::checkMask(Mat& mask, Mat& a, Mat& b)
	{
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

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, MMatIndex_t mat_b, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
			if (error_code < 0)
			{
				return error_code;
			}
			error_code = MNumericCalculation::checkMask(mask, std::get<0>(mats), std::get<1>(mats));
			if (error_code < 0)
			{
				return error_code;
			}
			cv::Mat res_mat;
			try
			{
				cv::add(mat_a, mat_b, res_mat, mask);
				return manager.setMat(cv::Mat(res_mat));
			}
			catch (const std::exception& e)
			{
				logger.w("MNumericCalculation::add", e.what());
				return -1;
			}
		}
		try
		{
			auto res_mat = std::get<0>(mats) + std::get<1>(mats);
			return manager.setMat(cv::Mat(res_mat));
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::add", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int error_code = MNumericCalculation::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(mat, mask);
			if (error_code < 0)
			{
				return error_code;
			}
			cv::Mat res_mat;
			try
			{
				cv::add(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), res_mat, mask);
				return manager.setMat(cv::Mat(res_mat));
			}
			catch (const std::exception& e)
			{
				logger.w("MNumericCalculation::add", e.what());
				return -1;
			}
		}
		try
		{
			cv::Mat dst_mat;
			cv::add(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), dst_mat);
			return manager.getNewMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::add", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, uint8_t value, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int error_code = MNumericCalculation::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(mat, mask);
			if (error_code < 0)
			{
				return error_code;
			}
			try
			{
				cv::Mat res_mat;
				cv::add(manager.getMat(mat_a), value, res_mat, mask);
				return manager.setMat(cv::Mat(res_mat));
			}
			catch (const std::exception& e)
			{
				logger.w("MNumericCalculation::add", e.what());
				return -1;
			}
		}
		try
		{
			cv::Mat dst_mat;
			cv::add(manager.getMat(mat_a), value, dst_mat);

			return manager.getNewMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::add", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, double alpha, MMatIndex_t mat_b, double beta, double gamma)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int error_code = MNumericCalculation::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}
		try
		{
			cv::Mat dst_mat;
			cv::addWeighted(mat_a, alpha, mat_b, beta, gamma, dst_mat);

			return manager.getNewMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::add", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::add(MMatIndex_t mat_a, double alpha, MMatIndex_t mat_b, Location location, double beta, double gamma)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}

		auto mat_a_size = std::get<0>(mats).size();
		auto mat_b_size = std::get<1>(mats).size();

		if (mat_b_size.width + location.x > mat_a_size.width)
		{
			location.x = mat_a_size.width - mat_b_size.width;
		}
		if (mat_b_size.height + location.y > mat_a_size.width)
		{
			location.y = mat_a_size.height - mat_b_size.width;
		}
		try
		{
			Mat target_mat;
			std::get<0>(mats).copyTo(target_mat);

			auto img_crop = target_mat(cv::Range(location.y, location.y + mat_b_size.height), cv::Range(location.x, location.x + mat_b_size.width));
			Mat img_add;
			cv::add(img_crop, mat_b, img_add);
			Mat img_add_w;
			cv::addWeighted(img_crop, alpha, std::get<1>(mats), beta, gamma, img_add_w);

			img_add_w.copyTo(img_crop);

			return manager.setMat(target_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::add", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, MMatIndex_t mat_b, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
			if (error_code < 0)
			{
				return error_code;
			}
			cv::Mat res_mat;
			cv::subtract(mat_a, mat_b, res_mat, mask);
			return manager.setMat(cv::Mat(res_mat));
		}
		auto res_mat = std::get<0>(mats) - std::get<1>(mats);
		return manager.setMat(cv::Mat(res_mat));
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, const std::tuple<uint8_t, uint8_t, uint8_t>& scalar, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a);
		int error_code = MNumericCalculation::checkMat(mats);
		if (error_code < 0)
		{
			return error_code;
		}
		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(mats, mask);
			if (error_code < 0)
			{
				return error_code;
			}
			cv::Mat res_mat;
			cv::subtract(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), res_mat, mask);
			return manager.setMat(cv::Mat(res_mat));
		}
		cv::Mat dst_mat;
		cv::subtract(manager.getMat(mat_a), cv::Scalar(std::get<0>(scalar), std::get<1>(scalar), std::get<2>(scalar)), dst_mat);
		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::sub(MMatIndex_t mat_a, uint8_t value, MMatIndex_t mask_index)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mat = MNumericCalculation::getMat(manager, mat_a);
		int error_code = MNumericCalculation::checkMat(mat);
		if (error_code < 0)
		{
			return error_code;
		}

		if (mask_index != -1)
		{
			auto& mask = manager.getMat(mask_index);
			error_code = MNumericCalculation::checkMat(mat, mask);
			if (error_code < 0)
			{
				return error_code;
			}
			cv::Mat res_mat;
			cv::subtract(manager.getMat(mat_a), value, res_mat, mask);
			return manager.setMat(cv::Mat(res_mat));
		}
		cv::Mat dst_mat;
		cv::subtract(manager.getMat(mat_a), value, dst_mat);

		return manager.getNewMat(dst_mat);
	}

	MMatIndex_t MNumericCalculation::multiply(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
	}

	MMatIndex_t MNumericCalculation::divide(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
	}

	MMatIndex_t MNumericCalculation::bitwiseAnd(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		try
		{
			Mat dst_mat;
			cv::bitwise_and(std::get<0>(mats), std::get<1>(mats), dst_mat);
			return manager.setMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::bitwiseAnd", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::bitwiseOr(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		try
		{
			Mat dst_mat;
			cv::bitwise_or(std::get<0>(mats), std::get<1>(mats), dst_mat);
			return manager.setMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::bitwiseAnd", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::bitwiseXor(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		try
		{
			Mat dst_mat;
			cv::bitwise_xor(std::get<0>(mats), std::get<1>(mats), dst_mat);
			return manager.setMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::bitwiseAnd", e.what());
			return -1;
		}
	}

	MMatIndex_t MNumericCalculation::bitwiseNot(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto mats = MNumericCalculation::getMat(manager, mat_a, mat_b);
		int error_code = MNumericCalculation::checkMat(std::get<0>(mats), std::get<1>(mats));
		if (error_code < 0)
		{
			return error_code;
		}
		try
		{
			Mat dst_mat;
			cv::bitwise_not(std::get<0>(mats), std::get<1>(mats), dst_mat);
			return manager.setMat(dst_mat);
		}
		catch (const std::exception& e)
		{
			logger.w("MNumericCalculation::bitwiseAnd", e.what());
			return -1;
		}
	}
}