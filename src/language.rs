//! Small, deterministic expression language for the Rust Mathematics API.
//!
//! This is an intentionally bounded expression evaluator, not a general-purpose
//! sandbox. It supports integer literals, identifiers, parentheses, arithmetic,
//! comparisons, and boolean logic. Function calls, assignments, loops, and object
//! declarations belong to later parser/runtime milestones.
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub message: String,
    pub offset: usize,
}

impl EvalError {
    fn new(message: impl Into<String>, offset: usize) -> Self {
        Self { message: message.into(), offset }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for EvalError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Integer(i64),
    Identifier(String),
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LeftParen,
    RightParen,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Bang,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    offset: usize,
}

fn tokenize(source: &str) -> Result<Vec<Token>, EvalError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];
        if byte.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        if byte.is_ascii_digit() {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
            let raw = &source[start..i];
            let value = raw.parse::<i64>()
                .map_err(|_| EvalError::new("integer literal is out of range", start))?;
            tokens.push(Token { kind: TokenKind::Integer(value), offset: start });
            continue;
        }
        if byte.is_ascii_alphabetic() || byte == b'_' {
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            let raw = &source[start..i];
            let kind = match raw {
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                _ => TokenKind::Identifier(raw.to_owned()),
            };
            tokens.push(Token { kind, offset: start });
            continue;
        }

        let pair = if i + 1 < bytes.len() { &source[i..i + 2] } else { "" };
        let (kind, width) = match pair {
            "==" => (Some(TokenKind::EqualEqual), 2),
            "!=" => (Some(TokenKind::BangEqual), 2),
            "<=" => (Some(TokenKind::LessEqual), 2),
            ">=" => (Some(TokenKind::GreaterEqual), 2),
            "&&" => (Some(TokenKind::AndAnd), 2),
            "||" => (Some(TokenKind::OrOr), 2),
            _ => {
                let kind = match byte {
                    b'+' => Some(TokenKind::Plus),
                    b'-' => Some(TokenKind::Minus),
                    b'*' => Some(TokenKind::Star),
                    b'/' => Some(TokenKind::Slash),
                    b'%' => Some(TokenKind::Percent),
                    b'(' => Some(TokenKind::LeftParen),
                    b')' => Some(TokenKind::RightParen),
                    b'<' => Some(TokenKind::Less),
                    b'>' => Some(TokenKind::Greater),
                    b'!' => Some(TokenKind::Bang),
                    _ => None,
                };
                (kind, 1)
            }
        };
        match kind {
            Some(kind) => {
                tokens.push(Token { kind, offset: i });
                i += width;
            }
            None => return Err(EvalError::new(format!("unexpected character '{}'", byte as char), i)),
        }
    }

    tokens.push(Token { kind: TokenKind::End, offset: source.len() });
    Ok(tokens)
}

/// Evaluate one expression using explicitly supplied integer/boolean variables.
pub fn evaluate(source: &str, variables: &HashMap<String, Value>) -> Result<Value, EvalError> {
    let tokens = tokenize(source)?;
    let mut parser = Parser { tokens, current: 0, variables };
    let result = parser.parse_or()?;
    if !matches!(parser.peek().kind, TokenKind::End) {
        return Err(EvalError::new("unexpected token after expression", parser.peek().offset));
    }
    Ok(result)
}

struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    variables: &'a HashMap<String, Value>,
}

