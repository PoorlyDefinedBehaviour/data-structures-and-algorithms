<?php
declare (strict_types = 1);

require_once "src/abstracts/Queue.php";

use Abstracts\Queue\Queue;
use PHPUnit\Framework\TestCase;

final class QueueTest extends TestCase {
  public function test_length(): void {
    $queue = new Queue();
    $this->assertEquals(0, $queue->length());
    $queue->push(444);
    $this->assertEquals(1, $queue->length());
  }

  public function test_is_empty(): void {
    $queue = new Queue();
    $this->assertEquals(true, $queue->is_empty());
    $queue->push(444);
    $this->assertEquals(false, $queue->is_empty());
  }

  public function test_push(): void {
    $queue = new Queue();
    $queue->push(4444)
      ->push(432432)
      ->push(44)
      ->push(444);
    $this->assertEquals(4, $queue->length());
  }

  public function test_clear(): void {
    $queue = new Queue();
    $queue->push(1)
      ->push(2)
      ->push(3);

    $this->assertEquals(3, $queue->length());

    $queue->clear();
    $this->assertEquals(0, $queue->length());
  }

  public function test_next(): void {
    $queue = new Queue();
    $queue->push(1)
      ->push(432432)
      ->push(44)
      ->push(444);

    $this->assertEquals(4, $queue->length());
    $this->assertEquals(1, $queue->next());
    $this->assertEquals(3, $queue->length());

    $queue->push(555);
    $this->assertEquals(4, $queue->length());
    $this->assertEquals(432432, $queue->next());
    $this->assertEquals(3, $queue->length());
  }

  public function test_peek(): void {
    $queue = new Queue();
    $queue->push(3233)
      ->push(432432)
      ->push(44)
      ->push(444);

    $this->assertEquals(3233, $queue->peek());
    $this->assertEquals(4, $queue->length());
  }
}