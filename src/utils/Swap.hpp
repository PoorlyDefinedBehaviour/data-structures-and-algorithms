#pragma once

namespace utils
{
static auto swap = [](auto &&a, auto &&b) -> void {
  auto temp = a;
  a = std::move(b);
  b = std::move(temp);
};
}