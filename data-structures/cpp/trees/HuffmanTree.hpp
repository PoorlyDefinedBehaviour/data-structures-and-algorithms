#pragma once

#include <vector>
#include <string>
#include <unordered_map>
#include <tuple>
#include <memory>
#include <algorithm>
#include <utility>
#include <stdexcept>

struct Node
{
  char character;
  int frequency;
  std::shared_ptr<Node> left;
  std::shared_ptr<Node> right;

  Node(char character, int frequency) : character(character), frequency(frequency) {}
};

class Huffman
{
private:
  static auto createFrequencyTable(std::string_view text) -> std::unordered_map<char, int>
  {
    std::unordered_map<char, int> table;

    for (auto character : text)
    {
      auto &value = table[character];
      value += 1;
    }

    return table;
  }

  static auto createNodes(std::unordered_map<char, int> const &frequencyTable) -> std::vector<std::shared_ptr<Node>>
  {
    std::vector<std::shared_ptr<Node>> nodes;
    nodes.reserve(frequencyTable.size());

    for (auto const &[character, frequency] : frequencyTable)
    {
      nodes.emplace_back(std::make_shared<Node>(character, frequency));
    }
    return nodes;
  }

  static auto sortNodes(std::vector<std::shared_ptr<Node>> nodes) -> std::vector<std::shared_ptr<Node>>
  {
    std::sort(std::begin(nodes), std::end(nodes), [](auto const &lhs, auto const &rhs) {
      return lhs->frequency < rhs->frequency;
    });

    return nodes;
  }

  static auto createTreeFromFrequencyTable(std::unordered_map<char, int> &frequencyTable,
                                           std::vector<std::shared_ptr<Node>> &nodes) -> std::shared_ptr<Node>
  {
    std::shared_ptr<Node> root;

    while (nodes.size() > 1)
    {
      auto [firstElement, secondElement] = popFirstTwoElements(nodes);
      auto frequency = frequencyTable[firstElement->character] + frequencyTable[secondElement->character];

      auto newNode = std::make_shared<Node>('$', frequency);
      newNode->left = firstElement;
      newNode->right = secondElement;

      if (nodes.empty())
      {
        root = newNode;
      }

      insert(nodes, std::move(newNode));
    }

    return nodes.empty() ? root : nodes[0];
  }

  static auto createTreeFromCodeTable(std::unordered_map<char, std::string> const &codeTable) -> std::shared_ptr<Node>
  {
    std::shared_ptr<Node> root = std::make_shared<Node>('$', -1);

    for (auto const &[key, code] : codeTable)
    {
      std::shared_ptr<Node> currentNode = root;

      for (auto character : code)
      {
        if (character == '0')
        {
          if (!currentNode->left)
          {
            currentNode->left = std::make_shared<Node>('$', -1);
          }
          currentNode = currentNode->left;
        }
        else
        {
          if (!currentNode->right)
          {
            currentNode->right = std::make_shared<Node>('$', -1);
          }
          currentNode = currentNode->right;
        }
      }

      currentNode->character = key;
    }

    return root;
  }

  static auto createCodeTable(
      std::shared_ptr<Node> const &root) -> std::unordered_map<char, std::string>
  {
    std::unordered_map<char, std::string> codeTable;

    setPathToleafs(codeTable, root);

    return codeTable;
  }

  static auto setPathToleafs(std::unordered_map<char, std::string> &codeTable,
                             std::shared_ptr<Node> const &node,
                             std::string const &code = "") -> void
  {
    if (isLeaf(node))
    {
      codeTable[node->character] = code;
    }
    else
    {
      setPathToleafs(codeTable, node->left, code + '0');
      setPathToleafs(codeTable, node->right, code + '1');
    }
  }

  static auto isLeaf(std::shared_ptr<Node> const &node) -> bool
  {
    return node->left == nullptr && node->right == nullptr;
  }

  static auto insert(std::vector<std::shared_ptr<Node>> &nodes, std::shared_ptr<Node> const &node) -> void
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

  static auto popFirstTwoElements(std::vector<std::shared_ptr<Node>> &nodes) -> std::tuple<std::shared_ptr<Node>, std::shared_ptr<Node>>
  {
    auto first = nodes[0];
    auto second = nodes[1];
    nodes.erase(std::begin(nodes));
    nodes.erase(std::begin(nodes));
    return std::make_tuple(first, second);
  }

  static auto translateToCode(std::unordered_map<char, std::string> &codeTable, std::string_view text) -> std::string
  {
    std::string result = "";

    for (auto character : text)
    {
      result += codeTable[character];
    }

    return result;
  }

  static auto assertEqual(std::unordered_map<char, std::string> codeTable,
                          std::unordered_map<char, std::string> codeTableCreatedFromTree) -> void
  {
    for (auto [key, value] : codeTable)
    {
      if (value != codeTableCreatedFromTree[key])
      {
        throw std::runtime_error("Failed to recreate tree from code table");
      }
    }
  }

public:
  static auto encode(std::string_view text) -> std::tuple<std::unordered_map<char, std::string>, std::string>
  {
    auto frequencyTable = createFrequencyTable(text);
    auto nodes = sortNodes(std::move(createNodes(frequencyTable)));
    auto treeRoot = createTreeFromFrequencyTable(frequencyTable, nodes);
    auto codeTable = createCodeTable(treeRoot);

    auto encoded = translateToCode(codeTable, text);

    return std::make_tuple(codeTable, encoded);
  }

  static auto decode(std::unordered_map<char, std::string> const &codeTable, std::string_view text) -> std::string
  {
    std::string result = "";
    std::shared_ptr<Node> treeRoot = createTreeFromCodeTable(codeTable);

    assertEqual(codeTable, createCodeTable(treeRoot));

    std::shared_ptr<Node> currentNode = treeRoot;

    for (auto character : text)
    {
      if (character == '0')
      {
        currentNode = currentNode->left;
      }
      else
      {
        currentNode = currentNode->right;
      }

      if (isLeaf(currentNode))
      {
        result += currentNode->character;
        currentNode = treeRoot;
      }
    }

    return result;
  }
};