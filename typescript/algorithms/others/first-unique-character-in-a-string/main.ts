interface IndexAndFrequency {
  index: number;
  frequency: number;
}

interface Dictionary<T> {
  [key: string]: T;
}

/**
 * time O(2n) -> O(n)
 * space O(n)
 */
function firstUniqueCharacter(text: string): number {
  const characterFrequencyTable: Dictionary<IndexAndFrequency> = {};

  for (let i = 0; i < text.length; ++i) {
    const character = text[i];

    if (!(character in characterFrequencyTable)) {
      characterFrequencyTable[character] = { index: i, frequency: 0 };
    }
    characterFrequencyTable[character].frequency++;
  }

  for (const { index, frequency } of Object.values(characterFrequencyTable)) {
    if (frequency === 1) {
      return index;
    }
  }

  return -1;
}

function main() {
  /*
	  Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.

	   Examples:

	   s = "leetcode"
	   return 0.

	   s = "loveleetcode",
	   return 2.

	   Note: You may assume the string contain only lowercase letters.
	*/
  console.log("leetcode", firstUniqueCharacter("leetcode"));
  console.log("loveleetcode", firstUniqueCharacter("loveleetcode"));
}
main();
