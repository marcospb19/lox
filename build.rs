use std::{
    fs,
    io::{self, Write},
};

const DATA: &[(&str, &str, &str)] = &[
    (
        "BinaryExpression",
        "pub left: ExpressionBox, pub operator: Token, pub right: ExpressionBox",
        "left, operator, right",
    ),
    (
        "GroupingExpression",
        "pub expression: ExpressionBox",
        "expression",
    ),
    ("LiteralExpression", "pub value: Token", "value"),
    (
        "UnaryExpression",
        "pub operator: Token, pub expression: ExpressionBox",
        "operator, expression",
    ),
];

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    define_ast("expr", DATA)
}

fn define_ast(name: &str, list: &[(&str, &str, &str)]) -> io::Result<()> {
    let path = format!("src/{name}.rs");
    let mut writer = fs::File::create(&path)?;

    writeln!(writer, "#![cfg_attr(not(test), allow(unused))]")?;
    writeln!(writer)?;
    writeln!(writer, "use std::fmt;")?;
    writeln!(writer)?;
    writeln!(writer, "use crate::token::Token;")?;
    writeln!(writer)?;
    writeln!(writer, "pub trait Expression: fmt::Display {{}}")?;
    writeln!(writer)?;
    writeln!(writer, "impl Expression for BinaryExpression {{}}")?;
    writeln!(writer, "impl Expression for GroupingExpression {{}}")?;
    writeln!(writer, "impl Expression for LiteralExpression {{}}")?;
    writeln!(writer, "impl Expression for UnaryExpression {{}}")?;
    writeln!(writer)?;
    writeln!(writer, "type ExpressionBox = Box<dyn Expression>;")?;
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
    let typed_fields = typed_fields.replace("pub ", "");
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
