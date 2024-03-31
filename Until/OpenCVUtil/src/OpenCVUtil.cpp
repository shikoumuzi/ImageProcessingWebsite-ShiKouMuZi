// OpenCVUtil.cpp: 定义应用程序的入口点。
//

#include "OpenCVUtil.h"
#include "MLog.h"
#include "MMat/MMatInf.h"

int main()
{
	MUZI::logger.bindOutput("F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Log\\log.txt");
	MUZI::logger.w("main", "this is warning message");

	MMat_t mat_1 = mat_readImg("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/Until/OpenCVUtil/test_resource/img/input/QQ图片20240325131215.png", 1);
	MMat_t mat_2 = mat_readImg("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/Until/OpenCVUtil/test_resource/img/input/QQ图片20240325131219.jpg", 0);
	mat_showImg(mat_1, "img");
	mat_showImg(mat_2, "img");

	MMat_t mat[2] = { mat_2, mat_2 };

	auto mat_h = vstack(mat, 2);
	mat_showImg(mat_h, "img");
	mat_saveImg(mat_h, "F://University//WorkAndReport//GraduationProject//ImageProcessingWebsite//Until//OpenCVUtil//test_resource//img//output/test.jpg");
	mat_freeImg(mat_2);
	mat_freeImg(mat_1);
	mat_freeImg(mat_h);

	return 0;
}