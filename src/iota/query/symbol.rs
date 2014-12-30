use std::collections::hash_map::{HashMap, Entry};
use std::kinds::marker;
use std::num::Int;

#[deriving(Copy, PartialEq, Eq, Hash, Show)]
pub enum Mark {
    Cursor(uint),           //For keeping track of cursors.
    DisplayMark(uint),      //For using in determining some display of characters.
}

type S = u32;

#[deriving(PartialEq, Eq, Hash, Show)]
pub struct Symbol(Mark, S);

struct Symbols<'a> {
    next_sym: S,
    symbols: HashMap<Mark, S>,
}

impl<'a> Symbols<'a> {
    pub fn new() -> Symbols<'a> {
        Symbols {
            next_sym: 0,
            symbols: HashMap::new(),
        }
    }

    /// Taking self is future proofing (if we need to shrink variable sizes).
    pub fn name<'b,'c>(&'b self, symbol: &'c Symbol) -> &'c Mark {
        &symbol.0
    }

    pub fn symbol(&mut self, name: Mark) -> Result<Symbol, ()> {
        Ok(match self.symbols.entry(name) {
            Entry::Occupied(o) => Symbol(name, *o.get()),
            Entry::Vacant(v) => {
                let i = self.next_sym;
                self.next_sym = match i.checked_add(Int::one()) {
                    Some(i) => i,
                    None => return Err(())
                };
                Symbol(name, *v.set(i))
            },
        })
    }

    /// Taking self is future proofing (if we need to shrink variable sizes).
    pub fn empty<'b, 'c, T>(&'b self) -> Table<'c, T> {
        Table {
            table: HashMap::new(),
        }
    }
}

#[deriving(Clone)]
pub struct Table<'a, T> {
    table: HashMap<S, T>,
}

impl<'a, T> Table<'a, T> {
    pub fn enter(&mut self, k: &Symbol, v: T) -> Option<T> {
        self.table.insert(k.1, v)
    }

    pub fn look(&self, k: &Symbol) -> Option<&T> {
        self.table.get(&k.1)
    }
}
