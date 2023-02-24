// use std::env;
// use std::fs::{/*File, */read_to_string};
// use std::io::{prelude::*, BufReader};
use std::iter::Peekable;


#[derive(Debug,Clone,Copy)]
enum Keyword {
    Namespace,
    Use,
    End,
    Visual,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "Namespace" => Some(Self::Namespace),
            "Use" => Some(Self::Use),
            "End" => Some(Self::End),
            "Visual" => Some(Self::Visual),
            _ => None,
        }
    }
}


#[derive(Debug)]
enum Token {
    Whitespace,
    Newline,
    Symbol(char),
    Identifier(Vec<String>),
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
            Self::Symbol(c) => Self::Symbol(*c),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
            Self::STDConstant(p) => Self::STDConstant(p.to_string()),
            Self::STDFunction(p) => Self::STDFunction(p.to_string()),
        }
    }
    */
    pub fn vec_from_chars<I>(chars: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = char>
    {
        let mut result = vec![];
        while let Some(&c) = chars.peek() {
            if c == '\n' {
                result.push(Self::Newline);
                chars.next();
            } else if c.is_whitespace() {
                result.push(Self::whitespace_from_chars(chars)?);
            } else if c.is_alphabetic() || c == '_' {
                result.push(Self::identifier_from_chars(chars)?);
            } else if c.is_numeric() {
                result.push(Self::number_from_chars(chars)?);
            } else {
                result.push(Self::Symbol(c));
                chars.next();
            }
        }
        return Ok(result);
    }
    pub fn whitespace_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        while let Some(&c) = chars.peek() {
            if c != '\n' && c.is_whitespace() {
                chars.next();
            } else {
                return Ok(Self::Whitespace);
            }
        }
        return Ok(Self::Whitespace);
    }
    pub fn identifier_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        let mut value = vec![];
        let mut last = String::new();
        while let Some(&c) = chars.peek() {
            if last.is_empty() {
                if c.is_alphabetic() || c == '_' {
                    last.push(c);
                    chars.next();
                } else {
                    return Err("Identifier starts with a letter or '_'");
                }
            } else if c.is_alphanumeric() || c == '_' {
                last.push(c);
                chars.next();
            } else {
                value.push(last);
                last = String::new();
                if c == '.' {
                    chars.next();
                } else {
                    return Ok(Self::Identifier(value));
                }
            }
        }
        value.push(last);
        return Ok(Self::Identifier(value));
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
                return Ok(Self::Number(value));
            }
        }
        return Ok(Self::Number(value));
    }
    /*
    pub fn vec_from_string(string: String) -> Vec<Self> {
        let mut result = vec![];
        let mut last: Option<Self> = None;
        let mut is_after_space = true;
        for c in string.chars() {
            if c.is_whitespace() {
                is_after_space = true;
                continue;
            } else if let Some(ref mut l) = last {
                if is_after_space || !l.try_push(c) {
                    if let Token::Identifier(ref s) = l {
                        let keyword = Keyword::from_string(s.to_string());
                        if let Some(k) = keyword {
                            *l = Self::Keyword(k);
                        }
                    }
                    result.push(Self::from_ref(l));
                    *l = Self::from_char(c);
                }
            } else {
                last = Some(Self::from_char(c));
            }
            is_after_space = false;
        }
        if let Some(mut l) = last {
            if let Token::Identifier(ref s) = l {
                let keyword = Keyword::from_string(s.to_string());
                if let Some(k) = keyword {
                    l = Self::Keyword(k);
                }
            }
            result.push(l)
        }
        result
    }
    pub fn to_latex(&self) -> String {
        match self {
            Self::Symbol(c) => match c {
                '{' => String::from("\\\\left\\\\{"),
                '}' => String::from("\\\\right\\\\}"),
                'X' => ".x".to_string(),
                'Y' => ".y".to_string(),
                '~' => "...".to_string(),
                _ => c.to_string(),
            },
            Self::Identifier(i) => format!("A_{{{}}}", i),
            Self::Number(n) => n.to_string(),
            Self::STDConstant(c) => format!("\\\\{c} "),
            Self::STDFunction(f) => format!("\\\\operatorname{{{}}}", f),
            _ => String::new(),
        }
    }
    fn from_char(c: char) -> Self {
        if c.is_alphabetic() {
            Token::Identifier(c.to_string().to_uppercase())
        } else if c.is_numeric() {
            Token::Number(c.to_string())
        } else if c == '$' {
            Token::STDConstant(String::new())
        } else if c == '\\' {
            Token::STDFunction(String::new())
        } else {
            Token::Symbol(c)
        }
    }
    fn try_push(&mut self, c: char) -> bool {
        let is_matching = match self {
            Token::Identifier(_) => c.is_alphanumeric(),
            Token::Number(_) => c.is_numeric() || c == '.',
            Token::STDConstant(_) | Token::STDFunction(_) | Token::Symbol('@') => c.is_alphabetic(),
            Token::Symbol('~') => c == 'Y' || c == 'X',
            Token::Keyword(_) | Token::Symbol(_) => false,
        };
        if is_matching {
            match self {
                Token::Identifier(s) | Token::Number(s) | Token::STDConstant(s) | 
                Token::STDFunction(s) => {
                    for i in c.to_lowercase() {
                        s.push(i);
                    }
                },
                Token::Symbol(s) => {
                    if *s == '~' {
                        *s = c; 
                    } else {
                        for i in c.to_lowercase() {
                            *s = i;
                        }
                    }
                },
                Token::Keyword(_) => (),
            }
            return true;
        }
        false
    }
    */
}


