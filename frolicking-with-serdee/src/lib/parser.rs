use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum State {
    Beginning,
    OpenBrace(Box<State>, usize),
    SpecialStringOpen(Box<State>, usize),
    OpenParenthesis(Box<State>, usize),
    End(Box<State>),
}

impl State {
    pub fn prev(&self) -> State {
        match self {
            State::Beginning => State::Beginning,
            State::OpenBrace(state, _)
            | State::SpecialStringOpen(state, _)
            | State::OpenParenthesis(state, _)
            | State::End(state) => state.deref().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Brace(usize, usize),
    Parenthesis(usize, usize),
    SpecialString(usize, usize),
}

#[derive(Debug, Clone)]
pub struct Ast(Vec<AstNode>);

pub struct AstVisitor;

impl<'de> serde::de::Visitor<'de> for AstVisitor {
    type Value = Ast;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(parse(v.to_string()))
    }
}

impl<'de> serde::de::Deserialize<'de> for Ast {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_string(AstVisitor)
    }
}

impl Ast {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_node(&mut self, node: AstNode) {
        self.0.push(node);
    }
}

pub fn parse(source_code: String) -> Ast {
    let mut ast = Ast::new();
    let mut state = State::Beginning;
    let mut index = 0;
    let mut chars = source_code.as_str().chars();

    while let Some(char) = chars.next() {
        match char {
            '(' => state = State::OpenParenthesis(Box::new(state), index),
            ')' => {
                match state {
                    State::OpenParenthesis(_, start_index) => {
                        state = state.prev();
                        ast.add_node(AstNode::Parenthesis(start_index, index));
                    }
                    _ => unreachable!(),
                };
            }
            '{' => {
                state = State::OpenBrace(Box::new(state), index);
            }
            '}' => {
                match state {
                    State::OpenBrace(_, start_index) => {
                        state = state.prev();
                        ast.add_node(AstNode::Brace(start_index, index))
                    }
                    _ => unreachable!(),
                };
            }
            '_' => match state {
                State::SpecialStringOpen(_, start_index) => {
                    ast.add_node(AstNode::SpecialString(start_index, index));
                    state = state.prev();
                }
                _ => {
                    state = State::SpecialStringOpen(Box::new(state), index);
                }
            },
            _ => (),
        }

        index += 1;
    }

    state = State::End(Box::new(state));

    ast
}
