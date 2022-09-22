pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }

    let mut min = prices[0];
    let mut max_profit = 0;

    for price in prices.into_iter().skip(1) {
        let profit = price - min;

        if price < min {
            min = price;
        }

        max_profit = std::cmp::max(profit, max_profit);
    }

    max_profit
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(8, max_profit(vec![2, 1, 4, 5, 2, 9, 7]));
        assert_eq!(2, max_profit(vec![2, 1, 2, 1, 0, 1, 2]));
        assert_eq!(3, max_profit(vec![2, 1, 4]));
        assert_eq!(5, max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, max_profit(vec![7, 6, 4, 3, 1]));
    }
}
