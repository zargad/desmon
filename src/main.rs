// use std::env;
// use std::fs::{/*File, */read_to_string};
// use std::io::{prelude::*, BufReader};
use std::iter::Peekable;
use std::{thread, time};


#[derive(Debug)]
struct SymbolTree {
    value: char,
    name: Option<Symbol>,
    children: Vec<Self>,
} impl SymbolTree {
    pub fn from_vec(value: char, vec: Vec<(&'static str, Symbol)>) -> Self{
        let mut result = Self::from_value(value);
        for (branch, name) in vec {
            result.set(branch.to_string(), name);
        }
        result
    }
    pub fn from_value(value: char) -> Self {
        Self {value, name: None, children: vec![]}
    }
    pub fn set(&mut self, branch: String, name: Symbol) {
        let mut chars = branch.chars();
        if let Some(first) = chars.next() {
            let tail = chars.collect();
            for child in &mut self.children {
                if first == child.value {
                    child.set(tail, name);
                    return;
                }
            }
            let mut temp = Self::from_value(first);
            temp.set(tail, name);
            self.children.push(temp);
        } else {
            self.name = Some(name);
        }
    }
    pub fn symbol_from_chars<I>(&self, chars: &mut Peekable<I>) -> Result<Symbol, &'static str>
    where I: Iterator<Item = char>
    {
        if let Some(&c) = chars.peek() {
            if let Some(child) = self.get(c) {
                chars.next();
                return child.symbol_from_chars(chars);
            }
        }
        self.name.ok_or("Invalid symbol")
    }
    pub fn get(&self, c: char) -> Option<&Self> {
        for child in &self.children {
            if c == child.value {
                return Some(child);
            }
        }
        return None;
    }
}


#[derive(Debug,Copy,Clone)]
enum Keyword {
    Namespace,
    This,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "namespace" => Some(Self::Namespace),
            "this" => Some(Self::This),
            _ => None
        }
    }
}


#[derive(Debug,Copy,Clone)]
enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
    Dot,
    Pipe,
    Equal,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    Arrow,
    Comma,
    Colon,
    Elipsis,
    Semicolon,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
} impl Symbol {
    pub fn from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char> 
    {
        SymbolTree::from_vec(' ', 
            vec![
                ("+", Symbol::Add),
                ("-", Symbol::Sub),
                ("*", Symbol::Mul),
                ("/", Symbol::Div),
                (".", Symbol::Dot),
                ("|", Symbol::Pipe),
                ("=", Symbol::Equal),
                ("+=", Symbol::AddEq),
                ("-=", Symbol::SubEq),
                ("*=", Symbol::MulEq),
                ("/=", Symbol::DivEq),
                ("->", Symbol::Arrow),
                (",", Symbol::Comma),
                (":", Symbol::Colon),
                ("...", Symbol::Elipsis),
                (";", Symbol::Semicolon),
                ("(", Symbol::LeftParen),
                (")", Symbol::RightParen),
                ("[", Symbol::LeftSquare),
                ("]", Symbol::RightSquare),
                ("{", Symbol::LeftCurly),
                ("}", Symbol::RightCurly),
            ]
        ).symbol_from_chars(chars)
    }
}


#[derive(Debug)]
enum Token {
    Whitespace,
    Newline,
    Symbol(Symbol),
    Identifier(String),
    Number(String),
    Keyword(Keyword),
} impl Token {
    /*
    pub fn vec_from_file(path: &str, print_preprocess: bool) -> Result<Vec<Self>, &'static str> {
        let contents = read_to_string(path)
            .expect("Should have been able to read the file");
        let preprocess_string = contents;
        /*
        let preprocess_string = preprocess(contents)?;
        if print_preprocess {
            eprintln!("{preprocess_string:?}");
        }
        */
        Ok(Self::vec_from_string(preprocess_string))
    }
    pub fn from_ref(token: &Self) -> Self {
        match token {
            Self::Symbol(c) => Self::Symbol(c.to_string()),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
            t => t,
        }
    }
    */
    pub fn vec_from_chars<I>(chars: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = char>
    {
        let mut result = vec![];
        while let Some(&c) = chars.peek() {
            //println!("{c}   {result:?}");
            if c == '\n' {
                result.push(Self::newline_from_chars(chars)?);
            } else if c.is_whitespace() {
                result.push(Self::whitespace_from_chars(chars)?);
            } else if c.is_alphabetic() || c == '_' {
                result.push(Self::identifier_or_keyword_from_chars(chars)?);
            } else if c.is_numeric() {
                result.push(Self::number_from_chars(chars)?);
            } else {
                result.push(Self::symbol_from_chars(chars)?);
            }
        }
        Ok(result)
    }
    pub fn symbol_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        Ok(Token::Symbol(Symbol::from_chars(chars)?))
    }
    pub fn newline_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        while let Some(&c) = chars.peek() {
            if c == '\n' {
                chars.next();
            } else {
                break;
            }
        }
        Ok(Self::Newline)
    }
    pub fn whitespace_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        while let Some(&c) = chars.peek() {
            if c != '\n' && c.is_whitespace() {
                chars.next();
            } else {
                break; 
            }
        }
        Ok(Self::Whitespace)
    }
    pub fn identifier_or_keyword_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        let mut value = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                chars.next();
            } else {
                break;
            }
        }
        Ok(if let Some(keyword) = Keyword::from_string(value.to_string()) {
            Self::Keyword(keyword)
        } else {
            Self::Identifier(value)
        })
    }
    pub fn number_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        let mut value = String::new();
        let mut is_decimal = false;
        while let Some(&c) = chars.peek() {
            if c.is_numeric() {
                value.push(c);
                chars.next();
            } else if c == '.' {
                if is_decimal {
                    return Err("Unexpected '.'");
                }
                value.push(c);
                chars.next();
                is_decimal = true;
            } else {
                break;
            }
        }
        Ok(Self::Number(value))
    }
}


