use std::iter::Peekable;


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
pub enum Symbol {
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


#[derive(Debug,Copy,Clone)]
pub enum Keyword {
    Namespace,
    This,
    Std,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "namespace" => Some(Self::Namespace),
            "this" => Some(Self::This),
            "std" => Some(Self::Std),
            _ => None
        }
    }
}


#[derive(Debug)]
pub enum Token {
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
    */
    pub fn from_ref(token: &Self) -> Self {
        match token {
            Self::Symbol(c) => Self::Symbol(*c),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
            Self::Whitespace => Self::Whitespace,
            Self::Newline => Self::Newline,
        }
    }
    pub fn vec_from_chars<I>(chars: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = char>
    {
        let mut result = vec![];
        while let Some(&c) = chars.peek() {
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
