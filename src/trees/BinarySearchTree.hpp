#pragma once

#include <memory>
#include <functional>
#include <tuple>

template <typename T>
struct Node
{
  T data;
  std::shared_ptr<Node<T>> left = nullptr;
  std::shared_ptr<Node<T>> right = nullptr;

  Node(T const &data) : data(data) {}
};

template <typename T>
class BinarySearchTree
{
private:
  std::function<int(T const &, T const &)> comparator = [](T const &lhs, T const &rhs) -> int {
    if (lhs < rhs)
      return -1;
    if (lhs > rhs)
      return 1;
    return 0;
  };
  std::shared_ptr<Node<T>> root = nullptr;
  size_t _size = 0;

  constexpr auto insert_impl(std::shared_ptr<Node<T>> &node, T const &value) -> void
  {
    if (node == nullptr)
    {
      node = std::make_shared<Node<T>>(value);
      return;
    }

    if (comparator(value, node->data) == -1)
      insert_impl(node->left, value);
    else if (comparator(value, node->data) == 1)
      insert_impl(node->right, value);
  }

  constexpr auto find_node_and_parent_by_value(std::shared_ptr<Node<T>> parent, std::shared_ptr<Node<T>> node, T const &value) const -> std::tuple<std::shared_ptr<Node<T>>, std::shared_ptr<Node<T>>>
  {
    if (node == nullptr)
      return std::make_tuple(nullptr, nullptr);

    if (comparator(node->data, value) == 0)
      return std::make_tuple(parent, node);
    if (comparator(node->data, value) == -1)
      return find_node_and_parent_by_value(node, node->left, value);
    if (comparator(node->data, value) == 1)
      return find_node_and_parent_by_value(node, node->left, value);
  }

public:
  BinarySearchTree() = default;
  BinarySearchTree(std::function<int(T const &, T const &)> const &comparator)
      : comparator(comparator) {}

  constexpr auto size() const -> size_t { return _size; }
  constexpr auto is_empty() const -> bool { return size() == 0; }

  constexpr auto insert(T const &value) -> BinarySearchTree<T>
  {
    insert_impl(root, value);
    ++_size;
    return *this;
  }

  constexpr auto remove(T const &value) -> BinarySearchTree<T>
  {
    auto [node_to_delete_parent, node_to_delete] = find_node_and_parent_by_value(nullptr, root, value);
    if (node_to_delete_parent == nullptr && node_to_delete == nullptr)
    {
      return *this;
    }

    --_size;

    auto biggest = node_to_delete->left;
    if (biggest == nullptr)
    {
      node_to_delete = node_to_delete->right;
      return *this;
    }

    std::shared_ptr<Node<T>> parent = nullptr;
    while (biggest->right != nullptr)
    {
      parent = biggest;
      biggest = biggest->right;
    }

    biggest->right = node_to_delete->right;
    if (parent != nullptr)
    {
      parent->right = biggest->left;
      biggest->left = node_to_delete->left;
    }

    return *this;
  }
};
