#include"MAffineTransform.h"
#include"../MMat/MMatInf.h"
extern "C"
{
	enum FLIP_CODE {
		X_AXIS = 0,
		Y_AXIS = 1,
		X_Y_AXIS = -1
	};

	MMat_t mat_leftRotate90(MMat_t mat);
	MMat_t mat_rightRotate90(MMat_t mat);
	MMat_t mat_flip(MMat_t mat, int8_t flip_code);
}