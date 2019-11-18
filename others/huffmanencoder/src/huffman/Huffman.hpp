#pragma once

#include <vector>
#include <string>
#include <unordered_map>
#include <tuple>
#include <memory>
#include <algorithm>
#include <cstddef>
#include <stdexcept>
#include <utility>

#include "../hash/HashTable.hpp"
#include "../console/Console.hpp"
#include "../utils/Quicksort.hpp"

struct Node
{
  char character;
  int frequency;
  std::shared_ptr<Node> left = nullptr;
  std::shared_ptr<Node> right = nullptr;

  Node(char character, int frequency) : character(character), frequency(frequency) {}
};

auto create_frequency_table(std::string const &text) -> HashTable<char, int>
{
  HashTable<char, int> table(text.length());

  for (auto character : text)
  {
    if (table.has(character))
      table.set(character, table.get(character) + 1);
    else
      table.set(character, 1);
  }

  return table;
}

auto create_nodes(HashTable<char, int> const &frequency_table) -> std::vector<std::shared_ptr<Node>>
{
  std::vector<std::shared_ptr<Node>> nodes;
  nodes.reserve(frequency_table.size());

  for (auto const &[character, frequency] : frequency_table.entries())
  {
    nodes.emplace_back(std::make_shared<Node>(character, frequency));
  }
  return nodes;
}

auto sort_nodes(std::vector<std::shared_ptr<Node>> nodes) -> std::vector<std::shared_ptr<Node>>
{
  quicksort(nodes, 0, nodes.size() - 1, [](auto const &lhs, auto const &rhs) {
    return lhs->frequency < rhs->frequency;
  });
  return nodes;
}

auto pop_first_two_elements(std::vector<std::shared_ptr<Node>> &nodes) -> std::tuple<std::shared_ptr<Node>, std::shared_ptr<Node>>
{
  auto first = nodes[0];
  auto second = nodes[1];
  nodes.erase(std::begin(nodes));
  nodes.erase(std::begin(nodes));
  return std::make_tuple(first, second);
}

auto insert(std::vector<std::shared_ptr<Node>> &nodes, std::shared_ptr<Node> const &node) -> void
{
  for (size_t i = 0; i < nodes.size(); ++i)
  {
    if (node->frequency < nodes[i]->frequency)
    {
      nodes.insert(std::begin(nodes) + i, node);
      return;
    }
  }

  nodes.emplace_back(node);
}

auto create_tree(HashTable<char, int> &frequency_table) -> std::shared_ptr<Node>
{
  std::shared_ptr<Node> root;
  std::vector<std::shared_ptr<Node>> nodes = sort_nodes(std::move(create_nodes(frequency_table)));

  while (nodes.size() > 1)
  {
    auto [first, second] = pop_first_two_elements(nodes);

    std::shared_ptr<Node> node = std::make_shared<Node>('\0', first->frequency + second->frequency);
    node->left = first;
    node->right = second;

    root = node;

    insert(nodes, node);
  }

  return root;
}

auto create_tree_from_code_table(HashTable<char, std::string> const &code_table) -> std::shared_ptr<Node>
{
  std::shared_ptr<Node> root = std::make_shared<Node>('\0', -1);

  for (auto const &[key, code] : code_table.entries())
  {
    std::shared_ptr<Node> current_node = root;

    for (auto character : code)
    {
      if (character == '0')
      {
        if (!current_node->left)
        {
          current_node->left = std::make_shared<Node>('\0', -1);
        }
        current_node = current_node->left;
      }
      else
      {
        if (!current_node->right)
        {
          current_node->right = std::make_shared<Node>('\0', -1);
        }
        current_node = current_node->right;
      }
    }

    current_node->character = key;
  }

  return root;
}

auto is_leaf(std::shared_ptr<Node> const &node) -> bool
{
  return node->left == nullptr && node->right == nullptr;
}

auto set_path_to_leafs(HashTable<char, std::string> &code_table,
                       std::shared_ptr<Node> const &node,
                       std::string const &code = "") -> void
{
  if (is_leaf(node))
  {
    code_table.set(node->character, code);
  }
  else
  {
    set_path_to_leafs(code_table, node->left, code + '0');
    set_path_to_leafs(code_table, node->right, code + '1');
  }
}

auto create_code_table(
    std::shared_ptr<Node> const &root,
    std::size_t const table_capacity) -> HashTable<char, std::string>
{
  HashTable<char, std::string> code_table(table_capacity);

  set_path_to_leafs(code_table, root);

  return code_table;
}

auto encode(HashTable<char, std::string> &code_table, std::string const &text) -> std::string
{
  std::string result = "";

  for (auto character : text)
  {
    result += code_table.get(character);
  }

  return result;
}

auto decode(HashTable<char, std::string> const &code_table, std::string const &text) -> std::string
{
  std::string result = "";
  std::shared_ptr<Node> tree_root = create_tree_from_code_table(code_table);

  std::shared_ptr<Node> current_node = tree_root;

  for (auto character : text)
  {
    if (character == '0')
    {
      current_node = current_node->left;
    }
    else
    {
      current_node = current_node->right;
    }

    if (is_leaf(current_node))
    {
      result += current_node->character;
      current_node = tree_root;
    }
  }

  return result;
}

auto is_null_node(std::shared_ptr<Node> const &node) -> bool
{
  return node->character == '\0';
}

auto pre_order_traversal(std::shared_ptr<Node> const &node) -> void
{
  if (node == nullptr)
    return;

  if (!is_null_node(node))
    Console::println(node->character, ":", node->frequency);

  pre_order_traversal(node->left);
  pre_order_traversal(node->right);
}

auto difference(std::string const &lhs, std::string const &rhs) -> int
{
  constexpr int BYTE_SIZE = 8;
  return BYTE_SIZE * lhs.length() - rhs.length() / BYTE_SIZE;
}