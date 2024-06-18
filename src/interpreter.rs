use crate::parser::ASTNode;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, nodes: Vec<ASTNode>) {
        for node in nodes {
            self.interpret_node(node);
        }
    }

    fn interpret_node(&self, node: ASTNode) {
        match node {
            ASTNode::Print(value) => println!("{}", value),
        }
    }
}