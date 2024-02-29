use std::ops::Range;

use pest::{iterators::*, Parser};
use pest_derive::Parser;

use anyhow::{Error, Result};

use crate::prelude::{Mamdani, Rule as FuzzyRule};

use super::{controller::FuzzyController, FuzzyVariable};

#[derive(Parser)]
#[grammar = "fuzzy_logic/grammar/fcl.pest"]
pub struct FCLParser;

/// Variable types used in the FCL grammar
#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Number,
    Boolean,
}

pub type AstRoot = AstNode;

/// Node types used to generate the Abstract Syntax Tree
#[derive(Debug, Clone)]
pub enum AstNode {
    FunctionBlock(Vec<AstNode>),
    Block(String, Vec<AstNode>),
    VarOutput(Vec<AstNode>),
    VarInput(Vec<AstNode>),
    Antecedent(Vec<AstNode>),
    Consequent(Vec<AstNode>),
    Identifier {
        name: String,
    },
    Number(f64),
    RuleBlock {
        rules: Vec<AstNode>,
        function_ops: Vec<AstNode>,
    },
    Operator {
        name: String,
    },
    Term {
        name: String,
        envelope: Vec<Range<f64>>,
    },
    Rule {
        antecedent: Box<AstNode>,
        consequent: Box<AstNode>,
    },
    VarType(VarType),
    EOI,
}

/// Parses the input string and returns rules from the FCL grammar
pub fn fcl_parser(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    let pairs = FCLParser::parse(Rule::fcl, input)?;
    Ok(pairs)
}

/// Generates recusively an AST from the grammar rules
pub fn generate_ast(pair: Pair<Rule>) -> anyhow::Result<AstNode> {
    match pair.as_rule() {
        Rule::fcl | Rule::function_block => {
            let mut function_blocks = Vec::new();
            for fb in pair.into_inner() {
                function_blocks.push(generate_ast(fb)?);
            }
            Ok(AstNode::FunctionBlock(function_blocks))
        }
        Rule::var_input => {
            let mut var_inputs = Vec::new();
            for vi in pair.into_inner() {
                var_inputs.push(generate_ast(vi)?);
            }
            Ok(AstNode::VarInput(var_inputs))
        }
        Rule::var_output => {
            let mut var_outputs = Vec::new();
            for vo in pair.into_inner() {
                var_outputs.push(generate_ast(vo)?);
            }
            Ok(AstNode::VarOutput(var_outputs))
        }
        Rule::block | Rule::declaration | Rule::fuzzify | Rule::defuzzify => {
            generate_ast(pair.into_inner().next().unwrap())
        }
        Rule::ruleblock => {
            println!("Ruleblock called : {:#?}", pair.as_str());
            let mut rules = Vec::new();
            for r in pair.into_inner() {
                rules.push(generate_ast(r)?);
            }
            Ok(AstNode::Consequent(rules))
        }
        Rule::operators => {
            let inner = pair.into_inner();
            Ok(AstNode::Operator {
                name: inner.as_str().into(),
            })
        }
        Rule::antecedent => {
            println!("Antecedent called : {:#?}", pair.as_str());
            let mut rules = Vec::new();
            for r in pair.into_inner() {
                rules.push(generate_ast(r)?);
            }
            Ok(AstNode::Antecedent(rules))
        }
        Rule::consequent => {
            println!("Consequent called : {:#?}", pair.as_str());
            let mut rules = Vec::new();
            for r in pair.into_inner() {
                rules.push(generate_ast(r)?);
            }
            Ok(AstNode::Consequent(rules))
        }
        Rule::rule => {
            dbg!("Rule called : {:#?}", pair.as_str());
            let mut inner = pair.into_inner();
            let antecedent = inner.next().unwrap();
            let consequent = inner.next().unwrap();
            Ok(AstNode::Rule {
                antecedent: Box::new(generate_ast(antecedent)?),
                consequent: Box::new(generate_ast(consequent)?),
            })
        }
        Rule::identifier => Ok(AstNode::Identifier {
            name: pair.as_str().to_string(),
        }),
        Rule::var_type => {
            let var_type = match pair.as_str() {
                "REAL" => VarType::Number,
                "BOOL" => VarType::Boolean,
                _ => unimplemented!("VarType not implemented: {:#?}", pair.as_str()),
            };
            Ok(AstNode::VarType(var_type))
        }
        Rule::term => {
            let inner = pair.clone().into_inner();
            let mut envelope = Vec::new();
            for t in inner {
                let mut t_inner = t.into_inner();
                let v1 = t_inner.next().unwrap();
                let v2 = t_inner.next().unwrap();
                envelope.push(v1.as_str().parse()?..v2.as_str().parse()?);
            }
            Ok(AstNode::Term {
                name: pair.as_str().to_string(),
                envelope,
            })
        }
        Rule::number => {
            let number: f64 = pair.as_str().parse()?;
            Ok(AstNode::Number(number))
        }
        Rule::EOI => Ok(AstNode::EOI),
        _ => unimplemented!("Rule not implemented: {:#?}", pair.as_rule()),
    }
}

