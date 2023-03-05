use std::env::args;
use std::collections::HashMap;

mod preprocessor;
use crate::preprocessor::preprocess;

mod ast;
use crate::ast::lexer::Token;
use crate::ast::{AbstractSyntaxItem, AbstractSyntaxTree, AbstractSyntaxTreeTrait, ExpressionItem, Variable};


#[derive(Debug)]
struct DesmosExpression {
    latex: String,
    folder_id: Option<String>,
    opacity: Option<String>,
    color_latex: Option<String>,
}


#[derive(Debug)]
enum DesmosLine {
    Expression(DesmosExpression),
    Folder(String),
    Text(String, Option<String>),
} impl DesmosLine {
    pub fn vec_from_ast(ast: AbstractSyntaxTree) -> Vec<Self> {
        let mut result = vec![];
        let ids = &ast.get_variable_ids();
        let usespace = &mut HashMap::new();
        Self::fill_from_ast(&mut result, ast, vec![], usespace, ids);
        result
    }
    pub fn fill_from_ast(vec: &mut Vec<Self>, ast: AbstractSyntaxTree, namespaces: Vec<String>, usespace: &mut HashMap<String, Variable>, ids: &HashMap<Vec<String>, usize>) {
        type T = AbstractSyntaxItem;
        let temp = namespaces.join(".");
        let mut folders = vec![];
        let mut current_uses = vec![];
        for i in ast {
            match i {
                T::Expression(e) => vec.push(Self::Expression(DesmosExpression {
                    latex: ExpressionItem::vec_to_latex(e, namespaces.to_vec(), usespace, ids),
                    folder_id: if temp.is_empty() { None } else { Some(temp.to_string()) }, 
                    opacity: None,
                    color_latex: None,
                })),
                T::Graph(c, opacity, e) => {
                    let color = if let Some(c) = c { 
                        c.get_latex(&namespaces, usespace, ids)
                    } else { 
                        String::new() 
                    };
                    vec.push(Self::Expression(DesmosExpression {
                        latex: ExpressionItem::vec_to_latex(e, namespaces.to_vec(), usespace, ids),
                        folder_id: if temp.is_empty() { None } else { Some(temp.to_string()) }, 
                        opacity,
                        color_latex: Some(color),
                    }));
                },
                T::Namespace(name, e) => {
                    let mut names = namespaces.to_vec();
                    names.push(name.to_string());
                    folders.push(Self::Folder(names.join(".")));
                    Self::fill_from_ast(&mut folders, e, names, usespace, ids);
                },
                T::Text(t) => vec.push(Self::Text(t, if temp.is_empty() { None } else { Some(temp.to_string()) })),
                T::Use(ExpressionItem::Variable(v)) => {
                    match v {
                        Variable::Absolute(is) => {
                            let mut key = String::new();
                            let mut value = vec![];
                            for i in is {
                                if !key.is_empty() {
                                    value.push(key.to_string());
                                }
                                key = i;
                            }
                            usespace.insert(key.to_string(), Variable::Absolute(value));
                            current_uses.push(key.to_string());
                        },
                        Variable::Relative(is) => {
                            let mut key = String::new();
                            let mut value = vec![];
                            for i in is {
                                if !key.is_empty() {
                                    value.push(key.to_string());
                                }
                                key = i;
                            }
                            usespace.insert(key.to_string(), Variable::Relative(value));
                            current_uses.push(key.to_string());
                        },
                        Variable::Std(name) => {
                            usespace.insert(name.to_string(), Variable::Std(String::new()));  
                            current_uses.push(name.to_string());
                        },
                    }
                },
                _ => (),
            }
        }
        vec.append(&mut folders);
        for i in current_uses {
            usespace.remove(&i);
        }
    }
    fn get_desmos_object_js(&self) -> HashMap<&'static str, String> {
        match self {
            Self::Expression(e) => {
                let mut result = HashMap::from([
                    ("type", "expression".to_string()),
                    ("latex", e.latex.to_string()),
                ]);
                if let Some(i) = &e.folder_id {
                    result.insert("folderId", i.to_string());
                }
                if let Some(c) = &e.color_latex {
                    result.insert("colorLatex", c.to_string());
                } else {
                    result.insert("hidden", "true".to_string());
                }
                if let Some(o) = &e.opacity {
                    result.insert("fillOpacity", o.to_string());
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
        println!("
        ]}},
        graph: {{ 
            showGrid: false, 
            showXAxis: false, 
            showYAxis: false,
            xAxisNumbers: false,
            yAxisNumbers: false,
        }},
    }})");
        println!("</script>");
        println!("</body>");
        println!("</html>");
    }
    fn get_api_link(&self) -> String {
        let url_start = "https://www.desmos.com/api/v1.7/calculator.js?apiKey=";
        format!("{url_start}{}", self.api_key)
    }
}


fn cli(options: Vec<String>) -> Result<(), &'static str> {
    let mut definitions = HashMap::new();
    if let Some(path) = options.get(1) {
        let chars = preprocess(path, &mut definitions)?;
        if options.contains(&"--preprocess".to_string()) {
            eprintln!("{chars}");
        }
        let tokens = Token::vec_from_chars(&mut chars.chars().peekable())?;
        if options.contains(&"--tokens".to_string()) {
            eprintln!("{tokens:?}"); 
        }
        let ast = AbstractSyntaxTree::from_tokens(&mut tokens.iter().peekable(), false)?;
        if options.contains(&"--ast".to_string()) {
            eprintln!("{ast:#?}"); 
        }
        let lines = DesmosLine::vec_from_ast(ast);
        if options.contains(&"--lines".to_string()) {
            eprintln!("{lines:#?}"); 
        }
        let calc = GraphingCalculator::from(lines);
        calc.print_html();
    } else {
        println!("----The-Desmon-Compiler----");
        println!("Compile Desmon code into an HTML file by running:");
        println!(">>> cargo run [path_to_file] > [path_to_html_output_file]");
    }
    Ok(())
}


fn main() {
    if let Err(e) = cli(args().collect()) {
        eprintln!("\x1b[31m{e}\x1b[0m");
    }
}
