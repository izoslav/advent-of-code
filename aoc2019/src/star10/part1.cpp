#include <algorithm>
#include <string>
#include <map>

#include "reading.h"
#include "printing.h"

using namespace utils;

int main()
{
  auto inputMap = reading::read_input<std::string>("inputs/star10.txt");
  int const Y = inputMap.size();
  int const X = inputMap[0].size();

  std::vector<std::vector<int>> visibilityMap(Y);
  for (auto& row : visibilityMap)
  {
    row.resize(X);
  }

  for (int y = 0; y < Y; ++y)
  {
    for (int x = 0; x < X; ++x)
    {
      if (inputMap[y][x] != '#') continue;

      // calculate LOSes
      std::map<double, double> los;

      for (int y2 = 0; y2 < Y; ++y2)
      {
        for (int x2 = 0; x2 < X; ++x2)
        {
          if (x == x2 && y == y2) continue;
          if (inputMap[y2][x2] != '#') continue;

          double const angle = std::atan2(x2 - x, y2 - y);
          double const distance = std::hypot(x2 - x, y2 - y);

//          if (x == 1 && y == 2)
//            fmt::print("a {} {} b {} {} angle {} distance {}\n", x, y, x2, y2, angle, distance);

          if (los.count(angle))
          {
            if (los[angle] < distance) los[angle] = distance;
          }
          else
          {
            los[angle] = distance;
          }

//          if (x == 1 && y == 2)
//            fmt::print("visibles {}\n", los.size());
        }
      }

      // count
      visibilityMap[y][x] = los.size();
      los.clear();
    }
  }

  for (auto const& row : visibilityMap)
  {
    fmt::print("{}\n", printing::vector_to_string(row));
  }

  int ymax = 0;
  int xmax = 0;
  int vmax = 0;

  for (int y = 0; y < Y; ++y)
  {
    for (int x = 0; x < X; ++x)
    {
      if (vmax < visibilityMap[y][x])
      {
        vmax = visibilityMap[y][x];
        ymax = y;
        xmax = x;
      }
    }
  }

  fmt::print("x{} y{} v{}\n", xmax, ymax, vmax);

  return 0;
}