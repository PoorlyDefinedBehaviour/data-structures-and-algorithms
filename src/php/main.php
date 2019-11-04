<?php declare(strict_types=1);


require_once("src/trees/BinarySearchTree.php");

use Trees\BinarySearchTree\BinarySearchTree;

function main(): void {
  $tree = new BinarySearchTree(function($lhs, $rhs) { 
    if($lhs < $rhs) return -1;
    if($lhs > $rhs) return 1;
    return 0;
  });

  $tree->insert(10)
       ->insert(20)
       ->insert(-10);


       $tree->remove(20);
       
       $tree->remove(10);
       $tree->pre_order_traversal(function($number){ echo "{$number}\n"; });
}
main();