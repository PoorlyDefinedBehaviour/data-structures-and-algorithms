<?php declare (strict_types = 1);

namespace Abstracts\NaivePriorityQueue;

class NaivePriorityQueue {
  private array $elements = [];
  private int $_length = 0;

  private \Closure $sorting_function;

  public function __construct(\Closure $sorting_function) {
    $this->sorting_function = $sorting_function;
  }

  private function assert_not_empty(): void {
    if ($this->_length === 0) {
      throw new \Exception("Operation requires a non empty queue");
    }
  }

  public function length(): int {return $this->_length;}
  public function is_empty(): bool {return $this->_length === 0;}

  public function push($element): NaivePriorityQueue {
    $this->elements[] = $element;
    $this->_length += 1;
    ($this->sorting_function)($this->elements);
    return $this;
  }

  public function next() {
    $this->assert_not_empty();

    $element = $this->elements[0];
    unset($this->elements[0]);
    $this->elements = array_values($this->elements);
    $this->_length -= 1;
    return $element;
  }

  public function peek() {
    $this->assert_not_empty();
    return $this->elements[0];
  }

  public function clear(): NaivePriorityQueue {
    $this->elements = array();
    $this->_length = 0;
    return $this;
  }
}