#include <algorithm>
#include <string>
#include <map>
#include <numeric>

#include "reading.h"
#include "printing.h"

using namespace utils;

struct Vec3
{
  int x{0};
  int y{0};
  int z{0};

  int absSum() const
  {
    return std::abs(x) + std::abs(y) + std::abs(z);
  }
};

struct Moon
{
  Vec3 position;
  Vec3 velocity;

  bool operator==(Moon const& other)
  {
    return this == &other;
  }

  int getPotentialEnergy() const
  {
    return position.absSum();
  }

  int getKineticEnergy() const
  {
    return velocity.absSum();
  }

  int getTotalEnergy() const
  {
    return getPotentialEnergy() * getKineticEnergy();
  }

  std::string to_string() const
  {
    return fmt::format(
      "Moon: pos(x{:>5} y{:>5} z {:>5}), vel(x{:>5} y{:>5} z{:>5}), energy: {:>5}",
      position.x,
      position.y,
      position.z,
      velocity.x,
      velocity.y,
      velocity.z,
      getTotalEnergy()
    );
  }
};

int main()
{
  std::vector<Moon> moons;

  {
    std::ifstream inputfile("inputs/star12.txt");

    for (std::string line; std::getline(inputfile, line);) {
      Moon moon;
      std::sscanf(
        line.c_str(),
        "<x=%d, y=%d, z=%d>\n",
        &moon.position.x,
        &moon.position.y,
        &moon.position.z
      );

      moons.push_back(moon);
    }
  }

  // simulation
  auto const applyGravity = [](int& p1, int& v1, int& p2, int& v2)
  {
    if (p1 < p2)
    {
      ++v1;
      --v2;
    }

    if (p1 > p2)
    {
      --v1;
      ++v2;
    }
  };

  auto const applyVelocity = [](Moon& moon)
  {
    moon.position.x += moon.velocity.x;
    moon.position.y += moon.velocity.y;
    moon.position.z += moon.velocity.z;
  };

  fmt::print("Simulation start\n");
  for (auto const& moon : moons)
  {
    fmt::print("{}\n", moon.to_string());
  }

  int steps = 1000;
  for (int step = 1; step <= steps; ++step)
  {
    fmt::print("Simulation step #{}\n", step);

    for (int i = 0; i < moons.size(); ++i)
    {
      for (int j = i + 1; j < moons.size(); ++j)
      {
        applyGravity(
          moons[i].position.x,
          moons[i].velocity.x,
          moons[j].position.x,
          moons[j].velocity.x
        );

        applyGravity(
          moons[i].position.y,
          moons[i].velocity.y,
          moons[j].position.y,
          moons[j].velocity.y
        );

        applyGravity(
          moons[i].position.z,
          moons[i].velocity.z,
          moons[j].position.z,
          moons[j].velocity.z
        );
      }

      applyVelocity(moons[i]);
    }

    for (auto const& moon : moons)
    {
      fmt::print("{}\n", moon.to_string());
    }

    auto energySum = std::accumulate(
      begin(moons),
      end(moons),
      0,
      [](int sum, Moon const& moon) {
        return sum + moon.getTotalEnergy();
      });

    fmt::print("Total energy of system: {}\n", energySum);
  }

  return 0;
}