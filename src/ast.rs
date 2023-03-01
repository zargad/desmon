use std::iter::Peekable;
use std::collections::HashMap;


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
                Token::Whitespace(_) => {
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
    pub fn get_variable_name(&self, namespaces: Vec<String>) -> Option<Vec<String>> {
        match self {
            Self::Variable(Some(Keyword::This), identifiers) => {
                let mut result = namespaces.to_vec();
                result.append(&mut identifiers.to_vec());
                return Some(result);
            },
            Self::Variable(None, identifiers) => Some(identifiers.to_vec()),
            _ => None,
        }
    }
    pub fn vec_to_latex(vec: Vec<Self>, namespaces: Vec<String>) -> String {
        todo!();
    }
}


#[derive(Debug)]
pub enum AbstractSyntaxItem {
    Expression(Vec<ExpressionItem>),
    Namespace(String, Vec<Self>),
    Text(String),
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
    pub fn get_variable_counts(&self, result: &mut HashMap<Vec<String>, u32>, namespaces: &Vec<String>) {
        match self {
            Self::Expression(items) => for i in items {
                if let Some(name) = i.get_variable_name(namespaces.to_vec()) {
                    result
                        .entry(name)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            },
            Self::Namespace(name, items) => {
                let names = &mut namespaces.to_vec();
                names.push(name.to_string());
                for i in items {
                    i.get_variable_counts(result, names);
                }
            },
            _ => (),
        }
    }
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
                Token::Text(t) => {
                    tokens.next();
                    result.push(Self::Text(t.to_string()));
                }
                Token::Whitespace(_) => {
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
        if let Some(Token::Whitespace(false)) = tokens.next() {} else {
            return Err("Whitespace is required after 'namespace'");
        }
        if let Some(Token::Identifier(name)) = tokens.next() {
            if let Some(&Token::Whitespace(_)) = tokens.peek() {
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
    pub fn to_strings_vec(&self, namespaces: &mut Vec<String>) -> Vec<String> {
        match self {
            Self::Expression(items) => {
                let temp = vec![ExpressionItem::vec_to_string(items.to_vec(), namespaces.to_vec())];
                return temp;
            },
            Self::Namespace(name, asts) => {
                namespaces.push(name.to_string());
                let mut result = vec![];
                for ast in asts {
                    result.append(&mut ast.to_strings_vec(namespaces));
                }
                namespaces.pop();
                return result;
            },
        }
    }
    */
}


pub type AbstractSyntaxTree = Vec<AbstractSyntaxItem>;


pub trait AbstractSyntaxTreeTrait {
    fn from_tokens<'a, I>(tokens: &mut Peekable<I>, is_namespace: bool) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>, Self: Sized;
    fn fill_from_tokens<'a, I>(&mut self, tokens: &mut Peekable<I>, is_namespace: bool) -> Result<(), &'static str>
    where I: Iterator<Item = &'a Token>, Self: Sized;
    fn get_variable_ids(&self) -> HashMap<Vec<String>, usize>;
}


impl AbstractSyntaxTreeTrait for AbstractSyntaxTree {
    fn get_variable_ids(&self) -> HashMap<Vec<String>, usize> {
        let mut counts = HashMap::new();
        for i in self {
            i.get_variable_counts(&mut counts, &vec![]);
        }
        let mut hash_vec: Vec<(&Vec<String>, &u32)> = counts.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = HashMap::new();
        for (index, (item, _)) in hash_vec.iter().enumerate() {
            result.insert(item.to_vec(), index);
        }
        result
    }
    fn from_tokens<'a, I>(tokens: &mut Peekable<I>, is_namespace: bool) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        let mut result = vec![];
        result.fill_from_tokens(tokens, is_namespace)?;
        Ok(result)
    }
    fn fill_from_tokens<'a, I>(&mut self, tokens: &mut Peekable<I>, is_namespace: bool) -> Result<(), &'static str>
    where I: Iterator<Item = &'a Token>
    {
        type A = AbstractSyntaxItem;
        while let Some(token) = tokens.peek() {
            match token {
                Token::Keyword(Keyword::Namespace) => {
                    tokens.next();
                    self.push(A::namespace_from_tokens(tokens)?);
                },
                Token::Symbol(Symbol::RightCurly) => {
                    tokens.next();
                    if is_namespace {
                        break;
                    }
                    return Err("'}' without an opening '{'");
                },
                Token::Text(t) => {
                    tokens.next();
                    self.push(A::Text(t.to_string()));
                }
                Token::Whitespace(_) => _ = tokens.next(),
                _ => _ = self.push(A::expression_from_tokens(tokens)?),
            }
        }
        Ok(())
    }
}
