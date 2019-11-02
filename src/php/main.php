<?php declare(strict_types=1);

require_once("lists/LinkedList.php");

function main(): void {
  $list = new LinkedList();
  $list->insert(10)
       ->insert(20)
       ->insert(30);  

  $list->map(function($number) { return $number * 2; })
       ->for_each(function($number) { echo "{$number}\n"; });
}
main();