#pragma once

#include <iostream>

class Console
{
public:
  template <typename... Ts>
  static auto print(Ts const &... args) -> void { (std::cout << ... << args); }

  template <typename... Ts>
  static auto println(Ts const &... args) -> void { print(args..., '\n'); }
};