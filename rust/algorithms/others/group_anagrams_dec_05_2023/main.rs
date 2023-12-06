use std::collections::HashMap;

/// time O(n * m)
/// space O(n * m)
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = HashMap::new();

    let a_ascii_number = 'a' as u32;

    for str in strs {
        let mut count = [0_isize; 26];

        for character in str.chars() {
            let index = (character as u32 - a_ascii_number) as usize;
            count[index] += 1;
        }

        let entry = groups.entry(count).or_insert_with(Vec::new);
        entry.push(str);
    }

    groups.into_values().collect()
}

fn main() {}
