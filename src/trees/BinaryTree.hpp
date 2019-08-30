#pragma once

#include <memory>

template <typename T>
class BinaryTree;

template <typename T>
class Node
{
private:
  T data;
  std::unique_ptr<Node<T>> left = nullptr;
  std::unique_ptr<Node<T>> right = nullptr;

  friend class BinaryTree<T>;

  auto insert(const T &value) -> void
  {
    if (value < data)
    {
      if (left == nullptr)
      {
        left = std::make_unique<Node<T>>(value);
      }
      else
      {
        left->insert(value);
      }
    }
    else
    {
      if (right == nullptr)
      {
        right = std::make_unique<Node<T>>(value);
      }
      else
      {
        right->insert(value);
      }
    }
  }

  auto includes(const T &value) const -> bool
  {
    if (value == data)
      return true;

    if (value < data)
      return left == nullptr ? false : left->includes(value);

    return right == nullptr ? false : right->includes(value);
  }

  auto find(const T &value) const -> Node<T> *
  {
    if (value == data)
      return this;

    if (value < data)
      return left == nullptr ? nullptr : left->find(value);

    return right == nullptr ? nullptr : right->find(value);
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
  Node(const T &data) : data(data) {}
};

template <typename T>
class BinaryTree
{
private:
  size_t size = 0;
  std::unique_ptr<Node<T>> head = nullptr;

public:
  auto length() const -> size_t { return size; }

  auto insert(const T &value) -> void
  {
    if (head == nullptr)
      head = std::make_unique<Node<T>>(value);
    else
      head->insert(value);

    ++size;
  }

  auto includes(const T &value) const -> bool { return head->includes(value); }

  auto find(const T &value) -> Node<T> *
  {
    if (head == nullptr)
      return nullptr;

    return head->find(value);
  }

  auto print() const -> void { head->print(); }
};
