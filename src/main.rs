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
} impl Token {
    pub fn from_ref(token: &Self) -> Self {
        match token {
            Self::Symbol(c) => Self::Symbol(*c),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
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
            _ => String::new(),
        }
    }
    fn from_char(c: char) -> Self {
        if c.is_alphabetic() {
            Token::Identifier(c.to_string().to_uppercase())
        } else if c.is_numeric() {
            Token::Number(c.to_string())
        } else {
            Token::Symbol(c)
        }
    }
    fn try_push(&mut self, c: char) -> bool {
        let is_matching = match self {
            Token::Identifier(_) => c.is_alphanumeric(),
            Token::Number(_) => c.is_numeric(),
            Token::Symbol(_) | Token::Keyword(_) => false,
        };
        if is_matching {
            match self {
                Token::Identifier(s) | Token::Number(s) => {
                    for i in c.to_lowercase() {
                        s.push(i);
                    }
                },
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
                    if !is_expression_end {
                        is_namespace_start = true;
                        continue;
                    }
                    return Err("Namespace can't start in the middle of an expression");
                },
                Token::Keyword(Keyword::End) => { 
                    if !is_expression_end {
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
    /*
    pub fn to_string(&self) -> Result<String, String> {
        let mut result = vec![]; 
        if let Self::Expression(ex) = self {
            for token in ex.iter() {
                result.push(token.to_string());
            }
            return Ok(result.join(""));
        }
        return Err("Not an Expression".to_string());
    }
    */
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
    /*
    pub fn from_file(path: &str) -> Self {
        let contents = read_to_string(path)
            .expect("Should have been able to read the file");
        let tokens = Token::vec_from_string(contents);
        let list = AbstractSyntaxItem::vec_from_tokens(tokens);
        let temp = AbstractSyntaxItem::unwrap_namespaces(list);
        return Self::new(temp);
    }
    pub fn print_html(&self) {
        println!(r"<script src='{}'></script>
<div id='calculator' style='width: 600px; height: 400px;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);", self.get_api_link());
        for (index, item) in self.expressions.iter().enumerate() {
            let temp = item.to_string().unwrap();
            println!("    calculator.setExpression({{id: '{index}', latex: '{temp}'}});");
        }
        println!("</script>");
    }
    */
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key).to_string()
    }
}


fn main() {
    let path = "test_tokenizer.ds";
    let contents = read_to_string(path)
        .expect("Should have been able to read the file");
    let tokens = Token::vec_from_string(contents);
    let list = AbstractSyntaxItem::vec_from_tokens(tokens);
    println!("{list:?}");
    // GraphingCalculator::from_file("test_tokenizer.ds").print_html();
}
