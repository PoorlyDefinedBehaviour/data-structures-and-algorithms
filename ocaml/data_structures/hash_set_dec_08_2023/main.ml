open Printf

let () =
  let set = Hashset.create 0 in
  Hashset.insert set 1;
  Hashset.insert set 2;
  Hashset.insert set 3;
  printf "1 is in the set: %b\n" (Hashset.mem set 1);
  printf "2 is in the set: %b\n" (Hashset.mem set 2);
  printf "3 is in the set: %b\n" (Hashset.mem set 3);
  printf "4 is in the set: %b\n" (Hashset.mem set 4);
  Hashset.remove set 1;
  Hashset.remove set 2;
  Hashset.remove set 3;
  printf "1 is in the set: %b\n" (Hashset.mem set 1);
  printf "2 is in the set: %b\n" (Hashset.mem set 2);
  printf "3 is in the set: %b\n" (Hashset.mem set 3);
  printf "4 is in the set: %b\n" (Hashset.mem set 4)
