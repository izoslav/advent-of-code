#include <algorithm>
#include <string>
#include <map>
#include <deque>

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

  // LASER!
  std::map<double, std::vector<std::pair<int, int>>> lasermap;
  for (int y = 0; y < Y; ++y)
  {
    for (int x = 0; x < X; ++x)
    {
      if (x == xmax && y == ymax) continue;
      if (inputMap[y][x] != '#') continue;

      constexpr double PI = 3.141592653589793238463;
      double angle = std::atan2(y - ymax, x - xmax);

      if (angle < 0) angle += PI*2;
      angle += PI/2;
      angle = std::fmod(angle, 2*PI);
      //angle = angle * (180 / PI);

      //double const distance = std::hypot(x - xmax, y - ymax);

      lasermap[angle].push_back({x, y});
    }
  }

  for (auto& angle : lasermap)
  {
    auto& asteroids = lasermap[angle.first];

    std::sort(
      begin(asteroids),
      end(asteroids),
      [&](auto const& p1, auto const& p2)
      {
        auto const d1 = std::hypot(p1.first - xmax, p1.second - ymax);
        auto const d2 = std::hypot(p2.first - xmax, p2.second - ymax);
        return d1 < d2;
      }
    );
  }

//  for (auto const& angle : lasermap)
//  {
//    fmt::print("{}: ", angle.first);
//    for (auto const& point : angle.second)
//    {
//      auto distance = std::hypot(point.first - xmax, point.second - ymax);
//      fmt::print("({},{})[{}] ", point.first, point.second, distance);
//    }
//    fmt::print("\n");
//  }

  int asteroids = 0;
  for (auto const& angle : lasermap)
  {
    asteroids += lasermap[angle.first].size();
  }

  fmt::print("asteroid count: {}\n", asteroids);

  for (int i = 0; i < asteroids;)
  {
    for (auto& angle : lasermap)
    {
      auto& row = lasermap[angle.first];

      if (row.size() == 0) continue;

      auto& asteroid = row[0];
      fmt::print("a {} n {} x {} y {}\n", angle.first, i++ + 1, asteroid.first, asteroid.second);
      row.erase(begin(row));
    }
  }

  return 0;
}