/*
#[derive(Debug)]
enum AbstractSyntaxItem {
    Visual(String),
    ExpressionStart,
    ExpressionEnd,
    NamespaceStart(String),
    NamespaceEnd,
    Use(Vec<String>),
    Token(Token),
    Variable(Vec<String>),
} impl AbstractSyntaxItem {
    pub fn vec_from_file(path: &str, print_tokens: bool, print_preprocess: bool) -> Result<Vec<Self>, &'static str> {
        let tokens = Token::vec_from_file(path, print_preprocess)?;
        if print_tokens {
            eprintln!("{tokens:?}");
        }
        return Self::vec_from_tokens(tokens);
    }
    pub fn vec_from_tokens(tokens: Vec<Token>) -> Result<Vec<Self>, &'static str> {
        let mut result = vec![];
        let mut variable = vec![];
        let mut is_variable_continue = false;
        let mut is_namespace_start = false;
        let mut is_expression_end = true;
        let mut is_use_start = false;
        let mut is_visual_start = false;
        let mut namespace_level = 0;
        for token in tokens.iter() {
            if is_use_start {
                match token {
                    Token::Identifier(path) => {
                        variable.push(path.to_string());
                    },
                    Token::Symbol(';') => {
                        result.push(Self::Use(variable));
                        variable = vec![];
                        is_use_start = false;
                    },
                    _ => return Err("There can only be identifiers after use"),
                }
                continue;
            } else if is_namespace_start {
                if let Token::Identifier(name) = token {
                    result.push(Self::NamespaceStart(name.to_string()));
                    is_namespace_start = false;
                    continue;
                }
                return Err("There can only be an identifier after namespace");
            } else if is_visual_start {
                /*
                if let Token::Color(n) = token {
                    result.push(Self::Visual(n.to_string()));
                    is_visual_start = false;
                    continue;
                }
                */
                return Err("There can only be a color after visual");
            }
            match token {
                Token::Keyword(Keyword::Visual) => {
                    if is_expression_end {
                        is_visual_start = true;
                        continue;
                    }
                    return Err("Visual can't be in the middle of an expression");
                },
                Token::Keyword(Keyword::Use) => {
                    if is_expression_end {
                        is_use_start = true;
                        continue;
                    }
                    return Err("Use can't start in the middle of an expression");
                },
                Token::Keyword(Keyword::Namespace) => {
                    namespace_level += 1;
                    if is_expression_end {
                        is_namespace_start = true;
                        continue;
                    }
                    return Err("Namespace can't start in the middle of an expression");
                },
                Token::Keyword(Keyword::End) => { 
                    if namespace_level == 0 {
                        return Err("Can't have more ends than namespaces");
                    }
                    namespace_level -= 1;
                    if is_expression_end {
                        result.push(Self::NamespaceEnd);
                        continue;
                    }
                    return Err("Namespace can't end in the middle of an expression");
                },
                Token::Identifier(i) => {
                    if is_expression_end {
                        result.push(Self::ExpressionStart);
                        is_expression_end = false;
                    }
                    if !variable.is_empty() && !is_variable_continue {
                        if is_expression_end {
                            result.push(Self::ExpressionStart);
                            is_expression_end = false;
                        }
                        result.push(Self::Variable(variable));
                        variable = vec![];
                    }
                    variable.push(i.to_string());
                },
                Token::Symbol(';') => {
                    if is_expression_end {
                        return Err("A semicolon can't have another semicolon after it");
                    }
                    if !variable.is_empty() {
                        result.push(Self::Variable(variable));
                        variable = vec![];
                    }
                    result.push(Self::ExpressionEnd);
                    is_expression_end = true;
                },
                Token::Symbol('.') => {
                    if is_expression_end {
                        result.push(Self::ExpressionStart);
                        is_expression_end = false;
                    }
                    if variable.is_empty() {
                        variable.push(String::new());
                    }
                    is_variable_continue = true;
                    continue;
                },
                t => {
                    if is_expression_end {
                        result.push(Self::ExpressionStart);
                        is_expression_end = false;
                    }
                    if !variable.is_empty() {
                        result.push(Self::Variable(variable));
                        variable = vec![];
                    }
                    result.push(Self::Token(Token::from_ref(t)));
                },
            }
            is_variable_continue = false;
        }
        if namespace_level != 0 {
            return Err("Can't have more namespaces than ends");
        }
        Ok(result)
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
}


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
    let chars = "xor.a37373737 + yor = 3.734757\nlol lol lol";
    let tokens = Token::vec_from_chars(&mut chars.chars().peekable());
    println!("{tokens:?}");
}
