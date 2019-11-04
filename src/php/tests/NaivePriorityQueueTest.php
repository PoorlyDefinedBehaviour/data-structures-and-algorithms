<?php
declare (strict_types = 1);

require_once "src/abstracts/NaivePriorityQueue.php";

use Abstracts\NaivePriorityQueue\NaivePriorityQueue;
use PHPUnit\Framework\TestCase;

final class NaivePriorityQueueTest extends TestCase {
  public function test_length(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $this->assertEquals(0, $queue->length());
    $queue->push(444);
    $this->assertEquals(1, $queue->length());
  }

  public function test_is_empty(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $this->assertEquals(true, $queue->is_empty());
    $queue->push(444);
    $this->assertEquals(false, $queue->is_empty());
  }

  public function test_push(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $queue->push(4444)
      ->push(432432)
      ->push(44)
      ->push(444);
    $this->assertEquals(4, $queue->length());
  }

  public function test_next(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $queue->push(5)
      ->push(4)
      ->push(3)
      ->push(2)
      ->push(1);

    $this->assertEquals(1, $queue->next());
    $this->assertEquals(2, $queue->next());
    $this->assertEquals(3, $queue->next());
    $this->assertEquals(4, $queue->next());
    $this->assertEquals(5, $queue->next());
  }

  public function test_clear(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $queue->push(1)
      ->push(2)
      ->push(3);

    $this->assertEquals(3, $queue->length());

    $queue->clear();
    $this->assertEquals(0, $queue->length());
  }

  public function test_peek(): void {
    $queue = new NaivePriorityQueue(\Closure::fromCallable("sort"));
    $queue->push(3233)
      ->push(432432)
      ->push(44)
      ->push(444);

    $this->assertEquals(44, $queue->peek());
    $this->assertEquals(4, $queue->length());

    $this->expectException(\Exception::class);
    $queue->clear();
    $queue->peek();
  }
}