impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {

        let mut n = n;
        while n > 0 && n % 3 != 2{
            if n%3 == 1{
                n -= 1;
            } else{
                n /= 3;
            }
        }
        n == 0

    }
}
