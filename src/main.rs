// use std::env;
use std::fs::{read_to_string};
// use std::io::{prelude::*, BufReader};
// use std::iter::Peekable;
// use std::{thread, time};
use std::collections::HashMap;


mod preprocessor;
use crate::preprocessor::preprocess;


mod ast;
use crate::ast::lexer::Token;
use crate::ast::{AbstractSyntaxItem, AbstractSyntaxTree, AbstractSyntaxTreeTrait, ExpressionItem};


enum DesmosLine {
    Expression(String, Option<String>, Option<String>),
    Folder(String),
    Text(String, Option<String>),
} impl DesmosLine {
    pub fn fill_from_ast(vec: &mut Vec<Self>, ast: AbstractSyntaxTree, namespaces: Vec<String>, ids: &HashMap<Vec<String>, usize>) {
        type T = AbstractSyntaxItem;
        let temp = namespaces.join(".");
        let mut folders = vec![];
        for i in ast {
            match i {
                T::Expression(e) => vec.push(Self::Expression(
                    ExpressionItem::vec_to_latex(e, namespaces.to_vec(), ids),
                    if temp.is_empty() { None } else { Some(temp.to_string()) }, 
                    None,
                )),
                T::Graph(c, e) => vec.push(Self::Expression(
                    ExpressionItem::vec_to_latex(e, namespaces.to_vec(), ids),
                    if temp.is_empty() { None } else { Some(temp.to_string()) }, 
                    Some(c.get_latex(&namespaces, ids)),
                )),
                T::Namespace(name, e) => {
                    let mut names = namespaces.to_vec();
                    names.push(name.to_string());
                    folders.push(Self::Folder(names.join(".")));
                    Self::fill_from_ast(&mut folders, e, names, ids);
                },
                T::Text(t) => vec.push(Self::Text(t, if temp.is_empty() { None } else { Some(temp.to_string()) })),
            }
        }
        vec.append(&mut folders);
    }
    fn get_desmos_object_js(&self) -> HashMap<&'static str, String> {
        match self {
            Self::Expression(latex, folder_id, color) => {
                let mut result = HashMap::from([
                    ("type", "expression".to_string()),
                    ("latex", latex.to_string()),
                ]);
                if let Some(i) = folder_id {
                    result.insert("folderId", i.to_string());
                }
                if let Some(c) = color {
                    result.insert("colorLatex", c.to_string());
                    result.insert("fillOpacity", "1".to_string());
                } else {
                    result.insert("hidden", "true".to_string());
                }
                result
            },
            Self::Folder(title) => HashMap::from([
                ("type", "folder".to_string()),
                ("title", title.to_string()),
                ("id", title.to_string()),
                ("collapsed", "true".to_string()),
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
    calculator.setState({{
        version: 9,
        expressions: {{ list: [");
        for expr in &self.expressions {
            println!("{:?},", expr.get_desmos_object_js());
        }
        println!("]}}}}
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
    let _raw = r"
// I PEE IN???
namespace lol
{ 
    # This namespace is very important.
    # Idk why but it is
    x = y + 1; 
    // bruh lol
    /=trait { \
        this.glock_clock = 5; \
        this.fuck_me -> 3; \
    }
    namespace hell_nah 
    {
        # This namespace is a little less important.
        #
        # It has a empty line!
        #

        #
        # THis is a different comment
        #
        this.mmmm.mmmm = 5;
    }
    this.bruh(y) = x * 3;
    this.a = std.pi;
    namespace myass ?trait
}
lol.bruh(this.a);
lol
  .hell_nah
  .mmmm
  .mmmm;
";
    let mut definitions = HashMap::new();
    let raw = read_to_string("game2.ds").expect("WHAT A FUCKED UP DAY!!!...");
    let raw = preprocess(&mut raw.chars().peekable(), &mut definitions);
    if let Ok(chars) = raw {
        eprintln!("{chars}");
        let tokens = Token::vec_from_chars(&mut chars.chars().peekable());
        eprintln!("{tokens:?}");
        if let Ok(t) = tokens {
            let abss = AbstractSyntaxTree::from_tokens(&mut t.iter().peekable(), false);
            if let Ok(a) = abss {
                eprintln!("{a:#?}");
                let ids = a.get_variable_ids();
                eprintln!("{ids:#?}");
                let mut lines = vec![];
                DesmosLine::fill_from_ast(&mut lines, a, vec![], &ids);
                let calc = GraphingCalculator::from(lines);
                calc.print_html();
            } else if let Err(e) = abss {
                eprintln!("{e:?}");
            }
        } else if let Err(e) = tokens {
            eprintln!("{e:?}");
        }
    } else if let Err(e) = raw {
        eprintln!("{e:?}"); 
    }
}
