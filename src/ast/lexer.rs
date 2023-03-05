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
        self.children.iter().find(|&child| c == child.value)
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
    Bang,
    X,
    Y,
    Theta,
    Radius,
    Equal,
    GE,
    LE,
    GT,
    LT,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    Arrow,
    Comma,
    Colon,
    Elipsis,
    Vertical,
    Horizontal,
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
                ("+", Self::Add),
                ("-", Self::Sub),
                ("*", Self::Mul),
                ("/", Self::Div),
                (".", Self::Dot),
                ("|", Self::Pipe),
                ("!", Self::Bang),
                ("@x", Self::X),
                ("@y", Self::Y),
                ("@t", Self::Theta),
                ("@r", Self::Radius),
                ("=", Self::Equal),
                ("<=", Self::LE),
                (">=", Self::GE),
                ("<", Self::LT),
                (">", Self::GT),
                ("+=", Self::AddEq),
                ("-=", Self::SubEq),
                ("*=", Self::MulEq),
                ("/=", Self::DivEq),
                ("->", Self::Arrow),
                (",", Self::Comma),
                (":", Self::Colon),
                ("...", Self::Elipsis),
                ("~y", Self::Vertical),
                ("~x", Self::Horizontal),
                (";", Self::Semicolon),
                ("(", Self::LeftParen),
                (")", Self::RightParen),
                ("[", Self::LeftSquare),
                ("]", Self::RightSquare),
                ("{", Self::LeftCurly),
                ("}", Self::RightCurly),
            ]
        ).symbol_from_chars(chars)
    }
    pub fn get_latex(&self) -> String {
        String::from(match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "\\cdot ",
            Self::Div => "/",
            Self::Dot => ".",
            Self::Pipe => "|",
            Self::Bang => "!",
            Self::X => "x",
            Self::Y => "y",
            Self::Theta => "\\theta",
            Self::Radius => "r",
            Self::Equal => "=",
            Self::LE => "\\le ",
            Self::GE => "\\ge ",
            Self::LT => "<",
            Self::GT => ">",
            Self::AddEq => "+=",
            Self::SubEq => "-=",
            Self::MulEq => "*=",
            Self::DivEq => "/=",
            Self::Arrow => "\\to ",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Elipsis => "...",
            Self::Vertical => ".y",
            Self::Horizontal => ".x",
            Self::Semicolon => ";",
            Self::LeftParen => "\\left(",
            Self::RightParen => "\\right)",
            Self::LeftSquare => "\\left[",
            Self::RightSquare => "\\right]",
            Self::LeftCurly => "\\left\\{",
            Self::RightCurly => "\\right\\}",
        })
    }
}


#[derive(Debug,Copy,Clone)]
pub enum Keyword {
    Namespace,
    Graph,
    This,
    Std,
    Use,
} impl Keyword {
    pub fn from_string(string: String) -> Option<Self> {
        match string.as_str() {
            "graph" => Some(Self::Graph),
            "namespace" => Some(Self::Namespace),
            "this" => Some(Self::This),
            "std" => Some(Self::Std),
            "use" => Some(Self::Use),
            _ => None
        }
    }
}


#[derive(Debug)]
pub enum Token {
    Whitespace(bool),
    Symbol(Symbol),
    Identifier(String),
    Number(String),
    Text(String),
    Keyword(Keyword),
} impl Token {
    pub fn from_ref(token: &Self) -> Self {
        match token {
            Self::Symbol(c) => Self::Symbol(*c),
            Self::Identifier(i) => Self::Identifier(i.to_string()),
            Self::Number(n) => Self::Number(n.to_string()),
            Self::Text(t) => Self::Text(t.to_string()),
            Self::Keyword(k) => Self::Keyword(*k),
            Self::Whitespace(b) => Self::Whitespace(*b),
        }
    }
    pub fn vec_from_chars<I>(chars: &mut Peekable<I>) -> Result<Vec<Self>, &'static str>
    where I: Iterator<Item = char>
    {
        let mut result = vec![];
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                result.push(Self::whitespace_from_chars(chars)?);
            } else if c.is_alphabetic() || c == '_' {
                result.push(Self::identifier_or_keyword_from_chars(chars)?);
            } else if c.is_numeric() {
                result.push(Self::number_from_chars(chars)?);
            } else if c == '#' {
                result.push(Self::text_from_chars(chars)?);
            } else {
                result.push(Self::symbol_from_chars(chars)?);
            }
        }
        Ok(result)
    }
    pub fn text_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        let mut value = String::new();
        let mut is_newline = true;
        while let Some(&_c) = chars.peek() {
            if let Self::Whitespace(true) = Self::whitespace_from_chars(chars)? {
                break;
            }
            if let Some('#') = chars.peek() {
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        value.push('\n');
                        is_newline = true;
                        break;
                    } else if c.is_whitespace() {
                        chars.next();
                    } else {
                        if is_newline {
                            is_newline = false;
                        } else {
                            value.push_str("  ");
                        }
                        break; 
                    }
                }
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        break;
                    } else {
                        value.push(c);
                    }
                }
            } else {
                break;
            }
        }
        Ok(Self::Text(value))
    }
    pub fn symbol_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        Ok(Token::Symbol(Symbol::from_chars(chars)?))
    }
    pub fn whitespace_from_chars<I>(chars: &mut Peekable<I>) -> Result<Self, &'static str>
    where I: Iterator<Item = char>
    {
        let mut value = false;
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                value = value || c == '\n';
                chars.next();
            } else {
                break; 
            }
        }
        Ok(Self::Whitespace(value))
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
    pub fn get_latex(&self) -> String {
        match self {
            Self::Symbol(s) => s.get_latex(),
            Self::Number(n) => n.to_string(),
            _ => String::new(),
        }
    }
}
