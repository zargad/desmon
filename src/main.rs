use std::fs::{File, read_to_string};
use std::io::{prelude::*, BufReader};


#[derive(Debug)]
enum Keyword {
    Namespace,
    Use,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "namespace" => Some(Self::Namespace),
            "use" => Some(Self::Use),
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
}
impl Token {
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
        result.push(last);
        return result;
    }
    fn is_none(&self) -> bool {
        if let Self::None = self {
            return true;
        }
        return false;
    }
    fn from_char(c: char) -> Self {
        if c.is_alphabetic() {
            Token::Identifier(c.to_string())
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
                Token::Identifier(s) | Token::Number(s) => s.push(c),
                Token::Symbol(_) | Token::Keyword(_) | Token::None => (),
            }
            return true;
        }
        return false;
    }
}



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
        if let Token::Identifier(namespace_name) = self.tokens[i] {
            let n = namespace_name.to_string();
            i += 1;
            if let Token::Symbol('{') = self.tokens[i] {
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
            if index >= self.tokens.len() {
                break;
            } else {
                match self.tokens[i] {
                    Token::Keyword(Keyword::Namespace) => {
                        match self.get_namespace(i+1) {
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
                    t => tree_tokens.push(AbstractSyntaxTree::Token(t)),
                }
            }
            i += 1;
        }
        return Ok((AbstractSyntaxTree::ABT(name, tree_tokens), i));
    }
}


struct GraphingCalculator {
    expressions: Vec<String>, 
    api_key: String,
} impl GraphingCalculator {
    pub fn new(expressions: Vec<String>) -> Self {
        Self {
            expressions: expressions,
            api_key: "dcb31709b452b1cf9dc26972add0fda6".to_string(),
        }
    }
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).expect("Well that sucked");
        let reader = BufReader::new(file);
        let mut expressions: Vec<String> = vec![];
        for line in reader.lines() {
            expressions.push(line.unwrap());
        }
        Self::new(expressions)
    }
    pub fn print_html(&self) {
        println!(r"<script src='{}'></script>
<div id='calculator' style='width: 600px; height: 400px;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);", self.get_api_link());
        for (index, item) in self.expressions.iter().enumerate() {
            println!("    calculator.setExpression({{id: 'graph{}', latex: '{}'}});", index, item);
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
    let factory = ASTFactory {tokens: tokens};
    let tree = factory.get_tree("".to_string(), 0);
    println!("{:?}", tree);
}
