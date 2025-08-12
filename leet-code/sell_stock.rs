pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        if price < min_price {
            min_price = price;
        } else {
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }

    max_profit
}

pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = 1;
    let mut max = 0;
    while r < prices.len() {
        if prices[l] < prices[r] {
            let profit = prices[r] - prices[l];
            if profit > max {
                max = profit;
            }
        } else {
            l = r;
        }
        r += 1;
    }
    max
}

fn main() {
    let test_cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![1], 0),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![5, 5, 5, 5], 0),
        (vec![1, 2, 3, 4, 5, 6], 5),
        (vec![6, 5, 4, 3, 2, 1], 0),
        (vec![1000, 2000, 3000, 4000, 5000], 4000),
    ];

    for (input, expected) in test_cases {
        let result = max_profit(input.clone());
        println!("Input: {:?} | Expected: {} | Got: {} -> {}", input, expected, result, if result == expected { "Ok" } else { "Fail" });
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;   
    #[test]
    fn test_max_profit_basic() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }   
    #[test]
    fn test_max_profit_no_profit() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
    #[test]
    fn test_max_profit_single_day() {
        assert_eq!(max_profit(vec![1]), 0);
    }
    #[test]
    fn test_max_profit_multiple_days() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
    #[test]
    fn test_max_profit_same_price() {
        assert_eq!(max_profit(vec![5, 5, 5, 5]), 0);
    }
    #[test]
    fn test_max_profit_increasing_prices() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5, 6]), 5);
    }
    #[test]
    fn test_max_profit_decreasing_prices() {
        assert_eq!(max_profit(vec![6, 5, 4, 3, 2, 1]), 0);
    }
    #[test]
    fn test_max_profit_large_numbers() {
        assert_eq!(max_profit(vec![1000, 2000, 3000, 4000, 5000]), 4000);
    }
}