#pragma once
#ifndef __MUZI_MAT__H__
#define __MUZI_MAT__H__

#include"../MObjectBase.h"
#include"../MError.h"
#include<vector>
#include<thread>
#include<memory>

#include <opencv2/highgui/highgui.hpp>
#include <opencv2/core/core.hpp>
#include <opencv2/opencv.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include<mutex>
#define __MUZI_MAX_MAT_SIZE__ 2048
namespace MUZI
{
	using MMatIndex_t = int32_t;

	class MMatManger
	{
	public:
		struct MatMsg
		{
			Mat mat;
			MMatIndex_t index;  // 索引号
			bool is_allocated; // 代表是否以及被分配出去了
		};
		using MatVecotr = std::vector<MatMsg>;
		struct MatMangerData
		{
			MatVecotr m_mats;
			MMatIndex_t m_tail_index; // 尾部最新的元素的索引
			std::mutex m_lock;
		};
		struct Attribute
		{
			int32_t dims;
			int32_t rows;
			int32_t cols;
			int32_t channels;
			uint64_t ele_size;
		};
	public:
		enum READIMG_OPTION
		{
			COLOR = 1,
			GRAY_SCALE = 0,
			UNCHANGED = -1
		};
	public:
		static MMatManger& getManager();
	private:
		MMatManger();
		MMatManger(const MMatManger&) = delete;
		MMatManger(MMatManger&&) = delete;
	public:
		~MMatManger();
	public:
		void operator=(const MMatManger&) = delete;
		void operator=(MMatManger&&) = delete;
	public:
		MMatIndex_t readImg(const Path& path, int flag);
		void saveImg(MMatIndex_t index, const Path& file_path);
		void saveImg(MMatIndex_t index, const Path& dir_path, std::string file_name);
		void showImg(MMatIndex_t index, const std::string& title = "Image");
		void showImgWithBlock(MMatIndex_t index, const std::string& title = "Image");
		void block();
	public:
		Attribute getAttribute(MMatIndex_t index);
		MMatIndex_t createMat(uint32_t width, uint32_t height, uint8_t channels, uint8_t init_value = 0);
		void copy(MMatIndex_t src_index, MMatIndex_t dst_index);
		MMatIndex_t resize(MMatIndex_t index, uint32_t width, uint32_t height);
		MMatIndex_t hstack(std::vector<MMatIndex_t>& imgs);
		MMatIndex_t vstack(std::vector<MMatIndex_t>& imgs);
		int32_t split(MMatIndex_t index, std::vector<MMatIndex_t>& mats);
		MMatIndex_t merge(const std::vector<MMatIndex_t>& index);

	private:
		MMatIndex_t getNewIndex();
	public:
		Mat& getMat(MMatIndex_t index);
		MMatIndex_t getNewMat(Mat& mat);
		MMatIndex_t setMat(const Mat& mat);
		void freeMat(MMatIndex_t index);
	private:
		std::unique_ptr<MatMangerData> m_data;
	};
}

#endif // !__MUZI_MAT__H__
