/**
 * time O(n)
 * space O(1)
 */

function balancedStringSplit(s: string): number {
  let balanced = 0
  let count = 0

  for (const character of s) {
    count = character === "R" ? count + 1 : count - 1

    if (count === 0) {
      ++balanced
    }
  }

  return balanced
}

function main() {
  /*
  Balanced strings are those who have equal quantity of 'L' and 'R' characters.

  Given a balanced string s split it in the maximum amount of balanced strings.

  Return the maximum amount of splitted balanced strings.

  Example 1:

  Input: s = "RLRRLLRLRL"
  Output: 4
  Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.

  Example 2:

  Input: s = "RLLLLRRRLR"
  Output: 3
  Explanation: s can be split into "RL", "LLLRRR", "LR", each substring contains same number of 'L' and 'R'.
  */

  console.log("RLRRLLRLRL", balancedStringSplit("RLRRLLRLRL"))
  console.log("RLLLLRRRLR", balancedStringSplit("RLLLLRRRLR"))
}
main()
