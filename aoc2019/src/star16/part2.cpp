#include <numeric>
#include "reading.h"
#include "printing.h"

using namespace utils;

int main()
{
  //std::vector<int> input = { 6,9,3,1,7,1,6,3,4,9,2,9,4,8,6,0,6,3,3,5,9,9,5,9,2,4,3,1,9,8,7,3 };

  std::ifstream inputfile("inputs/star16.txt");
  std::string inputStr;
  inputfile >> inputStr;

  std::vector<int> input;
  for (auto const& s : inputStr)
  {
    input.push_back(s - '0');
  }

  // repeat input
  input.reserve(10000 * input.size());
  int n = 9999 * input.size();
  std::copy_n(begin(input), n, std::back_inserter(input));

  // get offset
  int offset = std::stoi(std::accumulate(
    begin(input), begin(input) + 7,
    std::string{},
    [](std::string acc, int v) {
      return acc + std::to_string(v);
    }));

  int inputSize = input.size();
  std::vector<int> pattern = { 0, 1, 0, -1 };
  std::vector<std::vector<int>> patterns(input.size());

  for (int position = 0; position < patterns.size(); ++position)
  {
    int repeats = position + 1;
    int symbols = 0;

    for (;;)
    {
      for (auto const& symbol : pattern)
      {
        for (int i = 0; i < repeats; ++i)
        {
          patterns[position].push_back(symbol);
          symbols++;
        }
      }

      if (symbols > inputSize + 1)
      {
        patterns[position].erase(begin(patterns[position]));
        patterns[position].resize(inputSize);
        break;
      }
    }
  }

  std::vector<int> output(input.size());

  int phases = 100;
  for (int phase = 0; phase < phases; ++phase)
  {
    for (int position = 0; position < inputSize; ++position)
    {
      int sum = 0;

      for (int i = 0; i < inputSize; ++i)
      {
        sum += input[i] * patterns[position][i];
      }

      output[position] = std::abs(sum) % 10;
    }

    //printing::print_vector(output);

    input = output;
  }

  std::vector<int> printout(begin(output) + offset, begin(output) + offset + 8);
  printing::print_vector(printout);

  return 0;
}