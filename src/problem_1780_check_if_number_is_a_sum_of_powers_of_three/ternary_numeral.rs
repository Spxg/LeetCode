pub struct Solution;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n != 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

impl super::Solution for Solution {
    fn check_powers_of_three(n: i32) -> bool {
        Self::check_powers_of_three(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
