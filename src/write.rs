use crate::clause_types::{
    ArithmeticTerm, ClauseType, CompareNumberQT, CompareTermQT, SystemClauseType,
};
use crate::forms::{Level, ModuleSource, Number};
use crate::indexing::IndexingCodePtr;
use crate::instructions::{
    ArithmeticInstruction, ChoiceInstruction, ControlInstruction, CutInstruction, FactInstruction,
    IndexedChoiceInstruction, IndexingInstruction, IndexingLine, Line, QueryInstruction,
};
use crate::machine::machine_errors::{ExistenceError, SessionError};
use crate::machine::machine_indices::{
    Addr, DBRef, HeapCellValue, IndexPtr, LocalCodePtr, REPLCodePtr,
};

use std::fmt;

impl fmt::Display for LocalCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalCodePtr::DirEntry(p) => write!(f, "LocalCodePtr::DirEntry({})", p),
            LocalCodePtr::Halt => write!(f, "LocalCodePtr::Halt"),
            LocalCodePtr::IndexingBuf(p, o, i) => {
                write!(f, "LocalCodePtr::IndexingBuf({}, {}, {})", p, o, i)
            }
        }
    }
}

impl fmt::Display for REPLCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            REPLCodePtr::AddDynamicPredicate => write!(f, "REPLCodePtr::AddDynamicPredicate"),
            REPLCodePtr::AddGoalExpansionClause => write!(f, "REPLCodePtr::AddGoalExpansionClause"),
            REPLCodePtr::AddTermExpansionClause => write!(f, "REPLCodePtr::AddTermExpansionClause"),
            REPLCodePtr::BuiltInProperty => write!(f, "REPLCodePtr::BuiltInProperty"),
            REPLCodePtr::UserAssertz => write!(f, "REPLCodePtr::UserAssertz"),
            REPLCodePtr::UserAsserta => write!(f, "REPLCodePtr::UserAsserta"),
            REPLCodePtr::UserRetract => write!(f, "REPLCodePtr::UserRetract"),
            REPLCodePtr::ClauseToEvacuable => write!(f, "REPLCodePtr::ClauseToEvacuable"),
            REPLCodePtr::ConcludeLoad => write!(f, "REPLCodePtr::ConcludeLoad"),
            REPLCodePtr::DeclareModule => write!(f, "REPLCodePtr::DeclareModule"),
            REPLCodePtr::LoadCompiledLibrary => write!(f, "REPLCodePtr::LoadCompiledLibrary"),
            REPLCodePtr::LoadContextSource => write!(f, "REPLCodePtr::LoadContextSource"),
            REPLCodePtr::LoadContextFile => write!(f, "REPLCodePtr::LoadContextFile"),
            REPLCodePtr::LoadContextDirectory => write!(f, "REPLCodePtr::LoadContextDirectory"),
            REPLCodePtr::LoadContextModule => write!(f, "REPLCodePtr::LoadContextModule"),
            REPLCodePtr::LoadContextStream => write!(f, "REPLCodePtr::LoadContextStream"),
            REPLCodePtr::PopLoadContext => write!(f, "REPLCodePtr::PopLoadContext"),
            REPLCodePtr::PopLoadStatePayload => write!(f, "REPLCodePtr::PopLoadStatePayload"),
            REPLCodePtr::PushLoadContext => write!(f, "REPLCodePtr::PushLoadContext"),
            REPLCodePtr::PushLoadStatePayload => write!(f, "REPLCodePtr::PushLoadStatePayload"),
            REPLCodePtr::UseModule => write!(f, "REPLCodePtr::UseModule"),
            REPLCodePtr::MetaPredicateProperty => write!(f, "REPLCodePtr::MetaPredicateProperty"),
            REPLCodePtr::CompilePendingPredicates => {
                write!(f, "REPLCodePtr::CompilePendingPredicates")
            }
        }
    }
}

