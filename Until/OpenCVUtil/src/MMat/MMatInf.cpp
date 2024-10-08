
#include "MMatInf.h"

#include<string>
#include<vector>
extern "C"
{
	MMat_t mat_readImg(const char* path, int flag)
	{
		//printf("%s", path);
		return MUZI::MMatManger::getManager().readImg(path, flag);
	}
	void mat_saveImg(MMat_t img, const char* path)
	{
		MUZI::MMatManger::getManager().saveImg(img, path);
	}

	void mat_showImg(MMat_t img, const char* title)
	{
		MUZI::MMatManger::getManager().showImgWithBlock(img, title);
	}

	void mat_freeImg(MMat_t img)
	{
		MUZI::MMatManger::getManager().freeMat(img);
	}

	MMat_t mat_createImg(uint32_t weight, uint32_t hight, uint8_t channels, uint8_t init_value)
	{
		return MUZI::MMatManger::getManager().createMat(weight, hight, channels, init_value);
	}

	void copy(MMat_t src, MMat_t dst)
	{
		MUZI::MMatManger::getManager().copy(src, dst);
	}

	MMat_t hstack(const MMat_t* mats, uint32_t mat_size)
	{
		std::vector<MMat_t> mats_vec(mat_size);
		for (int i = 0; i < mat_size; ++i)
		{
			mats_vec[i] = mats[i];
		}
		return MUZI::MMatManger::getManager().hstack(mats_vec);
	}

	MMat_t vstack(const MMat_t* mats, uint32_t mat_size)
	{
		std::vector<MMat_t> mats_vec(mat_size);
		for (int i = 0; i < mat_size; ++i)
		{
			mats_vec[i] = mats[i];
		}
		return MUZI::MMatManger::getManager().vstack(mats_vec);
	}

	MMat_t resize(MMat_t img, uint32_t width, uint32_t height)
	{
		return MUZI::MMatManger::getManager().resize(img, width, height);
	}
}