import json
from BaseObject import BaseObject

class OpenCV(BaseObject):
    def __init__(self):
        super().__init__()
        self.title = "OpenCV"
        self.introduction = "OpenCV是一个基于Apache2.0许可（开源）发行的跨平台计算机视觉和机器学习软件库，可以运行在Linux、Windows、Android和Mac OS操作系统上。" \
                            "它轻量级而且高效——由一系列 C 函数和少量 C++ 类构成，同时提供了Python、Ruby、MATLAB等语言的接口，实现了图像处理和计算机视觉方面的很多通用算法。\ " \
                            "OpenCV用C++语言编写，它具有C ++，Python，Java和MATLAB接口，并支持Windows，Linux，Android和Mac " \
                            "OS，OpenCV主要倾向于实时视觉应用，并在可用时利用MMX和SSE指令， 如今也提供对于C#、Ch、Ruby，GO的支持。"
        self.offical_url = [
            {
                "title": "OpenCV - Open Computer Vision Library",
                "url": "https://opencv.org/"
            },
        ]
        self.dowload_url = [
            {
                "title": "Releases - OpenCV",
                "url": "https://opencv.org/"
            }
        ]
        self.recommended_article_url = [
            {
                "title": "数字图像处理(18): 图像灰度变换——线性灰度变换 和 非线性灰度变换(对数变换 与 伽马变换)",
                "url": "https://blog.csdn.net/zaishuiyifangxym/article/details/89817995"
            }
        ]

