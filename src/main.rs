use std::env;
use std::fs::{/*File, */read_to_string};
// use std::io::{prelude::*, BufReader};


pub fn preprocess(string: String) -> String {
    let mut result = String::new();
    let mut last = ' ';
    let mut is_in_comment = false;
    let mut inline_comment_level = 0;
    for c in string.chars() {
        match c {
            '/' => if !is_in_comment && inline_comment_level == 0 && last == '/' {
                result.pop();
                is_in_comment = true;
            } else if inline_comment_level != 0 && last == '*' {
                inline_comment_level -= 1;
                continue;
            },
            '*' => if !is_in_comment &&  last == '/' {
                if inline_comment_level == 0 {
                    result.pop();
                }
                inline_comment_level += 1;
            },
            '\n' => {
                is_in_comment = false;
            },
            _ => (),
        }
        if !is_in_comment && inline_comment_level == 0 {
            result.push(c);
        }
        last = c;
    }
    result
}


#[derive(Debug,Clone,Copy)]
enum Keyword {
    Namespace,
    Use,
    End,
    Visual,
    Hidden,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "Namespace" => Some(Self::Namespace),
            "Use" => Some(Self::Use),
            "End" => Some(Self::End),
            "Visual" => Some(Self::Visual),
            "Hidden" => Some(Self::Hidden),
            _ => None,
        }
    }
}


#[derive(Debug)]
enum Token {
    Symbol(char),
    Identifier(String),
    Number(String),
    Keyword(Keyword),
    STDFunction(String),
    STDConstant(String),
} impl Token {
    pub fn vec_from_file(path: &str) -> Vec<Self> {
        let contents = read_to_string(path)
            .expect("Should have been able to read the file");
        Self::vec_from_string(preprocess(contents))
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
            Token::STDConstant(_) | Token::STDFunction(_) => c.is_alphabetic(),
            Token::Symbol(s) => *s == '~' && (c == 'Y' || c == 'X'),
            Token::Keyword(_) => false,
        };
        if is_matching {
            match self {
                Token::Identifier(s) | Token::Number(s) => {
                    for i in c.to_lowercase() {
                        s.push(i);
                    }
                },
                Token::STDConstant(n) | Token::STDFunction(n) => n.push(c),
                Token::Symbol(s) => *s = c,
                Token::Keyword(_) => (),
            }
            return true;
        }
        false
    }
}


#[derive(Debug)]
enum AbstractSyntaxItem {
    Visual,
    Hidden,
    ExpressionStart,
    ExpressionEnd,
    NamespaceStart(String),
    NamespaceEnd,
    Use(Vec<String>),
    Token(Token),
    Variable(Vec<String>),
} impl AbstractSyntaxItem {
    pub fn vec_from_file(path: &str, is_print_tokens: bool) -> Result<Vec<Self>, &'static str> {
        let tokens = Token::vec_from_file(path);
        if is_print_tokens {
            println!("{tokens:?}");
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
            }
            match token {
                Token::Keyword(Keyword::Visual) => {
                    if is_expression_end {
                        result.push(Self::Visual);
                        continue;
                    }
                    return Err("Visual can't be in the middle of an expression");
                }
                Token::Keyword(Keyword::Hidden) => {
                    if is_expression_end {
                        result.push(Self::Hidden);
                        continue;
                    }
                    return Err("Hidden can't be in the middle of an expression");
                }
                Token::Keyword(Keyword::Use) => {
                    if is_expression_end {
                        is_use_start = true;
                        continue;
                    }
                    return Err("Use can't start in the middle of an expression");
                }
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
    pub fn get_expressions(list: &Vec<Self>, ext_namespace: String, is_print_tokens: bool) -> Result<Vec<(String, bool)>, &'static str> {
        let mut result = vec![]; 
        let mut namespaces = vec![];
        let mut expression = vec![];
        let mut is_hidden = true;
        for i in list {
            match i {
                Self::Visual => is_hidden = false,
                Self::Hidden => is_hidden = true,
                Self::ExpressionStart => expression = vec![],
                Self::ExpressionEnd => result.push((expression.join(""), is_hidden)),
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
                    match &Self::vec_from_file(full_path.as_str(), is_print_tokens) {
                        Ok(list) => match Self::get_expressions(list, format!("{ext_namespace}{}", namespaces.join("")), is_print_tokens) {
                            Ok(strings) => for i in strings {
                                result.push(i);
                            },
                            Err(e) => return Err(e),
                        },
                        Err(e) => return Err(e),
                    }
                },
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
    pub fn from_file(path: &str, is_print_tokens: bool, is_print_ast: bool) -> Result<Self, &'static str> {
        match AbstractSyntaxItem::vec_from_file(path, is_print_tokens) {
            Ok(list) => {
                if is_print_ast {
                    println!("{list:?}");
                }
                Ok(Self::new(list))
            },
            Err(e) => Err(e),
        }
    }
    pub fn print_html(&self, is_print_tokens: bool) -> Result<(), &'static str> {
        let api_link = self.get_api_link();
        println!(r"<script src='{api_link}'></script>
<div id='calculator' style='width: 600px; height: 400px;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);");
        match AbstractSyntaxItem::get_expressions(&self.expressions, String::new(), is_print_tokens) {
            Ok(expressions) => for (index, (item, is_hidden)) in expressions.iter().enumerate() {
                println!("    calculator.setExpression({{id: '{index}', latex: '{item}', hidden: '{is_hidden}'}});");
            },
            Err(e) => return Err(e),
        }
        println!("</script>");
        Ok(())
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_print_tokens = false;
    let mut is_print_ast = false;
    for arg in &args {
        is_print_tokens = is_print_tokens || arg == "--tokens";
        is_print_ast = is_print_ast || arg == "--ast";
    }
    if let Some(file_path) = &args.get(1) {
        match GraphingCalculator::from_file(file_path, is_print_tokens, is_print_ast) {
            Ok(gc) => match gc.print_html(is_print_tokens) {
                 Ok(()) => (),
                 Err(e) => println!("\x1b[31m{e}\x1b[0m"),
            },
            Err(e) => println!("\x1b[31m{e}\x1b[0m"),
        }
    } else {
        println!("\x1b[33mNo file specified\x1b[0m");
    }
}
