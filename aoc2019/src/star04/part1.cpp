#include <algorithm>
#include <string>

#include "printing.h"

int main()
{
    int lo = 123257;
    int hi = 647015;

    int count = 0;
    for (int i = lo; i <= hi; ++i)
    {
        auto password = std::to_string(i);

        if (std::adjacent_find(begin(password), end(password)) == end(password)) continue;
        if (!std::is_sorted(begin(password), end(password))) continue;

        ++count;
    }

    fmt::print("Combinations: {}\n", count);

    return 0;
}