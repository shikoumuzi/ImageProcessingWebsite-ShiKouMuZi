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
