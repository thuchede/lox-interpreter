use std::fs::File;
use std::io::Write;
fn main() {
    let mut buffer = File::create("src/expression.rs").unwrap();
    define_ast(vec![
        "Binary : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal : Any value",
        "Unary : Token operator, Expr right",
    ], &mut buffer);
}

fn define_ast(types: Vec<&str>, buffer: &mut File) {
    // header and enum for Expr
    buffer.write_all("use std::any::Any;\nuse crate::token::Token;\n".as_bytes()).unwrap();
    buffer.write_all("enum Expr {\n".as_bytes()).unwrap();
    for x in types.clone() {
        let (rule, _) = x.split_once(" : ").unwrap();
        buffer.write_all(format!("\t{rule}({rule}),\n").as_bytes()).unwrap();
    }
    buffer.write_all("}\n\n".as_bytes()).unwrap();

    // all structs for rules
    for x in types {
        let (rule, producer) = x.split_once(" : ").unwrap();
        let rule_struct_start = format!("struct {rule} {{\n");
        buffer.write_all(rule_struct_start.as_bytes()).unwrap();

        for x in producer.split(", ") {
            let (struct_fieldtype, struct_field) = x.split_once(" ").unwrap();

            let wrapped_type = if struct_fieldtype == "Expr" {
                format!("Box<{struct_fieldtype}>")
            } else if struct_fieldtype == "Any" {
                format!("Box<dyn {struct_fieldtype}>")
            } else {
                struct_fieldtype.to_string()
            };

            let field = format!("\t{struct_field}: {wrapped_type},\n");
            buffer.write_all(field.as_bytes()).unwrap();
        }

        let rule_struct_end = format!("}}\n\n");
        buffer.write_all(rule_struct_end.as_bytes()).unwrap();

        // all impl of constructor for rules
        let impl_start = format!("impl {} {{\n", rule);
        buffer.write_all(impl_start.as_bytes()).unwrap();
        buffer.write_all("\tfn new(".as_bytes()).unwrap();
        for x in producer.split(", ") {
            let (struct_fieldtype, struct_field) = x.split_once(" ").unwrap();
            let wrapped_type = if struct_fieldtype == "Expr" {
                format!("Box<{struct_fieldtype}>")
            } else if struct_fieldtype == "Any" {
                format!("Box<dyn {struct_fieldtype}>")
            } else {
                struct_fieldtype.to_string()
            };
            let field = format!("{struct_field}: {wrapped_type}, ");
            buffer.write_all(field.as_bytes()).unwrap();
        }
        buffer.write_all(") -> Self {\n".as_bytes()).unwrap();
        buffer.write_all(format!("\t\t{} {{\n", rule).as_bytes()).unwrap();
        for x in producer.split(", ") {
            let (_, struct_field) = x.split_once(" ").unwrap();
            // let field = if struct_fieldtype == "Expr" || struct_fieldtype == "Any" { format!("\t\t\t{struct_field}: Box::new({struct_field}),\n")} else { format!("\t\t\t{struct_field},\n")};
            buffer.write_all(format!("\t\t\t{struct_field},\n").as_bytes()).unwrap();
        }
        buffer.write_all("\t\t}\n".as_bytes()).unwrap();
        buffer.write_all("\t}\n".as_bytes()).unwrap();
        buffer.write_all("}\n\n".as_bytes()).unwrap();
    }
}