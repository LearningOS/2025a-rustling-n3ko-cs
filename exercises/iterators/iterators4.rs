// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // Solution 1
    // let mut sum = 1;
    // for i in 0..num {
    //     sum *= (i + 1)
    // }
    // sum
    // Solution 2: recursion
    // if num == 1 || num == 0 {
    //     1
    // } else {
    //     num * factorial(num - 1)
    // }
    // Best Solution
    // use iterator and fold
    // fold 是 Rust 迭代器（Iterator）的一个 折叠方法，核心作用是：把迭代器的所有元素，通过一个自定义逻辑 “合并” 成一个单一值。
    // 至于自定义逻辑，那自然是用闭包最好不过了。
    // 首先创建一个从1到num，包含num的迭代器，原来可以这样写吗
    // 然后返回一个值，大模型使用fold，表示初始值为一个1，如果能继续，那就按照闭包逻辑，acc = 1, 捕获acc 和 i，每次返回acc * i;
    (1..=num).fold(1, |acc, i| acc * i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
