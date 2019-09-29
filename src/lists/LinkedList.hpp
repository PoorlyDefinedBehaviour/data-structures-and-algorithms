#pragma once

#include <memory>
#include <stdexcept>

template <typename T>
struct Node
{
  T data;
  std::shared_ptr<Node<T>> next = nullptr;

  constexpr Node(const T &value) : data(value) {}
};

template <typename T>
class LinkedList
{
private:
  std::shared_ptr<Node<T>> _head = nullptr;
  std::shared_ptr<Node<T>> _tail = nullptr;

  size_t size = 0;

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

  template <typename Function>
  constexpr auto find_impl(std::shared_ptr<Node<T>> const &node, Function const &comparator) -> std::shared_ptr<Node<T>>
  {
    if (node == nullptr)
    {
      return nullptr;
    }
    if (comparator(node->data))
    {
      return node;
    }
    return find_impl(node->next, comparator);
  }

  constexpr auto insert_at_impl(size_t index, T const &value) -> void
  {
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
      return;
    }
    if (index == size - 1)
    {
      auto temp = _tail;
      _tail = std::make_shared<Node<T>>(value);
      temp->next = _tail;
      return;
    }

    size_t current_index = 0;

    std::shared_ptr<Node<T>> current = _head;
    while (current_index + 1 != index)
    {
      current = current->next;
    }

    auto temp = current->next;
    current->next = std::make_shared<Node<T>>(value);
    current->next->next = temp;
  }

  constexpr auto remove_at_impl(size_t index) -> void
  {
    std::shared_ptr<Node<T>> current = _head;
    size_t current_index = 0;
    while (current_index + 1 == index)
    {
      current = current->next;
      ++current_index;
    }

    current->next = std::move(current->next->next);
  }

  constexpr auto at(size_t index) const -> std::shared_ptr<Node<T>>
  {
    std::shared_ptr<Node<T>> current = _head;
    size_t current_index = 0;
    while (current_index + 1 == index)
    {
      current = current->next;
      ++current_index;
    }
    return current->next;
  }

public:
  constexpr auto length() const -> size_t { return size; }
  constexpr auto is_empty() const -> bool { return length() == 0; }

  // O(n)
  constexpr auto insert_at(size_t index, T const &value) -> void
  {
    assert_index_is_valid(index);
    ++size;

    insert_at_impl(index, value);
  }

  // O(1)
  constexpr auto insert(T &&value) -> void
  {
    insert_at(size, std::forward<T>(value));
  }

  // O(n)
  constexpr auto remove_at(size_t index) -> void
  {
    assert_index_is_valid(index);
    --size;

    if (index == 0)
    {
      _head = std::move(_head->next);

      if (size == 0)
        _tail = _head;
    }
    else
    {
      remove_at_impl(index);
    }
  }

  // O(1)
  constexpr auto pop() -> std::shared_ptr<Node<T>>
  {
    assert_not_empty();

    if (size == 1)
    {
      auto result = _head;
      _head = nullptr;
      _tail = nullptr;
      --size;
      return result;
    }

    auto result = _tail;

    traverse([this](auto &node) -> void {
      if (node.next->data == _tail->data)
      {
        _tail = std::make_shared<Node<T>>(node.data);
        node.next = nullptr;
      }
    });

    --size;
    return result;
  }

  // O(1)
  constexpr auto shift() -> std::shared_ptr<Node<T>>
  {
    assert_not_empty();
    auto temp = _head;
    _head = _head->next;
    --size;
    return temp;
  }

  // O(n)
  template <typename Function>
  constexpr auto find(Function const &comparator) -> std::shared_ptr<Node<T>>
  {
    return find_impl(_head, comparator);
  }

  // O(n)
  template <typename Function>
  constexpr auto find_index(Function const &comparator) -> size_t
  {
    size_t current_index = 0;
    std::shared_ptr<Node<T>> current = _head;
    while (current != nullptr)
    {
      if (comparator(current->data))
      {
        return current_index;
      }
      current = current->next;
      ++current_index;
    }
    return -1;
  }

  // O(n)
  constexpr auto includes(T const &value) const -> bool
  {
    return find([&value](auto const &v) -> bool { return v == value; }) != nullptr;
  }

  template <typename Function>
  constexpr auto traverse(Function const &fn) -> void
  {
    std::shared_ptr<Node<T>> current = _head;
    fn(current);
    while (current->next != nullptr)
    {
      current = current->next;
      fn(current);
    }
  }

  // O(1)
  constexpr auto head() const -> std::shared_ptr<Node<T>>
  {
    return _head;
  }

  // O(1)
  constexpr auto tail() const -> std::shared_ptr<Node<T>>
  {
    return _tail;
  }
};

template <typename T>
auto operator<<(std::ostream &stream, LinkedList<T> list) -> std::ostream &
{
  list.traverse([&stream](auto node) -> void { stream << node.get_data(); if(!node.is_last()) stream << " -> "; });
  return stream;
}