impl Parser<'_> {
    fn peek(&self) -> &Token { &self.tokens[self.current] }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if !matches!(token.kind, TokenKind::End) { self.current += 1; }
        token
    }

    fn matches(&mut self, predicate: fn(&TokenKind) -> bool) -> Option<Token> {
        if predicate(&self.peek().kind) { Some(self.advance()) } else { None }
    }

    fn parse_or(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_and()?;
        while self.matches(|k| matches!(k, TokenKind::OrOr)).is_some() {
            let right = self.parse_and()?;
            left = Value::Boolean(as_bool(left, self.peek().offset)? || as_bool(right, self.peek().offset)?);
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_equality()?;
        while self.matches(|k| matches!(k, TokenKind::AndAnd)).is_some() {
            let right = self.parse_equality()?;
            left = Value::Boolean(as_bool(left, self.peek().offset)? && as_bool(right, self.peek().offset)?);
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = if self.matches(|k| matches!(k, TokenKind::EqualEqual)).is_some() {
                Some(true)
            } else if self.matches(|k| matches!(k, TokenKind::BangEqual)).is_some() {
                Some(false)
            } else { None };
            let Some(equal) = op else { break };
            let right = self.parse_comparison()?;
            let same = left == right;
            left = Value::Boolean(if equal { same } else { !same });
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_term()?;
        loop {
            let op = if self.matches(|k| matches!(k, TokenKind::Less)).is_some() { Some(0) }
                else if self.matches(|k| matches!(k, TokenKind::LessEqual)).is_some() { Some(1) }
                else if self.matches(|k| matches!(k, TokenKind::Greater)).is_some() { Some(2) }
                else if self.matches(|k| matches!(k, TokenKind::GreaterEqual)).is_some() { Some(3) }
                else { None };
            let Some(op) = op else { break };
            let offset = self.peek().offset;
            let right = self.parse_term()?;
            let (a, b) = (as_int(left, offset)?, as_int(right, offset)?);
            left = Value::Boolean(match op { 0 => a < b, 1 => a <= b, 2 => a > b, _ => a >= b });
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = if self.matches(|k| matches!(k, TokenKind::Plus)).is_some() { Some(true) }
                else if self.matches(|k| matches!(k, TokenKind::Minus)).is_some() { Some(false) }
                else { None };
            let Some(add) = op else { break };
            let offset = self.peek().offset;
            let right = self.parse_factor()?;
            let (a, b) = (as_int(left, offset)?, as_int(right, offset)?);
            let value = if add { a.checked_add(b) } else { a.checked_sub(b) }
                .ok_or_else(|| EvalError::new("integer overflow", offset))?;
            left = Value::Integer(value);
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Value, EvalError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = if self.matches(|k| matches!(k, TokenKind::Star)).is_some() { Some(0) }
                else if self.matches(|k| matches!(k, TokenKind::Slash)).is_some() { Some(1) }
                else if self.matches(|k| matches!(k, TokenKind::Percent)).is_some() { Some(2) }
                else { None };
            let Some(op) = op else { break };
            let offset = self.peek().offset;
            let right = self.parse_unary()?;
            let (a, b) = (as_int(left, offset)?, as_int(right, offset)?);
            let value = match op {
                0 => a.checked_mul(b),
                1 => a.checked_div(b).filter(|_| b != 0 && a.checked_rem(b) == Some(0)),
                _ => a.checked_rem(b).filter(|_| b != 0),
            }.ok_or_else(|| EvalError::new("invalid arithmetic operation, non-integral division, or overflow", offset))?;
            left = Value::Integer(value);
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Value, EvalError> {
        if let Some(token) = self.matches(|k| matches!(k, TokenKind::Bang)) {
            return Ok(Value::Boolean(!as_bool(self.parse_unary()?, token.offset)?));
        }
        if let Some(token) = self.matches(|k| matches!(k, TokenKind::Minus)) {
            let value = as_int(self.parse_unary()?, token.offset)?;
            return value.checked_neg().map(Value::Integer)
                .ok_or_else(|| EvalError::new("integer overflow", token.offset));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Value, EvalError> {
        let token = self.advance();
        match token.kind {
            TokenKind::Integer(value) => Ok(Value::Integer(value)),
            TokenKind::True => Ok(Value::Boolean(true)),
            TokenKind::False => Ok(Value::Boolean(false)),
            TokenKind::Identifier(name) => self.variables.get(&name).cloned()
                .ok_or_else(|| EvalError::new(format!("unknown variable '{name}'"), token.offset)),
            TokenKind::LeftParen => {
                let value = self.parse_or()?;
                if self.matches(|k| matches!(k, TokenKind::RightParen)).is_none() {
                    return Err(EvalError::new("expected ')'", self.peek().offset));
                }
                Ok(value)
            }
            _ => Err(EvalError::new("expected a literal, variable, or parenthesized expression", token.offset)),
        }
    }
}

fn as_int(value: Value, offset: usize) -> Result<i64, EvalError> {
    match value {
        Value::Integer(v) => Ok(v),
        Value::Boolean(_) => Err(EvalError::new("expected integer value", offset)),
    }
}

fn as_bool(value: Value, offset: usize) -> Result<bool, EvalError> {
    match value {
        Value::Boolean(v) => Ok(v),
        Value::Integer(_) => Err(EvalError::new("expected boolean value", offset)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_precedence_and_parentheses() {
        let vars = HashMap::new();
        assert_eq!(evaluate("2 + 3 * (4 - 1)", &vars), Ok(Value::Integer(11)));
    }

    #[test]
    fn evaluates_variables_comparisons_and_logic() {
        let vars = HashMap::from([
            ("level".to_string(), Value::Integer(12)),
            ("enabled".to_string(), Value::Boolean(true)),
        ]);
        assert_eq!(evaluate("level >= 10 && enabled", &vars), Ok(Value::Boolean(true)));
        assert_eq!(evaluate("!(level < 10) || false", &vars), Ok(Value::Boolean(true)));
    }

    #[test]
    fn rejects_unknown_names_overflow_and_bad_math() {
        let vars = HashMap::new();
        assert!(evaluate("missing + 1", &vars).is_err());
        assert!(evaluate("9223372036854775807 + 1", &vars).is_err());
        assert!(evaluate("9 / 2", &vars).is_err());
        assert!(evaluate("4 / 0", &vars).is_err());
        assert!(evaluate("1 && true", &vars).is_err());
        assert!(evaluate("2 @ 3", &vars).is_err());
    }
}
