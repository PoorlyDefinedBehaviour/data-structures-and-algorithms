#pragma once

#include <cassert>
#include "../src/trees/BinarySearchTree.hpp"

namespace binary_search_tree_test_suite
{
static auto insert() -> void
{
    BinarySearchTree<int> tree;
    tree.insert(10);
    assert(tree.size() == 1);
}

static auto remove() -> void
{
    BinarySearchTree<int> tree;
    tree.insert(10);
    tree.remove(10);
    assert(tree.size() == 0);
}

static auto custom_comparator() -> void
{
    struct S
    {
        int data;
    };

    auto comparator = [](S const &lhs, S const &rhs) -> int {
        if (lhs.data < rhs.data)
            return -1;
        if (lhs.data > rhs.data)
            return 1;
        return 0;
    };
    BinarySearchTree<S> tree(comparator);
    tree.insert({10});
    tree.insert({20});
    assert(tree.size() == 2);
}

static auto start() -> void
{
    insert();
    custom_comparator();
    remove();
}
} // namespace binary_search_tree_test_suite