impl fmt::Display for IndexPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::DynamicUndefined | Self::Undefined => write!(f, "undefined"),
            Self::Index(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Display for FactInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::GetConstant(lvl, ref constant, ref r) => {
                write!(f, "get_constant {}, {}{}", constant, lvl, r.reg_num())
            }
            Self::GetList(lvl, ref r) => {
                write!(f, "get_list {}{}", lvl, r.reg_num())
            }
            Self::GetPartialString(lvl, ref s, r, has_tail) => {
                write!(f, "get_partial_string({}, {}, {}, {})", lvl, s, r, has_tail)
            }
            Self::GetStructure(ref ct, ref arity, ref r) => {
                write!(f, "get_structure {}/{}, {}", ct.name(), arity, r)
            }
            Self::GetValue(ref x, ref a) => {
                write!(f, "get_value {}, A{}", x, a)
            }
            Self::GetVariable(ref x, ref a) => {
                write!(f, "fact:get_variable {}, A{}", x, a)
            }
            Self::UnifyConstant(ref constant) => {
                write!(f, "unify_constant {}", constant)
            }
            Self::UnifyVariable(ref r) => {
                write!(f, "unify_variable {}", r)
            }
            Self::UnifyLocalValue(ref r) => {
                write!(f, "unify_local_value {}", r)
            }
            Self::UnifyValue(ref r) => {
                write!(f, "unify_value {}", r)
            }
            Self::UnifyVoid(n) => {
                write!(f, "unify_void {}", n)
            }
        }
    }
}

impl fmt::Display for QueryInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::GetVariable(ref x, ref a) => {
                write!(f, "query:get_variable {}, A{}", x, a)
            }
            Self::PutConstant(lvl, ref constant, ref r) => {
                write!(f, "put_constant {}, {}{}", constant, lvl, r.reg_num())
            }
            Self::PutList(lvl, ref r) => {
                write!(f, "put_list {}{}", lvl, r.reg_num())
            }
            Self::PutPartialString(lvl, ref s, r, has_tail) => {
                write!(f, "put_partial_string({}, {}, {}, {})", lvl, s, r, has_tail)
            }
            Self::PutStructure(ref ct, ref arity, ref r) => {
                write!(f, "put_structure {}/{}, {}", ct.name(), arity, r)
            }
            Self::PutUnsafeValue(y, a) => write!(f, "put_unsafe_value Y{}, A{}", y, a),
            Self::PutValue(ref x, ref a) => write!(f, "put_value {}, A{}", x, a),
            Self::PutVariable(ref x, ref a) => write!(f, "put_variable {}, A{}", x, a),
            Self::SetConstant(ref constant) => write!(f, "set_constant {}", constant),
            Self::SetLocalValue(ref r) => write!(f, "set_local_value {}", r),
            Self::SetVariable(ref r) => write!(f, "set_variable {}", r),
            Self::SetValue(ref r) => write!(f, "set_value {}", r),
            Self::SetVoid(n) => write!(f, "set_void {}", n),
        }
    }
}

impl fmt::Display for CompareNumberQT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::NotEqual => write!(f, "=\\="),
            Self::Equal => write!(f, "=:="),
        }
    }
}

impl fmt::Display for CompareTermQT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::GreaterThan => write!(f, "@>"),
            Self::GreaterThanOrEqual => write!(f, "@>="),
            Self::LessThan => write!(f, "@<"),
            Self::LessThanOrEqual => write!(f, "@<="),
        }
    }
}

impl fmt::Display for ClauseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::System(SystemClauseType::SetCutPoint(r)) => {
                write!(f, "$set_cp({})", r)
            }
            Self::Named(ref name, _, ref idx) | Self::Op(ref name, _, ref idx) => {
                let idx = idx.0.get();
                write!(f, "{}/{}", name, idx)
            }
            ref ct => {
                write!(f, "{}", ct.name())
            }
        }
    }
}

impl fmt::Display for HeapCellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Addr(ref addr) => write!(f, "{}", addr),
            Self::Atom(ref atom, _) => write!(f, "{}", atom.as_str()),
            Self::DBRef(ref db_ref) => write!(f, "{}", db_ref),
            Self::Integer(ref n) => write!(f, "{}", n),
            Self::LoadStatePayload(_) => write!(f, "LoadStatePayload"),
            Self::Rational(ref n) => write!(f, "{}", n),
            Self::NamedStr(arity, ref name, Some(ref cell)) => write!(
                f,
                "{}/{} (op, priority: {}, spec: {})",
                name.as_str(),
                arity,
                cell.prec(),
                cell.assoc()
            ),
            Self::NamedStr(arity, ref name, None) => {
                write!(f, "{}/{}", name.as_str(), arity)
            }
            Self::PartialString(ref pstr, has_tail) => {
                write!(
                    f,
                    "pstr ( buf: \"{}\", has_tail({}) )",
                    pstr.as_str_from(0),
                    has_tail,
                )
            }
            Self::Stream(ref stream) => {
                write!(f, "$stream({})", stream.as_ptr() as usize)
            }
            Self::TcpListener(ref tcp_listener) => {
                write!(f, "$tcp_listener({})", tcp_listener.local_addr().unwrap())
            }
        }
    }
}

