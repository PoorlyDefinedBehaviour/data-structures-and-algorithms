#pragma once

#include <string>
#include <fstream>

auto read_file(std::string path) -> std::string
{
  std::ifstream file(path);
  std::string file_contents((std::istreambuf_iterator<char>(file)),
                            std::istreambuf_iterator<char>());

  return file_contents;
}