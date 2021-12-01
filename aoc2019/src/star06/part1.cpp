#include <algorithm>
#include <string>
#include <map>

#include "reading.h"
#include "printing.h"

using namespace utils;

//struct Planet
//{
//    std::string name;
//    Planet* orbits;
//};

using Planet = std::pair<std::string, std::string>;

int main()
{
    std::vector<std::string> input = reading::read_input<std::string>("inputs/star06.txt");
    std::map<std::string, std::string> planets;

    for (auto const& row : input)
    {
        size_t delim = row.find(")");
        std::string p1 = row.substr(0, delim);
        std::string p2 = row.substr(delim + 1, row.size());

        planets.insert({p1, ""});
        planets.insert({p2, ""});
    }

    for (auto const& row : input)
    {
        size_t delim = row.find(")");
        std::string p1 = row.substr(0, delim);
        std::string p2 = row.substr(delim + 1, row.size());

        auto planetIt = planets.find(p2);
        auto orbitsIt = planets.find(p1);

        if (orbitsIt != end(planets))
        {
            planetIt->second = orbitsIt->first;
        }
    }

    int orbits = 0;

    for (auto const& planet : planets)
    {
        auto it = planets.find(planet.first);

        while (it->second != "")
        {
//            fmt::print("{})", it->firs)
            ++orbits;
            it = planets.find(it->second);
        }

//        int local = 0;
//        Planet const* temp = &planet;
//
//        while (temp)
//        {
//            ++local;
//            temp = temp->orbits;
//        }
//
//        fmt::print("{} - {}\n", planet.name, local);
//        orbits += local;
    }

    fmt::print("Number of orbits: {}\n", orbits);

    return 0;
}