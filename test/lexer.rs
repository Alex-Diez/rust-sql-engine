use expectest::prelude::{be_ok, be_err};

use sql::lexer::Tokenizer;
use sql::lexer::Token::{Ident, NumericConstant, CharactersConstant};

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!("".tokenize())
        .to(be_ok().value(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!("word".tokenize())
        .to(be_ok().value(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!("this is a sentence".tokenize())
        .to(be_ok().value(
            vec![
                Ident("this".to_owned()),
                Ident("is".to_owned()),
                Ident("a".to_owned()),
                Ident("sentence".to_owned())
            ]
        ));
}

#[test]
fn emits_number_token_when_given_number() {
    expect!("5".tokenize())
        .to(be_ok().value(vec![NumericConstant("5".to_owned())]));
}

#[test]
fn emits_number_token_when_given_number_with_float_point() {
    expect!("2.01".tokenize())
        .to(be_ok().value(vec![NumericConstant("2.01".to_owned())]));
}

#[test]
fn emits_error_when_given_number_with_two_delimeters() {
    expect!("2.0.1".tokenize())
        .to(be_err().value("Number format error"));
}

#[test]
fn escapes_single_quote_inside_string_token() {
    expect!("\'str\'\'str\'".tokenize())
        .to(be_ok().value(vec![CharactersConstant("str\'str".to_owned())]));
}

#[test]
fn escapes_new_line_chars() {
    expect!("\nword".tokenize())
        .to(be_ok().value(vec![Ident("word".to_owned())]));
}

#[test]
fn escapes_tabs() {
    expect!("\tword".tokenize())
        .to(be_ok().value(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_error_when_string_token_is_not_closed() {
    expect!("\'str".tokenize())
        .to(be_err().value("string const should be closed by \'".to_owned()));
}

#[test]
fn case_insensitive() {
    expect!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".tokenize())
        .to(be_ok().value(vec![Ident("abcdefghijklmnopqrstuvwxyz".to_owned())]));
}
