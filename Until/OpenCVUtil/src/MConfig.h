#include<string>
namespace MUZI {
	class Config
	{
	public:
		static const Config& getConfig() {
			static Config config;
			return config;
		}
	private:
		Config() {
			this->img_output_path = "";
		}
		Config(const Config&) = delete;
		Config(Config&&) = delete;
	public:
		std::string img_output_path;
	};

}