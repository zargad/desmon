use std::collections::HashMap;


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
    fn get_style(&self) -> String {
        format!("width: {}; height: {};", "600px", "400px").to_string()
    }
    fn get_api_link(&self) -> String {
        format!("https://www.desmos.com/api/v1.7/calculator.js?apiKey={}", self.api_key).to_string()
    }
}


fn main() {
    GraphingCalculator::new(vec![
        "y=x^2".to_string(),
        "y=2x".to_string(),
        "y=sin(x)".to_string(),
    ]).print_html();
}
