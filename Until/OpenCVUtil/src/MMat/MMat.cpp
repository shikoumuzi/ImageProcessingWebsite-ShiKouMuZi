#include"MMat.h"
#include<fstream>
#include<boost/filesystem.hpp>
#include<boost/filesystem/string_file.hpp>
#include<iostream>
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

	MMatIndex_t MMatManger::readImg(const Path& path, int flag)
	{
		MMatIndex_t ret_index = this->getNewIndex();
		if (ret_index == -1)
		{
			return MERROR::MATMANAGER_MAT_COUNT_REACH_MAX;
		}
		try
		{
			/*std::ifstream ifs(path.string(), std::ios::binary | std::ios::in);

			if (!ifs.is_open())
			{
				return MERROR::MATMANAGER_READ_IMG_FAILED;
			}
			auto file_begin = ifs.tellg();
			ifs.seekg(0, std::ios::end);
			auto file_end = ifs.tellg();
			std::string img_buffer_str;

			img_buffer_str.reserve(file_end - file_begin);
			ifs.seekg(0, std::ios::beg);

			std::string line_buffer;
			while (std::getline(ifs, line_buffer))
			{
				img_buffer_str += line_buffer;
			}
			std::vector<char> img_buffer_vec(img_buffer_str.begin(), img_buffer_str.end());
			img_buffer_vec.push_back('\0');*/

			//this->m_data->m_mats[ret_index].mat = cv::imdecode(img_buffer_vec, flag);;
			this->m_data->m_mats[ret_index].mat = cv::imread(path.string(), flag);;
			this->m_data->m_mats[ret_index].is_allocated = true;
		}
		catch (const std::exception& e)
		{
			std::cerr << "MMatManger::readImg: (" << path << "," << flag << ") -> " << e.what() << std::endl;
			return MERROR::MATMANAGER_READ_IMG_FAILED;
		}

		return ret_index;
	}

	void MMatManger::saveImg(MMatIndex_t index, const Path& file_path)
	{
		cv::imwrite(file_path.string(), this->getMat(index));
		return;
	}

	void MMatManger::saveImg(MMatIndex_t index, const Path& dir_path, std::string file_name)
	{
		return;
	}

	void MMatManger::showImg(MMatIndex_t index, const std::string& title)
	{
		try
		{
			cv::imshow(title, this->getMat(index));
		}
		catch (const std::exception& e)
		{
			std::cout << e.what() << std::endl;
		}
	}

	void MMatManger::showImgWithBlock(MMatIndex_t index, const std::string& title)
	{
		this->showImg(index, title);
		this->block();
	}

	void MMatManger::block()
	{
		cv::waitKey();
	}

	MMatManger::Attribute MMatManger::getAttribute(MMatIndex_t index)
	{
		auto& mat = this->getMat(index);

		return { .dims = mat.dims,
				.rows = mat.rows,
				.cols = mat.cols,
				.channels = mat.channels(),
				.ele_size = mat.elemSize() };
	}

	MMatIndex_t MMatManger::createMat(uint32_t width, uint32_t height, uint8_t channels, uint8_t init_value)
	{
		MMatIndex_t now_index = this->getNewIndex();
		if (now_index == -1)
		{
			return MERROR::MATMANAGER_MAT_COUNT_REACH_MAX;
		}
		switch (channels)
		{
		case 1:
			return this->setMat(Mat::zeros(cv::Size(width, height), CV_8UC1));
		case 3:
			return this->setMat(Mat::zeros(cv::Size(width, height), CV_8UC3));
		default:
			return MERROR::MNUMBERICCALCULATION_UNKONU_CHANNELS;
			break;
		}
	}

	void MMatManger::copy(MMatIndex_t src_index, MMatIndex_t dst_index)
	{
		this->getMat(src_index).copyTo(this->getMat(dst_index));
	}

	int32_t MMatManger::resize(MMatIndex_t index, uint32_t width, uint32_t height)
	{
		auto& mat = this->getMat(index);
		Mat dst_mat;
		if (mat.rows < height && mat.cols < width)
		{
			cv::resize(mat, dst_mat, cv::Size(width, height), 0, 0, cv::INTER_AREA);
		}
		else if (mat.rows > height && mat.cols > width)
		{
			cv::resize(mat, dst_mat, cv::Size(width, height), 0, 0, cv::INTER_CUBIC);
		}
		else
		{
			return MERROR::MATMANAGER_NOT_MATCH_SIZE;
		}
		return this->setMat(dst_mat);
	}

	MMatIndex_t MMatManger::hstack(std::vector<MMatIndex_t>& imgs)
	{
		cv::Mat dst_mat;
		cv::hconcat(imgs, dst_mat);
		return this->setMat(dst_mat);
	}

	MMatIndex_t MMatManger::vstack(std::vector<MMatIndex_t>& imgs)
	{
		cv::Mat dst_mat;
		cv::vconcat(imgs, dst_mat);
		return this->setMat(dst_mat);
	}

	int32_t MMatManger::split(MMatIndex_t index, std::vector<MMatIndex_t>& mats)
	{
		return 0;
	}

	MMatIndex_t MMatManger::merge(const std::vector<MMatIndex_t>& index)
	{
		return MMatIndex_t();
	}

	MMatIndex_t MMatManger::getNewIndex()
	{
		MMatIndex_t now_index = this->m_data->m_tail_index;
		for (MMatIndex_t i = now_index; i < this->m_data->m_mats.size(); ++i)
		{
			if (this->m_data->m_mats[i].is_allocated == false)
			{
				this->m_data->m_mats[i].is_allocated == true;
				return i;
			}
		}
		for (MMatIndex_t i = 0; i < now_index; ++i)
		{
			if (this->m_data->m_mats[i].is_allocated == false)
			{
				this->m_data->m_mats[i].is_allocated == true;
				return i;
			}
		}
		return MERROR::MATMANAGER_MAT_COUNT_REACH_MAX;
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
			return MERROR::MATMANAGER_MAT_COUNT_REACH_MAX;
		}
		mat = this->m_data->m_mats[ret_index].mat;
		this->m_data->m_mats[ret_index].is_allocated = true;
		return ret_index;
	}
	MMatIndex_t MMatManger::setMat(const Mat& mat)
	{
		MMatIndex_t ret_index = this->getNewIndex();
		if (ret_index == -1)
		{
			return MERROR::MATMANAGER_MAT_COUNT_REACH_MAX;
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