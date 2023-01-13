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
    None,
    Symbol(char),
    Identifier(String),
    Number(String),
    Keyword(Keyword),
} impl Token {
    pub fn from_ref(token: &Self) -> Self {
        match token {
            Self::None => Self::None,
            Self::Symbol(c) => Self::Symbol(*c),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
        }
    }
    pub fn vec_from_string(string: String) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let mut last: Self = Token::None;
        let mut is_after_space = true;
        for c in string.chars() {
            if c.is_whitespace() {
                is_after_space = true;
                continue;
            } else if is_after_space || !last.try_push(c) {
                if !last.is_none() {
                    if let Token::Identifier(ref s) = last {
                        let keyword = Keyword::from_string(s.to_string());
                        if let Some(k) = keyword {
                            last = Self::Keyword(k);
                        }
                    }
                    result.push(last);
                }
                last = Self::from_char(c);
            }
            is_after_space = false;
        }
        if let Token::Identifier(ref s) = last {
            let keyword = Keyword::from_string(s.to_string());
            if let Some(k) = keyword {
                last = Self::Keyword(k);
            }
        }
        result.push(last);
        return result;
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Symbol(c) => match c {
                '{' => String::from("\\\\left\\\\{"),
                '}' => String::from("\\\\right\\\\}"),
                cc => c.to_string(),
            },
            Self::Identifier(i) => format!("A_{{{}}}", i),
            Self::Number(n) => n.to_string(),
            _ => String::new(),
        }
    }
    pub fn set_namespace(&self, namespace: &str) -> Result<Self, String> {
        if let Self::Identifier(n) = self {
            return Ok(Self::Identifier(format!("{namespace}{n}")));
        }
        return Err("Not Identifier".to_string());
    }
    fn is_none(&self) -> bool {
        if let Self::None = self {
            return true;
        }
        return false;
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
            Token::Symbol(_) | Token::Keyword(_) | Token::None => false,
        };
        if is_matching {
            match self {
                Token::Identifier(s) | Token::Number(s) => {
                    for i in c.to_lowercase() {
                        s.push(i); 
                    }
                },
                Token::Symbol(_) | Token::Keyword(_) | Token::None => (),
            }
            return true;
        }
        return false;
    }
}


#[derive(Debug)]
enum AbstractSyntaxItem {
    NamespaceStart(String),
    Expression(Vec<Token>),
    NamespaceEnd,
} impl AbstractSyntaxItem {
    pub fn vec_from_tokens(tokens: Vec<Token>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let mut expression: Vec<Token> = vec![];
        let mut is_namespace_start = false;
        for token in tokens.iter() {
            match token {
                Token::Keyword(Keyword::Namespace) => {
                    is_namespace_start = true;
                },
                Token::Keyword(Keyword::End) => result.push(Self::NamespaceEnd),
                Token::Symbol(';') => {
                    result.push(Self::Expression(expression));
                    expression = vec![];
                },
                t => {
                    if is_namespace_start {
                        if let Token::Identifier(i) = t {
                            result.push(Self::NamespaceStart(i.to_string()));
                        }
                        is_namespace_start = false;
                        continue;
                    }
                    expression.push(match t {
                        Token::Identifier(n) => Token::Identifier(String::from(n)),
                        Token::Number(n) => Token::Number(String::from(n)),
                        Token::Symbol(n) => Token::Symbol(*n),
                        _ => Token::None,
                    });
                }
            }
        }
        if expression.len() != 0 {
            result.push(Self::Expression(expression));
        }
        return result;
    }
    pub fn unwrap_namespaces(list: Vec<Self>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let mut namespaces: Vec<&str> = vec![];
        for i in list.iter() {
            match i {
                Self::NamespaceStart(s) => namespaces.push(s.as_str()),
                Self::NamespaceEnd => _ = namespaces.pop().unwrap(),
                Self::Expression(ex) => {
                    let binding = namespaces.join("");
                    let namespace = binding.as_str();
                    result.push(Self::set_namespace(ex, namespace));
                },
            }
        }
        return result;
    }
    pub fn to_string(&self) -> Result<String, String> {
        let mut result: Vec<String> = vec![];
        if let Self::Expression(ex) = self {
            for token in ex.iter() {
                result.push(token.to_string());
            }
            return Ok(result.join(""));
        }
        return Err("Not an Expression".to_string());
    }
    fn set_namespace(expressions: &Vec<Token>, namespace: &str) -> Self {
        let mut result: Vec<Token> = vec![];
        for token in expressions.iter() {
            if let Ok(t) = token.set_namespace(namespace) {
                result.push(t);
            } else {
                result.push(Token::from_ref(token));
            }
        }
        return Self::Expression(result);
    }
}


