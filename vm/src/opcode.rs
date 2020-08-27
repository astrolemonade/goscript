#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::fmt;

pub type OpIndex = i16;

pub const OP_ADD_VALUE: isize = 300;
pub const OP_SUB_VALUE: isize = 301;
pub const OP_MUL_VALUE: isize = 302;
pub const OP_QUO_VALUE: isize = 303;
pub const OP_REM_VALUE: isize = 304;
pub const OP_AND_VALUE: isize = 305;
pub const OP_OR_VALUE: isize = 306;
pub const OP_XOR_VALUE: isize = 307;
pub const OP_SHL_VALUE: isize = 308;
pub const OP_SHR_VALUE: isize = 309;
pub const OP_AND_NOT_VALUE: isize = 310;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    // push pop load store
    PUSH_CONST = 100,
    PUSH_NIL,
    PUSH_FALSE,
    PUSH_TRUE,
    PUSH_IMM,
    POP = 110,
    LOAD_LOCAL0 = 200,
    LOAD_LOCAL1,
    LOAD_LOCAL2,
    LOAD_LOCAL3,
    LOAD_LOCAL4,
    LOAD_LOCAL5,
    LOAD_LOCAL6,
    LOAD_LOCAL7,
    LOAD_LOCAL8,
    LOAD_LOCAL9,
    LOAD_LOCAL10,
    LOAD_LOCAL11,
    LOAD_LOCAL12,
    LOAD_LOCAL13,
    LOAD_LOCAL14,
    LOAD_LOCAL15,
    LOAD_LOCAL = 220,
    STORE_LOCAL,    // stores the value on the top of the stack to local
    STORE_LOCAL_NT, // stores the value that is not on the top of the stack
    STORE_LOCAL_OP, // stores value with an operation. for +=/-= etc.
    // STORE_INTERFACE
    LOAD_UPVALUE = 230,
    STORE_UPVALUE,
    STORE_UPVALUE_NT,
    STORE_UPVALUE_OP,
    LOAD_FIELD = 240,
    STORE_FIELD,
    STORE_FIELD_NT,
    STORE_FIELD_OP,
    LOAD_FIELD_IMM = 250,
    STORE_FIELD_IMM,
    STORE_FIELD_IMM_NT,
    STORE_FIELD_IMM_OP,
    LOAD_THIS_PKG_FIELD = 260,
    STORE_THIS_PKG_FIELD,
    STORE_THIS_PKG_FIELD_NT,
    STORE_THIS_PKG_FIELD_OP,
    STORE_DEREF = 270,
    STORE_DEREF_NT,
    STORE_DEREF_OP,

    // arithmetic, logical, ref, deref, arrow
    ADD = OP_ADD_VALUE,         // +
    SUB = OP_SUB_VALUE,         // -
    MUL = OP_MUL_VALUE,         // *
    QUO = OP_QUO_VALUE,         // /
    REM = OP_REM_VALUE,         // %
    AND = OP_AND_VALUE,         // &
    OR = OP_OR_VALUE,           // |
    XOR = OP_XOR_VALUE,         // ^
    SHL = OP_SHL_VALUE,         // <<
    SHR = OP_SHR_VALUE,         // >>
    AND_NOT = OP_AND_NOT_VALUE, // $^
    UNARY_ADD,                  // +
    UNARY_SUB,                  // -
    UNARY_XOR,                  // ^
    REF,                        // &
    DEREF,                      // *
    ARROW,                      // <-
    NOT,                        // !
    EQL,                        // ==
    LSS,                        // <
    GTR,                        // >
    NEQ,                        // !=
    LEQ,                        // <=
    GEQ,                        // >=

    // call
    PRE_CALL = 400,
    CALL,
    CALL_ELLIPSIS, // call with the past parameter followed by ellipsis
    CLOSE_UPVALUE,
    RETURN,
    RETURN_INIT_PKG,

    // jump
    JUMP = 500,
    JUMP_IF,
    JUMP_IF_NOT,
    LOOP,
    RANGE, // for ... range statement

    // built-in functinalities
    IMPORT = 600, // imports a package
    SLICE,        //for slice expressions
    SLICE_FULL,   // for full slice expressions
    NEW,          // for built-in function new
    MAKE,         // for built-in function make
    LEN,          // for built-in function len
    CAP,          // for built-in function cap
    APPEND,       //for built-in function append
    ASSERT,       //for built-in function assert
}

