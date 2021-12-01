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

    std::vector<std::string> you;
    std::vector<std::string> san;

    {
        auto it = planets.find("YOU");

        while (it->second != "")
        {
            you.push_back(it->second);
            it = planets.find(it->second);
        }
    }

    {
        auto it = planets.find("SAN");

        while (it->second != "")
        {
            san.push_back(it->second);
            it = planets.find(it->second);
        }
    }

    auto mismatch = std::mismatch(
        rbegin(you), rend(you),
        rbegin(san), rend(san)
    );

    int jumps = (rend(you) - mismatch.first) + (rend(san) - mismatch.second);

    fmt::print("Jumps needed {}\n", jumps);
    return 0;
}