from BaseObject import BaseObject

class Rocket(BaseObject):
    def __init__(self):
        super().__init__()
        self.title = "Rocket"
        self.introduction = "A web framework for Rust gear Rust that makes it simple to write fast, type-safe, " \
                            "secure web applications with incredible usability, productivity and performance."
        self.offical_url = [
            {
                "title": "Rocket - Simple, Fast, Type-Safe Web Framework for Rust",
                "url": "https://rocket.rs/"
            },
        ]
        self.dowload_url = [{
            "title": "rwf2/Rocket: A web framework for Rust.",
            "url": "https://github.com/rwf2/Rocket"
        }]
        self.recommended_article_url = [{
            "title": "Rocket编程指南",
            "url": "https://villezuo.gitee.io/rocketdoc/#"
        }]