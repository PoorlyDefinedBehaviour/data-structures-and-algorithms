#pragma once

#include <functional>
#include <memory>
#include <algorithm>
#include <iostream>

template <typename T, int M>
class BTree
{
private:
  using Comparator = std::function<int(T, T)>;
  using Records = std::vector<T>;

  struct Page;
  using Children = std::vector<std::shared_ptr<Page>>;

  struct Page
  {
    std::shared_ptr<Page> parent = nullptr;
    Records records;
    Children children;

    Page() = default;
    Page(std::shared_ptr<Page> const &parent)
        : parent(parent) {}
  };

  Comparator comparator = [](auto const &lhs, auto const &rhs) -> int {
    if (lhs < rhs)
      return -1;
    if (lhs > rhs)
      return 1;
    return 0;
  };

  std::shared_ptr<Page> _root = std::make_shared<Page>();
  int _height = 0;

  auto is_equal(T const &lhs, T const &rhs) const -> bool { return comparator(lhs, rhs) == 0; }
  auto is_less_than(T const &lhs, T const &rhs) const -> bool { return comparator(lhs, rhs) == -1; }
  auto is_greater_than(T const &lhs, T const &rhs) const -> bool { return comparator(lhs, rhs) == 1; }
  auto is_greater_than_or_equal(T const &lhs, T const &rhs) const -> bool { return is_greater_than(lhs, rhs) || is_equal(lhs, rhs); }

  auto is_full(Children const &children) const -> bool { return children.size() > 2 * M + 1; }
  auto is_full(Records const &records) const -> bool { return records.size() > 2 * M; }

public:
  BTree() = default;
  BTree(Comparator const &comparator)
      : comparator(comparator) {}

  auto height() const -> int { return _height; }
  auto root() const -> std::shared_ptr<Page> const { return _root; }

  auto insert(T const &value) -> BTree &
  {
    _height += 1;

    std::shared_ptr<Page> page = search(_root, value);
    page->records.emplace_back(value);
    std::sort(std::begin(page->records), std::end(page->records), [this](auto const &lhs, auto const &rhs) {
      return is_less_than(lhs, rhs);
    });

    if (is_full(page->records))
      split(page);

    return *this;
  }

  auto search(std::shared_ptr<Page> page, T const &value) -> std::shared_ptr<Page>
  {
    while (!page->children.empty())
    {
      if (is_less_than(value, page->records[0]))
        page = page->children[0];

      for (std::size_t i = 0; i < page->records.size() - 1; ++i)
        if (is_greater_than_or_equal(value, page->records[i]) && is_less_than(value, page->records[i + 1]))
          if (!page->children.empty())
            page = page->children[i + 1];

      if (is_greater_than(value, page->records[page->records.size() - 1]))
        page = page->children[page->records.size()];
    }
    return page;
  }

  auto split(std::shared_ptr<Page> &page) -> void
  {
    std::shared_ptr<Page> parent = nullptr;
    std::shared_ptr<Page> left = std::make_shared<Page>();
    std::shared_ptr<Page> right = std::make_shared<Page>();

    parent = page->parent == nullptr ? std::make_shared<Page>() : page->parent;

    std::size_t const middle_index = page->records.size() / 2;
    T const middle_element = page->records[page->records.size() / 2];
    std::copy(std::begin(page->records),
              std::begin(page->records) + middle_index,
              std::back_inserter(left->records));

    std::copy(std::begin(page->records) + middle_index,
              std::end(page->records),
              std::back_inserter(right->records));

    if (page->parent == nullptr)
    {
      parent->records.emplace_back(middle_element);
      parent->children.emplace_back(left);
      parent->children.emplace_back(right);
      left->parent = right->parent = parent;
      _root = parent;
      return;
    }

    int n = 0;
    /* if(is_less_than(middle_element, parent->records[0])){
      n = 0;
    } */

    for (std::size_t i = 0; i < parent->records.size() - 1; ++i)
      if (is_greater_than_or_equal(middle_element, parent->records[i]) &&
          is_less_than(middle_element, parent->records[i + 1]))
        n = i + 1;

    if (is_greater_than(middle_element, parent->records[parent->records.size() - 1]))
    {
      parent->children.pop_back();
      parent->records.emplace_back(middle_element);
      parent->children.emplace_back(left);
      parent->children.emplace_back(right);
      left->parent = right->parent = parent;
    }
    else
    {
      parent->records.insert(std::begin(parent->records) + n, middle_index);
      parent->children.insert(std::begin(parent->children) + n + 1, right);
      parent->children[n] = left;
      left->parent = right->parent = parent;
    }

    if (is_full(parent->records))
      split(parent);
  }

  auto print(std::shared_ptr<Page> const &page) const -> void
  {
    for (auto const &record : page->records)
      std::cout << record << " -> ";

    std::cout << '\n';
    if (!page->children.empty())
      for (auto const &child : page->children)
        print(child);
  }
};
