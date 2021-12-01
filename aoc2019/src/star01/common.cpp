#include "common.h"

#include <numeric>

int calculate_fuel(std::vector<int> const& masses)
{
  return std::accumulate(
    begin(masses),
    end(masses),
    0,
    [](int sum, int mass) {
      return sum + (mass / 3 - 2);
    }
  );
}
