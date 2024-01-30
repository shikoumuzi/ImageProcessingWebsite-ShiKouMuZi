#include "MNumericCalculation.h"

namespace MUZI
{
	MMatIndex_t MUZI::MNumericCalculation::add(MMatIndex_t mat_a, MMatIndex_t mat_b)
	{
		auto& manager = MUZI::MMatManger::getManager();
		auto res_mat = manager.getMat(mat_a) + manager.getMat(mat_b);
		return manager.setMat(cv::Mat(res_mat));
	}
}