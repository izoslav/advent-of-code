#include <numeric>
#include <fmt/format.h>

#include <reading.h>

#include "common.h"

#define CATCH_CONFIG_RUNNER
#include <catch.hpp>

using namespace utils;

int calculate_total_fuel(std::vector<int> const& masses)
{
  std::vector<int> additionalfuels;

  for (auto const& mass : masses)
  {
    int additionalfuel = 0;
    int additionalmass = mass;

    while (additionalmass / 3 - 2 > 0)
    {
      additionalmass = (additionalmass / 3 - 2);
      additionalfuel += additionalmass;
    }

    additionalfuels.emplace_back(additionalfuel);
  }

  return std::accumulate(
    begin(additionalfuels),
    end(additionalfuels),
    0
  );
}

int main()
{
  Catch::Session().run();

  std::vector<int> masses = reading::read_input<int>("input/star01.txt");

  int modulesfuel = calculate_fuel(masses);
  fmt::print("Fuel needed for the modules: {}\n", modulesfuel);

  int totalfuel = calculate_total_fuel(masses);
  fmt::print("Total fuel required: {}\n", totalfuel);

  return 0;
}

TEST_CASE( "Test inputs", "[s01p2]" )
{
  std::vector<int> input = {100756};

  REQUIRE(calculate_total_fuel(input) == 50346);
}