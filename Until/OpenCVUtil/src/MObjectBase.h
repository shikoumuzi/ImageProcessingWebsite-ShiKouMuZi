#pragma once
#ifndef  __MUZI_MACRO_H__
#define __MUZI_MACRO_H__

#define WINDOWS
#ifdef  WINDOWS
#define DLLEXPORT __declspec(dllexport)
#define FUNCTION
#define DLLIMPORT __declspec(dllexport)
#endif //  WINDOWS

#ifdef LINUX
#define DLLEXPORT
#define FUNCTION __attribute__((visibility("hidden")))
#define DLLIMPORT
#endif // LINUX

#include <opencv2/highgui/highgui.hpp>
#include <opencv2/core/core.hpp>
#include<boost/filesystem.hpp>
#include<iostream>

namespace MUZI
{
	using Mat = cv::Mat;
	using Path = boost::filesystem::path;
}

#endif // ! __MUZI_MACRO_H__
