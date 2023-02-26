// use std::env;
// use std::fs::{/*File, */read_to_string};
// use std::io::{prelude::*, BufReader};
use std::iter::Peekable;
use std::{thread, time};


mod ast;


/*
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
    pub fn from_file(path: &str, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<Self, &'static str> {
        let list = AbstractSyntaxItem::vec_from_file(path, print_tokens, print_preprocess)?;
        if print_ast {
            eprintln!("{list:?}");
        }
        Ok(Self::new(list))
    }
    pub fn print_html(&self, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<(), &'static str> {
        let api_link = self.get_api_link();
        let expressions = AbstractSyntaxItem::get_expressions(&self.expressions, String::new(), print_tokens, print_ast, print_preprocess)?; 
        println!(r"<!DOCTYPE html>
<html style='height: 100%;'>
<body style='height: 100%; margin: 0%'>
<script src='{api_link}'></script>
<div id='calculator' style='width: 100%; height: 100%;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);");
        for (index, (item, color)) in expressions.iter().enumerate() {
            let options = if let Some(c) = color {
                format!("color: '#{c}'")
            } else {
                "hidden: true".to_string()
            };
            println!("    calculator.setExpression({{id: '{index}', latex: '{item}', {options}}});");
        }
        println!("</script>");
        println!("</body>");
        println!("</html>");
        Ok(())
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key)
    }
}
*/


fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let print_tokens = args.contains(&"--tokens".to_string());
    let print_ast = args.contains(&"--ast".to_string());
    let print_preprocess = args.contains(&"--preprocess".to_string());
    if let Some(file_path) = &args.get(1) {
        match GraphingCalculator::from_file(file_path, print_tokens, print_ast, print_preprocess) {
            Ok(gc) => match gc.print_html(print_tokens, print_ast, print_preprocess) {
                 Ok(()) => (),
                 Err(e) => eprintln!("\x1b[31m{e}\x1b[0m"),
            },
            Err(e) => eprintln!("\x1b[31m{e}\x1b[0m"),
        }
    } else {
        eprintln!("\x1b[33mNo file specified\x1b[0m");
    }
    */
    /*
    let chars = "namespace lol { x = y + 1; }";
    let tokens = Token::vec_from_chars(&mut chars.chars().peekable());
    println!("{tokens:?}");
    */
}
