#include <vector>
#include <fmt/format.h>

#include <reading.h>
#include <printing.h>
#include <intcode.h>

using namespace utils;

int main()
{
  std::vector<int> codes = reading::read_input<int>("inputs/star02.txt", true);

  codes[1] = 12;
  codes[2] = 2;

  intcode::Interpreter interpreter(codes, false);
  interpreter.run();

  fmt::print("codes[0] = {}\n", interpreter.data()[0]);

  return 0;
}