impl fmt::Display for DBRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NamedPred(ref name, arity, _) => write!(f, "db_ref:named:{}/{}", name, arity),
            Self::Op(priority, spec, ref name, ..) => {
                write!(f, "db_ref:op({}, {}, {})", priority, spec, name)
            }
        }
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Char(c) => write!(f, "Addr::Char({})", c),
            Self::EmptyList => write!(f, "Addr::EmptyList"),
            Self::Fixnum(n) => write!(f, "Addr::Fixnum({})", n),
            Self::Float(fl) => write!(f, "Addr::Float({})", fl),
            Self::CutPoint(cp) => write!(f, "Addr::CutPoint({})", cp),
            Self::Con(ref c) => write!(f, "Addr::Con({})", c),
            Self::Lis(l) => write!(f, "Addr::Lis({})", l),
            Self::LoadStatePayload(s) => write!(f, "Addr::LoadStatePayload({})", s),
            Self::AttrVar(h) => write!(f, "Addr::AttrVar({})", h),
            Self::HeapCell(h) => write!(f, "Addr::HeapCell({})", h),
            Self::StackCell(fr, sc) => write!(f, "Addr::StackCell({}, {})", fr, sc),
            Self::Str(s) => write!(f, "Addr::Str({})", s),
            Self::PStrLocation(h, n) => write!(f, "Addr::PStrLocation({}, {})", h, n),
            Self::Stream(stream) => write!(f, "Addr::Stream({})", stream),
            Self::TcpListener(tcp_listener) => write!(f, "Addr::TcpListener({})", tcp_listener),
            Self::Usize(cp) => write!(f, "Addr::Usize({})", cp),
        }
    }
}

impl fmt::Display for ControlInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Allocate(num_cells) => write!(f, "allocate {}", num_cells),
            Self::CallClause(ref ct, arity, pvs, true, true) => {
                write!(f, "call_with_default_policy {}/{}, {}", ct, arity, pvs)
            }
            Self::CallClause(ref ct, arity, pvs, false, true) => {
                write!(f, "execute_with_default_policy {}/{}, {}", ct, arity, pvs)
            }
            Self::CallClause(ref ct, arity, pvs, true, false) => {
                write!(f, "execute {}/{}, {}", ct, arity, pvs)
            }
            Self::CallClause(ref ct, arity, pvs, false, false) => {
                write!(f, "call {}/{}, {}", ct, arity, pvs)
            }
            Self::Deallocate => write!(f, "deallocate"),
            Self::JmpBy(arity, offset, pvs, false) => {
                write!(f, "jmp_by_call {}/{}, {}", offset, arity, pvs)
            }
            Self::JmpBy(arity, offset, pvs, true) => {
                write!(f, "jmp_by_execute {}/{}, {}", offset, arity, pvs)
            }
            Self::RevJmpBy(offset) => {
                write!(f, "rev_jmp_by {}", offset)
            }
            Self::Proceed => write!(f, "proceed"),
        }
    }
}

impl fmt::Display for IndexedChoiceInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Try(offset) => write!(f, "try {}", offset),
            Self::Retry(offset) => write!(f, "retry {}", offset),
            Self::Trust(offset) => write!(f, "trust {}", offset),
        }
    }
}

impl fmt::Display for ChoiceInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::TryMeElse(offset) => write!(f, "try_me_else {}", offset),
            Self::DefaultRetryMeElse(offset) => {
                write!(f, "retry_me_else_by_default {}", offset)
            }
            Self::RetryMeElse(offset) => write!(f, "retry_me_else {}", offset),
            Self::DefaultTrustMe(_) => write!(f, "trust_me_by_default"),
            Self::TrustMe(_) => write!(f, "trust_me"),
        }
    }
}

impl fmt::Display for IndexingCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::External(o) => {
                write!(f, "IndexingCodePtr::External({})", o)
            }
            Self::Fail => {
                write!(f, "IndexingCodePtr::Fail")
            }
            Self::Internal(o) => {
                write!(f, "IndexingCodePtr::Internal({})", o)
            }
        }
    }
}

