<?php
declare (strict_types = 1);

require_once "src/abstracts/Stack.php";

use Abstracts\Stack\Stack;
use PHPUnit\Framework\TestCase;

final class StackTest extends TestCase {
  public function test_size(): void {
    $stack = new Stack();
    $this->assertEquals(0, $stack->size());
    $stack->push(444);
    $this->assertEquals(1, $stack->size());
  }

  public function test_is_empty(): void {
    $stack = new Stack();
    $this->assertEquals(true, $stack->is_empty());
    $stack->push(444);
    $this->assertEquals(false, $stack->is_empty());
  }

  public function test_clear(): void {
    $stack = new Stack();
    $stack->push(1)
      ->push(2)
      ->push(3);

    $this->assertEquals(3, $stack->size());

    $stack->clear();
    $this->assertEquals(0, $stack->size());
  }

  public function test_push(): void {
    $stack = new Stack();
    $stack->push(4444)
      ->push(432432)
      ->push(44)
      ->push(444);
    $this->assertEquals(4, $stack->size());
  }

  public function test_pop(): void {
    $stack = new Stack();
    $stack->push(4444)
      ->push(432432)
      ->push(44)
      ->push(444);

    $this->assertEquals(4, $stack->size());
    $this->assertEquals(444, $stack->pop());
    $this->assertEquals(3, $stack->size());

    $stack->push(555);
    $this->assertEquals(4, $stack->size());
    $this->assertEquals(555, $stack->pop());
    $this->assertEquals(3, $stack->size());
  }

  public function test_peek(): void {
    $stack = new Stack();
    $stack->push(4444)
      ->push(432432)
      ->push(44)
      ->push(444);

    $this->assertEquals(444, $stack->peek());
    $this->assertEquals(4, $stack->size());
  }
}