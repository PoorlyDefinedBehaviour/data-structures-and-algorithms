<?php declare (strict_types = 1);

namespace Abstracts\Stream;

class Stream {
  private iterable $collection = [];

  public static function to_stream(iterable $collection): Stream {
    return new Stream($collection);
  }

  public function __construct(iterable $collection) {
    $this->collection = $collection;
  }

  public function map(\Closure $mapper): Stream {
    $this->collection = array_values(array_map($mapper, $this->collection));
    return $this;
  }

  public function filter(\Closure $predicate): Stream {
    $this->collection = array_values(array_filter($this->collection, $predicate));
    return $this;
  }

  public function reduce(\Closure $reducer) {
    return array_reduce($this->collection, $reducer);
  }

  public function for_each(\Closure $fn): Stream {
    foreach ($this->collection as $element) {
      $fn($element);
    }
    return $this;
  }

  public function all_of(\Closure $predicate): bool {
    foreach ($this->collection as $element) {
      if (!$predicate($element)) {
        return false;
      }
    }
    return true;
  }

  public function any_of(\Closure $predicate): bool {
    foreach ($this->collection as $element) {
      if ($predicate($element)) {
        return true;
      }
    }
    return false;
  }

  public function count_if(\Closure $predicate): int {
    $count = 0;

    foreach ($this->collection as $element) {
      if ($predicate($element)) {
        $count += 1;
      }
    }
    return $count;
  }

  public function length(): int {
    return \count($this->collection);
  }

  public function is_empty(): bool {
    return $this->length() === 0;
  }

  public function fill_n(int $positions, $value): Stream {
    for ($i = 0; $i < $positions; ++$i) {
      $this->collection[$i] = $value;
    }

    return $this;
  }

  public function fill($value): Stream {
    foreach ($this->collection as $key => $_) {
      $this->collection[$key] = $value;
    }
    return $this;
  }

  public function reverse(): Stream {
    $this->collection = array_reverse($this->collection);
    return $this;
  }

  public function unique(): Stream {
    $this->collection = array_values(array_unique($this->collection));
    foreach ($this->collection as $element) {
      echo $element, " ";
    }
    return $this;
  }

  public function unwrap(): iterable {
    return $this->collection;
  }
}