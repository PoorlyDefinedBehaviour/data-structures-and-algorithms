#pragma once

#include <memory>
#include <stdexcept>
#include <utility>

template <typename T>
class DoubleLinkedList;

template <typename T>
class Node
{
private:
  T data;

  std::shared_ptr<Node<T>> previous = nullptr;
  std::shared_ptr<Node<T>> next = nullptr;

  friend class DoubleLinkedList<T>;

  // O(n)
  template <typename Fn>
  constexpr auto right_traverse(Fn const &fn) const -> void
  {
    fn(*this);

    if (next)
      next->right_traverse(std::forward<decltype(fn)>(fn));
  }

  // O(n)
  template <typename Fn>
  constexpr auto left_traverse(Fn const &fn) const -> void
  {
    fn(*this);

    if (previous)
      previous->left_traverse(std::forward<decltype(fn)>(fn));
  }

  // O(n)
  auto insert_at(size_t current_index, size_t target_index, T const &value) -> void
  {
    if (current_index + 1 == target_index)
    {
      auto temp = next;
      next = std::make_shared<Node<T>>(value);
      next->next = temp;
    }
    else
    {
      next->insert_at(current_index + 1, target_index, value);
    }
  }

  // O(n)
  constexpr auto remove_at(size_t current_index, size_t target_index) -> void
  {
    if (current_index + 1 == target_index)
      next = std::move(next->next);
    else
      next->remove_at(current_index + 1, target_index);
  }

  // O(n)
  constexpr auto remove(T &&value) -> void
  {
    if (!next)
      return;

    if (value == next->data)
      next = std::move(next->next);
    else
      next->remove(std::forward<T>(value));
  }

  // O(n)
  constexpr auto find(T &&value) -> std::shared_ptr<Node<T>>
  {
    if (data == value)
      return std::shared_ptr<Node<T>>(this);

    if (next != nullptr)
      return next->find(std::forward<T>(value));

    return nullptr;
  }

  // O(n)
  constexpr auto at(size_t current_index, size_t target_index) const -> std::shared_ptr<Node<T>>
  {
    if (current_index + 1 == target_index)
      return next;
    return next->at(current_index + 1, target_index);
  }

public:
  constexpr Node(const T &value) : data(value) {}

  constexpr auto get_data() const -> T { return data; }
  constexpr auto has_next() const -> bool { return next != nullptr; }
  constexpr auto has_previous() const -> bool { return previous != nullptr; }
};

template <typename T>
class DoubleLinkedList
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
      return;
    }

    if (index == size - 1)
    {
      auto temp = _tail;
      _tail = std::make_shared<Node<T>>(value);
      _tail->previous = temp;
      temp->next = _tail;
      return;
    }

    _head->insert_at(0, index, value);
  }

  // O(1)
  constexpr auto insert(T &&value) -> void
  {
    insert_at(size, std::forward<T>(value));
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
    else
    {
      _head->remove_at(0, index);
    }
  }

  // O(n)
  constexpr auto remove(T &&value) -> void
  {
    if (value == _tail->get_data())
      _tail = _tail->previous;
    else
      _head->remove(std::forward<T>(value));

    --size;
  }

  // O(1)
  constexpr auto pop() -> std::shared_ptr<Node<T>>
  {
    assert_not_empty();
    auto temp = _tail;
    _tail = _tail->previous;
    --size;
    return temp;
  }

  // O(n)
  constexpr auto find(T &&value) const -> std::shared_ptr<Node<T>>
  {
    if (!_head)
      return nullptr;

    return _head->find(std::forward<T>(value));
  }

  // O(n)
  constexpr auto at(size_t index) const -> std::shared_ptr<Node<T>>
  {
    assert_index_is_valid(index);

    if (index == 0)
      return _head;
    if (index == size - 1)
      return _tail;

    return _head->at(0, index);
  }

  // O(n)
  constexpr auto includes(T &&value) const -> bool
  {
    return static_cast<bool>(find(std::forward<T>(value)));
  }

  // O(n)
  template <typename Fn>
  constexpr auto left_traverse(Fn const &fn) -> void
  {
    _head->left_traverse(std::forward<decltype(fn)>(fn));
  }

  // O(n)
  template <typename Fn>
  constexpr auto right_traverse(Fn const &fn) -> void
  {
    _head->right_traverse(std::forward<decltype(fn)>(fn));
  }

  // O(1)
  constexpr auto tail() const -> std::shared_ptr<Node<T>>
  {
    return _tail;
  }
};

template <typename T>
auto operator<<(std::ostream &stream, DoubleLinkedList<T> list) -> std::ostream &
{
  list.right_traverse([&stream](auto node) -> void {
    stream << node.get_data();

    if (node.has_next())
      stream << " <-> ";
  });
  return stream;
}