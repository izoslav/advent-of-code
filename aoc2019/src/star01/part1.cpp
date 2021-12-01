#include <fmt/format.h>
#include <reading.h>

#include "common.h"

#define CATCH_CONFIG_RUNNER
#include <catch.hpp>

using namespace utils;

int main()
{
  Catch::Session().run();

  std::vector<int> masses = reading::read_input<int>("inputs/star01.txt");

  int sum = calculate_fuel(masses);

  fmt::print("Required fuel: {}\n", sum);

  return 0;
}

TEST_CASE( "Test inputs", "[s01p1]" )
{
  std::vector<int> input = {12, 14, 1969, 100756};

  REQUIRE(calculate_fuel(input) == 34241);
}