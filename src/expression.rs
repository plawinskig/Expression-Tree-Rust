
enum Expression {
    Token(String, Box<Expression>),
    End,
}
