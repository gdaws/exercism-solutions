pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for char in string.chars() {
        let open = char == '(' || char == '{' || char == '[';
        let close = char == ')' || char == '}' || char == ']';
        if open {
            stack.push(char);
        } else if close {
            match stack.pop() {
                Some(open_char) => {
                    let mismatched = (char == ')' && open_char != '(')
                        || (char == '}' && open_char != '{')
                        || (char == ']' && open_char != '[');
                    if mismatched {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
    stack.len() == 0
}
