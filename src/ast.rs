use std::iter::Peekable;
use std::collections::HashMap;


pub mod lexer;
use crate::ast::lexer::{Token, Keyword, Symbol};


fn latex_from_id(id: usize) -> (String, String) {
    let letters = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut prefixes = vec![];
    for c in "abcdfghijklmnopqstuvwzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        prefixes.push(c.to_string());
    }
    let index = id % prefixes.len();
    let prefix = prefixes.get(index).unwrap();
    let mut result = String::new();
    let mut i = id / prefixes.len();
    while i != 0 {
        let index = i % letters.len();
        let b: u8 = letters.as_bytes()[index];
        let c: char = b as char;
        result.push(c);
        i /= letters.len();
    }
    (prefix.to_string(), result)
}


#[derive(Debug)]
pub enum Variable {
    Absolute(Vec<String>),
    Relative(Vec<String>),
    Std(String),
} impl Variable {
    pub fn get_latex(&self, namespaces: &Vec<String>, ids: &HashMap<Vec<String>, usize>) -> String {
        let consts = vec!["pi", "e", "tau"];
        let funcs = vec!["floor", "random", "abs", "sin", "cos", "tan", "rgb", "hsv"];
        if let Self::Std(name) = self {
            let name = &name.as_str();
            if consts.contains(name) {
                format!("\\{name}")
            } else if funcs.contains(name) {
                format!("\\operatorname{{{name}}}")
            } else {
                String::new()
            }
        } else if let Some(name) = self.get_name(namespaces.to_vec()) {
            if let Some(id) = ids.get(&name) {
                let (prefix, code) = latex_from_id(*id);
                if code.is_empty() {
                    format!("{prefix}")
                } else {
                    format!("{prefix}_{{{code}}}")
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }

    }
    pub fn std_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        if let Some(Token::Keyword(Keyword::Std)) = tokens.next() {
            if let Some(Token::Symbol(Symbol::Dot)) = tokens.next() {
                if let Some(Token::Identifier(name)) = tokens.next() {
                    Ok(Self::Std(name.to_string()))
                } else {
                    Err("Variable can not end with '.'")
                }
            } else {
                Err("Unexpected keyword")
            }
        } else {
            Err("Unexpected token")
        }
    }
    pub fn absolute_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        Ok(Self::Absolute(Self::identifiers_from_tokens(tokens)?))
    }
    pub fn relative_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        if let Some(Token::Keyword(Keyword::This)) = tokens.next() {
            if let Some(Token::Symbol(Symbol::Dot)) = tokens.next() {
                Ok(Self::Relative(Self::identifiers_from_tokens(tokens)?))
            } else {
                Err("Unexpected keyword")
            }
        } else {
            Err("Unexpected token")
        }
    }
    pub fn get_name(&self, namespaces: Vec<String>) -> Option<Vec<String>> {
        match self {
            Self::Relative(identifiers) => {
                let mut result = namespaces.to_vec();
                result.append(&mut identifiers.to_vec());
                Some(result)
            },
            Self::Absolute(identifiers) => Some(identifiers.to_vec()),
            _ => None,
        }
    }
    fn identifiers_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Vec<String>, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        let mut identifiers: Vec<String> = vec![];
        while let Some(token) = tokens.peek() {
            if let Token::Whitespace(_) = token {
                tokens.next();
            }
            if let Some(Token::Identifier(name)) = tokens.next() {
                identifiers.push(name.to_string());
                if let Some(Token::Whitespace(_)) = tokens.peek() {
                    tokens.next();
                }
                if let Some(Token::Symbol(Symbol::Dot)) = tokens.peek() {
                    tokens.next();
                } else {
                    break;
                }
            } else {
                return Err("Expected identifier");
            }
        }
        Ok(identifiers)
    }
}

