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

enum ExpressionTyped {
    Token(NodeType, Box<ExpressionTyped>),
    End,
}

struct Node {
    node_type: NodeType,
    children: Option<Vec<Node>>,
}

impl Node {
    fn get_num_of_children(&self) -> i32 {
        match &self.node_type {
            NodeType::Val(_) | NodeType::Var(_) => 0,
            NodeType::Op(operation) => operation.get_num_of_args(),
        }
    }

    fn load_children(
        &mut self,
        mut expression: ExpressionTyped,
    ) -> Result<ExpressionTyped, String> {
        for _ in 0..self.get_num_of_children() {
            match expression {
                ExpressionTyped::End => return Ok(ExpressionTyped::End),
                ExpressionTyped::Token(head, box_tail) => {
                    let tail = *box_tail;
                    let mut child = Node {
                        node_type: head,
                        children: None,
                    };
                    expression = child.load_children(tail)?;

                    self.children.get_or_insert_with(Vec::new).push(child);
                }
            }
        }
        Ok(expression)
    }
}
