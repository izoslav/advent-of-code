#include <fmt/core.h>
#include <cmath>
#include "intcode.h"

namespace utils::intcode
{
// old
std::vector<int>& parse(std::vector<int>& memory)
{
  for (int i = 0; i < memory.size(); i += 4)
  {
    if (memory[i] == 99)
    {
      return memory;
    }

    // parameters are pointers!
    process(
      memory[i],              // code
      memory[memory[i + 1]],  // param1
      memory[memory[i + 2]],  // param2
      memory[memory[i + 3]]   // output
    );
  }

  return memory;
}

void process(int const& code, int const& value1, int const& value2, int& output)
{
  switch(code)
  {
    case 1:
      output = value1 + value2;
      break;
    case 2:
      output = value1 * value2;
      break;
    default:
      break;
  }
}

//
Interpreter::Interpreter()
: Interpreter(true)
{
}

Interpreter::Interpreter(bool debugMode)
: debugMode(debugMode),
  ip(0),
  rb(0)
{}

Interpreter::Interpreter(memory_t initMemory, bool debugMode = false)
: initRam(initMemory),
  ram(initMemory),
  debugMode(debugMode),
  ip(0),
  rb(0)
{
}

void Interpreter::reset()
{
  ip = 0;
  rb = 0;
  inputMemory = std::deque<value_t>();
  outputMemory = std::deque<value_t>();
  ram.clear();
  ram = initRam;
}

void Interpreter::run()
{
  while(ip < ram.size())
  {
    auto& opcode = ram[ip];

    if (opcode == Instruction::EXIT) return;

    step(opcode);
  }
}

void Interpreter::runUntilInput()
{
  while(ip < ram.size())
  {
    auto& opcode = ram[ip];

    if (opcode == Instruction::EXIT) return;

    if (opcode == Instruction::INPUT)
    {
      if (inputMemory.empty()) return;
    }

    step(opcode);
  }
}

bool Interpreter::isRunning() const
{
  return ram[ip] != Instruction::EXIT;
}

void Interpreter::step(value_t const& opcode)
{
  int op = getOp(opcode);
  int mode = getModesCode(opcode);

  switch (op)
  {
    case ADD:     add(mode); break;
    case MUL:     mul(mode); break;
    case INPUT:   input(); break;
    case OUTPUT:  output(); break;
    case JMP:     jmp(mode); break;
    case NJMP:    njmp(mode); break;
    case LT:      lt(mode); break;
    case EQ:      eq(mode); break;
    case ARB:     arb(mode); break;
    default:
      log(INFO, fmt::format("[ERROR] Unknown op {}!", op));
      break;
  }
}

void Interpreter::log(logtag_t const& tag, std::string const& message)
{
  if (tag != DEBUG || debugMode)
  {
    fmt::print("[ip {}] {}\n", ip, message);
  }
}

void Interpreter::push(value_t const& value)
{
  inputMemory.push_back(value);
}

bool Interpreter::hasOutput() const
{
  return !outputMemory.empty();
}

int Interpreter::pop()
{
  if (outputMemory.empty())
  {
    fmt::print("[ERROR] Output stack is empty!\n");
    return -1;
  }

  int value = outputMemory.front();
  outputMemory.pop_front();
  return value;
}

Interpreter::memory_t const& Interpreter::data() const
{
  return ram;
}

// Operations

void Interpreter::add(value_t const& mode)
{
  value_t& a = getParam(1, mode);
  value_t& b = getParam(2, mode);
  value_t& output = getParam(3, mode);

  output = a + b;

  if (!isModyfingIP(output)) ip += 4;
}

void Interpreter::mul(value_t const& mode)
{
  value_t& a = getParam(1, mode);
  value_t& b = getParam(2, mode);
  value_t& output = getParam(3, mode);

  output = a * b;

  if (!isModyfingIP(output)) ip += 4;
}

void Interpreter::input()
{
  value_t& output = getParam(1);

  if (!inputMemory.empty())
  {
    output = inputMemory.front();
    inputMemory.pop_front();
  }
  else
  {
    log(INFO, "[Error] Trying to read empty stack.");
    ram[ip] = 99;
    return;
  }

  if (!isModyfingIP(output)) ip += 2;
}

void Interpreter::output()
{
  value_t& output = getParam(1);

  outputMemory.push_back(output);

  if (!isModyfingIP(output)) ip += 2;
}

void Interpreter::jmp(value_t const& mode)
{
  value_t const& test = getParam(1, mode);
  value_t const& target = getParam(2, mode);

  ip = (test != 0) ? target : ip + 3;
}

void Interpreter::njmp(value_t const& mode)
{
  value_t const& test = getParam(1, mode);
  value_t const& target = getParam(2, mode);

  ip = (test == 0) ? target : ip + 3;
}

void Interpreter::lt(value_t const& mode)
{
  value_t const& a = getParam(1, mode);
  value_t const& b = getParam(2, mode);
  value_t& output = getParam(3, mode);

  output = (a < b);

  if (!isModyfingIP(output)) ip += 4;
}

void Interpreter::eq(value_t const& mode)
{
  value_t const& a = getParam(1, mode);
  value_t const& b = getParam(2, mode);
  value_t& output = getParam(3, mode);

  output = (a == b);

  if (!isModyfingIP(output)) ip += 4;
}

void Interpreter::arb(value_t const& mode)
{
  value_t const& v = getParam(1, mode);

  rb += v;

  ip += 2;
}

// Operation helpers
Interpreter::value_t Interpreter::getOp(value_t const& op)
{
  return op % 100;
}

Interpreter::value_t Interpreter::getModesCode(value_t const& op)
{
  return op / 100;
}

Interpreter::value_t Interpreter::isImmediate(value_t const& position, value_t const& mode)
{
  return mode & (value_t)std::pow(10, position - 1);
}

Interpreter::AccessMode Interpreter::getAccessMode(value_t const& position, value_t const& modesCode)
{
  value_t upper = std::pow(10, position);
  value_t lower = std::pow(10, position - 1);
  value_t mode = (modesCode % upper) / lower;

  switch (mode)
  {
    case 0: return POSITION;
    case 1: return IMMEDIATE;
    case 2: return RELATIVE;
    default: return UNKNOWN;
  }
}

Interpreter::value_t& Interpreter::getParam(value_t const& position, value_t const& modesCode)
{
  value_t address = ip + position;

  AccessMode mode = getAccessMode(position, modesCode);

  switch (mode)
  {
    case POSITION: address = ram[address]; break;
    case IMMEDIATE: address = address; break;
    case RELATIVE: address = rb + ram[address]; break;
    default: break;
  }

  return ram[address];
}

bool Interpreter::isModyfingIP(value_t const& output)
{
  value_t const* p1 = &output;
  value_t const* p2 = &ram[ip];
  return &output == &(ram[ip]);
}

} // namespace utils::intcode