#![allow(dead_code)]

struct Value {
    val: i32,
}

struct Variable {
    name: String,
    val: Option<i32>,
}

enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

struct Operation {
    op_type: OperationType,
}

impl Operation {
    fn get_num_of_args(&self) -> i32 {
        match self.op_type {
            OperationType::Addition
            | OperationType::Subtraction
            | OperationType::Multiplication
            | OperationType::Division => 2,
        }
    }
}

enum NodeType {
    Val(Value),
    Var(Variable),
    Op(Operation),
}
