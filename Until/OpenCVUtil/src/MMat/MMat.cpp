#include"MMat.h"

namespace MUZI
{
	MMatManger& MMatManger::getManager()
	{
		static MMatManger* p_manager;
		static std::once_flag m_once_flag;
		std::call_once(m_once_flag,
			[](MMatManger** p_manager)
			{
				(*p_manager) = new MMatManger;
			},
			&p_manager);
		return *p_manager;
	}

	MMatManger::MMatManger() :
		m_data(new MatMangerData{ MatVecotr(__MUZI_MAX_MAT_SIZE__), 0 })
	{
		for (MMatIndex_t i = 0; i < this->m_data->m_mats.size(); ++i)
		{
			this->m_data->m_mats[i].index = i;
			this->m_data->m_mats[i].is_allocated = false;
		}
	}

	MMatManger::~MMatManger()
	{
		if (this->m_data != nullptr)
		{
			this->m_data.reset();
		}
	}

	MMatIndex_t MMatManger::readImg(Path& path, int flag)
	{
		MMatIndex_t ret_index = this->getNewIndex();
		if (ret_index == -1)
		{
			return MERROR::MAT_COUNT_REACH_MAX;
		}
		try
		{
			this->m_data->m_mats[ret_index].mat = cv::imread(path.string(), flag);
			this->m_data->m_mats[ret_index].is_allocated = true;
		}
		catch (const std::exception& e)
		{
			std::cerr << "MMatManger::readImg: (" << path << "," << flag << ") -> " << e.what() << std::endl;
			return MERROR::READ_IMG_FAILED;
		}

		return ret_index;
	}

	MMatIndex_t MMatManger::saveImg(Path& file_path)
	{
		return MMatIndex_t();
	}

	MMatIndex_t MMatManger::saveImg(Path& dir_path, std::string file_name)
	{
		return MMatIndex_t();
	}

	MMatIndex_t MMatManger::getNewIndex()
	{
		MMatIndex_t now_index = this->getNewIndex();
		if (now_index == -1)
		{
			return MERROR::MAT_COUNT_REACH_MAX;
		}
		for (MMatIndex_t i = now_index; i < this->m_data->m_mats.size(); ++i)
		{
			if (this->m_data->m_mats[i].is_allocated == false)
			{
				return i;
			}
		}
		for (MMatIndex_t i = 0; i < now_index; ++i)
		{
			if (this->m_data->m_mats[i].is_allocated == false)
			{
				return i;
			}
		}
		return MERROR::MAT_COUNT_REACH_MAX;
	}

	Mat& MMatManger::getMat(MMatIndex_t index)
	{
		return this->m_data->m_mats[index].mat;
	}
	MMatIndex_t MMatManger::getNewMat(Mat& mat)
	{
		MMatIndex_t ret_index = this->getNewIndex();
		if (ret_index == -1)
		{
			return MERROR::MAT_COUNT_REACH_MAX;
		}
		mat = this->m_data->m_mats[ret_index].mat;
		this->m_data->m_mats[ret_index].is_allocated = true;
		return ret_index;
	}
	MMatIndex_t MMatManger::setMat(Mat& mat)
	{
		MMatIndex_t ret_index = this->getNewIndex();
		if (ret_index == -1)
		{
			return MERROR::MAT_COUNT_REACH_MAX;
		}
		this->m_data->m_mats[this->m_data->m_tail_index].mat = mat;
		this->m_data->m_mats[this->m_data->m_tail_index].is_allocated = true;
		return ret_index;
	}
	void MMatManger::freeMat(MMatIndex_t index)
	{
		this->m_data->m_mats[index].is_allocated = false;
	}
}