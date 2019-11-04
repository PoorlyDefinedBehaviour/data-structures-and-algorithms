<?php declare (strict_types = 1);

class Stream {
  private iterable $collection = [];

  public function __construct(iterable $collection) {
    $this->collection = $collection;
  }

  public function map(\Closure $fn): Stream {
    $this->collection = array_map($fn, $this->collection);
    return $this;
  }

  public function filter(\Closure $fn): Stream {
    $this->collection = array_filter($this->collection, $fn);
    return $this;
  }

  public function reduce(\Closure $fn) {
    return array_reduce($this->collection, $fn);
  }

  public function for_each(\Closure $fn): Stream {
    foreach ($this->collection as $element) {
      $fn($element);
    }
    return $this;
  }

  public function unwrap(): iterable {
    return $this->collection;
  }
}

function to_stream(iterable $collection): Stream {
  return new Stream($collection);
}

function main(): void {
  $array = to_stream(\range(1, 5))
    ->map(function ($number): int {return $number * 2;})
    ->filter(function ($number): bool {return $number > 5;})
    ->for_each(function ($number): void {echo "{$number} ";})
    ->unwrap();
}
main();
