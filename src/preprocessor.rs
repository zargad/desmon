use std::collections::HashMap;
use std::iter::Peekable;


pub fn preprocess<I>(chars: &mut Peekable<I>) -> Result<String, &'static str>
where I: Iterator<Item = char>
{
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '/' {
            match chars.next() {
                Some('/') => while let Some(c) = chars.next() {
                    if c == '\n' { break; }
                },
                Some('*') => preprocess_multiline_comments(chars)?,
                _ => result.push('/'),
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}


/*
fn preprocess_comments<I>(chars: &mut Peekable<I>) -> Result<(), &'static str>
where I: Iterator<Item = char>
{
    match c.next() {
        '/' => while let Some(c) = chars.next() {
            if c == '\n' { break; }
        },
        '*' => preprocess_multiline_comments(chars)?,
        _ => result.push('/'),
    }
    Ok(())
}
*/


fn preprocess_multiline_comments<I>(chars: &mut Peekable<I>) -> Result<(), &'static str>
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


/*
pub fn preprocess_definitions<I>(chars: &mut Peekable<I>, definitions: &mut D) -> Result<String, &'static str>
where 
    I: Iterator<Item = &'a char>,
    D: HashMap<String, String>,
{
    let mut result = String::new();
    let mut key = String::new();
    while let Some(c) = chars.next() {
        if *c == '\n' {
            is_in_comment = false;
        } else if *c == '_' {
            
            result.push_str(preprocess)
        }
        if !is_in_comment {
            result.push(*c);
        }
        last = *c;
    }
    return Ok<result>;
}
*/
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
