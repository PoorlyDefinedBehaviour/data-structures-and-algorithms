<?php declare (strict_types = 1);

namespace Abstracts\Queue;

class Queue {
  private array $elements = [];
  private int $_length = 0;

  private function assert_not_empty(): void {
    if ($this->_length === 0) {
      throw new \Exception("Operation requires a non empty queue");
    }
  }

  public function length(): int {return $this->_length;}
  public function is_empty(): bool {return $this->_length === 0;}

  public function push($element): Queue {
    $this->elements[] = $element;
    $this->_length += 1;
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

  public function clear(): Queue {
    $this->elements = array();
    $this->_length = 0;
    return $this;
  }
}