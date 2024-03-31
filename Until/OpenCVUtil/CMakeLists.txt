# CMakeList.txt: OpenCVUtil 的 CMake 项目，在此处包括源代码并定义
# 项目特定的逻辑。
#
cmake_minimum_required (VERSION 3.8)

# 如果支持，请为 MSVC 编译器启用热重载。
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

project ("OpenCVUtil")
set(PROJECT_NAME "OpenCVUtil")

# 设置输出路径
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/bin)# 可执行文件
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})# 静态库输出

# 设置cpp标准
set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD 20)


# BOOST的设置
# 设置不使用静态库
set (Boost_USE_STATIC_LIBS OFF)
# 设置使用多线程库
set (Boost_USE_MULTITHREADED ON)
# 设置不使用静态运行时库
set (Boost_USE_STATIC_RUNTIME OFF)

set(BOOST_CODE_LIBRARY_PATH "F:/CodeLibrary/boost_1_79_0")
set(OpenCV_DIR "D:/opencv/OpenCV-4.8.0/opencv/build")
set(SQLITE3_CODE_LIBRARY_PATH "D:/sqlite")
set(RAPIDJSON_CODE_LIBRARY_PATH "F:/CodeLibrary/rapidjson")

# 添加头文件路径
file(GLOB_RECURSE CPP_FILE_PATH src/*.cpp)
file(GLOB_RECURSE TEST_PATH test/*.h test/*.cpp)
file(GLOB_RECURSE H_FILE_PATH src/*.h)

macro(initBoost)
	# BOOST的设置
	# 设置不使用静态库
	set (Boost_USE_STATIC_LIBS OFF)

	# 设置使用多线程库
	set (Boost_USE_MULTITHREADED ON)

	# 设置不使用静态运行时库
	set (Boost_USE_STATIC_RUNTIME OFF)

	# 添加boost路径
	set(CMAKE_INCLUDE_PATH ${CMAKE_INCLUDE_PATH} ${BOOST_CODE_LIBRARY_PATH})
	set(CMAKE_LIBRARY_PATH ${CMAKE_LIBRARY_PATH} ${BOOST_CODE_LIBRARY_PATH}/stage/lib)
	set(BOOST_ROOT ${BOOST_CODE_LIBRARY_PATH})# 设置自己的路径

	find_package(BOOST REQUIRED  COMPONENTS  system filesystem asio)
	# target_link_libraries 需要在add_excutable后面
	if(Boost_FOUND)
		include_directories(${Boost_INCLUDE_DIRS}) 
		link_directories(${Boost_LIBRARY_DIRS})
		message(STATUS "Boost 库信息:")
		message(STATUS "Boost_INCLUDE_DIRS=${Boost_INCLUDE_DIRS}")
		message(STATUS "Boost_LIBRARY_DIRS=${Boost_LIBRARY_DIRS}")
	endif()
endmacro()
initBoost()

macro(initOpenCV)
	set(CMAKE_INCLUDE_PATH ${CMAKE_INCLUDE_PATH} ${OpenCV_DIR}/include/opencv2)
	set(CMAKE_LIBRARY_PATH ${CMAKE_LIBRARY_PATH} ${OpenCV_DIR}/x64/vc16/lib)

	LIST(APPEND CMAKE_PROGRAM_PATH ${CMAKE_LIBRARY_PATH} ${OpenCV_DIR}/x64/vc16/bin)

	set(OpenCV_ROOT ${OpenCV_DIR})
	find_package(OpenCV REQUIRED)
	if(OpenCV_FOUND)
		include_directories(${OpenCV_INCLUDE_DIRS})
		link_directories(${OpenCV_DIR}/x64/vc16/bin)
		message(STATUS "OpenCV 库信息")
		message(STATUS "OpenCV_INCLUDE_DIRS=${OpenCV_INCLUDE_DIRS}")
		message(STATUS "OpenCV_LIBRARY_DIRS=${OpenCV_LIBRARY_DIRS}")
		message(STATUS "OpenCV_LIBRARYS=${OpenCV_LIBRARIES}")
	endif()
endmacro()
initOpenCV()

macro(initSQLite) 
	# sqlte配置
	set(CMAKE_INCLUDE_PATH ${CMAKE_INCLUDE_PATH} ${SQLITE3_CODE_LIBRARY_PATH}/sqlite-amalgamation-3440200)
	set(CMAKE_LIBRARY_PATH ${CMAKE_LIBRARY_PATH} ${SQLITE3_CODE_LIBRARY_PATH}/sqlite-dll-win-x64-3440200)
	set(SQLite3_ROOT ${SQLITE3_CODE_LIBRARY_PATH})

	# 连接sqlite
	find_package(SQLite3 REQUIRED)

	if(SQLite3_FOUND)
		include_directories(${SQLite3_INCLUDE_DIRS}) # Assume sqlite header is in /usr/include
		link_directories(${SQLite3_LIBRARIES}) # Assume sqlite library is in /usr/lib
		message(STATUS "Sqlite3 库信息:")
		message(STATUS "Sqlite3_INCLUDE_DIRS=${SQLite3_INCLUDE_DIRS}")
		message(STATUS "Sqlite3_LIBRARY_DIRS=${SQLite3_LIBRARIES}")
		# add_library(sqlite3 SHARED imported)
		# set_target_properties(sqlite3 PROPERTIES IMPORTED_LOCATION ${SQLITE3_CODE_LIBRARY_PATH}/sqlite-dll-win-x64-3440200/sqlite3.dll)

	endif()

endmacro()
initSQLite()

macro(initRapidJson)
	# rapidcpp 配置
	
	set(CMAKE_INCLUDE_PATH ${CMAKE_INCLUDE_PATH} ${RAPIDJSON_CODE_LIBRARY_PATH}/include/rapidjson)
	set(CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH} ${RAPIDJSON_CODE_LIBRARY_PATH}/build)
	set(RapidJSON_ROOT ${RAPIDJSON_CODE_LIBRARY_PATH}/build)

	find_package(RapidJSON REQUIRED)
	# 连接rapidjson
	if(RapidJSON_FOUND)
		include_directories(${RapidJSON_INCLUDE_DIRS})
		message(STATUS "rapidjson 库信息:")
		message(STATUS "rapidjson_INCLUDE_DIRS=${RapidJSON_INCLUDE_DIRS}")
	endif()
endmacro()
initRapidJson()

# 将源代码添加到此项目的可执行文件。
add_executable (OpenCVUtil 
				"src/OpenCVUtil.cpp" 
				"src/OpenCVUtil.h" 
				"src/MObjectBase.h" 
				"src/MError.h" 
				"src/MLog.h" 

				"src/MMat/MMat.h" 
				"src/MMat/MMat.cpp" 
				"src/MMat/MMatInf.h" 
				"src/MMat/MMatInf.cpp" 

				"src/MNumericCalculation/MNumericCalculation.h" 
				"src/MNumericCalculation/MNumericCalculation.cpp" 
				"src/MNumericCalculation/MNumericCalculationInf.h" 
				"src/MNumericCalculation/MNumericCalculationInf.cpp" "src/MAffineTransformation/MAffineTransform.h" "src/MAffineTransformation/MAffineTransform.cpp" "src/MConfig.h")

# cmake变量名是区分大小写的
macro(targetLinkLib)
	target_link_libraries(${PROJECT_NAME} PUBLIC ${SQLite3_LIBRARIES}) # Link with sqlite3 library
	message(STATUS "sqlite is targeted")

	target_link_libraries(${PROJECT_NAME} PUBLIC ${BOOST_LIBRARIES})
	message(STATUS "boost is targeted")

	target_link_libraries(${PROJECT_NAME} PUBLIC ${rapidjson_LIRARIES})
	message(STATUS "rapidjson is targeted")

	target_link_libraries(${PROJECT_NAME} PUBLIC ${OpenCV_LIBRARIES})
	target_link_directories(${PROJECT_NAME} PUBLIC ${OpenCV_DIR}/x64/vc16/bin)
	message(STATUS ${OpenCV_LIBRARIES})

endmacro()
targetLinkLib()

install(FILES ${SQLITE3_CODE_LIBRARY_PATH}/sqlite-dll-win-x64-3440200/sqlite3.dll DESTINATION ${EXECUTABLE_OUTPUT_PATH}/../bin)
install(FILES ${OpenCV_DIR}/x64/vc16/bin/opencv_world480.dll DESTINATION ${EXECUTABLE_OUTPUT_PATH}/../bin)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET OpenCVUtil PROPERTY CXX_STANDARD 20)
endif()

# TODO: 如有需要，请添加测试并安装目标。
