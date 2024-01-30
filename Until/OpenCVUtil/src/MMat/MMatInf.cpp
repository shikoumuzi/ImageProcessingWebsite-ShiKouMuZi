#include "MMatInf.h"
#include<string>
extern "C"
{
	MMat_t readImg(const char* path, int flag)
	{
		return MUZI::MMatManger::getManager().readImg(path, flag);
	}
	void saveImg(MMat_t img, const char* path)
	{
		MUZI::MMatManger::getManager().saveImg(img, path);
	}

	void showImg(MMat_t img, const char* title)
	{
		MUZI::MMatManger::getManager().saveImg(img, title);
	}
	void freeImg(MMat_t img)
	{
		MUZI::MMatManger::getManager().freeMat(img);
	}
}


