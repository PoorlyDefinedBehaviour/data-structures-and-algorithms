<?php declare (strict_types = 1);

namespace Abstracts\Stack;

class Stack {
  private array $elements = [];
  private int $_size = 0;

  private function assert_not_empty(): void {
    if ($this->_size === 0) {
      throw new \Exception("Operation requires a non empty stack");
    }
  }

  public function size(): int {return $this->_size;}
  public function is_empty(): bool {return $this->_size === 0;}

  public function push($element): Stack {
    $this->elements[] = $element;
    $this->_size += 1;
    return $this;
  }

  public function pop() {
    $this->assert_not_empty();

    $element = $this->elements[$this->_size - 1];
    unset($this->elements[$this->_size - 1]);
    $this->elements = array_values($this->elements);
    $this->_size -= 1;
    return $element;
  }

  public function peek() {
    $this->assert_not_empty();
    return $this->elements[$this->_size - 1];
  }

  public function clear(): Stack {
    $this->elements = array();
    $this->_size = 0;
    return $this;
  }
}