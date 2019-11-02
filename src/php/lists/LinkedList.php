<?php declare(strict_types=1);

class Node {
  public $data;
  public $next;

  public function __construct($data){
    $this->data = $data;
  }
}

class LinkedList {
  private ?Node $_head = null;
  private int $_length = 0;

  public function head() { return $this->_head->data; }
  public function length(): int { return $this->_length; }

  public function insert($value): LinkedList {
    $this->_length += 1;

    if(!$this->_head){
      $this->_head = new Node($value);
      return $this;
    }

    $current_node = $this->_head;
    while($current_node->next){
      $current_node = $current_node->next;
    }

    $current_node->next = new Node($value);

    return $this;
  }

  public function remove($value): LinkedList {
    if($this->_head->data === $value){
      $this->_head = $this->_head->next;
      $this->_length -= 1;
      return $this;
    }

     $current_node = $this->_head;
     while($current_node){
       if($current_node->next->data === $value){
        $current_node->next = $current_node->next->next;
        $this->_length -= 1;
        return $this;
       }
       $current_node = $current_node->next;
     }
    
     throw new Exception("value '{$vaue}' not found at LinkedList.remove");
  }

  public function find($value) {
    return $this->find_if(function($v) { return $v === $value; });
  }

  public function find_if(Closure $predicate) {
    $current_node = $this->_head;
    
    while($current_node){
      if($predicate($current_node->data))
        return $current_node->data;
      $current_node = $current_node->next;
    }

    return null;
  }

  public function remove_if(Closure $predicate): LinkedList {
    if($predicate($this->_head->data)){
      $this->_head = $this->_head->next;
      $this->_length -= 1;
      return $this;
    }

    $current_node = $this->_head;
    while($current_node){
      if($predicate($current_node->next->data)){
       $current_node->next = $current_node->next->next;
       $this->_length -= 1;
       return $this;
      }
      $current_node = $current_node->next;
    }

    return $this;
  }

  public function map(Closure $transformer): LinkedList{
    $list = new LinkedList();
    
    $current_node = $this->_head;
    while($current_node){
      $list->insert($transformer($current_node->data));
      $current_node = $current_node->next;
    }

    return $list;
  }

  public function filter(Closure $predicate): LinkedList {
    $list = new LinkedList();

    $current_node = $this->_head;
    while($current_node){
      if($predicate($current_node->data))
        $list->insert($current_node->data);
      $current_node = $current_node->next;
    }

    return $list;
  }

  public function reduce(Closure $reducer) {
    $result;

    $current_node = $this->_head;
    while($current_node){
      $result = $reducer($result, $current->node->data);
      $current_node = $current_node->next;
    }

    return $result;
  }

  public function for_each(Closure $fn): LinkedList {
    $current_node = $this->_head;

    while($current_node){
      $fn($current_node->data);
      $current_node = $current_node->next;
    }

    return $this;
  }
}