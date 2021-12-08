pub struct Ast;

pub trait FromTokens {
    fn from_tokens(ast: &Ast) -> Self;
}

pub trait ToTokens {
    fn to_tokens(&self) -> Ast;
}
