<?php
declare (strict_types = 1);

require_once "src/abstracts/Stream.php";

use Abstracts\Stream\Stream;
use PHPUnit\Framework\TestCase;

final class StreamTest extends TestCase {
  public function test_map(): void {
    $array = Stream::to_stream(\range(1, 5))
      ->map(function ($number): int {return $number * 2;})
      ->unwrap();

    $this->assertEquals(2, $array[0]);
    $this->assertEquals(4, $array[1]);
    $this->assertEquals(6, $array[2]);
    $this->assertEquals(8, $array[3]);
    $this->assertEquals(10, $array[4]);
    $this->assertEquals(5, \count($array));
  }

  public function test_filter(): void {
    $array = Stream::to_stream(\range(1, 5))
      ->filter(function ($number): bool {return $number % 2 === 0;})
      ->unwrap();

    $this->assertEquals(2, $array[0]);
    $this->assertEquals(4, $array[1]);
    $this->assertEquals(2, \count($array));
  }

  public function test_reduce(): void {
    $stream = Stream::to_stream(\range(1, 5));
    $result = $stream->reduce(function ($accum, $number): int {return $accum + $number;});

    $this->assertEquals(15, $result);
  }

  public function test_all_of(): void {
    $stream = Stream::to_stream(\range(1, 5));

    $all_odd = $stream->all_of(function ($number): bool {return $number % 2 === 0;});
    $all_even = $stream->all_of(function ($number): bool {return $number % 2 === 0;});
    $all_in_range = $stream->all_of(function ($number): bool {return $number > -1 && $number < 6;});

    $this->assertEquals(false, $all_odd);
    $this->assertEquals(false, $all_even);
    $this->assertEquals(true, $all_in_range);
  }

  public function test_any_of(): void {
    $stream = Stream::to_stream(\range(1, 5));

    $equal_to_three = $stream->any_of(function ($number): bool {return $number === 3;});
    $bigger_than_ten = $stream->any_of(function ($number): bool {return $number > 10;});

    $this->assertEquals(true, $equal_to_three);
    $this->assertEquals(false, $bigger_than_ten);
  }

  public function test_length(): void {
    $this->assertEquals(30, Stream::to_stream(\range(1, 30))->length());
    $this->assertEquals(0, Stream::to_stream([])->length());
    $this->assertEquals(1000, Stream::to_stream(\range(0, 999))->length());
    $this->assertEquals(61, Stream::to_stream(\range(-10, 50))->length());
  }

  public function test_is_empty(): void {
    $empty_stream = Stream::to_stream([]);
    $this->assertEquals(true, $empty_stream->is_empty());

    $non_empty_stream = Stream::to_stream(\range(1, 50));
    $this->assertEquals(false, $non_empty_stream->is_empty());
  }

  public function test_count_if(): void {
    $even_numbers_count = Stream::to_stream(\range(1, 20))
      ->count_if(function ($number): bool {return $number % 2 === 1;});

    $odd_numbers_count = Stream::to_stream(\range(1, 20))
      ->count_if(function ($number): bool {return $number % 2 === 0;});

    $bigger_than_ten_count = Stream::to_stream(\range(1, 20))
      ->count_if(function ($number): bool {return $number > 10;});

    $this->assertEquals(10, $even_numbers_count);
    $this->assertEquals(10, $odd_numbers_count);
    $this->assertEquals(10, $bigger_than_ten_count);
  }

  public function test_fill_n(): void {
    $result = Stream::to_stream([])
      ->fill_n(10, 1)
      ->reduce(function ($accum, $number): int {return $accum + $number;});

    $this->assertEquals($result, 10);
  }

  public function test_fill(): void {
    $array = Stream::to_stream(\range(1, 5))
      ->fill(0)
      ->unwrap();

    $this->assertEquals(5, \count($array));
    $this->assertEquals(0, $array[0]);
    $this->assertEquals(0, $array[1]);
    $this->assertEquals(0, $array[2]);
    $this->assertEquals(0, $array[3]);
    $this->assertEquals(0, $array[4]);
  }

  public function test_reverse(): void {
    $array = Stream::to_stream(\range(5, 1))
      ->reverse()
      ->unwrap();

    $this->assertEquals(5, \count($array));
    $this->assertEquals(1, $array[0]);
    $this->assertEquals(2, $array[1]);
    $this->assertEquals(3, $array[2]);
    $this->assertEquals(4, $array[3]);
    $this->assertEquals(5, $array[4]);
  }

  public function test_unique(): void {
    $array = Stream::to_stream([1, 2, 2, 3, 4, 4, 5])
      ->unique()
      ->unwrap();

    $this->assertEquals(5, \count($array));
    $this->assertEquals(1, $array[0]);
    $this->assertEquals(2, $array[1]);
    $this->assertEquals(3, $array[2]);
    $this->assertEquals(4, $array[3]);
    $this->assertEquals(5, $array[4]);
  }
}