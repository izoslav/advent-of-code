cmake_minimum_required(VERSION 3.10)

project(aoc2019 VERSION 0.1 LANGUAGES CXX)

# C++ settings
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_EXTENSIONS OFF)

# Additional includes
include_directories(src/utils)

# Conan setup
set(CMAKE_MODULE_PATH ${CMAKE_BINARY_DIR})

# Copy inputs
file(COPY inputs DESTINATION ${CMAKE_BINARY_DIR})

# Finding libraries
find_package(fmt REQUIRED)
find_package(Catch REQUIRED)

# Utility library
add_library(utils src/utils/intcode.cpp)
target_link_libraries(utils fmt::fmt Catch::Catch)

# Executables
## Day1
add_library(s01common src/star01/common.cpp)

add_executable(s01p1 src/star01/part1.cpp)
target_link_libraries(s01p1 s01common utils)

add_executable(s01p2 src/star01/part2.cpp)
target_link_libraries(s01p2 s01common utils)

## Day2
add_executable(s02p1 src/star02/part1.cpp)
target_link_libraries(s02p1 utils)

add_executable(s02p2 src/star02/part2.cpp)
target_link_libraries(s02p2 utils)

## Day3
add_executable(s03p1 src/star03/part1.cpp)
target_link_libraries(s03p1 utils)

add_executable(s03p2 src/star03/part2.cpp)
target_link_libraries(s03p2 utils)

## Day4
add_executable(s04p1 src/star04/part1.cpp)
target_link_libraries(s04p1 utils)

add_executable(s04p2 src/star04/part2.cpp)
target_link_libraries(s04p2 utils)

## Day5
add_executable(s05p1 src/star05/part1.cpp)
target_link_libraries(s05p1 utils)

add_executable(s05p2 src/star05/part2.cpp)
target_link_libraries(s05p2 utils)

## Day6
add_executable(s06p1 src/star06/part1.cpp)
target_link_libraries(s06p1 utils)

add_executable(s06p2 src/star06/part2.cpp)
target_link_libraries(s06p2 utils)

## Day7
add_executable(s07p1 src/star07/part1.cpp)
target_link_libraries(s07p1 utils)

add_executable(s07p2 src/star07/part2.cpp)
target_link_libraries(s07p2 utils)

## Day8
add_executable(s08p1 src/star08/part1.cpp)
target_link_libraries(s08p1 utils)

add_executable(s08p2 src/star08/part2.cpp)
target_link_libraries(s08p2 utils)

## Day10
add_executable(s10p1 src/star10/part1.cpp)
target_link_libraries(s10p1 utils)

add_executable(s10p2 src/star10/part2.cpp)
target_link_libraries(s10p2 utils)

## Day11
add_executable(s11p1 src/star11/part1.cpp)
target_link_libraries(s11p1 utils)

add_executable(s11p2 src/star11/part2.cpp)
target_link_libraries(s11p2 utils)

## Day12
add_executable(s12p1 src/star12/part1.cpp)
target_link_libraries(s12p1 utils)

add_executable(s12p2 src/star12/part2.cpp)
target_link_libraries(s12p2 utils)

## Day13
add_executable(s13p1 src/star13/part1.cpp)
target_link_libraries(s13p1 utils)

add_executable(s13p2 src/star13/part2.cpp)
target_link_libraries(s13p2 utils)

## Day14
add_executable(s14p1 src/star14/part1.cpp)
target_link_libraries(s14p1 utils)

add_executable(s14p2 src/star14/part2.cpp)
target_link_libraries(s14p2 utils)

## Day16
add_executable(s16p1 src/star16/part1.cpp)
target_link_libraries(s16p1 utils)

add_executable(s16p2 src/star16/part2.cpp)
target_link_libraries(s16p2 utils)
