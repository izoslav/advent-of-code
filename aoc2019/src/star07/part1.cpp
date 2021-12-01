#include <vector>
#include <cmath>

#include "reading.h"
#include "printing.h"
#include "intcode.h"

using namespace utils;

int main()
{
  std::vector<int> codes = reading::read_input<int>("inputs/star07.txt", true);
  std::vector<int> combination = {0,1,2,3,4};

  intcode::Interpreter interpreter(codes, false);

  std::vector<std::pair<std::string, int>> outputs;

  do
  {
    int output = 0;

    for (auto const& phase : combination)
    {
      interpreter.push(output);
      interpreter.push(phase);

      interpreter.run();

      output = interpreter.pop();
      interpreter.reset();
    }

    std::string comb = printing::vector_to_string(combination);

    outputs.push_back({comb, output});
  } while (std::next_permutation(begin(combination), end(combination)));

  auto best = std::max_element(
    begin(outputs),
    end(outputs),
    [](auto e1, auto e2) {
      return e2.second > e1.second;
    }
  );
  fmt::print("best combination {} with output {}\n", best->first, best->second);

  return 0;
}