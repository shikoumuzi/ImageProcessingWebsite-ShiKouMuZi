#pragma once
#ifndef __MUZI_LOG_H__
#define __MUZI_LOG_H__
#include <boost/log/core.hpp>
#include <boost/log/trivial.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/sources/record_ostream.hpp>
#include <boost/log/sources/basic_logger.hpp>
#include <boost/log/sources/logger.hpp>
#include <boost/log/sinks/sync_frontend.hpp>
#include <boost/log/sinks/text_ostream_backend.hpp>
#include <boost/log/sinks/text_file_backend.hpp>
#include <boost/smart_ptr/shared_ptr.hpp>
#include <boost/log/attributes.hpp>
#include <boost/log/expressions.hpp>
#include <boost/locale/generator.hpp>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/date_time/posix_time/posix_time_types.hpp>
#include <boost/log/utility/setup/common_attributes.hpp>

#include <boost/log/support/date_time.hpp>
#include <memory>
#include <boost/filesystem.hpp>
#include <thread>
#include <map>
#include <iostream>
#include <optional>
#define __MUZI_ROTATION_SIZE__ 1024*1024*8

namespace MUZI
{
	class MLog
	{
	public:
		using Logger = boost::log::sources::logger_mt;
		using Recorder = boost::log::record;
		//using RecordMap = std::map<std::thread::id, Logger>;
		using TextStreamSink = boost::log::sinks::synchronous_sink< boost::log::sinks::text_file_backend >;
		using Path = boost::filesystem::path;
		using RecordOStream = boost::log::record_ostream;
	public:
		MLog()
		{}
	public:
		void bindOutput(const Path& path)
		{
			boost::shared_ptr<TextStreamSink> sink = boost::make_shared<TextStreamSink>
				(
					boost::log::keywords::auto_flush = true
				);
			sink->locked_backend()->set_file_name_pattern(path);
			sink->locked_backend()->set_rotation_size(__MUZI_ROTATION_SIZE__);
			sink->locked_backend()->set_open_mode(std::ios_base::app);
			sink->set_formatter(
				boost::log::expressions::stream \
				<< "LineID" << "."
				<< boost::log::expressions::format_date_time<boost::posix_time::ptime>("TimeStamp", "[%Y-%m-%d %H:%M:%S]")
				<< "[" << boost::log::trivial::severity << "]"
				<< "[" << "ThreadID" << "]"
				<< "->" << boost::log::expressions::message
			);
			boost::log::core::get()->add_sink(sink);
			boost::log::add_common_attributes();
		}
	public:
		void w(const std::string& caller, const std::string& s)
		{
			Recorder recorder = this->logger.open_record();
			if (recorder)
			{
				RecordOStream stream(recorder);
				stream << caller << ":" << s;
				this->logger.push_record(std::move(recorder));
			}
		}
	private:
		void initLogger(Logger& logger)
		{
			logger.add_attribute("TimeStamp", boost::log::attributes::local_clock());
			logger.add_attribute("LineID", boost::log::attributes::counter<uint32_t>(1));
			logger.add_attribute("Timeline", boost::log::attributes::timer());
		}
	private:
		//RecordMap m_id_recoder;
		Logger logger;
		//std::ostream m_ostream;
	};

	static MLog logger;
}

#endif // !__MUZI_LOG_H__
