#pragma once
#ifndef __MUZI_LOG_H__
#define __MUZI_LOG_H__
#include <boost/log/core.hpp>
#include <boost/log/trivial.hpp>
#include <boost/log/expressions.hpp>
#include <thread>
#include <map>
namespace MUZI
{
	class MLog
	{
	public:
	private:
		std::map<std::thread::id, boost::log::record> m_id_recoder;
	};

	static MLog logging;
}

#endif // !__MUZI_LOG_H__
