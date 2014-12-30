use racc::runtime::ParserTables;

grammar! {
    () ctx;
    ();

    WINDOW;
    PLUS; MINUS;
    LPAREN; RPAREN;
    LBRACK; RBRACK;
    COMMA;
    LARROW;
    ID;
    INT;
    STRING;
    VBAR;

    query: window;

    exp: exp PLUS exp
       | exp MINUS exp
       //| var
       | ID
       | INT
       | STRING;

    range: LPAREN /*exp*/ COMMA /*exp*/ RPAREN;

    //window : LBRACK exp VBAR decls RBRACK;
    window: LBRACK exp VBAR ID LARROW range RBRACK;

    //decls : decl;
          //| decls decl;

    //decl: var LARROW range;
    //decls: ID LARROW range;

    //var: ID;
}

#[cfg(test)]
mod tests {
    use query::grm as S;
    use super::get_parser_tables;
    use racc::runtime::{ParserState, FinishParseResult};

    #[test]
    fn parse() {
        let toks = vec![
            S::LBRACK,
            S::INT,
            S::VBAR,
            S::ID,
            S::LARROW,
            S::LPAREN,
            //S::INT,
            S::COMMA,
            //S::INT,
            S::RPAREN,
            S::RBRACK,
        ];
        let mut parser = ParserState::new(get_parser_tables());
        let mut ctx = ();
        for &tok in toks.iter() {
            parser.push_token(&mut ctx, tok, ());
        }
        match parser.finish(&mut ctx) {
            FinishParseResult::Accepted(final_value) => {
                println!("Accepted: {}", final_value);
            }
            FinishParseResult::SyntaxError => {
                println!("SyntaxError");
            }
        }
    }
}
