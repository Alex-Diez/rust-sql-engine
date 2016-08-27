pub use expectest::prelude::{be_ok, be_err};

use sql::lexer::Tokenizer;
use sql::parser::Parser;
use sql::query_executer::QueryExecuter;

#[test]
fn it_creates_single_column_table() {
    let mut executer = QueryExecuter::default();
    expect!(executer.execute("create table table_name (col int);".tokenize().unwrap().parse().unwrap()))
        .to(be_ok().value("'table_name' was created".to_owned()));
}

#[test]
fn it_creates_a_table_with_list_of_columns() {
    let mut executer = QueryExecuter::default();
    expect!(executer.execute("create table table_name (col1 int, col2 int, col3 int);".tokenize().unwrap().parse().unwrap()))
        .to(be_ok().value("'table_name' was created".to_owned()));
}

#[test]
fn it_inserts_row_in_created_table() {
    let mut executer = QueryExecuter::default();
    executer.execute("create table table_name (col int);".tokenize().unwrap().parse().unwrap());

    expect!(executer.execute("insert into table_name values(1);".tokenize().unwrap().parse().unwrap()))
        .to(be_ok().value("row was inserted".to_owned()));
}

#[test]
fn it_inserts_row_in_table_with_many_columns() {
    let mut executer = QueryExecuter::default();
    executer.execute("create table table_name (col1 int, col2 int);".tokenize().unwrap().parse().unwrap());

    expect!(executer.execute("insert into table_name values(1, 2);".tokenize().unwrap().parse().unwrap()))
        .to(be_ok().value("row was inserted".to_owned()));
}

#[test]
fn it_does_not_insert_into_table_that_does_not_exist() {
    let mut executer = QueryExecuter::default();
    expect!(executer.execute("insert into table_name values(1);".tokenize().unwrap().parse().unwrap()))
        .to(be_err().value("[ERR 100] table 'table_name' does not exist".to_owned()));
}

#[test]
fn it_does_not_insert_when_column_type_does_not_match() {
    let mut executer = QueryExecuter::default();
    executer.execute("create table table_name (col int);".tokenize().unwrap().parse().unwrap());

    expect!(executer.execute("insert into table_name values('string');".tokenize().unwrap().parse().unwrap()))
        .to(be_err().value("column type is INT find VARCHAR".to_owned()));
}