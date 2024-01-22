use forge_lib::expression::{BinaryOp, ExpressionNode};

use crate::error::ParseError;

use super::{
    address::{parse_bin16_with_position, parse_hex16_with_position},
    Scanner, Token,
};

impl Scanner {
    pub fn parse_scoped_reference(&mut self) -> Result<Option<ExpressionNode>, ParseError> {
        let mut references = Vec::new();

        if let Some(Token::Identifier(id)) = self.identifier()? {
            references.push(id);
            while self.peek_chars("::") {
                self.consume_chars(2); // Consume "::"
                if let Some(Token::Identifier(id)) = self.identifier()? {
                    references.push(id);
                } else {
                    return Err(ParseError::UnexpectedEndOfInput);
                }
            }

            if references.len() > 1 {
                return Ok(Some(ExpressionNode::ScopedReference(references)));
            } else {
                return Ok(Some(ExpressionNode::Identifier(references[0].clone())));
            }
        }

        Ok(None)
    }

    pub fn high_precedence_operator(&mut self) -> Result<Option<BinaryOp>, ParseError> {
        let start_pos = self.cursor;

        match self.peek() {
            Some('*') => {
                self.next();
                Ok(Some(BinaryOp::Multiply))
            }
            Some('/') => {
                self.next();
                Ok(Some(BinaryOp::Divide))
            }
            Some('<') => {
                self.next(); // Consume <
                if let Some('<') = self.peek() {
                    self.next(); // consume second '<'
                    Ok(Some(BinaryOp::ShiftLeft))
                } else {
                    self.cursor = start_pos;
                    Ok(None)
                }
            }
            Some('>') => {
                self.next();
                if let Some('>') = self.peek() {
                    self.next();
                    Ok(Some(BinaryOp::ShiftRight))
                } else {
                    self.cursor = start_pos;
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn low_precedence_operator(&mut self) -> Result<Option<BinaryOp>, ParseError> {
        match self.peek() {
            Some('+') => {
                self.next();
                Ok(Some(BinaryOp::Add))
            }
            Some('-') => {
                self.next();
                Ok(Some(BinaryOp::Subtract))
            }
            Some('|') => {
                self.next();
                Ok(Some(BinaryOp::Or))
            }
            Some('&') => {
                self.next();
                Ok(Some(BinaryOp::And))
            }
            _ => Ok(None),
        }
    }

    pub fn number(&mut self) -> Result<Option<u16>, ParseError> {
        let _start_pos = self.cursor;

        if let Some(c) = self.peek() {
            match c {
                // Hex number
                '$' => {
                    self.next();

                    // Now parse until there is no more hex digits
                    let parse_pos = self.cursor;
                    while let Some(c) = self.peek() {
                        if !c.is_ascii_hexdigit() {
                            break;
                        }
                        self.next();
                    }

                    let value: String = self.input[parse_pos..self.cursor].iter().collect();
                    let number = parse_hex16_with_position(&value, self.cursor)?;

                    // Convert to a string
                    Ok(Some(number))
                }
                // Binary
                '%' => {
                    self.next();

                    // Now parse until there is no more binary digits
                    let parse_pos = self.cursor;
                    while let Some(_) = self.peek() {
                        if !self.consume_char('0') && !self.consume_char('1') {
                            break;
                        }
                    }

                    let value: String = self.input[parse_pos..self.cursor].iter().collect();
                    let number = parse_bin16_with_position(&value, self.cursor)?;
                    Ok(Some(number))
                }
                // Decimal
                char if char.is_digit(10) => {
                    // Now parse until there are no more base 10 digits
                    let parse_pos = self.cursor;
                    while let Some(c) = self.peek() {
                        if !c.is_digit(10) {
                            break;
                        }
                        self.next();
                    }

                    let value: String = self.input[parse_pos..self.cursor].iter().collect();
                    let number = value
                        .parse::<u16>()
                        .map_err(|_| ParseError::ParseIntError {
                            msg: format!("failed to convert {} to base 10", value),
                            position: self.cursor,
                        })?;
                    Ok(Some(number))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn expression(&mut self) -> Result<Option<ExpressionNode>, ParseError> {
        let mut left = match self.term()? {
            Some(left) => left,
            None => return Ok(None),
        };

        while let Some(op) = self.low_precedence_operator()? {
            let right = match self.term()? {
                Some(right) => right,
                None => return Ok(None),
            };
            left = ExpressionNode::BinOp(op, Box::new(left), Box::new(right));
        }

        Ok(Some(left))
    }

    fn term(&mut self) -> Result<Option<ExpressionNode>, ParseError> {
        let mut left = match self.factor()? {
            Some(left) => left,
            None => return Ok(None),
        };

        while let Some(op) = self.high_precedence_operator()? {
            let right = match self.factor()? {
                Some(right) => right,
                None => return Ok(None),
            };
            left = ExpressionNode::BinOp(op, Box::new(left), Box::new(right))
        }

        Ok(Some(left))
    }

    fn factor(&mut self) -> Result<Option<ExpressionNode>, ParseError> {
        // Consume all whitespaces
        self.consume_all_whitespace();

        let result = if let Some(num) = self.number()? {
            ExpressionNode::Number(num)
        } else if let Some(ref_expr) = self.parse_scoped_reference()? {
            ref_expr
        } else if let Some(token) = self.identifier()? {
            match token {
                Token::Identifier(ident) => ExpressionNode::Identifier(ident),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: Token::Identifier("".to_string()),
                        received: token,
                        position: self.cursor,
                    })
                }
            }
        } else if self.consume_char('(') {
            let expr = match self.expression()? {
                Some(expr) => expr,
                None => return Ok(None),
            };
            if !self.consume_char(')') {
                return Err(ParseError::MissingClosingParenthesis);
            }
            ExpressionNode::Parenthesized(Box::new(expr))
        } else {
            return Ok(None);
        };

        self.consume_all_whitespace();

        Ok(Some(result))
    }
}

#[cfg(test)]
mod expression_tests {
    use std::collections::HashMap;

    use forge_lib::expression::evaluate_expression;

    use crate::scanner::{
        expression::{BinaryOp, ExpressionNode},
        Scanner,
    };

    #[test]
    fn test_parse_high_precedence_operators() {
        let mut scanner = Scanner::new("*/<<>>");
        let result = scanner.high_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::Multiply));

        let result = scanner.high_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::Divide));

        let result = scanner.high_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::ShiftLeft));