impl AstNode {
    fn generate_fuzzy_mamdani_program(&self) -> Result<FuzzyController<Mamdani>> {
        let mut inputs = Vec::new();
        let mut output = None;
        let mut rules = Vec::new();

        self.traverse(&mut inputs, &mut output, &mut rules)?;

        let output = output.ok_or_else(|| Error::msg("No output variable defined"))?;

        let mamdani = Mamdani::new(inputs, output, rules);

        Ok(FuzzyController::new(mamdani))
    }

    fn traverse(
        &self,
        inputs: &mut Vec<FuzzyVariable>,
        outputs: &mut Option<Vec<FuzzyVariable>>,
        rules: &mut Vec<FuzzyRule>,
    ) -> Result<()> {
        match self {
            AstNode::FunctionBlock(blocks) => {
                for block in blocks {
                    block.traverse(inputs, output, rules)?;
                }
            }
            AstNode::VarInput(var_inputs) => {
                for vi in var_inputs {
                    
                }
            }
            AstNode::VarOutput(var_output) => {
               output = Some(var_output);
            }
            AstNode::VarOutput(var_inputs) => {
                for vi in var_inputs {
                    
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_generate_ast() {
        let input = r#"
            FUNCTION_BLOCK RoomTempController 

            VAR_INPUT
              temperature: REAL;  // Input variable for temperature in degrees Celsius
              humidity: REAL;     // Input variable for humidity in percentage
            END_VAR

            VAR_OUTPUT
              heating: REAL;      // Output variable for heating in percentage
            END_VAR

            FUZZIFY temperature
              TERM cold :=(-20, 1) (-10, 1) (0, 0);
              TERM comfortable := (0, 0) (20, 1) (40, 1) (60, 0);
              TERM hot := (60, 0) (70, 1) (100, 1);
            END_FUZZIFY

            FUZZIFY humidity
              TERM low := (0, 1) (20, 1) (40, 0);
              TERM normal := (20, 0) (50, 1) (80, 1) (110, 0);
              TERM high := (80, 0) (100, 1) (120, 1);
            END_FUZZIFY

            DEFUZZIFY heating
              TERM off := (0, 1) (20, 1) (40, 0);
              TERM low := (20, 0) (40, 1) (60, 1) (80, 0);
              TERM medium := (40, 0) (60, 1) (80, 1) (100, 0);
              TERM high := (60, 0) (80, 1) (100, 1) (120, 0);
              METHOD: COG;
            END_DEFUZZIFY

            RULEBLOCK No1
              AND: MIN;
              ACCU: MAX;

              RULE 1 : IF temperature IS cold AND humidity IS low THEN heating IS high;
              RULE 2 : IF temperature IS cold AND humidity IS normal THEN heating IS high;
              RULE 3 : IF temperature IS cold AND humidity IS high OR humidity IS medium THEN heating IS medium;
              RULE 4 : IF temperature IS comfortable AND humidity IS low THEN heating IS low;
              RULE 5 : IF temperature IS comfortable AND humidity IS normal THEN heating IS low;
              RULE 6 : IF temperature IS comfortable AND humidity IS high THEN heating IS medium;
              RULE 7 : IF temperature IS hot AND humidity IS low THEN heating IS off;
              RULE 8 : IF temperature IS hot AND humidity IS normal THEN heating IS off;
              RULE 9 : IF temperature IS hot AND humidity IS high THEN heating IS off;
            END_RULEBLOCK


            END_FUNCTION_BLOCK
        "#;
        let mut pairs = super::fcl_parser(input).unwrap();
        for pair in pairs.clone() {
            let ast = super::generate_ast(pair).unwrap();
            println!("{:#?}", ast);
        }
        let ast = generate_ast(pairs.next().unwrap()).expect("Failed to generate ast");
        assert!(matches!(ast, AstNode::FunctionBlock(_)));
    }
}

// #[derive(Parser)]
// #[grammar = "fuzzy_logic/grammar/fll.pest"]
// pub struct FLLParser;

// pub fn fll_parser(input: &str) -> Result<(), pest::error::Error<Rule>> {
//     todo!("Fill this function")
// }
