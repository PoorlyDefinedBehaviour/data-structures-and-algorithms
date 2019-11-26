import FamilyTree from "./trees/FamilyTree";

function main(): void {
  const family_tree = new FamilyTree();

  family_tree
    .insert("John", "", "")
    .insert("John", "mother", "Alice")
    .insert("John", "father", "Bob")
    .insert("Bob", "mother", "Jane")
    .insert("Jane", "father", "James");

  console.log(JSON.stringify(family_tree, null, 2));

  family_tree.remove("Bob");

  console.log(JSON.stringify(family_tree, null, 2));

  console.log(
    "family_tree.contains(node => node.person_name === 'Alice') =>",
    family_tree.contains(node => node.person_name === "Alice")
  );

  const alice = family_tree.find(node => node.person_name === "Alice");
  console.log(alice);

  const jackson = family_tree.find(node => node.person_name === "Jackson");
  console.log(jackson);

  console.log(
    "family_tree.contains(node => node.person_name === 'Jackson') =>",
    family_tree.contains(node => node.person_name === "Jackson")
  );
}
main();