        let result = scanner.high_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::ShiftRight));
    }

    #[test]
    fn test_parse_low_precedence_operators() {
        let mut scanner = Scanner::new("+-|&");
        let result = scanner.low_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::Add));

        let result = scanner.low_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::Subtract));

        let result = scanner.low_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::Or));

        let result = scanner.low_precedence_operator();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(BinaryOp::And));
    }

    #[test]
    fn test_parse_number() {
        let mut scanner = Scanner::new("$4400");
        let result = scanner.number();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0x4400));

        let mut scanner = Scanner::new("$44");
        let result = scanner.number();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0x44));

        let mut scanner = Scanner::new("$44FFF");
        let result = scanner.number();

        assert!(result.is_err());

        let mut scanner = Scanner::new("%1000");
        let result = scanner.number();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0x08));

        let mut scanner = Scanner::new("%1010011101101000");
        let result = scanner.number();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0xA768));

        let mut scanner = Scanner::new("42635");
        let result = scanner.number();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(42635));
    }

    #[test]
    fn test_parse_expression() {
        let mut scanner = Scanner::new("((mapper & $0f) << 4) | (mirroring & 1)");
        let result = scanner.expression();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ExpressionNode::BinOp(
                BinaryOp::Or,
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::ShiftLeft,
                        Box::new(ExpressionNode::Parenthesized(Box::new(
                            ExpressionNode::BinOp(
                                BinaryOp::And,
                                Box::new(ExpressionNode::Identifier("mapper".to_string())),
                                Box::new(ExpressionNode::Number(0x0F))
                            )
                        ))),
                        Box::new(ExpressionNode::Number(4))
                    )
                ))),
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::And,
                        Box::new(ExpressionNode::Identifier("mirroring".to_string())),
                        Box::new(ExpressionNode::Number(1))
                    )
                )))
            ))
        );

        let mut scanner = Scanner::new("; Comment");
        let result = scanner.expression();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_eval_expression() {
        let mut scanner = Scanner::new("1 + 2 + 3");
        let expression = scanner.expression().unwrap().unwrap();

        let constant_map: HashMap<String, u16> = HashMap::new();

        let num = evaluate_expression(&expression, &constant_map);
        assert_eq!(num, 6);

        let mut scanner = Scanner::new("((mapper & $0f) << 4) | (mirroring & 1)");
        let expression = scanner.expression().unwrap().unwrap();

        let mut constant_map: HashMap<String, u16> = HashMap::new();
        constant_map.insert(String::from("mapper"), 0);
        constant_map.insert(String::from("mirroring"), 1);

        let num = evaluate_expression(&expression, &constant_map);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_parse_expression_scopes() {
        let mut scanner = Scanner::new("Joypad::Down");
        let result = scanner.expression();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ExpressionNode::ScopedReference(vec![
                String::from("Joypad"),
                String::from("Down")
            ]))
        );

        let mut scanner = Scanner::new("PPUSTATUS");
        let result = scanner.expression();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ExpressionNode::Identifier(String::from("PPUSTATUS")))
        );

        let mut scanner = Scanner::new("Player::Joypad::Down");
        let result = scanner.expression();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ExpressionNode::ScopedReference(vec![
                String::from("Player"),
                String::from("Joypad"),
                String::from("Down")
            ]))
        );
    }
}
