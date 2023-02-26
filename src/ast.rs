use std::iter::Peekable;


pub mod lexer;
use crate::ast::lexer::{Token, Keyword, Symbol};


#[derive(Debug)]
enum ExpressionItem {
    Variable(Option<Keyword>, Vec<String>),
    Other(Token),
} impl ExpressionItem {
    pub fn vec_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        let mut result = vec![];
        if let Some(Token::Symbol(Symbol::Semicolon)) = tokens.peek() {
            return Err("Unexpected semicolon (blank expression)");
        }
        while let Some(token) = tokens.peek() {
            match token {
                Token::Symbol(Symbol::Semicolon) => {
                    tokens.next();
                    return Ok(result);
                },
                Token::Keyword(Keyword::This | Keyword::Std) | Token::Identifier(_) => result.push(Self::variable_from_tokens(tokens)?),
                Token::Symbol(_) | Token::Number(_) => {
                    result.push(Self::Other(Token::from_ref(token)));
                    tokens.next();
                },
                Token::Whitespace | Token::Newline => {
                    tokens.next();
                },
                _ => {
                    println!("{:?}", tokens.peek());
                    return Err("Unexpected token in an expression");
                },
            }
        }
        Err("Expression needs to end with ';'")
    }
    pub fn variable_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        let mut identifiers: Vec<String> = vec![];
        let mut prefix = None;
        if let Some(token) = tokens.next() {
            match token {
                Token::Keyword(keyword) => match keyword {
                    Keyword::This | Keyword::Std => {
                        prefix = Some(keyword);
                    },
                    _ => return Err("Unexpected keyword"),
                },
                Token::Identifier(name) => identifiers.push(name.to_string()),
                _ => return Err("Unexpected token"),
            }
        }
        while let Some(token) = tokens.peek() {
            if let Token::Symbol(Symbol::Dot) = token {
                tokens.next();
                if let Some(Token::Identifier(name)) = tokens.next() {
                    identifiers.push(name.to_string());
                } else {
                    return Err("Variable can not end with '.'");
                }
            } else {
                break;
            }
        }
        Ok(Self::Variable(prefix.copied(), identifiers))
    }
}


#[derive(Debug)]
pub enum AbstractSyntaxItem {
    Expression(Vec<ExpressionItem>),
    Namespace(String, Vec<Self>),
} impl AbstractSyntaxItem {
    /*
    pub fn vec_from_file(path: &str, print_tokens: bool, print_preprocess: bool) -> Result<Vec<Self>, &'static str> {
        let tokens = Token::vec_from_file(path, print_preprocess)?;
        if print_tokens {
            eprintln!("{tokens:?}");
        }
        return Self::vec_from_tokens(tokens);
    }
    */
    pub fn vec_from_tokens<'a, I>(tokens: &mut Peekable<I>, is_namespace: bool) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        let mut result = vec![];
        while let Some(token) = tokens.peek() {
            match token {
                Token::Keyword(Keyword::Namespace) => {
                    tokens.next();
                    result.push(Self::namespace_from_tokens(tokens)?);
                },
                Token::Symbol(Symbol::RightCurly) => {
                    tokens.next();
                    if is_namespace {
                        break;
                    }
                    return Err("'}' without an opening '{'");
                },
                Token::Whitespace | Token::Newline => {
                    tokens.next();
                },
                _ => {
                    result.push(Self::expression_from_tokens(tokens)?);
                },
            }
        }
        Ok(result)
    }
    pub fn namespace_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        if let Some(Token::Whitespace) = tokens.next() {} else {
            return Err("Whitespace is required after 'namespace'");
        }
        if let Some(Token::Identifier(name)) = tokens.next() {
            if let Some(&Token::Whitespace) = tokens.peek() {
                tokens.next();
            } else if let Some(&Token::Symbol(Symbol::LeftCurly)) = tokens.peek() {
                tokens.next();
                return Ok(Self::Namespace(name.to_string(), Self::vec_from_tokens(tokens, true)?));
            } else {
                return Err("Unexpected token");
            }
            if let Some(Token::Symbol(Symbol::LeftCurly)) = tokens.next() {
                return Ok(Self::Namespace(name.to_string(), Self::vec_from_tokens(tokens, true)?));
            }
            return Err("'{' is required after a namespace declaration");
        }
        Err("Namespace name should be an identifier")
    }
    pub fn expression_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        Ok(Self::Expression(ExpressionItem::vec_from_tokens(tokens)?))
    }
    /*
    pub fn get_expressions(list: &Vec<Self>, ext_namespace: String, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<Vec<(String, Option<String>)>, &'static str> {
        let mut result = vec![]; 
        let mut namespaces = vec![];
        let mut expression = vec![];
        let mut color = None;
        for i in list {
            match i {
                Self::Visual(c) => color = Some(c.to_string()),
                Self::ExpressionStart => expression = vec![],
                Self::ExpressionEnd => {
                    result.push((expression.join(""), color));
                    color = None;
                },
                Self::NamespaceStart(n) => namespaces.push(n.to_string()),
                Self::NamespaceEnd => _ = namespaces.pop(),
                Self::Token(t) => expression.push(t.to_latex()),
                Self::Variable(i) => {
                    expression.push("A_{{".to_string());
                    if !i.get(0).unwrap().is_empty() {
                        expression.push(ext_namespace.to_string());
                        expression.push(namespaces.join(""));
                    }
                    expression.push(i.join(""));
                    expression.push("}}".to_string());
                },
                Self::Use(path) => {
                    let full_path = format!("./{}.ds", path.join("/")); 
                    let list = &Self::vec_from_file(full_path.as_str(), print_tokens, print_preprocess)?;
                    if print_ast {
                        eprintln!("{list:?}");
                    }
                    let strings = Self::get_expressions(list, format!("{ext_namespace}{}", namespaces.join("")), print_tokens, print_ast, print_preprocess)?;
                    for i in strings {
                        result.push(i);
                    }
                }
            }
        }
        Ok(result)
    }
    */
}
