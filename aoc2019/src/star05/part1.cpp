#include <vector>
#include <cmath>

#include "reading.h"
#include "printing.h"
#include "intcode.h"

using namespace utils;

int main()
{
    std::vector<int> codes = reading::read_input<int>("inputs/star05.txt", true);

    intcode::Interpreter interpreter(codes, false);

    interpreter.push(1);
    interpreter.run();

    fmt::print("Output: {}\n", interpreter.pop());

    return 0;
}