#![allow(dead_code)]

use crate::expression::Expression;

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
        match &self.op_type {
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

struct Node<'a> {
    node_type: NodeType,
    children: Vec<&'a Node<'a>>,
}

impl Node<'_> {
    fn get_num_of_children(&self) -> i32 {
        match &self.node_type {
            NodeType::Val(_) | NodeType::Var(_) => 0,
            NodeType::Op(operation) => operation.get_num_of_args(),
        }
    }

    fn load_node_and_children(&self, expression: Expression) -> Result<(), String> {
        


        Ok(())
    }
}