impl fmt::Display for IndexingInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Term(a, var, constant, list, string) => {
                write!(
                    f,
                    "switch_on_term {}, {}, {}, {}, {}",
                    a, var, constant, list, string
                )
            }
            Self::Constant(ref constants) => {
                write!(f, "switch_on_constant {}", constants.len())
            }
            Self::Structure(ref structures) => {
                write!(f, "switch_on_structure {}", structures.len())
            }
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ExistenceError(ref err) => {
                write!(f, "{}", err)
            }
            // Self::CannotOverwriteBuiltIn(ref msg) => {
            //     write!(f, "cannot overwrite {}", msg)
            // }
            // Self::CannotOverwriteImport(ref msg) => {
            //     write!(f, "cannot overwrite import {}", msg)
            // }
            // Self::InvalidFileName(ref filename) => {
            //     write!(f, "filename {} is invalid", filename)
            // }
            // Self::ModuleDoesNotContainExport(ref module, ref key) => {
            //     write!(
            //         f,
            //         "module {} does not contain claimed export {}/{}",
            //         module,
            //         key.0,
            //         key.1,
            //     )
            // }
            Self::OpIsInfixAndPostFix(_) => {
                write!(f, "cannot define an op to be both postfix and infix.")
            }
            Self::NamelessEntry => {
                write!(f, "the predicate head is not an atom or clause.")
            }
            Self::CompilationError(ref e) => {
                write!(f, "syntax_error({:?})", e)
            }
            Self::QueryCannotBeDefinedAsFact => {
                write!(f, "queries cannot be defined as facts.")
            }
            Self::ModuleCannotImportSelf(ref module_name) => {
                write!(
                    f,
                    "modules ({}, in this case) cannot import themselves.",
                    module_name
                )
            }
        }
    }
}

impl fmt::Display for ExistenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Module(ref module_name) => {
                write!(f, "the module {} does not exist", module_name)
            }
            Self::ModuleSource(ref module_source) => {
                write!(f, "the source/sink {} does not exist", module_source)
            }
            Self::Procedure(ref name, arity) => {
                write!(f, "the procedure {}/{} does not exist", name, arity)
            }
            Self::SourceSink(ref addr) => {
                write!(f, "the source/sink {} does not exist", addr)
            }
            Self::Stream(ref addr) => {
                write!(f, "the stream at {} does not exist", addr)
            }
        }
    }
}

impl fmt::Display for ModuleSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::File(ref file) => {
                write!(f, "at the file {}", file)
            }
            Self::Library(ref library) => {
                write!(f, "at library({})", library)
            }
        }
    }
}

impl fmt::Display for IndexingLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Indexing(ref indexing_instr) => {
                write!(f, "{}", indexing_instr)
            }
            Self::IndexedChoice(ref indexed_choice_instrs) => {
                for indexed_choice_instr in indexed_choice_instrs {
                    write!(f, "{}", indexed_choice_instr)?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Arithmetic(ref arith_instr) => write!(f, "{}", arith_instr),
            Self::Choice(ref choice_instr) => write!(f, "{}", choice_instr),
            Self::Control(ref control_instr) => write!(f, "{}", control_instr),
            Self::Cut(ref cut_instr) => write!(f, "{}", cut_instr),
            Self::Fact(ref fact_instr) => write!(f, "{}", fact_instr),
            Self::IndexingCode(ref indexing_instrs) => {
                for indexing_instr in indexing_instrs {
                    write!(f, "{}", indexing_instr)?;
                }

                Ok(())
            }
            Self::IndexedChoice(ref indexed_choice_instr) => write!(f, "{}", indexed_choice_instr),
            Self::Query(ref query_instr) => write!(f, "{}", query_instr),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Fixnum(n) => write!(f, "{}", n),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::Integer(ref bi) => write!(f, "{}", bi),
            Self::Rational(ref r) => write!(f, "{}", r),
        }
    }
}

