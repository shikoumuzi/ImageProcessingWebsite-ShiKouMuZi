#include"MAffineTransform.h"
#include "MAffineTransformInf.h"

extern "C"
{
	MMat_t mat_leftRotate90(MMat_t mat)
	{
		return MUZI::MAffineTransform::leftRotate90(mat);
	}

	MMat_t mat_rightRotate90(MMat_t mat)
	{
		return MUZI::MAffineTransform::leftRotate90(mat);
	}
	MMat_t mat_flip()
	{
		return MMat_t();
	}
}


