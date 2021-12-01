#include <regex>
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

        bool pair{false};
        for (int j = 0; j < password.size(); ++j)
        {
            int k = j + 1;
            for (; k < password.size(); ++k)
            {
                if (password[j] != password[k]) break;
            }

            if (k - j == 2)
            {
                pair = true;
                break;
            }

            j = k - 1;
        }
        if (!pair) continue;

        if (!std::is_sorted(begin(password), end(password))) continue;

        ++count;
    }

    fmt::print("Combinations: {}\n", count);

    return 0;
}