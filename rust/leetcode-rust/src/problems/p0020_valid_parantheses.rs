pub fn is_valid(s: String) -> bool {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Parentheses {
        Round,
        Square,
        Curly,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ParenthesesType {
        Open(Parentheses),
        Close(Parentheses),
    }

    impl TryFrom<char> for ParenthesesType {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '(' => Ok(ParenthesesType::Open(Parentheses::Round)),
                ')' => Ok(ParenthesesType::Close(Parentheses::Round)),
                '[' => Ok(ParenthesesType::Open(Parentheses::Square)),
                ']' => Ok(ParenthesesType::Close(Parentheses::Square)),
                '{' => Ok(ParenthesesType::Open(Parentheses::Curly)),
                '}' => Ok(ParenthesesType::Close(Parentheses::Curly)),
                _ => Err("Invalid character"),
            }
        }
    }

    let mut stack: Vec<Parentheses> = Vec::new();

    s.chars()
        .map(|c| ParenthesesType::try_from(c))
        .filter_map(Result::ok)
        .find_map(|p| match p {
            ParenthesesType::Open(parentheses) => {
                stack.push(parentheses);
                None
            }
            ParenthesesType::Close(parentheses) => match stack.pop() {
                Some(last_paranthesis) => match last_paranthesis == parentheses {
                    true => None,
                    false => Some(()),
                },
                None => Some(()),
            },
        })
        .is_none() && stack.is_empty()
}
