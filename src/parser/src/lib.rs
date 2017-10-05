extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// Doing this to force recompile
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest"); 

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ScriptParser;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn single_line_comment() {
        let pairs = ScriptParser::parse_str(Rule::temp_wrapper,
                                            "// whatever\r\n//test2").unwrap_or_else(|e| panic!("{}", e));
        for pair in pairs.clone().next().unwrap().into_inner() {
            println!("{:?}({}):\n  {}",
                     pair.as_rule(),
                     pair.clone().into_span().start(),
                     pair.as_str());
        }
    }
}
