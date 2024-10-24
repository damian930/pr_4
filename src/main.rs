use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {
    let result = Grammar::parse(Rule::file, "-273.15,-15\n");
    println!("{:?}", result);
    
    return Ok(());
}
