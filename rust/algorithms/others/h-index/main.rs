pub fn h_index(citations: Vec<i32>) -> i32 {
    let num_papers = citations.len();
    let mut count = vec![0; num_papers + 1];

    for citation in citations.iter() {
        if *citation as usize >= num_papers {
            count[citations.len()] += 1;
        } else {
            count[*citation as usize] += 1;
        }
    }

    let mut total = 0;
    for i in (0..=num_papers).rev() {
        total += count[i];
        if total >= i {
            return i as i32;
        }
    }

    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_index() {
        assert_eq!(3, h_index(vec![3, 0, 6, 1, 5]));
        assert_eq!(1, h_index(vec![1, 2, 1]));
        assert_eq!(1, h_index(vec![1, 3, 1]));
    }
}
