// OpenCVUtil.cpp: 定义应用程序的入口点。
//

#include "OpenCVUtil.h"
#include "MLog.h"
#include "MMat/MMatInf.h"

int main()
{
	MUZI::logger.bindOutput("F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Log\\log.txt");
	MUZI::logger.w("main", "this is warning message");

	MMat_t mat = mat_readImg("F:/University/WorkAndReport/GraduationProject/ImageProcessingWebsite/Until/OpenCVUtil/test_resource/img/input/QQ图片20240325131215.png", 1);

	mat_showImg(mat, "img");
	mat_freeImg(mat);

	return 0;
}