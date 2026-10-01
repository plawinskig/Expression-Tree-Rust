#![allow(dead_code)]

#[derive(Debug)]
struct Value {
    val: i32,
}

#[derive(Debug)]
struct Variable {
    name: String,
    val: Option<i32>,
}

#[derive(Debug)]
enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum NodeType {
    Val(Value),
    Var(Variable),
    Op(Operation),
    Root
}

enum ExpressionTyped {
    Token(NodeType, Box<ExpressionTyped>),
    End,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    children: Option<Vec<Node>>,
}

impl Node {
    fn get_num_of_children(&self) -> i32 {
        match &self.node_type {
            NodeType::Root => 1,
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_text() {
        let correct_root = Node{ node_type: NodeType::Root,
            children: { Some(vec![
                Node { node_type: NodeType::Op(Operation{op_type: OperationType::Addition}),
                    children: { Some(vec![
                        Node { node_type: NodeType::Op(Operation { op_type: OperationType::Multiplication }),
                            children: Some(vec![
                                Node { node_type: NodeType::Val(Value{val: 1}),
                                    children: None
                                },
                                Node { node_type: NodeType::Val(Value{val: 2}),
                                    children: None
                                }
                            ])
                        },
                        Node { node_type: NodeType::Val(Value{val: 3}),
                            children: None
                        }
                    ])}
                }
            ])}
        };
        
        print!("{:#?}", correct_root);
        
        //assert_eq!(load_children("     56789"), Some(5));
    }
}
