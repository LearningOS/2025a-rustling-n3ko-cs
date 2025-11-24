// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
// just like the linklist with a shabby head node in C/C++
// Wow, do you like what you see?
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2. 对切片中的每个字符串首字母大写，返回 Vec<String>
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 迭代 words 中的每个 &str，应用 capitalize_first，收集到 Vec<String>
    words.iter()
        .map(|&s| capitalize_first(s)) // 解引用 &(&str) 得到 &str，传给 capitalize_first
        .collect() // collect 自动推断目标类型为 Vec<String>（因函数返回值指定）
}

// Step 3. 对切片中的每个字符串首字母大写，拼接成单个 String
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 先通过 map 处理每个字符串，再用 fold 拼接成一个 String
    words.iter()
        .map(|&s| capitalize_first(s)) // 每个元素首字母大写
        .fold(String::new(), |acc, s| acc + &s) // 累积拼接：acc 是累加结果，s 是当前处理的字符串
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
