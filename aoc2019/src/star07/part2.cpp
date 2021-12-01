#include <vector>
#include <cmath>

#include "reading.h"
#include "printing.h"
#include "intcode.h"

using namespace utils;

int main()
{
  std::vector<long long> codes = reading::read_input<long long>("inputs/star07.txt", true);
  std::vector<int> combination = {5,6,7,8,9};

  std::vector<std::pair<std::string, int>> outputs;

  do
  {
    intcode::Interpreter A(codes, false);
    intcode::Interpreter B(codes, false);
    intcode::Interpreter C(codes, false);
    intcode::Interpreter D(codes, false);
    intcode::Interpreter E(codes, false);

    // Set phases
    A.push(combination[0]);
    B.push(combination[1]);
    C.push(combination[2]);
    D.push(combination[3]);
    E.push(combination[4]);

    // Init value
    A.push(0);
    int output = 0;
    int n = 0;

    while (E.isRunning())
    {
      if (A.isRunning()) { A.runUntilInput(); B.push(A.pop()); }
      if (B.isRunning()) { B.runUntilInput(); C.push(B.pop()); }
      if (C.isRunning()) { C.runUntilInput(); D.push(C.pop()); }
      if (D.isRunning()) { D.runUntilInput(); E.push(D.pop()); }
      if (E.isRunning()) E.runUntilInput();

      output = E.pop();

      if (E.isRunning()) A.push(output);
    }

    std::string comb = printing::vector_to_string(combination);

    outputs.push_back({comb, output});
    fmt::print("Output: {}\n", output);
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