#include <vector>
#include <fmt/format.h>

namespace utils::printing
{

template<typename T>
void print_vector(std::vector<T> const& v)
{
  fmt::print("{}\n", fmt::join(begin(v), end(v), ","));
}

template<typename T>
std::string vector_to_string(std::vector<T> const& v, std::string const& separator = ",")
{
  return fmt::format("{}", fmt::join(begin(v), end(v), separator));
}

template <typename Container>
std::string container_to_string(Container const& c, std::string const& separator = ",")
{
  return fmt::format("{}", fmt::join(begin(c), end(c), separator));
}

} // namespace utils::printing