#include"MAffineTransform.h"
#include"../MMat/MMatInf.h"
extern "C"
{
	MMat_t mat_leftRotate90(MMat_t mat);
	MMat_t mat_rightRotate90(MMat_t mat);
	MMat_t mat_flip(MMat_t mat, int8_t file_code);
}