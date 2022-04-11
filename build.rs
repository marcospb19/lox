#![allow(unused)]
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

const ASD: &[(&str, &str, &str)] = &[
    (
        "BinaryExpression",
        "left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>",
        "left, operator, right",
    ),
    (
        "GroupingExpression",
        "expression: Box<dyn Expression>",
        "expression",
    ),
    ("LiteralExpression", "value: TokenType", "value"),
    (
        "UnaryExpression",
        "operator: Token, expression: Box<dyn Expression>",
        "operator, expression",
    ),
];

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    define_ast("expr", ASD)
}

fn define_ast(name: &str, list: &[(&str, &str, &str)]) -> io::Result<()> {
    let path = format!("src/{name}.rs");
    let mut writer = fs::File::create(&path)?;

    writeln!(writer, "use crate::token::{{Token, TokenType}};")?;
    writeln!(writer)?;
    writeln!(writer, "pub trait Expression {{}}")?;
    writeln!(writer)?;

    for (struct_name, typed_fields, untyped_fields) in list {
        define_struct(&mut writer, struct_name, typed_fields, untyped_fields)?;
    }
    Ok(())
}

fn define_struct(
    writer: &mut impl Write,
    struct_name: &str,
    typed_fields: &str,
    untyped_fields: &str,
) -> io::Result<()> {
    writeln!(writer, "pub struct {struct_name} {{")?;
    writeln!(writer, "    {typed_fields}")?;
    writeln!(writer, "}}")?;
    writeln!(writer)?;
    writeln!(writer, "impl {struct_name} {{")?;
    writeln!(writer, "    pub fn new({typed_fields}) -> Self {{")?;
    writeln!(writer, "        Self {{ {untyped_fields} }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;
    writeln!(writer)?;

    Ok(())
}

// fn create_dir_if_not_existent(path: impl AsRef<Path>) {
//     let path = path.as_ref();
//     if !path.exists() {
//         fs::create_dir_all(path).unwrap()
//     }
// }
