use std::collections::HashMap;
use std::iter::Peekable;
use std::fs::read_to_string;


pub fn preprocess<I>(chars: &mut Peekable<I>, definitions: &mut HashMap<String, String>) -> Result<String, &'static str>
where I: Iterator<Item = char>
{
    let mut result = String::new();
    while let Some(c) = chars.next() {
        match c {
            '/' => if let Some(c) = chars.next() {
                match c {
                    '/' => preprocess_comment(chars)?,
                    '*' => preprocess_multiline_comment(chars)?,
                    '=' => preprocess_set_definition(chars, definitions)?,
                    '#' => add_file(chars, definitions, &mut result)?,
                    c => {
                        result.push('/');
                        result.push(c);
                    },
                }
            } else {
                result.push('/');
            },
            '?' => result.push_str(preprocess_get_definition(chars, definitions)?.as_str()),
            _ => result.push(c),
        }
    }
    Ok(result)
}


pub fn add_file<I>(chars: &mut Peekable<I>, definitions: &mut HashMap<String, String>, result: &mut String) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    let mut file_name = String::new();
    while let Some(c) = chars.next() {
        if c == '\n' {
            break;
        } else {
            file_name.push(c);
        }
    }
    if let Ok(contents) = read_to_string(file_name) {
        result.push_str(preprocess(&mut contents.chars().peekable(), definitions)?.as_str());
        Ok(())
    } else {
        Err("Module was not found")
    }
}


fn preprocess_comment<I>(chars: &mut Peekable<I>) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    while let Some(c) = chars.next() {
        if c == '\n' { break; }
    }
    Ok(())
}


fn preprocess_multiline_comment<I>(chars: &mut Peekable<I>) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    let mut level = 1;
    while let Some(c) = chars.next() {
        if c == '*' {
            if let Some('/') = chars.next() {
                level -= 1;
            }
        }
        if level == 0 {
            break;
        }
    }
    Ok(())
}


pub fn preprocess_set_definition<I>(chars: &mut Peekable<I>, definitions: &mut HashMap<String, String>) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    let mut key = String::new();
    if let Some(&c) = chars.peek() {
        if !(c.is_alphabetic() || c == '_' || c.is_whitespace()) {
            Err("Definition can only start with a letter or '_'")?;
        }
    }
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            break;
        } else if c.is_alphanumeric() || c == '_' {
            key.push(c);
        } else {
            Err("Definition can only be alphanumeric or '_'")?;
        }
    }
    let mut value = String::new();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if chars.next() == Some('\n') {
                value.push('\n');
            } else {
                value.push('\\');
                value.push(c);
            }
        } else if c == '\n' {
            break;
        } else {
            value.push(c);
        }
    }
    definitions.insert(key, value);
    Ok(())
}


pub fn preprocess_get_definition<I>(chars: &mut Peekable<I>, definitions: &mut HashMap<String, String>) -> Result<String, &'static str>
where I: Iterator<Item = char>
{
    let mut key = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            key.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if let Some(&'(') = chars.peek() {
        chars.next();
        preprocess_set_args(chars, definitions)?;
    }
    if let Some(value) = definitions.get(&key) {
        let mut result = String::new();
        let temp = value.to_string();
        let v_chars = &mut temp.chars().peekable();
        while let Some(c) = v_chars.next() {
            if c == '?' {
                result.push_str(preprocess_get_definition(v_chars, definitions)?.as_str());
            } else {
                result.push(c);
            }
        }
        Ok(result.to_string())
    } else {
        Err("Unkown definition")
    }
}


pub fn preprocess_set_args<I>(chars: &mut Peekable<I>, definitions: &mut HashMap<String, String>) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    let mut index = 0;
    let mut value = String::new();
    let mut paren_level = 1;
    while let Some(c) = chars.next() {
        if c == ';' {
            definitions.insert(index.to_string(), value);
            index += 1;
            value = String::new();
        } else if c == '(' {
            paren_level += 1;
        } else if c == ')' {
            paren_level -= 1;
            if paren_level == 0 {
                definitions.insert(index.to_string(), value);
                break;
            }
        } else {
            value.push(c);
        }
    }
    Ok(())
}


