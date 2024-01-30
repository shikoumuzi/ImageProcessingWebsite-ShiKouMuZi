#pragma once
#ifndef __MUZI_MAT_INF_H__
#include"MMat.h"
extern "C"
{
	typedef MUZI::MMatIndex_t MMat_t;
	MMat_t mat_readImg(const char* path, int flag);

	void mat_saveImg(MMat_t img, const char* path);

	void mat_showImg(MMat_t img, const char* title);

	void mat_freeImg(MMat_t img);
}
#endif // !__MUZI_MAT_INF_H__
