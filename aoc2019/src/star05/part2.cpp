#include <vector>
#include <cmath>

#include "reading.h"
#include "printing.h"
#include "intcode.h"

using namespace utils;

int& getparam(std::vector<int>& memory, int const& address, int const& mode = 0, int const& position = 0)
{
    bool immediate = mode & (int)std::pow(10, position - 1);
//    bool immediate = (int)(mode / std::pow(10, position - 1)) % 10 == 1;

    if (immediate)
    {
//        fmt::print("immediate {} value {} address {}\n", immediate, memory[address], address);
        return memory[address];
    }
    else
    {
//        fmt::print("immediate {} value {} address {}\n", immediate, memory[memory[address]], memory[address]);
        return memory[memory[address]];
    }
}

void process(std::vector<int>& memory, int& address)
{
    int const& instruction = memory[address];
    int const& code = memory[address] % 100;
    int const mode = instruction / 100;

//    fmt::print("i{} c{} m{} a{}\n", instruction, code, mode, address);

    switch(code)
    {
        // ADD
        case 1: {
            int const &value1 = getparam(memory, address + 1, mode, 1);
            int const &value2 = getparam(memory, address + 2, mode, 2);
            int &output = getparam(memory, address + 3);
            bool selfmodify = (output == address);

            output = value1 + value2;
            if (!selfmodify) address += 4;

            break;
        }
        // MUL
        case 2: {
            int const &value1 = getparam(memory, address + 1, mode, 1);
            int const &value2 = getparam(memory, address + 2, mode, 2);
            int &output = getparam(memory, address + 3);
            bool selfmodify = (output == address);

            output = value1 * value2;
            if (!selfmodify) address += 4;

            break;
        }
        // INPUT
        case 3: {
            //int const& value = getparam(memory, address + 1, mode, 1);
            int& output = getparam(memory, address + 1);
            bool selfmodify = (output == address);

            output = 5; // RADIATOR TEST MODE
            if (!selfmodify) address += 2;

            break;
        }
        // OUTPUT
        case 4: {
            fmt::print("output: {}\n", getparam(memory, address + 1, mode, 1));
            address += 2;
            break;
        }
        // JMPT
        case 5: {
            int const &test = getparam(memory, address + 1, mode, 1);
            int const &jump = getparam(memory, address + 2, mode, 2);

            if (test != 0) {
                address = jump;
            }
            else
            {
                address += 3;
            }

            break;
        }
        // JMPF
        case 6: {
            int const &test = getparam(memory, address + 1, mode, 1);
            int const &jump = getparam(memory, address + 2, mode, 2);

            if (test == 0) {
                address = jump;
            }
            else
            {
                address += 3;
            }

            break;
        }
        // LT
        case 7: {
            int const &value1 = getparam(memory, address + 1, mode, 1);
            int const &value2 = getparam(memory, address + 2, mode, 2);
            int &output = getparam(memory, address + 3);
            bool selfmodify = (output == address);

            output = (value1 < value2) ? 1 : 0;
            if (!selfmodify) address += 4;

            break;
        }
        // EQ
        case 8: {
            int const &value1 = getparam(memory, address + 1, mode, 1);
            int const &value2 = getparam(memory, address + 2, mode, 2);
            int &output = getparam(memory, address + 3);
            bool selfmodify = (output == address);

            output = (value1 == value2) ? 1 : 0;
            if (!selfmodify) address += 4;

            break;
        }
        default:
            break;
    }
}

std::vector<int>& parse(std::vector<int>& memory)
{
    for (int i = 0; i < memory.size();)
    {
        if (memory[i] == 99)
        {
            return memory;
        }

        // parameters are pointers!
        process(memory, i);
    }

    return memory;
}

int main()
{
    std::vector<int> codes = reading::read_input<int>("inputs/star05.txt", true);

    intcode::Interpreter interpreter(codes, false);

    interpreter.push(5);
    interpreter.run();

    fmt::print("Output: {}\n", interpreter.pop());

    return 0;
}