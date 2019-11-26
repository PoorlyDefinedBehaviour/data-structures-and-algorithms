<?php
declare (strict_types = 1);

require_once "src/trees/BinarySearchTree.php";

use PHPUnit\Framework\TestCase;
use Trees\BinarySearchTree\BinarySearchTree;

final class BinarySearchTreeTest extends TestCase {
  private function bst_comparator($lhs, $rhs): int {
    if ($lhs < $rhs) {
      return -1;
    }

    if ($lhs > $rhs) {
      return 1;
    }

    return 0;
  }

  public function test_height(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $this->assertEquals(0, $tree->height());
  }

  public function test_is_empty(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $this->assertEquals(true, $tree->is_empty());

    $tree->insert(300);

    $this->assertEquals(false, $tree->is_empty());
  }

  public function test_insert(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $tree->insert(10)
      ->insert(20)
      ->insert(-10);

    $this->assertEquals(3, $tree->height());
    $this->assertEquals(10, $tree->root());
  }

  public function test_remove(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $tree->insert(10)
      ->insert(20)
      ->insert(-10);

    $this->assertEquals(3, $tree->height());
    $this->assertEquals(10, $tree->root());

    $tree->remove(20);
    $this->assertEquals(2, $tree->height());

    $tree->remove(10);
    $this->assertEquals(1, $tree->height());

    $this->assertEquals(-10, $tree->root());
  }

  public function test_min(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $tree->insert(10)
      ->insert(20)
      ->insert(-10);

    $this->assertEquals(-10, $tree->min());

    $tree->insert(-50);
    $this->assertEquals(-50, $tree->min());
  }

  public function test_max(): void {
    $tree = new BinarySearchTree(\Closure::fromCallable([$this, "bst_comparator"]));

    $tree->insert(10)
      ->insert(20)
      ->insert(-10);

    $this->assertEquals(20, $tree->max());

    $tree->insert(9999);
    $this->assertEquals(9999, $tree->max());
  }
}