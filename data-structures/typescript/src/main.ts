import Trie from "./trees/Trie";

function main(): void {
  const trie = new Trie();

  trie
    .insert("ten")
    .insert("tea")
    .print();

  console.log(`trie.search("ten") => ${trie.search("ten")}`);
  console.log(`trie.search("tea") => ${trie.search("tea")}`);
  console.log(`trie.search("te") => ${trie.search("te")}`);
  console.log(`trie.search("tex") => ${trie.search("tex")}`);
}
main();
