<?php declare (strict_types = 1);

namespace Lists\LinkedList;

class Node {
  public $data;
  public  ? Node $next;

  public function __construct($data) {
    $this->data = $data;
  }
}

class LinkedList {
  private  ? Node $_head = null;
  private int $_length = 0;

  private function assert_index_is_valid(int $index) : void {
    if ($index < 0 || $index > $this->_length) {
      throw new \Exception("Invalid index '{$index}' at LinkedList.insert_at");
    }
  }

  public function head() {return $this->_head ? $this->_head->data : null;}
  public function length() : int {return $this->_length;}

  public function at($index) {
    $this->assert_index_is_valid($index);

    $current_node = $this->_head;
    $current_index = 0;
    while ($current_index !== $index) {
      $current_node = $current_node->next;
      $current_index += 1;
    }

    return $current_node->data;
  }

  public function insert($value): LinkedList {
    return $this->insert_at($this->_length, $value);
  }

  public function insert_at(int $index, $value): LinkedList {
    $this->assert_index_is_valid($index);

    $this->_length += 1;

    if ($index === 0) {
      $temp = $this->_head;
      $this->_head = new Node($value);
      $this->_head->next = $temp;
      return $this;
    }

    $current_node = $this->_head;
    $current_index = 0;
    while ($current_index + 1 !== $index) {
      $current_node = $current_node->next;
      $current_index += 1;
    }

    $temp = $current_node->next;
    $current_node->next = new Node($value);
    $current_node->next->next = $temp;
    return $this;
  }

  public function remove_at(int $index): LinkedList {
    $this->assert_index_is_valid($index);

    $this->_length -= 1;

    if ($index === 0) {
      $this->_head = $this->_head->next;
      return $this;
    }

    $current_node = $this->_head;
    $current_index = 0;
    while ($current_index + 1 !== $index) {
      $current_node = $current_node->next;
      $current_index += 1;
    }

    $current_node->next = $current_node->next ? $current_node->next->next : null;

    return $this;
  }

  public function remove($value): LinkedList {
    if ($this->_head->data === $value) {
      $this->_head = $this->_head->next;
      $this->_length -= 1;
      return $this;
    }

    $current_node = $this->_head;
    while ($current_node) {
      if ($current_node->next->data === $value) {
        $current_node->next = $current_node->next->next;
        $this->_length -= 1;
        return $this;
      }
      $current_node = $current_node->next;
    }

    throw new Exception("value '{$value}' not found at LinkedList.remove");
  }

  public function find($value) {
    return $this->find_if(function ($v) use (&$value) {return $v === $value;});
  }

  public function find_if(\Closure $predicate) {
    $current_node = $this->_head;

    while ($current_node) {
      if ($predicate($current_node->data)) {
        return $current_node->data;
      }

      $current_node = $current_node->next;
    }

    return null;
  }

  public function find_index(\Closure $predicate): int {
    $current_index = 0;
    $current_node = $this->_head;

    while ($current_node) {
      if ($predicate($current_node->data)) {
        return $current_index;
      }

      $current_node = $current_node->next;
      $current_index += 1;
    }

    $NOT_FOUND_INDEX = -1;
    return $NOT_FOUND_INDEX;
  }

  public function remove_if(\Closure $predicate): LinkedList {
    if ($this->_head && $predicate($this->_head->data)) {
      $this->_head = $this->_head->next;
      $this->_length -= 1;
      return $this;
    }

    $current_node = $this->_head;
    while ($current_node) {
      if ($predicate($current_node->next->data)) {
        $current_node->next = $current_node->next->next;
        $this->_length -= 1;
        return $this;
      }
      $current_node = $current_node->next;
    }

    return $this;
  }

  public function map(\Closure $transformer): LinkedList {
    $list = new LinkedList();

    $current_node = $this->_head;
    while ($current_node) {
      $list->insert($transformer($current_node->data));
      $current_node = $current_node->next;
    }

    return $list;
  }

  public function filter(\Closure $predicate): LinkedList {
    $list = new LinkedList();

    $current_node = $this->_head;
    while ($current_node) {
      if ($predicate($current_node->data)) {
        $list->insert($current_node->data);
      }

      $current_node = $current_node->next;
    }

    return $list;
  }

  public function reduce(\Closure $reducer) {
    $result = null;

    $current_node = $this->_head;
    while ($current_node) {
      $result = $reducer($result, $current_node->data);
      $current_node = $current_node->next;
    }

    return $result;
  }

  public function for_each(\Closure $fn): LinkedList {
    $current_node = $this->_head;

    while ($current_node) {
      $fn($current_node->data);
      $current_node = $current_node->next;
    }

    return $this;
  }
}