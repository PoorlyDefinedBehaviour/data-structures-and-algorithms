#pragma once

#include <memory>
#include <stdexcept>
#include <utility>

template <typename T>
struct Node
{
  T data;

  std::shared_ptr<Node<T>> previous = nullptr;
  std::shared_ptr<Node<T>> next = nullptr;

  constexpr Node(const T &value) : data(value) {}
};

template <typename T>
class CircularDoubleLinkedList
{
private:
  constexpr auto assert_index_is_valid(size_t index) const -> void
  {
    if (index < 0 || index > size)
      throw std::range_error("Invalid index");
  }

  constexpr auto assert_not_empty() const -> void
  {
    if (size == 0)
    {
      throw std::logic_error("Trying to perform an operation on an empty list");
    }
  }

  std::shared_ptr<Node<T>> _head = nullptr;
  std::shared_ptr<Node<T>> _tail = nullptr;

  size_t size = 0;

public:
  constexpr auto length() const -> size_t { return size; }
  constexpr auto is_empty() const -> bool { return length() == 0; }

  // O(1)
  constexpr auto head() const -> std::shared_ptr<Node<T>>
  {
    return _head;
  }

  // O(n)
  constexpr auto insert_at(size_t index, T const &value) -> void
  {
    assert_index_is_valid(index);

    ++size;

    if (_head == nullptr)
    {
      _head = std::make_shared<Node<T>>(value);
      _tail = _head;
      return;
    }

    if (index == 0)
    {
      auto temp = _head;
      _head = std::make_shared<Node<T>>(value);
      _head->next = temp;
      temp->previous = _head;
      return;
    }

    if (index == size - 1)
    {
      std::shared_ptr<Node<T>> node = std::make_shared<Node<T>>(value);
      node->next = _head;
      _tail->next = node;
      _tail = node;

      return;
    }

    std::shared_ptr<Node<T>> current = _head;
    size_t current_index = 0;
    while (current_index + 1 != index)
    {
      _head = _head->next;
      ++current_index;
    }

    std::shared_ptr<Node<T>> node = std::make_shared<Node<T>>(value);
    node->next = current->next;
    node->previous = current;
    current->next->previous = node;
    current->next = node;
  }

  // O(1)
  constexpr auto insert(T const &value) -> void
  {
    insert_at(size, value);
  }

  // O(n)
  constexpr auto remove_at(size_t index) -> void
  {
    --size;

    assert_index_is_valid(index);

    if (index == 0)
    {
      _head = std::move(_head->next);

      if (size == 0)
        _tail = _head;
    }
    else if (index == size)
    {
      _tail = _tail->previous;
      _tail->next = _head;
    }
    else
    {
      std::shared_ptr<Node<T>> current = _head;
      size_t current_index = 0;
      while (current_index + 1 != index)
      {
        current = current->next;
        ++current_index;
      }
      current->next = current->next->next;
      current->next->previous = current;
    }
  }

  // O(1)
  constexpr auto pop() -> std::shared_ptr<Node<T>>
  {
    assert_not_empty();
    auto temp = _tail;
    _tail = _tail->previous;
    _tail->next = _head;
    --size;
    return temp;
  }

  // O(n)
  template <typename Function>
  constexpr auto find(Function const &fn) const -> std::shared_ptr<Node<T>>
  {
    std::shared_ptr<Node<T>> current = _head;
    size_t index = 0;
    while (current && index != size - 1)
    {
      fn(current);
      current = current->next;
      ++index;
    }
    return current;
  }

  // O(n)
  constexpr auto at(size_t index) const -> std::shared_ptr<Node<T>>
  {
    assert_index_is_valid(index);

    std::shared_ptr<Node<T>> current = _head;
    size_t current_index = 0;
    while (current_index != index)
    {
      current = current->next;
      ++current_index;
    }
    return current;
  }

  // O(n)
  constexpr auto includes(T const &value) const -> bool
  {
    return find(value) != nullptr;
  }

  // O(n)
  template <typename Fn>
  constexpr auto left_traverse(Fn const &fn) -> void
  {
    std::shared_ptr<Node<T>> current = _head;
    size_t index = 0;
    while (current && index != size)
    {
      fn(current);
      current = current->previous;
      ++index;
    }
  }

  // O(n)
  template <typename Fn>
  constexpr auto right_traverse(Fn const &fn) -> void
  {
    std::shared_ptr<Node<T>> current = _head;
    size_t index = 0;
    while (current && index != size)
    {
      fn(current);
      current = current->next;
      ++index;
    }
  }

  // O(1)
  constexpr auto tail() const -> std::shared_ptr<Node<T>>
  {
    return _tail;
  }
};

template <typename T>
auto operator<<(std::ostream &stream, CircularDoubleLinkedList<T> list) -> std::ostream &
{
  list.right_traverse([&stream](auto const &node) -> void {
    stream << node->data << " <-> ";
  });

  return stream;
}