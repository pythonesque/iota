use super::Symbol;

use std::cell::Cell;
use std::rc::Rc;

pub type Pos = uint;

pub enum Var {
    Simple(Symbol),
}

pub enum Exp {
    Var(Var),
    Nil,
    /// Currently, offsets are always positive.
    Int(uint),
    String(::std::string::String, Pos),
    Call { func: Symbol, args: Vec<Exp>, pos: Pos },
    Op { left: Box<Exp>, oper: Oper, right: Box<Exp>, pos: Pos },
    Let { decs: Vec<Dec>, body: Box<Exp>, pos: Pos },
}

pub enum Dec {
    Function(Vec<FunDec>),
    Var { name: Symbol,
          escape: Rc<Cell<bool>>,
          typ: Option<(Symbol, Pos)>,
          init: Exp,
          pos: Pos },
    Type(Vec<TypeDec>),
}

pub struct TypeDec {
    pub name: Symbol,
    pub ty: Ty,
    pub pos: Pos,
}

pub enum Ty {
    Name(Symbol, Pos),
}

pub enum Oper {
    Plus,
    Minus,
}

pub struct Field {
    pub name: Symbol,
    pub escape: Rc<Cell<bool>>,
    pub typ: Symbol,
    pub pos: Pos,
}

pub struct FunDec {
    pub name: Symbol,
    pub params: Vec<Field>,
    pub result: Option<(Symbol,Pos)>,
    pub body: Exp,
    pub pos: Pos,
}
