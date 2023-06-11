//! ```
//! struct Rule {
//!     divisor: u32,
//!     output: String,
//! }
//! 
//! impl Rule {
//!     fn new(divisor: u32, output: &str) -> Rule {
//!         Rule { divisor, output: output.to_string() }
//!     }
//! 
//!     fn apply(&self, value: u32) -> Option<String> {
//!         if value % self.divisor == 0 {
//!             Some(self.output.clone())
//!         } else {
//!             None
//!         }
//!     }
//! }
//! 
//! fn fizz_buzz_whizz(start: u32, end: u32) {
//!     let rules = vec![
//!         Rule::new(3, "Fizz"),
//!         Rule::new(5, "Buzz"),
//!         Rule::new(7, "Whizz"),
//!     ];
//! 
//!     for i in start..=end {
//!         let mut output = String::new();
//! 
//!         for rule in &rules {
//!             if let Some(s) = rule.apply(i) {
//!                 output.push_str(&s);
//!             }
//!         }
//! 
//!         if output.is_empty() {
//!             output = i.to_string();
//!         }
//! 
//!         println!("{}", output);
//!     }
//! }
//! 
//! fn main() {
//!     fizz_buzz_whizz(1, 100);
//! }
//! ```


/// # `Rule` 구조체
/// 
/// `Rule` 구조체는 FizzBuzzWhizz 게임의 각 규칙을 표현합니다.
///
/// ## Fields
/// 
/// * `divisor`: 규칙이 적용될 때 해당 숫자가 나누어 떨어져야 하는 수입니다.
/// * `output`: 해당 규칙이 적용될 때 출력되어야 하는 문자열입니다.
pub struct Rule {
    divisor: u32,
    output: String,
}

impl Rule {
    /// `new` 함수는 새로운 `Rule` 객체를 생성합니다.
    ///
    /// ## Parameters
    ///
    /// * `divisor`: 규칙이 적용될 때 해당 숫자가 나누어 떨어져야 하는 수입니다.
    /// * `output`: 해당 규칙이 적용될 때 출력되어야 하는 문자열입니다.
    pub fn new(divisor: u32, output: &str) -> Rule {
        Rule { divisor, output: output.to_string() }
    }

    /// `apply` 함수는 주어진 숫자에 규칙을 적용하는 함수입니다.
    ///
    /// ## Parameters
    ///
    /// * `value`: 규칙을 적용할 숫자입니다.
    ///
    /// ## Returns
    ///
    /// 규칙이 적용될 경우, 규칙에 따른 출력 문자열을 `Option`으로 감싼 형태로 반환합니다.
    /// 규칙이 적용되지 않을 경우, `None`을 반환합니다.
    pub fn apply(&self, value: u32) -> Option<String> {
        if value % self.divisor == 0 {
            Some(self.output.clone())
        } else {
            None
        }
    }
}

/// 이 함수는 주어진 범위의 숫자에 대해 FizzBuzzWhizz 게임의 규칙을 적용하고 결과를 출력합니다.
///
/// # Parameters
///
/// * `start`: 범위의 시작 숫자입니다.
/// * `end`: 범위의 끝 숫자입니다.
///
/// # 규칙
///
/// * 3의 배수는 "Fizz"
/// * 5의 배수는 "Buzz"
/// * 7의 배수는 "Whizz"
pub fn fizz_buzz_whizz(start: u32, end: u32) {
    let rules = vec![
        Rule::new(3, "Fizz"),
        Rule::new(5, "Buzz"),
        Rule::new(7, "Whizz"),
    ];

    for i in start..=end {
        let mut output = String::new();

        for rule in &rules {
            if let Some(s) = rule.apply(i) {
                output.push_str(&s);
            }
        }

        if output.is_empty() {
            output = i.to_string();
        }

        println!("{}", output);
    }
}

fn main() {
    fizz_buzz_whizz(1, 105);
}
