#include <map>
#include <utility>
#include <cstdlib>
#include <cmath>

#include <reading.h>
#include <printing.h>

using namespace utils;

int main()
{
    auto wire1 = reading::read_input<std::string>("inputs/star03-1.txt");
    auto wire2 = reading::read_input<std::string>("inputs/star03-2.txt");

    std::map<std::pair<int, int>, int> wire1map;
    std::map<std::pair<int, int>, int> wire2map;

    auto mark = [](int x, int y, int dist, std::map<std::pair<int, int>, int>& map)
    {
        std::pair<int, int> coords = {x, y};
        int curdist = map[coords];

        if (curdist == 0) {
            map[coords] = dist;
        }
    };

    auto walk = [&](std::vector<std::string> const& wire, std::map<std::pair<int, int>, int>& map) {
        int x = 0;
        int y = 0;

        for (auto const& step : wire)
        {
            int dist = std::atoi(step.data() + 1);

            switch(step[0])
            {
                case 'U':
                    for (int i = 1; i <= dist; ++i) mark(x, y + i, 1, map);
                    y += dist;
                    break;
                case 'D':
                    for (int i = 1; i <= dist; ++i) mark(x, y - i, 1, map);
                    y -= dist;
                    break;
                case 'L':
                    for (int i = 1; i <= dist; ++i) mark(x - i, y, 1, map);
                    x -= dist;
                    break;
                case 'R':
                    for (int i = 1; i <= dist; ++i) mark(x + i, y, 1, map);
                    x += dist;
                    break;
            }
        }
    };

    walk(wire1, wire1map);
    walk(wire2, wire2map);

    int mindist = 10000000;
    for (auto const& step : wire1map)
    {
        auto coords = step.first;

        if (wire2map.count(coords) != 0)
        {
            int dist = std::abs(coords.first) + std::abs(coords.second);

            if (dist < mindist)
            {
                mindist = dist;
            }
        }
    }

    fmt::print("Lowest distance: {}\n", mindist);

    return 0;
}