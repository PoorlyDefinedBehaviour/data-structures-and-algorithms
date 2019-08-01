#pragma once

#include <utility>
#include <memory>

template <typename T>
class DoubleLinkedList;

template <typename T>
struct Node
{
private:
  T data;
  std::unique_ptr<Node<T>> left = nullptr;
  std::unique_ptr<Node<T>> right = nullptr;

  friend class DoubleLinkedList<T>;

  auto insert_right(T const &value) -> void
  {
    if (right == nullptr)
      right = std::make_unique<Node<T>>(value);
    else
      right->insert_right(value);
  }

  auto insert_left(T const &value) -> void
  {
    if (left == nullptr)
      left = std::make_unique<Node<T>>(value);
    else
      left->insert_left(value);
  }

  auto remove_right(T const &value) -> void
  {
    if (value == right->data)
      right = std::move(right->right);
    else
      right->remove_right(value);
  }

  auto remove_left(T const &value) -> void
  {
    if (value == left->data)
      left = std::move(left->left);
    else
      left->remove_left(value);
  }

  auto find_right(T const &value) -> Node<T> *
  {
    if (data == value)
      return this;
    else if (right != nullptr)
      return right->find_right(value);

    return nullptr;
  }

  auto find_left(T const &value) -> Node<T> *
  {
    if (data == value)
      return this;
    else if (left != nullptr)
      return left->find_left(value);

    return nullptr;
  }

  auto print() const -> void
  {
    if (left != nullptr)
      left->print();

    utils::print(data);

    if (right != nullptr)
      right->print();
  }

public:
  Node(T const &value) : data(value) {}

  auto get_data() -> T & { return data; }
};

template <typename T>
class DoubleLinkedList
{
private:
  std::unique_ptr<Node<T>> start = nullptr;
  size_t size_right = 0;
  size_t size_left = 0;

public:
  auto length() -> std::pair<size_t, size_t> { return std::make_pair(size_left, size_right); }

  auto insert_right(T const &value) -> void
  {
    ++size_right;
    if (start == nullptr)
      start = std::make_unique<Node<T>>(value);
    else
      start->insert_right(value);
  }

  auto insert_left(T const &value) -> void
  {
    ++size_left;
    if (start == nullptr)
      start = std::make_unique<Node<T>>(value);
    else
      start->insert_left(value);
  }

  auto head() const -> Node<T> *
  {
    return start.get();
  }

  auto find_right(T const &value) -> Node<T> *
  {
    if (start == nullptr)
      return nullptr;
    return start->find_right(value);
  }

  auto find_left(T const &value) const -> Node<T> *
  {
    if (start == nullptr)
      return nullptr;
    return start->find_left(value);
  }

  auto remove_right(T const &value) const -> void
  {
    if (start != nullptr)
      start->remove_right(value);
  }

  auto remove_left(T const &value) const -> void
  {
    if (start != nullptr)
      start->remove_left(value);
  }

  auto print() const -> void
  {
    if (start != nullptr)
      start->print();
  }
};