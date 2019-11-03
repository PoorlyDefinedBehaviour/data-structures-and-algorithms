<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

require_once("src/lists/LinkedList.php");

final class LinkedListTest extends TestCase {
  public function test_ength(): void {
    $list = new LinkedList();
    $this->assertEquals(0, $list->length());
    $list->insert(550);
    $this->assertEquals(1, $list->length());
  }

  public function test_at(): void {
    $list = new LinkedList();
    $list->insert(10)
         ->insert(20)
         ->insert(30);

    $this->assertEquals(10, $list->at(0));
    $this->assertEquals(20, $list->at(1));
    $this->assertEquals(30, $list->at(2));
  }

  public function test_head(): void {
    $list = new LinkedList();
    $list->insert(10)
         ->insert(20)
         ->insert(30);
    $this->assertEquals(10, $list->head());
  }

  public function test_insert(): void {
    $list = new LinkedList();
    $list->insert(30);
    $this->assertEquals(30, $list->head());
  }

  public function test_insert_at(): void {
    $list = new LinkedList();
    $list->insert_at(0, 10)
         ->insert_at(0, 20)
         ->insert_at(0, 30);
    $this->assertEquals(30, $list->head());
  }

  public function test_remove(): void {
    $list = new LinkedList();
    $list->insert_at(0, 10)
         ->insert_at(0, 20)
         ->insert_at(0, 30);

    $list->remove(20);
    $this->assertEquals(2, $list->length());
  }

  public function test_remove_at(): void {
    $list = new LinkedList();
    $list->insert_at(0, 10)
         ->insert_at(0, 20)
         ->insert_at(0, 30)
         ->remove_at(2);
    $this->assertEquals(2, $list->length());
  }

  public function test_find(): void {
    $list = new LinkedList();
    $list->insert(10)
         ->insert(20)
         ->insert(30);

    $this->assertEquals(30, $list->find(30));
    $this->assertEquals(null, $list->find(40));
  }

  public function test_find_if(): void {
    $list = new LinkedList();
    $list->insert_at(0, 10)
         ->insert_at(0, 20)
         ->insert_at(0, 30);

    $this->assertEquals(20, $list->find_if(function($number) { return $number === 20; }));
    $this->assertEquals(null, $list->find_if(function($number) { return $number === 50; }));
  }

  public function test_find_index(): void {
    $list = new LinkedList();
    $list->insert(10)
         ->insert(20)
         ->insert(30);

    $this->assertEquals(0, $list->find_index(function($number) { return $number === 10; }));
    $this->assertEquals(1, $list->find_index(function($number) { return $number === 20; }));
    $this->assertEquals(2, $list->find_index(function($number) { return $number === 30; }));
    $this->assertEquals(-1, $list->find_index(function($number) { return $number === 50; }));
  }

  public function test_remove_if(): void {
    $list = new LinkedList();
    $list->insert_at(0, 10)
         ->insert_at(0, 20)
         ->insert_at(0, 30);

    $this->assertEquals(3, $list->length());

    $list->remove_if(function($number) { return $number === 10; });
    $list->remove_if(function($number) { return $number === 20; });
    $list->remove_if(function($number) { return $number === 30; });
    $list->remove_if(function($number) { return $number === 40; });
    $list->remove_if(function($number) { return $number === 50; });

    $this->assertEquals(0, $list->length());
  }

  public function test_map(): void {
    $list = new LinkedList();
    $list->insert(10)
         ->insert(20)
         ->insert(30);

    $mapped_list = $list->map(function($number) { return $number * 2; });

    $this->assertEquals(20, $mapped_list->at(0));
    $this->assertEquals(40, $mapped_list->at(1));
    $this->assertEquals(60, $mapped_list->at(2));
  }

  public function test_filter(): void {
    $list = new LinkedList();
    $list->insert(1)
         ->insert(2)
         ->insert(3);

    $filtered_list = $list->filter(function($number) { return $number % 2 === 0; });

    $this->assertEquals(1, $filtered_list->length());
    $this->assertEquals(2, $filtered_list->head());
    $this->assertEquals(2, $filtered_list->at(0));
  }

  public function test_reduce(): void {
    $list = new LinkedList();
    $list->insert(1)
         ->insert(2)
         ->insert(3);

    $result = $list->reduce(function($accum, $number) { return $accum ? $accum + $number : $number;  });

    $this->assertEquals(3 * 4 / 2, $result);
  }
}