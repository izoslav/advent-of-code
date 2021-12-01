#include <vector>
#include <fmt/format.h>

#include <reading.h>
#include <printing.h>
#include <intcode.h>

using namespace utils;

int main()
{
  std::vector<int> codes = reading::read_input<int>("inputs/star02.txt", true);

  int target{ 19690720 };
  int arg1{ 0 };
  int arg2{ 0 };
  bool hit1{ false };
  bool hit2{ false };

  for (;;)
  {
    codes[1] = arg1;
    codes[2] = arg2;

    intcode::Interpreter interpreter(codes, false);
    interpreter.run();

    auto const& temp = interpreter.data();

    if (temp[0] == target)
    {
      fmt::print("answer: {}\n", temp[1] * 100 + temp[2]);
      break;
    }

    if (!hit1 && temp[0] > target)
    {
      --arg1;
      hit1 = true;
    }

    if (!hit1)
    {
      ++arg1;
      continue;
    }

    if (!hit2)
    {
      ++arg2;
      continue;
    }
  }

  return 0;
}