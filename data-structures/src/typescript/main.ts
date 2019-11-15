import LinkedList from "./lists/linkedlist/LinkedList";

function main(): void {
  const list: LinkedList<number> = new LinkedList<number>();
  list
    .insert(1)
    .insert(2)
    .insert(3);

  console.log(`list.find_if() => ${list.find_if((n: number) => n == 3)}`);
}
main();
