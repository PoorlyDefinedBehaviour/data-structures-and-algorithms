/// Bella is a simple girl with a simple life: wake up, go to work, work, go home, rest, sleep, and repeat. Bella commutes via bus, and it rains often in her city, so sometimes she needs an umbrella. However, the local weather forecast is unreliable, so Bella can’t be sure if it’s going to rain or not until right before she begins a trip. To avoid getting caught unprepared, Bella created a system.
///
/// She owns one or more umbrellas, keeping them at home or her workplace. Before any trip (from home to work, or vice versa), Bella looks outside to decide whether to bring an umbrella on that trip:
///
/// if it’s raining, she brings an umbrella;
/// otherwise, if there are currently no umbrellas at her destination (either work or home), she still brings an umbrella, just in case;
/// otherwise, she doesn’t bring an umbrella.
/// The second rule above is meant to prevent a situation where Bella needs an umbrella but has none at her current location (a bad memory she will talk about to anyone who asks). This guarantees that Bella will never catch rain and get sick.
///
/// Now we need you to simulate Bella’s method for a certain period. The simulation starts with Bella at home. Each day she takes two bus trips: to and back from work. Given the starting numbers of umbrellas at her home and workplace, and the weather reports during N consecutive days, find out whether or not Bella brought an umbrella on each of her 2N bus trips.
///
/// Input
/// The first line contains three integers N (1 ≤ N ≤ 104), H (1 ≤ H ≤ 100), and W (0 ≤ W ≤ 100), indicating respectively the duration of the simulation period in days, and the starting numbers of umbrellas at Bella’s home and workplace. For i = 1, 2, ... , N, the i-th of the next N lines contains two characters representing whether it rained on each trip of the i-th day. The first character refers to the first trip of the day (from home to work), while the second character refers to the second trip of the day (from work to home). Each character is the uppercase letter “Y” if it rained, and the uppercase letter “N” otherwise.
///
/// Output
/// Output N lines. For i = 1, 2, ... , N, the i-th line must contain two characters indicating whether Bella brought an umbrella on each trip of the i-th day. The first character refers to the first trip while the second character refers to the second trip. Each character must be the uppercase letter “Y” if Bella brought an umbrella, and the uppercase letter “N” otherwise.
///
/// Examples
///
/// Input
/// 5 2 1
/// Y N
/// N N
/// Y N
/// N Y
/// Y Y
///
/// Output
/// Y N
/// N N
/// Y Y
/// N Y
/// Y Y

#[derive(Debug)]
enum Location {
    Home,
    Workplace,
}

fn solve(mut umbrellas_at_home: u32, mut umbrellas_at_workplace: u32, trips: &[char]) -> Vec<char> {
    let mut results = Vec::with_capacity(trips.len());
    let mut location = Location::Home;

    for rained in trips {
        match location {
            Location::Home => {
                if (rained == &'Y' || umbrellas_at_workplace == 0) && umbrellas_at_home > 0 {
                    umbrellas_at_home -= 1;
                    umbrellas_at_workplace += 1;
                    results.push('Y');
                } else {
                    results.push('N');
                }

                location = Location::Workplace;
            }
            Location::Workplace => {
                if (rained == &'Y' || umbrellas_at_home == 0) && umbrellas_at_workplace > 0 {
                    umbrellas_at_workplace -= 1;
                    umbrellas_at_home += 1;
                    results.push('Y');
                } else {
                    results.push('N');
                }

                location = Location::Home;
            }
        }
    }

    results
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut parts = buffer.split_whitespace();
    let days = parts.next().unwrap().parse::<u32>().unwrap();
    let umbrellas_at_home = parts.next().unwrap().parse::<u32>().unwrap();
    let umbrellas_at_workplace = parts.next().unwrap().parse::<u32>().unwrap();

    let mut weather = Vec::new();
    for _ in 0..days {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        weather.extend(buffer.chars().filter(|c| !c.is_whitespace()));
    }

    let result = solve(umbrellas_at_home, umbrellas_at_workplace, &weather);

    let mut i = 0;
    while i < result.len() {
        println!("{} {}", result[i], result[i + 1]);
        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(2, 1, &['Y', 'N', 'N', 'N', 'Y', 'N', 'N', 'Y', 'Y', 'Y']);
        assert_eq!(
            vec!['Y', 'N', 'N', 'N', 'Y', 'Y', 'N', 'Y', 'Y', 'Y',],
            result
        );
    }
}