/*
#[derive(Debug)]
enum ExpressionItem {
    Variable(bool, Vec<String>),
    Other(Token),
} impl ExpressionItem {
    pub fn vec_from_tokens<I>(tokens: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = Token>
    {
        let mut result = vec![];
        while let Some(token) = tokens.next() {
            match token {
                Token::Symbol(";".to_string()) => return Ok(result),
                Token::Keyword("this".to_string()) => {
                    if let Token::Symbol(".".to_string())
                }
            }
        }
        Err("Expression needs to end with ';'");
    }
    pub fn variable_from_tokens<I>(tokens: &mut Peekable<I>, relative: bool) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = Token>
    {
    }
}


#[derive(Debug)]
enum AbstractSyntaxItem {
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
    pub fn vec_from_tokens<I>(tokens: &mut Peekable<I>, is_namespace: bool) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = Token>
    {
        let mut result = vec![];
        while let Some(&token) = tokens.peek() {
            match token {
                Token::Namespace => {
                    tokens.next();
                    result.push(Self::namespace_from_tokens(tokens)?);
                },
                Token::Symbol("}".to_string()) => {
                    tokens.next();
                    if is_namespace {
                        break;
                    }
                    Err("'}' without an opening '{'")
                },
                _ => {
                    result.push(Self::expression_from_tokens(tokens)?);
                },
            }
        }
        Ok(result)
    }
    /*
    pub fn namespace_from_tokens<I>(tokens: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = Token>
    {
        if let Some(Token::Idetifier(name)) = tokens.next() {
            if let Some(Token::Symbol("{".to_string())) = tokens.next() {
                Ok(Self::Namespace(name, Self::vec_from_tokens(tokens, true)?))
            }
            Err("'{' is required after namespace declaration")
        }
        Err("Namespace name should be an identifier")
    }
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
*/


/*
struct GraphingCalculator {
    expressions: Vec<AbstractSyntaxItem>, 
    api_key: String,
} impl GraphingCalculator {
    pub fn new(expressions: Vec<AbstractSyntaxItem>) -> Self {
        Self {
            expressions,
            api_key: "dcb31709b452b1cf9dc26972add0fda6".to_string(),
        }
    }
    pub fn from_file(path: &str, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<Self, &'static str> {
        let list = AbstractSyntaxItem::vec_from_file(path, print_tokens, print_preprocess)?;
        if print_ast {
            eprintln!("{list:?}");
        }
        Ok(Self::new(list))
    }
    pub fn print_html(&self, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<(), &'static str> {
        let api_link = self.get_api_link();
        let expressions = AbstractSyntaxItem::get_expressions(&self.expressions, String::new(), print_tokens, print_ast, print_preprocess)?; 
        println!(r"<!DOCTYPE html>
<html style='height: 100%;'>
<body style='height: 100%; margin: 0%'>
<script src='{api_link}'></script>
<div id='calculator' style='width: 100%; height: 100%;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);");
        for (index, (item, color)) in expressions.iter().enumerate() {
            let options = if let Some(c) = color {
                format!("color: '#{c}'")
            } else {
                "hidden: true".to_string()
            };
            println!("    calculator.setExpression({{id: '{index}', latex: '{item}', {options}}});");
        }
        println!("</script>");
        println!("</body>");
        println!("</html>");
        Ok(())
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key)
    }
}
*/


fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let print_tokens = args.contains(&"--tokens".to_string());
    let print_ast = args.contains(&"--ast".to_string());
    let print_preprocess = args.contains(&"--preprocess".to_string());
    if let Some(file_path) = &args.get(1) {
        match GraphingCalculator::from_file(file_path, print_tokens, print_ast, print_preprocess) {
            Ok(gc) => match gc.print_html(print_tokens, print_ast, print_preprocess) {
                 Ok(()) => (),
                 Err(e) => eprintln!("\x1b[31m{e}\x1b[0m"),
            },
            Err(e) => eprintln!("\x1b[31m{e}\x1b[0m"),
        }
    } else {
        eprintln!("\x1b[33mNo file specified\x1b[0m");
    }
    */
    let chars = "namespace lol { x = y + 1; }";
    let tokens = Token::vec_from_chars(&mut chars.chars().peekable());
    println!("{tokens:?}");
}
