<?php declare (strict_types = 1);

namespace Trees\BinarySearchTree;

class Node {
  public $data;
  public  ? Node $left = null;
  public  ? Node $right = null;

  public function __construct($data) {
    $this->data = $data;
  }
}

class BinarySearchTree {
  private static int $LESS = -1;
  private static int $GREATER = 1;
  private static int $EQUAL = 0;

  private $comparator;

  private  ? Node $_root = null;
  private int $_height = 0;

  private function is_less_than(&$lhs, &$rhs) : bool {
    return ($this->comparator)($lhs, $rhs) === self::$LESS;
  }

  private function is_greater_than(&$lhs, &$rhs) : bool {
    return ($this->comparator)($lhs, $rhs) === self::$GREATER;
  }

  private function is_equal(&$lhs, &$rhs) : bool {
    return ($this->comparator)($lhs, $rhs) === self::$EQUAL;
  }

  private function assert_not_empty(): void {
    if ($this->is_empty()) {
      throw new \Exception("Tried to call BinarySearchTree.min() on a empty tree");
    }
  }

  public function __construct(\Closure $comparator) {
    $this->comparator = $comparator;
  }

  public function height(): int {return $this->_height;}
  public function is_empty(): bool {return $this->_height === 0;}
  public function root() {return $this->_root ? $this->_root->data : null;}

  public function insert($value): BinarySearchTree {
    if (!$this->_root) {
      $this->_root = new Node($value);
      $this->_height += 1;
      return $this;
    }

    $current_node = $this->_root;
    while ($current_node) {
      if ($this->is_equal($value, $current_node->data)) {
        return $this;
      }

      if ($this->is_less_than($value, $current_node->data)) {
        if ($current_node->left) {
          $current_node = $current_node->left;
        } else {
          $current_node->left = new Node($value);
          break;
        }
      }

      if ($this->is_greater_than($value, $current_node->data)) {
        if ($current_node->right) {
          $current_node = $current_node->right;
        } else {
          $current_node->right = new Node($value);
          break;
        }
      }
    }

    $this->_height += 1;
    return $this;
  }

  private function find_node_and_parent_by_value(?Node $parent, ?Node $node, $value) {
    if (!$node) {
      return [null, null];
    }

    if ($this->is_equal($value, $node->data)) {
      return [$parent, $node];
    }

    if ($this->is_less_than($value, $node->data)) {
      return $this->find_node_and_parent_by_value($node, $node->left, $value);
    }

    if ($this->is_greater_than($value, $node->data)) {
      return $this->find_node_and_parent_by_value($node, $node->right, $value);
    }

  }

  private function max_node_with_parent(?Node $parent, ?Node $node): array{
    return !$node->right ? [$parent, $node] : $this->max_node($node, $noode->right);
  }

  public function remove($value): BinarySearchTree {
    [$node_to_delete_parent, $node_to_delete] = $this->find_node_and_parent_by_value(null, $this->_root, $value);
    if (!$node_to_delete_parent && !$node_to_delete) {
      return $this;
    }

    $this->_height -= 1;

    if (!$node_to_delete->left) {
      $node_to_delete = $node_to_delete->right;
      $node_to_delete_parent->right = $node_to_delete;
      return $this;
    }

    [$biggest_node_parent, $biggest_node] = $this->max_node_with_parent($node_to_delete_parent, $node_to_delete->left);

    $biggest_node->right = $node_to_delete->right;
    if ($biggest_node_parent) {
      $biggest_node_parent->right = $biggest_node->left;
      $biggest_node->left = $node_to_delete->left;
    } else {
      $this->_root = $biggest_node;
    }

    return $this;
  }

  private function min_impl(?Node $node) {return $node->left ? $this->min_impl($node->left) : $node->data;}

  public function min() {
    $this->assert_not_empty();

    return $this->min_impl($this->_root);
  }

  private function max_impl(?Node $node) {return $node->right ? $this->max_impl($node->right) : $node->data;}

  public function max() {
    $this->assert_not_empty();

    return $this->max_impl($this->_root);
  }

  private function pre_order_traversel_impl(\Closure $fn, ?Node $current_node): void {
    if (!$current_node) {
      return;
    }

    $fn($current_node->data);
    $this->pre_order_traversel_impl($fn, $current_node->left);
    $this->pre_order_traversel_impl($fn, $current_node->right);
  }

  public function pre_order_traversal(\Closure $fn): BinarySearchTree {
    $this->pre_order_traversel_impl($fn, $this->_root);
    return $this;
  }

  private function in_order_traversal_impl(\Closure $fn, ?Node $current_node): void {
    if (!$current_node) {
      return;
    }

    $this->in_order_traversal_impl($fn, $current_node->left);
    $fn($current_node->data);
    $this->in_order_traversal_impl($fn, $current_node->right);
  }

  public function in_order_traversal(\Closure $fn): BinarySearchTree {
    $this->in_order_traversal_impl($fn, $this->_root);
    return $this;
  }

  private function post_order_traversel_impl(\Closure $fn, ?Node $current_node): void {
    if (!$current_node) {
      return;
    }

    $this->post_order_traversel_impl($fn, $current_node->left);
    $this->post_order_traversel_impl($fn, $current_node->right);
    $fn($current_node->data);
  }

  public function post_order_traversal(\Closure $fn): BinarySearchTree {
    $this->post_order_traversal_impl($fn, $this->_root);
    return $this;
  }
}