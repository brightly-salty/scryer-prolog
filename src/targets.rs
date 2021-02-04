use crate::prolog_parser_rebis::ast::{Constant, RegType, Term};

use crate::clause_types::ClauseType;
use crate::forms::Level;
use crate::instructions::{FactInstruction, QueryInstruction};
use crate::iterators::{breadth_first_iter, post_order_iter, FactIterator, QueryIterator, TermRef};

pub trait CompilationTarget<'a> {
    type Iterator: Iterator<Item = TermRef<'a>>;

    fn iter(_: &'a Term) -> Self::Iterator;

    fn from_constant(_: Level, _: Constant, _: RegType) -> Self;
    fn from_list(_: Level, _: RegType) -> Self;
    fn from_structure(_: ClauseType, _: usize, _: RegType) -> Self;

    fn from_void(_: usize) -> Self;
    fn is_void_instr(&self) -> bool;

    fn from_pstr(lvl: Level, string: String, r: RegType, has_tail: bool) -> Self;

    fn incr_void_instr(&mut self);

    fn constant_subterm(_: Constant) -> Self;

    fn argument_to_variable(_: RegType, _: usize) -> Self;
    fn argument_to_value(_: RegType, _: usize) -> Self;

    fn move_to_register(_: RegType, _: usize) -> Self;

    fn subterm_to_variable(_: RegType) -> Self;
    fn subterm_to_value(_: RegType) -> Self;

    fn clause_arg_to_instr(_: RegType) -> Self;
}

impl<'a> CompilationTarget<'a> for FactInstruction {
    type Iterator = FactIterator<'a>;

    fn iter(term: &'a Term) -> Self::Iterator {
        breadth_first_iter(term, false) // do not iterate over the root clause if one exists.
    }

    fn from_constant(lvl: Level, constant: Constant, reg: RegType) -> Self {
        FactInstruction::GetConstant(lvl, constant, reg)
    }

    fn from_structure(ct: ClauseType, arity: usize, reg: RegType) -> Self {
        FactInstruction::GetStructure(ct, arity, reg)
    }

    fn from_list(lvl: Level, reg: RegType) -> Self {
        FactInstruction::GetList(lvl, reg)
    }

    fn from_void(subterms: usize) -> Self {
        FactInstruction::UnifyVoid(subterms)
    }

    fn is_void_instr(&self) -> bool {
        matches!(self, FactInstruction::UnifyVoid(_))
    }

    fn from_pstr(lvl: Level, string: String, r: RegType, has_tail: bool) -> Self {
        FactInstruction::GetPartialString(lvl, string, r, has_tail)
    }

    fn incr_void_instr(&mut self) {
        if let FactInstruction::UnifyVoid(ref mut incr) = *self {
            *incr += 1
        }
    }

    fn constant_subterm(constant: Constant) -> Self {
        FactInstruction::UnifyConstant(constant)
    }

    fn argument_to_variable(arg: RegType, val: usize) -> Self {
        FactInstruction::GetVariable(arg, val)
    }

    fn move_to_register(arg: RegType, val: usize) -> Self {
        FactInstruction::GetVariable(arg, val)
    }

    fn argument_to_value(arg: RegType, val: usize) -> Self {
        FactInstruction::GetValue(arg, val)
    }

    fn subterm_to_variable(val: RegType) -> Self {
        FactInstruction::UnifyVariable(val)
    }

    fn subterm_to_value(val: RegType) -> Self {
        FactInstruction::UnifyValue(val)
    }

    fn clause_arg_to_instr(val: RegType) -> Self {
        FactInstruction::UnifyVariable(val)
    }
}

impl<'a> CompilationTarget<'a> for QueryInstruction {
    type Iterator = QueryIterator<'a>;

    fn iter(term: &'a Term) -> Self::Iterator {
        post_order_iter(term)
    }

    fn from_structure(ct: ClauseType, arity: usize, r: RegType) -> Self {
        QueryInstruction::PutStructure(ct, arity, r)
    }

    fn from_constant(lvl: Level, constant: Constant, reg: RegType) -> Self {
        QueryInstruction::PutConstant(lvl, constant, reg)
    }

    fn from_list(lvl: Level, reg: RegType) -> Self {
        QueryInstruction::PutList(lvl, reg)
    }

    fn from_pstr(lvl: Level, string: String, r: RegType, has_tail: bool) -> Self {
        QueryInstruction::PutPartialString(lvl, string, r, has_tail)
    }

    fn from_void(subterms: usize) -> Self {
        QueryInstruction::SetVoid(subterms)
    }

    fn is_void_instr(&self) -> bool {
        matches!(self, &QueryInstruction::SetVoid(_))
    }

    fn incr_void_instr(&mut self) {
        if let QueryInstruction::SetVoid(ref mut incr) = *self {
            *incr += 1
        }
    }

    fn constant_subterm(constant: Constant) -> Self {
        QueryInstruction::SetConstant(constant)
    }

    fn argument_to_variable(arg: RegType, val: usize) -> Self {
        QueryInstruction::PutVariable(arg, val)
    }

    fn move_to_register(arg: RegType, val: usize) -> Self {
        QueryInstruction::GetVariable(arg, val)
    }

    fn argument_to_value(arg: RegType, val: usize) -> Self {
        QueryInstruction::PutValue(arg, val)
    }

    fn subterm_to_variable(val: RegType) -> Self {
        QueryInstruction::SetVariable(val)
    }

    fn subterm_to_value(val: RegType) -> Self {
        QueryInstruction::SetValue(val)
    }

    fn clause_arg_to_instr(val: RegType) -> Self {
        QueryInstruction::SetValue(val)
    }
}
