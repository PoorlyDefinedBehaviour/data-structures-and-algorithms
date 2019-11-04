<?php declare (strict_types = 1);

require_once "src/abstracts/Stream.php";

use Abstracts\Stream\Stream;

function main(): void {
  $array = Stream::to_stream([1, 2, 2, 3, 4, 4, 5])
    ->unique()
    ->unwrap();

}
main();
