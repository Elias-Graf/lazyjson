use crate::tokenizer::Token;
use std::{error::Error, fmt};

#[derive(PartialEq, Eq, Debug)]
pub enum TreebuilderErrTyp {
    NotAKey,
    NotASep,
    NotAVal,
    NotAnAssignmentOp,
    OutOfBounds,
    TrailingSep,
    UnknownKwd,
    UnterminatedArr,
    UnterminatedObj,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreebuilderErr {
    pub typ: TreebuilderErrTyp,
    pub from: usize,
    pub to: usize,
}

impl fmt::Display for TreebuilderErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TreebuilderErr {}

impl TreebuilderErr {
    pub fn new_not_a_key(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAKey,
            from: i,
            to: i + i,
        }
    }
    pub fn new_not_a_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotASep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_a_val(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAVal,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_not_an_assignment_op(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::NotAnAssignmentOp,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_trailing_sep(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::TrailingSep,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_out_of_bounds() -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::OutOfBounds,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
    pub fn new_unknown_kwd(i: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnknownKwd,
            from: i,
            to: i + 1,
        }
    }
    pub fn new_unterminated_arr(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedArr,
            from,
            to,
        }
    }
    pub fn new_unterminated_obj(from: usize, to: usize) -> TreebuilderErr {
        TreebuilderErr {
            typ: TreebuilderErrTyp::UnterminatedObj,
            from,
            to,
        }
    }

    pub fn gen_msg(&self, toks: &[Token]) -> String {
        match self.typ {
            TreebuilderErrTyp::NotAKey => {
                format!("expected a key but found {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::NotASep => format!(
                "expected a separator but found {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::NotAVal => format!(
                "expected a valid value[composition] but found {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::NotAnAssignmentOp => format!(
                "expected the assignment operator ':' but received {:?}",
                &toks[self.from..self.to]
            ),
            TreebuilderErrTyp::TrailingSep => {
                format!("found trailing separator which is not allowed")
            }
            TreebuilderErrTyp::OutOfBounds => {
                format!("tried to consume a none value - reached the end of the tokens")
            }
            TreebuilderErrTyp::UnknownKwd => {
                format!("unknown keyword {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::UnterminatedArr => {
                format!("unterminated array {:?}", &toks[self.from..self.to])
            }
            TreebuilderErrTyp::UnterminatedObj => {
                format!("unterminated object {:?}", &toks[self.from..self.to])
            }
        }
    }
}
