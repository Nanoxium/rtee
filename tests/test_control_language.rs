use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/fuzzy_logic/grammar/fcl.pest"]
pub struct FCLParser;

#[test]
fn test_fcl_var_declaration() {
    let input_text = r#"temperature: REAL;"#;
    let values = FCLParser::parse(Rule::declaration, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())

}

#[test]
fn test_fcl_var_input_block() {
    let input_text = r#"VAR_INPUT
          temperature: REAL;  // Input variable for temperature in degrees Celsius
          humidity: REAL;     // Input variable for humidity in percentage
        END_VAR"#;
    let values = FCLParser::parse(Rule::var_input, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())

}

#[test]
fn test_fcl_var_output_block() {
    let input_text = r#"VAR_OUTPUT
                          heating: REAL;      
                        END_VAR"#;
    let values = FCLParser::parse(Rule::var_output, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())

}

#[test]
fn test_fcl_term() {
    let input_text = r#"TERM cold := (-20,1) (-10,1) (0, 0);"#;
    let values = FCLParser::parse(Rule::term, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_fuzzify_block() {
    let input_text = r#"FUZZIFY temperature
                          TERM cold :=(-20, 1) (-10, 1) (0, 0);
                          TERM comfortable := (0, 0) (20, 1) (40, 1) (60, 0);
                          TERM hot := (60, 0) (70, 1) (100, 1);
                        END_FUZZIFY"#;
    let values = FCLParser::parse(Rule::fuzzify, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}


#[test]
fn test_fcl_defuzzify_block() {
    let input_text = r#"DEFUZZIFY heating
                          TERM off := (0, 1) (20, 1) (40, 0);
                          TERM low := (20, 0) (40, 1) (60, 1) (80, 0);
                          TERM medium := (40, 0) (60, 1) (80, 1) (100, 0);
                          TERM high := (60, 0) (80, 1) (100, 1) (120, 0);
                          METHOD: COG;
                        END_DEFUZZIFY"#;
    let values = FCLParser::parse(Rule::defuzzify, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_antecedent() {
    let input_text = r#"temperature IS cold"#;
    let values = FCLParser::parse(Rule::antecedent, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_consequent() {
    let input_text = r#"temperature IS cold"#;
    let values = FCLParser::parse(Rule::antecedent, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_rule_parsing() {
    let input_text = r#"RULE 1 : IF temperature IS cold AND humidity IS low THEN heating IS high;"#;
    let values = FCLParser::parse(Rule::rule, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_rule_operators_parsing() {
    let input_text = r#"
          AND: MIN;
          ACCU: MAX;
    "#;
    let values = FCLParser::parse(Rule::operators, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_rule_block_parsing() {
    let input_text = r#"RULEBLOCK No1
          AND: MIN;
          ACCU: MAX;

          RULE 1 : IF temperature IS cold AND humidity IS low THEN heating IS high;
          RULE 2 : IF temperature IS cold AND humidity IS normal THEN heating IS high;
          RULE 3 : IF temperature IS cold AND humidity IS high THEN heating IS medium;
          RULE 4 : IF temperature IS comfortable AND humidity IS low THEN heating IS low;
          RULE 5 : IF temperature IS comfortable AND humidity IS normal THEN heating IS low;
          RULE 6 : IF temperature IS comfortable AND humidity IS high THEN heating IS medium;
          RULE 7 : IF temperature IS hot AND humidity IS low THEN heating IS off;
          RULE 8 : IF temperature IS hot AND humidity IS normal THEN heating IS off;
          RULE 9 : IF temperature IS hot AND humidity IS high THEN heating IS off;
        END_RULEBLOCK
    "#;
    let values = FCLParser::parse(Rule::ruleblock, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}

#[test]
fn test_fcl_parser() {
    let input_text = r#"
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
          RULE 3 : IF temperature IS cold AND humidity IS high THEN heating IS medium;
          RULE 4 : IF temperature IS comfortable AND humidity IS low THEN heating IS low;
          RULE 5 : IF temperature IS comfortable AND humidity IS normal THEN heating IS low;
          RULE 6 : IF temperature IS comfortable AND humidity IS high THEN heating IS medium;
          RULE 7 : IF temperature IS hot AND humidity IS low THEN heating IS off;
          RULE 8 : IF temperature IS hot AND humidity IS normal THEN heating IS off;
          RULE 9 : IF temperature IS hot AND humidity IS high THEN heating IS off;
        END_RULEBLOCK

        END_FUNCTION_BLOCK"#;
    let values = FCLParser::parse(Rule::fcl, input_text);
    if let Err(error) = &values {
        eprintln!("Error: {error:?}");
    }
    assert!(values.is_ok())
}
