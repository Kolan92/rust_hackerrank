fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0, 0];

    for i in 0..3 {
        if a[i] > b[i] {
            result[0] += 1;
        } else if a[i] < b[i] {
            result[1] += 1;
        }
    }

    return result;
}

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let max_index = arr.len() - 1;

    let mut first_diag_sum = 0;
    let mut second_diag_sum = 0;
    for i in 0..=max_index {
        first_diag_sum += arr[i][i];
        second_diag_sum += arr[i][max_index - i];
    }

    let diff: i32 = first_diag_sum - second_diag_sum;
    diff.abs()
}

fn plus_minus(arr: &[i32]) {
    let mut neg_counter = 0;
    let mut pos_counter = 0;
    let mut zeros_counter = 0;

    for i in 0..arr.len() {
        let value = arr[i];
        if value < 0 {
            neg_counter += 1;
        } else if value > 0 {
            pos_counter += 1;
        } else {
            zeros_counter += 1;
        }
    }
    let count = arr.len() as f64;
    println!("{:.6}", pos_counter as f64 / count);
    println!("{:.6}", neg_counter as f64 / count);
    println!("{:.6}", zeros_counter as f64 / count);
}

fn staircase(n: i32) {
    for i in 0..n {
        println!(
            "{}{}",
            " ".repeat((n - i - 1) as usize),
            "#".repeat((i + 1) as usize)
        );
    }
}

fn mini_max_sum(arr: &[i32]) {
    let mut vec: Vec<i64> = arr.iter().map(|x| i64::from(*x)).collect();
    vec.sort();
    let min_sum = &vec[0..4];
    let max_sum = &vec[1..5];
    let min_sum: i64 = min_sum.iter().sum();
    let max_sum: i64 = max_sum.iter().sum();
    println!("{min_sum} {max_sum}");
}

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let mut candles = candles.to_vec();
    candles.sort();
    let max = candles.last().unwrap();

    let max_candles: Vec<&i32> = candles.iter().filter(|x| x.eq(&max)).collect();
    max_candles.len() as i32
}

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let house_range = s..=t;
    let apples_count: Vec<i32> = apples
        .iter()
        .map(|apple| apple + a)
        .filter(|apple| house_range.contains(apple))
        .collect();
    let apples_count = apples_count.len();

    let oranges_count: Vec<i32> = oranges
        .iter()
        .map(|orange| orange + b)
        .filter(|orange| house_range.contains(orange))
        .collect();
    let orange_count = oranges_count.len();

    println!("{apples_count}");
    println!("{orange_count}");
}

fn time_conversion(s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let hour = match s[8] {
        'A' => {
            let hour = s[..2].iter().collect::<String>().parse::<i32>().unwrap();
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        'P' => {
            let hour = s[..2].iter().collect::<String>().parse::<i32>().unwrap() + 12;
            if hour == 24 {
                12
            } else {
                hour
            }
        }
        other => panic!("All time stamps provided to the function should be valid {other}"),
    };

    format!("{:0>2}:{}", hour, s[3..8].iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_triplets_test() {
        let a = [5, 6, 7];
        let b = [3, 6, 10];
        let result = compare_triplets(&a, &b);
        assert_eq!(result, [1, 1]);
    }

    #[test]
    fn diagonal_difference_test() {
        let input = [vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
        let result = diagonal_difference(&input);

        assert_eq!(result, 15);
    }

    #[test]
    fn plus_minus_test() {
        plus_minus(&[-4, 3, -9, 0, 4, 1])
    }

    #[test]
    fn staircase_test() {
        staircase(4);
    }

    #[test]
    fn mini_max_sum_test() {
        mini_max_sum(&[1, 2, 3, 4, 5]);
        mini_max_sum(&[1426980153, 354802167, 142980735, 968217435, 734892650])
    }

    #[test]
    fn count_apples_and_oranges_test() {
        count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
    }

    #[test]
    fn time_conversion_test() {
        let result = time_conversion("12:45:54PM");
        assert_eq!(result, "12:45:54");
    }
}
