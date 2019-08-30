#pragma once

#include <iostream>

namespace utils
{
static auto print = [](auto &&... args) -> void {
  (std::cout << ... << args) << '\n';
};
}