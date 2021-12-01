#include <algorithm>
#include <string>
#include <map>
#include <numeric>
#include <string>
#include <set>

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

  bool operator==(Vec3 const& other) {
    return x == other.x
        && y == other.y
        && z == other.z;
  }
};

struct Moon
{
  Vec3 position;
  Vec3 velocity;

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
      "Moon: pos(x{:>5} y{:>5} z {:>5}), vel(x{:>5} y{:>5} z{:>5})",
      position.x,
      position.y,
      position.z,
      velocity.x,
      velocity.y,
      velocity.z
    );
  }

  std::string hash() const
  {
    return std::to_string(std::hash<std::string>{}(to_string()));
  }

  bool operator==(Moon const& other)
  {
    return position == other.position
        && velocity == other.velocity;
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

  std::set<std::string> states;

  auto const getStateHash = [&]() {
    return std::accumulate(
      begin(moons),
      end(moons),
      std::string{},
      [](std::string acc, Moon const& moon) {
        return acc + moon.hash();
      });
  };

  auto const prevState = [&]() {
    auto const hash = getStateHash();

    return states.contains(hash);
  };

  int steps = 0;
  for (int step = 1; !prevState(); ++step)
  {
    fmt::print("Simulation step #{}\n", step);
    states.insert(getStateHash());

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
  }

  return 0;
}