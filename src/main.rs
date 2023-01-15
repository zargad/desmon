use std::fs::{File, read_to_string};
use std::io::{prelude::*, BufReader};


#[derive(Debug,Clone,Copy)]
enum Keyword {
    Namespace,
    Use,
    End,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "Namespace" => Some(Self::Namespace),
            "Use" => Some(Self::Use),
            "End" => Some(Self::End),
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
        let mut is_in_comment = false;
        for c in string.chars() {
            if c == '\n' {
                is_in_comment = false;
            } else if c == '"' {
                is_in_comment = true;
            }
            if is_in_comment {
                continue;
            }
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
            result.push(l);
        }
        return result;
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Symbol(c) => match c {
                '{' => String::from("\\\\left\\\\{"),
                '}' => String::from("\\\\right\\\\}"),
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
            Token::Symbol(_) | Token::Keyword(_) => false,
        };
        if is_matching {
            match self {
                Token::Identifier(s) | Token::Number(s) => {
                    for i in c.to_lowercase() {
                        s.push(i);
                    }
                },
                Token::STDConstant(n) | Token::STDFunction(n) => n.push(c),
                Token::Symbol(_) | Token::Keyword(_) => (),
            }
            return true;
        }
        return false;
    }
}


#[derive(Debug)]
enum AbstractSyntaxItem {
    ExpressionStart,
    ExpressionEnd,
    NamespaceStart(String),
    NamespaceEnd,
    Token(Token),
    Variable(Vec<String>),
} impl AbstractSyntaxItem {
    pub fn vec_from_tokens(tokens: Vec<Token>) -> Result<Vec<Self>, &'static str> {
        let mut result = vec![];
        let mut variable = vec![];
        let mut is_variable_continue = false;
        let mut is_namespace_start = false;
        let mut is_expression_end = true;
        for token in tokens.iter() {
            if is_namespace_start {
                if let Token::Identifier(name) = token {
                    result.push(Self::NamespaceStart(name.to_string()));
                    is_namespace_start = false;
                    continue;
                }
                return Err("There can only be an identifier after namespace");
            }
            match token {
                Token::Keyword(Keyword::Namespace) => {
                    if is_expression_end {
                        is_namespace_start = true;
                        continue;
                    }
                    return Err("Namespace can't start in the middle of an expression");
                },
                Token::Keyword(Keyword::End) => { 
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
        }
        return Ok(result);
    }
    pub fn get_strings(list: &Vec<Self>) -> Vec<String> {
        let mut result = vec![]; 
        let mut namespaces = vec![];
        let mut expression = vec![];
        for i in list {
            match i {
                Self::ExpressionStart => expression = vec![],
                Self::ExpressionEnd => result.push(expression.join("")),
                Self::NamespaceStart(n) => namespaces.push(n.to_string()),
                Self::NamespaceEnd => _ = namespaces.pop(),
                Self::Token(t) => expression.push(t.to_string()),
                Self::Variable(i) => {
                    expression.push("A_{{".to_string());
                    if !i.get(0).unwrap().is_empty() {
                        expression.push(namespaces.join(""));
                    }
                    expression.push(i.join(""));
                    expression.push("}}".to_string());
                },
            }
        }
        return result;
    }
}


struct GraphingCalculator {
    expressions: Vec<AbstractSyntaxItem>, 
    api_key: String,
} impl GraphingCalculator {
    pub fn new(expressions: Vec<AbstractSyntaxItem>) -> Self {
        Self {
            expressions: expressions,
            api_key: "dcb31709b452b1cf9dc26972add0fda6".to_string(),
        }
    }
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        let contents = read_to_string(path)
            .expect("Should have been able to read the file");
        let tokens = Token::vec_from_string(contents);
        match AbstractSyntaxItem::vec_from_tokens(tokens) {
            Ok(list) => Ok(Self::new(list)),
            Err(e) => Err(e),
        }
    }
    pub fn print_html(&self) {
        println!(r"<script src='{}'></script>
<div id='calculator' style='width: 600px; height: 400px;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);", self.get_api_link());
        let temp = AbstractSyntaxItem::get_strings(&self.expressions);
        for (index, item) in temp.iter().enumerate() {
            println!("    calculator.setExpression({{id: '{index}', latex: '{item}'}});");
        }
        println!("</script>");
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key).to_string()
    }
}


fn main() {
    GraphingCalculator::from_file("test_tokenizer.ds").unwrap().print_html();
}
