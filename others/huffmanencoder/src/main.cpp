#include <fstream>
#include <string>

#include "console/Console.hpp"
#include "hash/HashTable.hpp"
#include "utils/Split.hpp"
#include "utils/ReadFile.hpp"
#include "huffman/Huffman.hpp"

auto main() -> int
{
  HashTable<std::string, int> hash(20);

  std::string const text = read_file("text.txt");

  HashTable<std::string, std::size_t> word_frequency_table = create_word_frequency_table(text);

  show_table_entries("Word Frequency Table", word_frequency_table);
  show_table_collisions("Word Frequency Table Collisions", word_frequency_table);

  HashTable<char, int> frequency_table = create_frequency_table(text);
  std::shared_ptr<Node> tree_root = create_tree(frequency_table);
  HashTable<char, std::string> code_table = create_code_table(tree_root, text.length());

  show_table_entries("Huffman characters frequency table", frequency_table);
  show_table_entries("Huffman characters code table", code_table);

  Console::println("<Tree pre order traversal>");
  pre_order_traversal(tree_root);
  Console::println("</Tree pre order traversal>");

  std::string encoded_text = encode(code_table, text);
  Console::println(encoded_text);
  Console::println(decode(code_table, encoded_text));

  Console::println("The encoded text is ", difference(text, encoded_text), " bits smaller than the original text");
  Console::println(8 * (3 - 8 / 8));
}
