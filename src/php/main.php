<?php declare(strict_types=1);

require_once("lists/LinkedList.php");

function main(): void {
  $list = new LinkedList();
  $list->insert(10)
       ->insert(20)
       ->insert(30);  

  echo "list->find_index() => {$list->find_index(function($number) { return $number === 20; })}\n";
  echo "list->find_index() => {$list->find_index(function($number) { return $number === 40; })}\n";

  $list->map(function($number) { return $number * 2; })
       ->for_each(function($number) { echo "{$number}\n"; });
}
main();