impl fmt::Display for ArithmeticTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Reg(r) => write!(f, "{}", r),
            Self::Interm(i) => write!(f, "@{}", i),
            Self::Number(ref n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for ArithmeticInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Abs(ref a1, ref t) => write!(f, "abs {}, @{}", a1, t),
            Self::Add(ref a1, ref a2, ref t) => {
                write!(f, "add {}, {}, @{}", a1, a2, t)
            }
            Self::Sub(ref a1, ref a2, ref t) => {
                write!(f, "sub {}, {}, @{}", a1, a2, t)
            }
            Self::Mul(ref a1, ref a2, ref t) => {
                write!(f, "mul {}, {}, @{}", a1, a2, t)
            }
            Self::Pow(ref a1, ref a2, ref t) => {
                write!(f, "** {}, {}, @{}", a1, a2, t)
            }
            Self::IntPow(ref a1, ref a2, ref t) => {
                write!(f, "^ {}, {}, @{}", a1, a2, t)
            }
            Self::Div(ref a1, ref a2, ref t) => {
                write!(f, "div {}, {}, @{}", a1, a2, t)
            }
            Self::IDiv(ref a1, ref a2, ref t) => {
                write!(f, "idiv {}, {}, @{}", a1, a2, t)
            }
            Self::Max(ref a1, ref a2, ref t) => {
                write!(f, "max {}, {}, @{}", a1, a2, t)
            }
            Self::Min(ref a1, ref a2, ref t) => {
                write!(f, "min {}, {}, @{}", a1, a2, t)
            }
            Self::IntFloorDiv(ref a1, ref a2, ref t) => {
                write!(f, "int_floor_div {}, {}, @{}", a1, a2, t)
            }
            Self::RDiv(ref a1, ref a2, ref t) => {
                write!(f, "rdiv {}, {}, @{}", a1, a2, t)
            }
            Self::Gcd(ref a1, ref a2, ref t) => {
                write!(f, "gcd {}, {}, @{}", a1, a2, t)
            }
            Self::Shl(ref a1, ref a2, ref t) => {
                write!(f, "shl {}, {}, @{}", a1, a2, t)
            }
            Self::Shr(ref a1, ref a2, ref t) => {
                write!(f, "shr {}, {}, @{}", a1, a2, t)
            }
            Self::Xor(ref a1, ref a2, ref t) => {
                write!(f, "xor {}, {}, @{}", a1, a2, t)
            }
            Self::And(ref a1, ref a2, ref t) => {
                write!(f, "and {}, {}, @{}", a1, a2, t)
            }
            Self::Or(ref a1, ref a2, ref t) => {
                write!(f, "or {}, {}, @{}", a1, a2, t)
            }
            Self::Mod(ref a1, ref a2, ref t) => {
                write!(f, "mod {}, {}, @{}", a1, a2, t)
            }
            Self::Rem(ref a1, ref a2, ref t) => {
                write!(f, "rem {}, {}, @{}", a1, a2, t)
            }
            Self::ATan2(ref a1, ref a2, ref t) => {
                write!(f, "atan2 {}, {}, @{}", a1, a2, t)
            }
            Self::Plus(ref a, ref t) => write!(f, "plus {}, @{}", a, t),
            Self::Sign(ref a, ref t) => write!(f, "sign {}, @{}", a, t),
            Self::Neg(ref a, ref t) => write!(f, "neg {}, @{}", a, t),
            Self::Cos(ref a, ref t) => write!(f, "cos {}, @{}", a, t),
            Self::Sin(ref a, ref t) => write!(f, "sin {}, @{}", a, t),
            Self::Tan(ref a, ref t) => write!(f, "tan {}, @{}", a, t),
            Self::ATan(ref a, ref t) => write!(f, "atan {}, @{}", a, t),
            Self::ASin(ref a, ref t) => write!(f, "asin {}, @{}", a, t),
            Self::ACos(ref a, ref t) => write!(f, "acos {}, @{}", a, t),
            Self::Log(ref a, ref t) => write!(f, "log {}, @{}", a, t),
            Self::Exp(ref a, ref t) => write!(f, "exp {}, @{}", a, t),
            Self::Sqrt(ref a, ref t) => write!(f, "sqrt {}, @{}", a, t),
            Self::BitwiseComplement(ref a, ref t) => {
                write!(f, "bitwise_complement {}, @{}", a, t)
            }
            Self::Truncate(ref a, ref t) => write!(f, "truncate {}, @{}", a, t),
            Self::Round(ref a, ref t) => write!(f, "round {}, @{}", a, t),
            Self::Ceiling(ref a, ref t) => write!(f, "ceiling {}, @{}", a, t),
            Self::Floor(ref a, ref t) => write!(f, "floor {}, @{}", a, t),
            Self::Float(ref a, ref t) => write!(f, "float {}, @{}", a, t),
        }
    }
}

impl fmt::Display for CutInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Cut(r) => write!(f, "cut {}", r),
            Self::NeckCut => write!(f, "neck_cut"),
            Self::GetLevel(r) => write!(f, "get_level {}", r),
            Self::GetLevelAndUnify(r) => write!(f, "get_level_and_unify {}", r),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Root | Self::Shallow => write!(f, "A"),
            Self::Deep => write!(f, "X"),
        }
    }
}
