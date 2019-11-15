#pragma once

#include <functional>
#include <memory>
#include <algorithm>

template <typename T>
struct Node
{
  T data;
  int height = 1;
  std::shared_ptr<Node<T>> left = nullptr;
  std::shared_ptr<Node<T>> right = nullptr;

  Node(T const &data)
      : data(data) {}
};

template <typename T>
class AvlTree
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

  auto is_less_than(T const &a, T const &b) const -> bool
  {
    return comparator(a, b) == -1;
  }

  auto is_greater_than(T const &a, T const &b) const -> bool
  {
    return comparator(a, b) == 1;
  }

  auto is_equal(T const &a, T const &b) const -> bool
  {
    return comparator(a, b) == 0;
  }

  template <typename Function>
  auto pre_order_traversal_impl(std::shared_ptr<Node<T>> &node, Function const &fn) -> void
  {
    if (node == nullptr)
    {
      return;
    }
    fn(node);
    pre_order_traversal_impl(node->left, fn);
    pre_order_traversal_impl(node->right, fn);
  }

  template <typename Function>
  auto in_order_traversal_impl(std::shared_ptr<Node<T>> &node, Function const &fn) -> void
  {
    if (node == nullptr)
    {
      return;
    }
    in_order_traversal_impl(node->left, fn);
    fn(node);
    in_order_traversal_impl(node->right, fn);
  }

  template <typename Function>
  auto post_order_traversal_impl(std::shared_ptr<Node<T>> &node, Function const &fn) -> void
  {
    if (node == nullptr)
    {
      return;
    }
    post_order_traversal_impl(node->left, fn);
    post_order_traversal_impl(node->right, fn);
    fn(node);
  }

  auto insert_impl(std::shared_ptr<Node<T>> &node, T const &value) -> std::shared_ptr<Node<T>>
  {
    if (node == nullptr)
    {
      return std::make_shared<Node<T>>(value);
    }

    if (is_less_than(value, node->data))
    {
      node->left = insert_impl(node->left, value);
    }
    else if (is_greater_than(value, node->data))
    {
      node->right = insert_impl(node->right, value);
    }
    else
    {
      return node;
    }

    set_height(node);

    constexpr int BALANCE_TO_ROTATE_RIGHT = 1;
    constexpr int BALANCE_TO_ROTATE_LEFT = -1;
    int const balance = get_balance(node);

    if (balance > BALANCE_TO_ROTATE_RIGHT && is_less_than(value, node->left->data))
    {
      return rotate_right(node);
    }

    if (balance < BALANCE_TO_ROTATE_LEFT && is_greater_than(value, node->right->data))
    {
      return rotate_left(node);
    }

    if (balance > BALANCE_TO_ROTATE_RIGHT && is_greater_than(value, node->left->data))
    {
      node->left = rotate_left(node->left);
      return rotate_right(node);
    }

    if (balance < BALANCE_TO_ROTATE_LEFT && is_less_than(value, node->right->data))
    {
      node->right = rotate_right(node->right);
      return rotate_left(node);
    }

    return node;
  }

  auto set_height(std::shared_ptr<Node<T>> &node) -> void
  {
    node->height = 1 + std::max(height(node->left), height(node->right));
  }

  auto height(std::shared_ptr<Node<T>> const &node) const -> int
  {
    return node == nullptr ? 0 : node->height;
  }

  auto get_balance(std::shared_ptr<Node<T>> const &node) const -> int
  {
    return node == nullptr ? 0 : height(node->left) - height(node->right);
  }

  auto rotate_right(std::shared_ptr<Node<T>> &node) -> std::shared_ptr<Node<T>>
  {
    auto a = node->left;
    auto b = a->right;

    a->right = node;
    node->left = b;

    set_height(node);
    set_height(a);

    return a;
  }

  auto rotate_left(std::shared_ptr<Node<T>> &node) -> std::shared_ptr<Node<T>>
  {
    auto a = node->right;
    auto b = a->left;

    a->left = node;
    node->right = b;

    set_height(node);
    set_height(a);
    return a;
  }

  auto min_node(std::shared_ptr<Node<T>> const &node) const -> std::shared_ptr<Node<T>>
  {
    std::shared_ptr<Node<T>> current = node;

    while (current->left != nullptr)
    {
      current = current->left;
    }

    return current;
  }

  auto count_children(std::shared_ptr<Node<T>> const &node) const -> int
  {
    return (node->left == nullptr) + (node->right = nullptr);
  }

public:
  AvlTree() = default;
  AvlTree(std::function<int(T const &, T const &)> const &comparator)
      : comparator(comparator) {}

  auto insert(T const &value) -> AvlTree &
  {
    root = insert_impl(root, value);
    return *this;
  }

  auto remove(std::shared_ptr<Node<T>> &node) -> AvlTree &
  {
    if (root == nullptr)
    {
      return root;
    }

    if (is_less_than(node->data, root->data))
    {
      root->left = remove(root->left);
    }
    else if (is_greater_than(node->data, root->data))
    {
      root->right = remove(root->right);
    }
    else
    {
      if (count_children(node) < 2)
      {
        std::shared_ptr<Node<T>> temp = root->left != nullptr
                                            ? root->left
                                            : root->right;
        if (temp == nullptr)
        {
          temp = std::exchange(root, nullptr);
        }
        else
        {
          std::swap(root, temp);
        }
      }
      else
      {
        std::shared_ptr<Node<T>> temp = min_node(root->right);
        root->data = temp->data;
        root->right = remove(temp);
      }
    }

    return *this;
  }

  auto find(T const &value) const -> std::shared_ptr<Node<T>>
  {
    if (root == nullptr)
    {
      return root;
    }

    std::shared_ptr<Node<T>> current = root;

    while (current != nullptr)
    {
      if (is_equal(value, current->data))
      {
        return current;
      }

      current = is_less_than(value, current->data)
                    ? current->left
                    : current->right;
    }

    return nullptr;
  }

  template <typename Function>
  constexpr auto pre_order_traversal(Function &&fn) -> AvlTree<T> &
  {
    pre_order_traversal_impl(root, std::forward<Function>(fn));

    return *this;
  }

  template <typename Function>
  constexpr auto in_order_traversal(Function &&fn) -> AvlTree<T> &
  {
    in_order_traversal_impl(root, std::forward<Function>(fn));

    return *this;
  }

  template <typename Function>
  constexpr auto post_order_traversal(Function &&fn) -> AvlTree<T> &
  {
    post_order_traversal_impl(root, std::forward<Function>(fn));

    return *this;
  }
};