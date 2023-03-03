use std::collections::HashMap;
use std::iter::Peekable;


pub fn preprocess<I>(chars: &mut Peekable<I>) -> Result<String, &'static str>
where I: Iterator<Item = char>
{
    let mut result = String::new();
    let mut definitions = HashMap::new();
    while let Some(c) = chars.next() {
        if c == '/' {
            match chars.next() {
                Some('/') => preprocess_comment(chars)?,
                Some('*') => preprocess_multiline_comment(chars)?,
                Some('=') => preprocess_set_definition(chars, &mut definitions)?,
                _ => result.push('/'),
            }
        } else if c == '?' {
            result.push_str(preprocess_get_definition(chars, &definitions)?.as_str());
        } else {
            result.push(c);
        }
    }
    eprintln!("{definitions:#?}");
    Ok(result)
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
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            break;
        } else if c.is_alphanumeric() {
            key.push(c);
        } else {
            Err("Definition can only be alphanumeric")?;
        }
    }
    let mut value = String::new();
    while let Some(c) = chars.next() {
        if c == '?' {
            value.push_str(preprocess_get_definition(chars, definitions)?.as_str());
        } else if c == '\\' {
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


pub fn preprocess_get_definition<I>(chars: &mut Peekable<I>, definitions: &HashMap<String, String>) -> Result<String, &'static str>
where I: Iterator<Item = char>
{
    let mut current_key = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() {
            current_key.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if let Some(current_value) = definitions.get(&current_key) {
        Ok(current_value.to_string())
    } else {
        Err("Unkown definition")
    }
}


/*
pub fn preprocess(string: String) -> Result<String, &'static str> {
    let mut result = String::new();
    let mut last = ' ';
    let mut is_in_definition = false;
    let mut is_in_comment = false;
    let mut inline_comment_level = 0;
    let mut is_definition_start = false;
    let mut is_recursive_definition = false;
    let mut definitions = HashMap::<String, String>::new();
    let mut key = String::new();
    let mut recursive_key = String::new();
    let mut value = String::new();
    for c in string.chars() {
        match c {
            '/' => if !is_in_comment && inline_comment_level == 0 && last == '/' {
                result.pop();
                is_in_comment = true;
            } else if inline_comment_level != 0 && last == '*' {
                inline_comment_level -= 1;
                continue;
            },
            '!' => if !is_in_comment && inline_comment_level == 0 && last  == '/' {
                result.pop();
                is_in_definition = true;
                is_definition_start = true;
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
                if is_in_definition {
                    if is_recursive_definition {
                        if let Some(val) = definitions.get(&recursive_key) {
                            value.push_str(val.as_str());
                            recursive_key = String::new();
                            is_recursive_definition = false;
                        } else {
                            eprintln!("{definitions:#?}");
                            eprintln!("rec {recursive_key}");
                            return Err("Use of undefined definition");
                        }
                    }
                    definitions.insert(key, value);
                    key = String::new();
                    value = String::new();
                    is_in_definition = false;
                }
            },
            _ => (),
        }
        if !is_in_comment && inline_comment_level == 0 {
            if is_in_definition {
                if is_definition_start {
                    if c.is_alphanumeric() {
                        key.push(c);
                    } else if c == '=' {
                        is_definition_start = false;
                    } else if let Some(val) = definitions.get(&key) {
                        result.push_str(val.as_str());
                        key = String::new();
                        is_in_definition = false;
                        result.push(c);
                    } else {
                        eprintln!("{definitions:#?}");
                        eprintln!("reg {key}");
                        return Err("Use of undefined definition");
                    }
                } else if is_recursive_definition {
                    if c.is_alphanumeric() {
                        recursive_key.push(c);
                    } else if let Some(val) = definitions.get(&recursive_key) {
                        value.push_str(val.as_str());
                        recursive_key = String::new();
                        is_recursive_definition = false;
                        value.push(c);
                    } else {
                        eprintln!("{definitions:#?}");
                        eprintln!("rec {recursive_key}");
                        return Err("Use of undefined definition");
                    }
                } else if last == '/' && c == '!' {
                    value.pop();
                    is_recursive_definition = true;
                } else {
                    value.push(c);
                }
            } else {
                result.push(c);
            }
        }
        last = c;
    }
    Ok(result)
}
*/
