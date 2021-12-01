#include <fstream>
#include <vector>

namespace utils::reading
{

template<typename T>
std::vector<T> read_input(std::string filename, bool commadelimetered = false)
{
  std::vector<T> output;
  std::ifstream inputfile(filename);
  T in;
  
  while(inputfile >> in)
  {
    output.emplace_back(in);

    if (commadelimetered)
    {
      inputfile.ignore();
    }
  }

  return output;
}

} // namespace utils::reading