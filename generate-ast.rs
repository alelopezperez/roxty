fn main() {
    println!("hola");
    define_ast(vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right",
    ]);
}

fn define_ast(types: Vec<String>) {
    
}
