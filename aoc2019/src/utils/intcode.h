#include <vector>
#include <stack>
#include <string>

namespace utils::intcode
{

// old
std::vector<int>& parse(std::vector<int>& memory);
void process(int const& code, int const& value1, int const& value2, int& output);

// new
class Interpreter
{
private:
  using value_t = long long;
  using memory_t = std::vector<value_t>;
  using list_t = std::deque<value_t>;

public:
  enum Instruction
  {
      ADD     = 1,  // Add
      MUL     = 2,  // Multiply
      INPUT   = 3,
      OUTPUT  = 4,
      JMP     = 5,  // Jump if
      NJMP    = 6,  // Jump if not
      LT      = 7,  // Less than
      EQ      = 8,  // Equals
      ARB     = 9,  // Adjust relative base
      EXIT    = 99
  };

  enum AccessMode
  {
    POSITION,   // Pointers
    IMMEDIATE,  // Direct access
    RELATIVE,   // Calculated using relative base
    UNKNOWN = -1
  };

  enum logtag_t
  {
      DEBUG,
      INFO
  };

  Interpreter();
  Interpreter(bool debugMode);
  Interpreter(memory_t initMemory, bool debugMode);

  void reset();
  void run();
  void runUntilInput();

  bool isRunning() const;

  void push(value_t const& value);

  bool hasOutput() const;
  int pop();

  memory_t const& data() const;

private:
  void log(logtag_t const& tag, std::string const& message);
  void step(value_t const& opcode);

  // operations
  void add(value_t const& mode);
  void mul(value_t const& mode);
  void input();
  void output();
  void jmp(value_t const& mode);
  void njmp(value_t const& mode);
  void lt(value_t const& mode);
  void eq(value_t const& mode);
  void arb(value_t const& mode);

  // operation helpers
  value_t getOp(value_t const& op);
  value_t getModesCode(value_t const& op);
  value_t isImmediate(value_t const& position, value_t const& mode);

  AccessMode getAccessMode(value_t const& position, value_t const& modesCode);

  value_t& getParam(value_t const& position, value_t const& modesCode = 0);
  value_t getAddress(value_t const& address, AccessMode const& mode);

  bool isModyfingIP(value_t const& output);

  memory_t initRam;
  memory_t ram;
  list_t inputMemory;
  list_t outputMemory;

  bool debugMode;
  size_t ip; // instruction pointer
  size_t rb; // relative base
};

} // namespace utils::intcode