#[derive(Debug)]
pub enum ExpressionItem {
    Variable(Variable),
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
                Token::Whitespace(_) => { tokens.next(); },
                _ => {
                    return Err("Unexpected token in an expression");
                },
            }
        }
        Err("Expression needs to end with ';'")
    }
    pub fn variable_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        if let Some(&token) = tokens.peek() {
            return Ok(Self::Variable(
                match token {
                    Token::Keyword(Keyword::This) => Variable::relative_from_tokens(tokens)?,
                    Token::Keyword(Keyword::Std) => Variable::std_from_tokens(tokens)?,
                    Token::Identifier(_) => Variable::absolute_from_tokens(tokens)?,
                    _ => Err("Unexpected token")?,
                }
            ));
        }
        Err("Empty variable")
    }
    pub fn get_variable_name(&self, namespaces: Vec<String>) -> Option<Vec<String>> {
        if let Self::Variable(v) = self {
            v.get_name(namespaces)
        } else {
            None
        }
    }
    pub fn get_latex(&self, namespaces: &Vec<String>, ids: &HashMap<Vec<String>, usize>) -> String {
        match self {
            Self::Variable(v) => v.get_latex(namespaces, ids),
            Self::Other(t) => t.get_latex(),
            // _ => String::new(),
        }
    }
    pub fn vec_to_latex(vec: Vec<Self>, namespaces: Vec<String>, ids: &HashMap<Vec<String>, usize>) -> String {
        let mut result = String::new();
        for i in vec {
            result.push_str(i.get_latex(&namespaces, ids).as_str());
        }
        result
    }
}


#[derive(Debug)]
pub enum AbstractSyntaxItem {
    Expression(Vec<ExpressionItem>),
    Graph(ExpressionItem, Vec<ExpressionItem>),
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
            Self::Graph(color, items) => {
                for i in items {
                    if let Some(name) = i.get_variable_name(namespaces.to_vec()) {
                        result
                            .entry(name)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                if let Some(name) = color.get_variable_name(namespaces.to_vec()) {
                    result
                        .entry(name)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            },
            _ => (),
        }
    }
    pub fn graph_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        if let Some(Token::Whitespace(false)) = tokens.next() {} else {
            Err("Whitespace is required after 'visual'")?;
        }
        let color = ExpressionItem::variable_from_tokens(tokens)?;
        if let Some(Token::Whitespace(_)) = tokens.peek() {
            tokens.next();
        }
        if let Some(Token::Symbol(Symbol::Colon)) = tokens.next() {} else {
            Err("':' expected")?;
        }
        let graph = ExpressionItem::vec_from_tokens(tokens)?;
        eprintln!("{color:?} - {graph:?}");
        Ok(Self::Graph(color, graph))
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
            }
            return if let Some(&Token::Symbol(Symbol::LeftCurly)) = tokens.next() {
                 Ok(Self::Namespace(name.to_string(), AbstractSyntaxTree::from_tokens(tokens, true)?))
            } else {
                 Err("Unexpected token")
            };
        }
        Err("Namespace name should be an identifier")
    }
    pub fn expression_from_tokens<'a, I>(tokens: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = &'a Token>
    {
        Ok(Self::Expression(ExpressionItem::vec_from_tokens(tokens)?))
    }
    /*
    pub fn to_latex_vec(&self, namespaces: &mut Vec<String>) -> Vec<String> {
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
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));
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
                        return Ok(());
                    }
                    return Err("'}' without an opening '{'");
                },
                Token::Keyword(Keyword::Graph) => {
                    tokens.next();    
                    self.push(A::graph_from_tokens(tokens)?);
                },
                Token::Text(t) => {
                    tokens.next();
                    self.push(A::Text(t.to_string()));
                }
                Token::Whitespace(_) => { tokens.next(); },
                _ => { self.push(A::expression_from_tokens(tokens)?); },
            }
        }
        if is_namespace {
            Err("Unclosed namespace")
        } else {
            Ok(())
        }
    }
}
