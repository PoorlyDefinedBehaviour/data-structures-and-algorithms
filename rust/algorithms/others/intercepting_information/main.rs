/// Spies Breaching Computers (SBC) is a private digital spy agency that is developing a new device for intercepting information using electromagnetic waves, which allows spying even without physical contact with the target.
///
/// The device tries to collect information one byte at a time, this is, a sequence of 8 bits where each of them, naturally, can have a value of 0 or 1. In certain situations, due to interference from other devices, the reading cannot be done successfully. In this case, the device returns the value 9 for the corresponding bit, informing that the reading could not be performed.
///
/// In order to automate the recognition of the information the device reads, a request was made for a program that, based on the information read by the device, informs whether all bits were read successfully or not. Your task is to write this program.
///
/// Input
/// The input consists of a single line, containing 8 integers N1, N2, N3, N4, N5, N6, N7 and N8, indicating the values read by the device (Ni  is 0, 1 or 9 for 1≤i≤8).
///
/// Output
/// Print a single line containing the capital letter “S” if all bits are read successfully; otherwise print a single line containing the capital letter “F”, corresponding to a failure.
///
/// Examples
/// 0 0 1 1 0 1 0 1 -> S
/// 0 0 1 9 0 1 0 1 -> F

fn solve(byte: &str) -> char {
    if byte.chars().all(|bit| bit != '9') {
        'S'
    } else {
        'F'
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", solve(&buffer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!('S', solve("00110101"));
        assert_eq!('F', solve("00190101"));
    }
}
