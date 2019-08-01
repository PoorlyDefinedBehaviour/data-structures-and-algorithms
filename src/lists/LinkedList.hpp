#pragma once

#include <memory>

template <typename T>
class LinkedList;

template <typename T>
class Node
{
private:
  T data;
  std::unique_ptr<Node<T>> next = nullptr;

  friend class LinkedList<T>;

  auto insert(const T &value) -> void
  {
    if (next == nullptr)
      next = std::make_unique<Node<T>>(value);
    else
      next->insert(value);
  }

  auto remove(const T &value) -> void
  {
    if (value == left->data)
      next = std::move(next->next);
    else
      next->remove(value);
  }

  auto find(const T &value) -> Node<T> *
  {
    if (data == value)
      return this;

    if (next != nullptr)
      return next.find(value);

    return nullptr;
  }

  auto last() -> Node<T> *
  {
    if (next == nullptr)
      return this;
    return next->last();
  }

public:
  Node(const T &value) : data(value) {}

  auto get_data() -> T { return data; }
};

template <typename T>
class LinkedList
{
private:
  std::unique_ptr<Node<T>> head = nullptr;
  size_t size = 0;

public:
  auto length() const -> size_t { return size; }

  auto insert(const T &value) -> void
  {
    ++size;

    if (head == nullptr)
      head = std::make_unique<Node<T>>(value);
    else
      head->insert(value);
  }

  auto remove(const T &value) -> void
  {
    head->remove(value);
  }

  auto find(const T &value) const -> Node<T> *
  {
    if (!head)
      return nullptr;

    head->find(value);
  }

  auto first() const -> Node<T> *
  {
    return head;
  }

  auto last() const -> Node<T> *
  {
    if (head == nullptr)
      return nullptr;

    return head->last();
  }
};