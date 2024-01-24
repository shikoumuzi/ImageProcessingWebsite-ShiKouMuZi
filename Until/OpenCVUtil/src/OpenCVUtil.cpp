// OpenCVUtil.cpp: 定义应用程序的入口点。
//

#include "OpenCVUtil.h"
#include "MLog.h"
#include "MMat/MMat.h"

int main()
{
	MUZI::logger.bindOutput("F:\\University\\WorkAndReport\\GraduationProject\\ImageProcessingWebsite\\Log\\log.txt");
	MUZI::logger.w("this is warning message");
	return 0;
}