impl Opcode {
    pub fn get_load_local(i: OpIndex) -> Opcode {
        match i {
            0 => Opcode::LOAD_LOCAL0,
            1 => Opcode::LOAD_LOCAL1,
            2 => Opcode::LOAD_LOCAL2,
            3 => Opcode::LOAD_LOCAL3,
            4 => Opcode::LOAD_LOCAL4,
            5 => Opcode::LOAD_LOCAL5,
            6 => Opcode::LOAD_LOCAL6,
            7 => Opcode::LOAD_LOCAL7,
            8 => Opcode::LOAD_LOCAL8,
            9 => Opcode::LOAD_LOCAL9,
            10 => Opcode::LOAD_LOCAL10,
            11 => Opcode::LOAD_LOCAL11,
            12 => Opcode::LOAD_LOCAL12,
            13 => Opcode::LOAD_LOCAL13,
            14 => Opcode::LOAD_LOCAL14,
            15 => Opcode::LOAD_LOCAL15,
            _ => Opcode::LOAD_LOCAL,
        }
    }

    #[inline]
    pub fn offset(&self, base: Opcode) -> OpIndex {
        (*self as i16 - base as i16) as OpIndex
    }

    pub fn property(&self) -> (&str, i8) {
        match self {
            Opcode::PUSH_CONST => ("PUSH_CONST", 1),
            Opcode::PUSH_NIL => ("PUSH_NIL", 1),
            Opcode::PUSH_FALSE => ("PUSH_FALSE", 1),
            Opcode::PUSH_TRUE => ("PUSH_TRUE", 1),
            Opcode::PUSH_IMM => ("PUSH_IMM", 1),
            Opcode::POP => ("POP", -1),
            Opcode::LOAD_LOCAL0 => ("LOAD_LOCAL0", 1),
            Opcode::LOAD_LOCAL1 => ("LOAD_LOCAL1", 1),
            Opcode::LOAD_LOCAL2 => ("LOAD_LOCAL2", 1),
            Opcode::LOAD_LOCAL3 => ("LOAD_LOCAL3", 1),
            Opcode::LOAD_LOCAL4 => ("LOAD_LOCAL4", 1),
            Opcode::LOAD_LOCAL5 => ("LOAD_LOCAL5", 1),
            Opcode::LOAD_LOCAL6 => ("LOAD_LOCAL6", 1),
            Opcode::LOAD_LOCAL7 => ("LOAD_LOCAL7", 1),
            Opcode::LOAD_LOCAL8 => ("LOAD_LOCAL8", 1),
            Opcode::LOAD_LOCAL9 => ("LOAD_LOCAL9", 1),
            Opcode::LOAD_LOCAL10 => ("LOAD_LOCAL10", 1),
            Opcode::LOAD_LOCAL11 => ("LOAD_LOCAL11", 1),
            Opcode::LOAD_LOCAL12 => ("LOAD_LOCAL12", 1),
            Opcode::LOAD_LOCAL13 => ("LOAD_LOCAL13", 1),
            Opcode::LOAD_LOCAL14 => ("LOAD_LOCAL14", 1),
            Opcode::LOAD_LOCAL15 => ("LOAD_LOCAL15", 1),
            Opcode::LOAD_LOCAL => ("LOAD_LOCAL", 1),
            Opcode::STORE_LOCAL => ("STORE_LOCAL", 0),
            Opcode::STORE_LOCAL_NT => ("STORE_LOCAL_NT", 0),
            Opcode::STORE_LOCAL_OP => ("STORE_LOCAL_OP", 0),
            Opcode::LOAD_UPVALUE => ("LOAD_LOCAL", 1),
            Opcode::STORE_UPVALUE => ("STORE_UPVALUE", 0),
            Opcode::STORE_UPVALUE_NT => ("STORE_UPVALUE_NT", 0),
            Opcode::STORE_UPVALUE_OP => ("STORE_UPVALUE_OP", 0),
            Opcode::LOAD_FIELD => ("LOAD_FIELD", -1),
            Opcode::STORE_FIELD => ("STORE_FIELD", 0),
            Opcode::STORE_FIELD_NT => ("STORE_FIELD_NT", 0),
            Opcode::STORE_FIELD_OP => ("STORE_FIELD_OP", 0),
            Opcode::LOAD_FIELD_IMM => ("LOAD_FIELD_IMM", 0),
            Opcode::STORE_FIELD_IMM => ("STORE_FIELD_IMM", 0),
            Opcode::STORE_FIELD_IMM_NT => ("STORE_FIELD_IMM_NT", 0),
            Opcode::STORE_FIELD_IMM_OP => ("STORE_FIELD_IMM_OP", 0),
            Opcode::LOAD_THIS_PKG_FIELD => ("LOAD_THIS_PKG_FIELD", -1),
            Opcode::STORE_THIS_PKG_FIELD => ("STORE_THIS_PKG_FIELD", 0),
            Opcode::STORE_THIS_PKG_FIELD_NT => ("STORE_THIS_PKG_FIELD_NT", 0),
            Opcode::STORE_THIS_PKG_FIELD_OP => ("STORE_THIS_PKG_FIELD_OP", 0),
            Opcode::STORE_DEREF => ("STORE_DEREF", 0),
            Opcode::STORE_DEREF_NT => ("STORE_DEREF_NT", 0),
            Opcode::STORE_DEREF_OP => ("STORE_DEREF_OP", 0),

            Opcode::ADD => ("ADD", -1),
            Opcode::SUB => ("SUB", -1),
            Opcode::MUL => ("MUL", -1),
            Opcode::QUO => ("QUO", -1),
            Opcode::REM => ("REM", -1),
            Opcode::AND => ("AND", -1),
            Opcode::OR => ("OR", -1),
            Opcode::XOR => ("XOR", -1),
            Opcode::SHL => ("SHL", -1),
            Opcode::SHR => ("SHR", -1),
            Opcode::AND_NOT => ("AND_NOT", -1),
            Opcode::UNARY_ADD => ("UNARY_ADD", 0),
            Opcode::UNARY_SUB => ("UNARY_SUB", 0),
            Opcode::UNARY_XOR => ("UNARY_XOR", 0),
            Opcode::REF => ("REF", 0),
            Opcode::DEREF => ("DEREF", 0),
            Opcode::ARROW => ("ARROW", 0),
            Opcode::NOT => ("LNOT", 0),
            Opcode::EQL => ("EQL", -1),
            Opcode::LSS => ("LSS", -1),
            Opcode::GTR => ("GTR", -1),
            Opcode::NEQ => ("NEQ", -1),
            Opcode::LEQ => ("LEQ", -1),
            Opcode::GEQ => ("GEQ", -1),

            Opcode::PRE_CALL => ("PRE_CALL", -128),
            Opcode::CALL => ("CALL", -128),
            Opcode::CALL_ELLIPSIS => ("CALL_ELLIPSIS", -128),
            Opcode::CLOSE_UPVALUE => ("CLOSE_UPVALUE", -1),
            Opcode::RETURN => ("RETURN", -128),
            Opcode::RETURN_INIT_PKG => ("RETURN_INIT_PKG", -128),

            Opcode::JUMP => ("JUMP", 0),
            Opcode::LOOP => ("LOOP", 0),
            Opcode::JUMP_IF => ("JUMP_IF", -1),
            Opcode::JUMP_IF_NOT => ("JUMP_IF_NOT", -1),
            Opcode::RANGE => ("RANGE", 1),

            Opcode::IMPORT => ("IMPORT", 0),
            Opcode::SLICE => ("SLICE", -2),
            Opcode::SLICE_FULL => ("SLICE_FULL", -3),
            Opcode::NEW => ("NEW", 0),
            Opcode::MAKE => ("MAKE", 0),
            Opcode::LEN => ("LEN", 0),
            Opcode::CAP => ("CAP", 0),
            Opcode::APPEND => ("APPEND", -128),
            Opcode::ASSERT => ("ASSERT", 0),
        }
    }

    pub fn text(&self) -> &str {
        let (t, _) = self.property();
        t
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (t, _) = self.property();
        write!(f, "OPCODE: {}", t)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CodeData {
    Code(Opcode),
    Data(OpIndex),
}

impl CodeData {
    pub fn unwrap_code(&self) -> &Opcode {
        match self {
            CodeData::Code(code) => code,
            CodeData::Data(_) => unreachable!(),
        }
    }

    pub fn unwrap_data(&self) -> &OpIndex {
        match self {
            CodeData::Code(_) => unreachable!(),
            CodeData::Data(data) => data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcode() {
        println!("opcode {} \n", Opcode::POP);
    }
}
