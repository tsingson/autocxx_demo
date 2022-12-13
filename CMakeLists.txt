cmake_minimum_required(VERSION 3.24)
project(autocxx_demo)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)

add_executable(autocxx_demo
        src/messages.cc
        src/messages.h)