/*
#[derive(Debug)]
enum AbstractSyntaxTree {
    Token(Token),
    ABT(String, Vec<Self>),
} /* impl AbstractSyntaxTree {
    fn get_namespace(tokens: &Vec<Token>, index: usize) -> Result<(Self, usize), String> {
        let mut i = index;
        if let Token::Identifier(namespace_name) = &tokens[i] {
            let n = namespace_name.to_string();
            i += 1;
            if let Token::Symbol('{') = &tokens[i] {
                return Self::get_tree(tokens, n, i + 1); 
            }
        }
        return Err("Improper namespace declaration syntax.".to_string());
    }
    pub fn get_tree(tokens: &Vec<Token>, name: String, index: usize) -> Result<(Self, usize), String> {
        let mut tree_tokens: Vec<Self> = vec![];
        let mut curly_layer = 0;
        let mut i = index;
        loop {
            if index >= tokens.len() {
                break;
            } else {
                match &tokens[i] {
                    Token::Keyword(Keyword::Namespace) => {
                        match Self::get_namespace(tokens, i+1) {
                            Ok((abt, temp_i)) => {
                                tree_tokens.push(abt);
                                i = temp_i;
                                continue;
                            },
                            e => return e,
                        }
                    },
                    Token::Symbol('{') => curly_layer += 1,
                    Token::Symbol('}') => {
                        if curly_layer == 0 {
                            break;
                        } else {
                            curly_layer += 1;
                        }
                    },
                    t => tree_tokens.push(Self::Token(t)),
                }
            }
            i += 1;
        }
        return Ok((Self::ABT(name, tree_tokens), i));
    }
} */


struct ASTFactory {
    tokens: Vec<Token>,
} impl ASTFactory {
    fn get_namespace(&self, index: usize) -> Result<(AbstractSyntaxTree, usize), String> {
        let mut i = index;
        if let Token::Identifier(namespace_name) = &self.tokens[i] {
            let n = namespace_name.to_string();
            i += 1;
            if let Token::Symbol('{') = &self.tokens[i] {
                return self.get_tree(n, i+1); 
            }
        }
        return Err("Improper namespace declaration syntax.".to_string());
    }
    pub fn get_tree(&self, name: String, index: usize) -> Result<(AbstractSyntaxTree, usize), String> {
        let mut tree_tokens: Vec<AbstractSyntaxTree> = vec![];
        let mut curly_layer = 0;
        let mut i = index;
        loop {
            if let Some(token) = self.tokens.get(i) {
                match token {
                    Token::Keyword(Keyword::Namespace) => match self.get_namespace(i+1) {
                        Ok((abt, temp_i)) => {
                            tree_tokens.push(abt);
                            i = temp_i;
                            continue;
                        },
                        e => return e,
                    },
                    Token::Symbol('{') => curly_layer += 1,
                    Token::Symbol('}') => {
                        if curly_layer == 0 {
                            break;
                        } else {
                            curly_layer += 1;
                        }
                    },
                    t => tree_tokens.push(AbstractSyntaxTree::Token(t)),
                }
            } else {
                break;
            }
            i += 1;
        }
        return Ok((AbstractSyntaxTree::ABT(name, tree_tokens), i));
    }
}
*/


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
        let file = File::open(path).expect("Well that sucked");
        let reader = BufReader::new(file);
        let mut expressions: Vec<String> = vec![];
        for line in reader.lines() {
            expressions.push(line.unwrap());
        }
        Self::new(expressions)
    }
    */
    pub fn print_html(&self) {
        println!(r"<script src='{}'></script>
<div id='calculator' style='width: 600px; height: 400px;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);", self.get_api_link());
        for (index, item) in self.expressions.iter().enumerate() {
            let temp = item.to_string().unwrap();
            println!("    calculator.setExpression({{id: 'graph{}', latex: '{}'}});", index, temp);
        }
        println!("</script>");
    }
    fn get_api_link(&self) -> String {
        format!("https://www.desmos.com/api/v1.7/calculator.js?apiKey={}", self.api_key).to_string()
    }
}


fn main() {

    let contents = read_to_string("test_tokenizer.ds")
        .expect("Should have been able to read the file");
    let tokens = Token::vec_from_string(contents);
    let list = AbstractSyntaxItem::vec_from_tokens(tokens);
    // println!("{:?}", list);
    let temp = AbstractSyntaxItem::unwrap_namespaces(list);
    /*
    for i in temp {
        let ttt = i.to_string().unwrap();
        println!("{ttt}");
    }
    */
    GraphingCalculator::new(temp).print_html();
}
