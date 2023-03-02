// use std::env;
// use std::fs::{/*File, */read_to_string};
// use std::io::{prelude::*, BufReader};
// use std::iter::Peekable;
// use std::{thread, time};
use std::collections::HashMap;


mod ast;
use crate::ast::lexer::Token;
use crate::ast::{AbstractSyntaxTree, AbstractSyntaxTreeTrait};


enum DesmosLine {
    Expression(String, Option<u32>, Option<String>),
    Folder(String, u32),
    Text(String, Option<u32>),
} impl DesmosLine {
    fn get_desmos_object_js(&self) -> HashMap<&'static str, String> {
        match self {
            Self::Expression(latex, folder_id, color) => {
                let mut result = HashMap::from([("latex", latex.to_string())]);
                if let Some(i) = folder_id {
                    result.insert("folderId", i.to_string());
                }
                if let Some(c) = color {
                    result.insert("colorLatex", c.to_string());
                } else {
                    result.insert("hidden", "true".to_string());
                }
                result
            },
            Self::Folder(title, id) => HashMap::from([
                ("type", "folder".to_string()),
                ("title", title.to_string()),
                ("id", id.to_string()),
            ]),
            Self::Text(text, folder_id) => {
                let mut result = HashMap::from([
                    ("type", "text".to_string()),
                    ("text", text.to_string()),
                ]);
                if let Some(i) = folder_id {
                    result.insert("folderId", i.to_string());
                }
                result
            },

        }
    }
}


struct GraphingCalculator {
    expressions: Vec<DesmosLine>, 
    api_key: String,
} impl GraphingCalculator {
    pub fn from(expressions: Vec<DesmosLine>) -> Self {
        Self {
            expressions,
            api_key: "dcb31709b452b1cf9dc26972add0fda6".to_string(),
        }
    }
    /*
    pub fn from_file(path: &str, print_tokens: bool, print_ast: bool, print_preprocess: bool) -> Result<Self, &'static str> {
        let list = AbstractSyntaxItem::vec_from_file(path, print_tokens, print_preprocess)?;
        if print_ast {
            eprintln!("{list:?}");
        }
        Ok(Self::new(list))
    }
    */
    pub fn print_html(&self) {
        let api_link = self.get_api_link();
        println!(r"<!DOCTYPE html>
<html style='height: 100%;'>
<body style='height: 100%; margin: 0%'>
<script src='{api_link}'></script>
<div id='calculator' style='width: 100%; height: 100%;'></div>
<script>
    var elt = document.getElementById('calculator');
    var calculator = Desmos.GraphingCalculator(elt);
    calculator.setState(
        expressions: {{ list: [");
        for expr in &self.expressions {
            println!("{:?},", expr.get_desmos_object_js());
        }
        println!("]}}
    )");
        println!("</script>");
        println!("</body>");
        println!("</html>");
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key)
    }
}


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
    let chars = r"
# This namespace is very important.
namespace lol { 
    x = y + 1; 
    namespace hell_nah 
    {
        this.mmmm.mmmm = 5;
        # This namespace is a little less important.
        a = 5;
    }
    this.bruh(y) = x * 3;
    this.a =1;
}
lol.bruh(this.a);
lol
  .hell_nah
  .mmmm
  .mmmm;
";
    let _chars = "hello.bruh;";
    let tokens = Token::vec_from_chars(&mut chars.chars().peekable());
    println!("{tokens:?}");
    if let Ok(t) = tokens {
        let abss = AbstractSyntaxTree::from_tokens(&mut t.iter().peekable(), false);
        if let Ok(a) = abss {
            println!("{a:#?}");
            let ids = a.get_variable_ids();
            println!("{ids:#?}");
        } else if let Err(e) = abss {
            println!("{e:?}");
        }
    } else if let Err(e) = tokens {
        println!("{e:?}");
    }
}
