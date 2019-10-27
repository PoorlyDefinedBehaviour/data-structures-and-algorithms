#pragma once

#include <memory>

enum class NodeColor
{
  RED,
  BLACK
};

template <typename T>
struct Node
{
  T data;
  NodeColor color;
  std::shared_ptr<Node<T>> left;
  std::shared_ptr<Node<T>> right;

  Node(T const &data)
      : data(data) {}

  Node(T const &data, NodeColor const &color)
      : data(data), color(color) {}
};

class RedBlackTree
{
private:
public:
};