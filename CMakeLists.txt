# Minimum CMake version
cmake_minimum_required(VERSION 3.16)

# Project name and language
project(MISO VERSION 0.1 LANGUAGES CXX)

# Set C++17 as the standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt6 package and required components
find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets)

# Executable target and source files
add_executable(MISO
	src/main.cpp
	src/MainWindow.h
	src/MainWindow.cpp
)

# Link executable against Qt
target_link_libraries(MISO PRIVATE Qt6::Core Qt6::Gui Qt6::Widgets)
