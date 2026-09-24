cmake_minimum_required(VERSION 3.22.1)
project("a_launcher")

add_library(a_launcher SHARED
    a_launcher_jni.cpp
)

find_library(log-lib log)
target_link_libraries(a_launcher ${log-lib})
