pub fn calculate(str: &str) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut cur_num: Option<i32> = None;

    for char in str.trim().chars() {
        if char.is_numeric() {
            let digit = char.to_digit(10).unwrap() as i32;
            match cur_num {
                Some(num) => cur_num = Some(num * 10 + digit),
                None => cur_num = Some(digit),
            }
        }
        if char.is_whitespace() && cur_num.is_some() {
            stack.push(cur_num.unwrap());
            cur_num = None;
        }
        match char {
            '+' | '-' | '*' | '/' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match char {
                    '+' => stack.push(left + right),
                    '-' => stack.push(left - right),
                    '*' => stack.push(left * right),
                    '/' => stack.push(left / right),
                    _ => (),
                }
            }
            _ => (),
        }
    }
    stack.pop().unwrap()
}

#[test]
fn add() {
    assert_eq!(calculate("3 3 +"), 6);
    assert_eq!(calculate("3 3 + 3 3 + +"), 12);
}

#[test]
fn multiply() {
    assert_eq!(calculate("3 3 * 3 3 * *"), 81);
    assert_eq!(calculate("10 4 2 / *"), 20);
}
