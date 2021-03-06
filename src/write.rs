use crate::clause_types::*;
use crate::forms::*;
use crate::indexing::IndexingCodePtr;
use crate::instructions::*;
use crate::machine::loader::CompilationTarget;
use crate::machine::machine_errors::*;
use crate::machine::machine_indices::*;

use std::fmt;

impl fmt::Display for LocalCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalCodePtr::DirEntry(p) => write!(f, "LocalCodePtr::DirEntry({})", p),
            LocalCodePtr::Halt => write!(f, "LocalCodePtr::Halt"),
            LocalCodePtr::IndexingBuf(p, o, i) => write!(f, "LocalCodePtr::IndexingBuf({}, {}, {})", p, o, i),
        }
    }
}

impl fmt::Display for REPLCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            REPLCodePtr::AddDiscontiguousPredicate =>
                write!(f, "REPLCodePtr::AddDiscontiguousPredicate"),
            REPLCodePtr::AddDynamicPredicate =>
                write!(f, "REPLCodePtr::AddDynamicPredicate"),
            REPLCodePtr::AddMultifilePredicate =>
                write!(f, "REPLCodePtr::AddMultifilePredicate"),
            REPLCodePtr::AddGoalExpansionClause =>
                write!(f, "REPLCodePtr::AddGoalExpansionClause"),
            REPLCodePtr::AddTermExpansionClause =>
                write!(f, "REPLCodePtr::AddTermExpansionClause"),
            REPLCodePtr::AddInSituFilenameModule =>
                write!(f, "REPLCodePtr::AddInSituFilenameModule"),
            REPLCodePtr::AbolishClause =>
                write!(f, "REPLCodePtr::AbolishClause"),
            REPLCodePtr::Assertz =>
                write!(f, "REPLCodePtr::Assertz"),
            REPLCodePtr::Asserta =>
                write!(f, "REPLCodePtr::Asserta"),
            REPLCodePtr::Retract =>
                write!(f, "REPLCodePtr::Retract"),
            REPLCodePtr::ClauseToEvacuable =>
                write!(f, "REPLCodePtr::ClauseToEvacuable"),
            REPLCodePtr::ScopedClauseToEvacuable =>
                write!(f, "REPLCodePtr::ScopedClauseToEvacuable"),
            REPLCodePtr::ConcludeLoad =>
                write!(f, "REPLCodePtr::ConcludeLoad"),
	        REPLCodePtr::DeclareModule =>
		        write!(f, "REPLCodePtr::DeclareModule"),
            REPLCodePtr::LoadCompiledLibrary =>
                write!(f, "REPLCodePtr::LoadCompiledLibrary"),
            REPLCodePtr::LoadContextSource =>
                write!(f, "REPLCodePtr::LoadContextSource"),
            REPLCodePtr::LoadContextFile =>
                write!(f, "REPLCodePtr::LoadContextFile"),
            REPLCodePtr::LoadContextDirectory =>
                write!(f, "REPLCodePtr::LoadContextDirectory"),
            REPLCodePtr::LoadContextModule =>
                write!(f, "REPLCodePtr::LoadContextModule"),
            REPLCodePtr::LoadContextStream =>
                write!(f, "REPLCodePtr::LoadContextStream"),
            REPLCodePtr::PopLoadContext =>
                write!(f, "REPLCodePtr::PopLoadContext"),
            REPLCodePtr::PopLoadStatePayload =>
                write!(f, "REPLCodePtr::PopLoadStatePayload"),
            REPLCodePtr::PushLoadContext =>
                write!(f, "REPLCodePtr::PushLoadContext"),
            REPLCodePtr::PushLoadStatePayload =>
                write!(f, "REPLCodePtr::PushLoadStatePayload"),
	        REPLCodePtr::UseModule =>
		        write!(f, "REPLCodePtr::UseModule"),
            REPLCodePtr::MetaPredicateProperty =>
                write!(f, "REPLCodePtr::MetaPredicateProperty"),
            REPLCodePtr::BuiltInProperty =>
                write!(f, "REPLCodePtr::BuiltInProperty"),
            REPLCodePtr::DynamicProperty =>
                write!(f, "REPLCodePtr::DynamicProperty"),
            REPLCodePtr::MultifileProperty =>
                write!(f, "REPLCodePtr::MultifileProperty"),
            REPLCodePtr::DiscontiguousProperty =>
                write!(f, "REPLCodePtr::DiscontiguousProperty"),
            REPLCodePtr::IsConsistentWithTermQueue =>
                write!(f, "REPLCodePtr::IsConsistentWithTermQueue"),
            REPLCodePtr::FlushTermQueue =>
                write!(f, "REPLCodePtr::FlushTermQueue"),
            REPLCodePtr::RemoveModuleExports =>
                write!(f, "REPLCodePtr::RemoveModuleExports"),
            REPLCodePtr::AddNonCountedBacktracking =>
                write!(f, "REPLCodePtr::AddNonCountedBacktracking"),
        }
    }
}

impl fmt::Display for IndexPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IndexPtr::DynamicUndefined => write!(f, "undefined"),
            &IndexPtr::Undefined => write!(f, "undefined"),
            &IndexPtr::DynamicIndex(i) | &IndexPtr::Index(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Display for CompilationTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilationTarget::User => write!(f, "user"),
            CompilationTarget::Module(ref module_name) => write!(f, "{}", module_name),
        }
    }
}

impl fmt::Display for FactInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FactInstruction::GetConstant(lvl, ref constant, ref r) => {
                write!(f, "get_constant {}, {}{}", constant, lvl, r.reg_num())
            }
            &FactInstruction::GetList(lvl, ref r) => {
                write!(f, "get_list {}{}", lvl, r.reg_num())
            }
            &FactInstruction::GetPartialString(lvl, ref s, r, has_tail) => {
                write!(f, "get_partial_string({}, {}, {}, {})",
                       lvl, s, r, has_tail)
            }
            &FactInstruction::GetStructure(ref ct, ref arity, ref r) => {
                write!(f, "get_structure {}/{}, {}", ct.name(), arity, r)
            }
            &FactInstruction::GetValue(ref x, ref a) => {
                write!(f, "get_value {}, A{}", x, a)
            }
            &FactInstruction::GetVariable(ref x, ref a) => {
                write!(f, "fact:get_variable {}, A{}", x, a)
            }
            &FactInstruction::UnifyConstant(ref constant) => {
                write!(f, "unify_constant {}", constant)
            }
            &FactInstruction::UnifyVariable(ref r) => {
                write!(f, "unify_variable {}", r)
            }
            &FactInstruction::UnifyLocalValue(ref r) => {
                write!(f, "unify_local_value {}", r)
            }
            &FactInstruction::UnifyValue(ref r) => {
                write!(f, "unify_value {}", r)
            }
            &FactInstruction::UnifyVoid(n) => {
                write!(f, "unify_void {}", n)
            }
        }
    }
}

impl fmt::Display for QueryInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &QueryInstruction::GetVariable(ref x, ref a) => {
                write!(f, "query:get_variable {}, A{}", x, a)
            }
            &QueryInstruction::PutConstant(lvl, ref constant, ref r) => {
                write!(f, "put_constant {}, {}{}", constant, lvl, r.reg_num())
            }
            &QueryInstruction::PutList(lvl, ref r) => {
                write!(f, "put_list {}{}", lvl, r.reg_num())
            }
            &QueryInstruction::PutPartialString(lvl, ref s, r, has_tail) => {
                write!(f, "put_partial_string({}, {}, {}, {})",
                       lvl, s, r, has_tail)
            }
            &QueryInstruction::PutStructure(ref ct, ref arity, ref r) => {
                write!(f, "put_structure {}/{}, {}", ct.name(), arity, r)
            }
            &QueryInstruction::PutUnsafeValue(y, a) => write!(f, "put_unsafe_value Y{}, A{}", y, a),
            &QueryInstruction::PutValue(ref x, ref a) => write!(f, "put_value {}, A{}", x, a),
            &QueryInstruction::PutVariable(ref x, ref a) => write!(f, "put_variable {}, A{}", x, a),
            &QueryInstruction::SetConstant(ref constant) => write!(f, "set_constant {}", constant),
            &QueryInstruction::SetLocalValue(ref r) => write!(f, "set_local_value {}", r),
            &QueryInstruction::SetVariable(ref r) => write!(f, "set_variable {}", r),
            &QueryInstruction::SetValue(ref r) => write!(f, "set_value {}", r),
            &QueryInstruction::SetVoid(n) => write!(f, "set_void {}", n),
        }
    }
}

impl fmt::Display for CompareNumberQT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CompareNumberQT::GreaterThan => write!(f, ">"),
            &CompareNumberQT::GreaterThanOrEqual => write!(f, ">="),
            &CompareNumberQT::LessThan => write!(f, "<"),
            &CompareNumberQT::LessThanOrEqual => write!(f, "<="),
            &CompareNumberQT::NotEqual => write!(f, "=\\="),
            &CompareNumberQT::Equal => write!(f, "=:="),
        }
    }
}

impl fmt::Display for CompareTermQT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CompareTermQT::GreaterThan => write!(f, "@>"),
            &CompareTermQT::GreaterThanOrEqual => write!(f, "@>="),
            &CompareTermQT::LessThan => write!(f, "@<"),
            &CompareTermQT::LessThanOrEqual => write!(f, "@<="),
        }
    }
}

impl fmt::Display for ClauseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ClauseType::System(SystemClauseType::SetCutPoint(r)) => {
                write!(f, "$set_cp({})", r)
            }
            &ClauseType::Named(ref name, _, ref idx) | &ClauseType::Op(ref name, _, ref idx) => {
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
        match self {
            &HeapCellValue::Addr(ref addr) => write!(f, "{}", addr),
            &HeapCellValue::Atom(ref atom, _) => write!(f, "{}", atom.as_str()),
            &HeapCellValue::DBRef(ref db_ref) => write!(f, "{}", db_ref),
            &HeapCellValue::Integer(ref n) => write!(f, "{}", n),
            &HeapCellValue::LoadStatePayload(_) => write!(f, "LoadStatePayload"),
            &HeapCellValue::Rational(ref n) => write!(f, "{}", n),
            &HeapCellValue::NamedStr(arity, ref name, Some(ref cell)) => write!(
                f,
                "{}/{} (op, priority: {}, spec: {})",
                name.as_str(),
                arity,
                cell.prec(),
                cell.assoc()
            ),
            &HeapCellValue::NamedStr(arity, ref name, None) => {
                write!(f, "{}/{}", name.as_str(), arity)
            }
            &HeapCellValue::PartialString(ref pstr, has_tail) => {
                write!(
                    f,
                    "pstr ( buf: \"{}\", has_tail({}) )",
                    pstr.as_str_from(0),
                    has_tail,
                )
            }
            &HeapCellValue::Stream(ref stream) => {
                write!(f, "$stream({})", stream.as_ptr() as usize)
            }
            &HeapCellValue::TcpListener(ref tcp_listener) => {
                write!(f, "$tcp_listener({})", tcp_listener.local_addr().unwrap())
            }
        }
    }
}

impl fmt::Display for DBRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DBRef::NamedPred(ref name, arity, _) => write!(f, "db_ref:named:{}/{}", name, arity),
            &DBRef::Op(priority, spec, ref name, ..) => {
                write!(f, "db_ref:op({}, {}, {})", priority, spec, name)
            }
        }
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Addr::Char(c) => write!(f, "Addr::Char({})", c),
            &Addr::EmptyList => write!(f, "Addr::EmptyList"),
            &Addr::Fixnum(n) => write!(f, "Addr::Fixnum({})", n),
            &Addr::Float(fl) => write!(f, "Addr::Float({})", fl),
            &Addr::CutPoint(cp) => write!(f, "Addr::CutPoint({})", cp),
            &Addr::Con(ref c) => write!(f, "Addr::Con({})", c),
            &Addr::Lis(l) => write!(f, "Addr::Lis({})", l),
            &Addr::LoadStatePayload(s) => write!(f, "Addr::LoadStatePayload({})", s),
            &Addr::AttrVar(h) => write!(f, "Addr::AttrVar({})", h),
            &Addr::HeapCell(h) => write!(f, "Addr::HeapCell({})", h),
            &Addr::StackCell(fr, sc) => write!(f, "Addr::StackCell({}, {})", fr, sc),
            &Addr::Str(s) => write!(f, "Addr::Str({})", s),
            &Addr::PStrLocation(h, n) => write!(f, "Addr::PStrLocation({}, {})", h, n),
            &Addr::Stream(stream) => write!(f, "Addr::Stream({})", stream),
            &Addr::TcpListener(tcp_listener) => write!(f, "Addr::TcpListener({})", tcp_listener),
            &Addr::Usize(cp) => write!(f, "Addr::Usize({})", cp),
        }
    }
}

impl fmt::Display for ControlInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ControlInstruction::Allocate(num_cells) => write!(f, "allocate {}", num_cells),
            &ControlInstruction::CallClause(ref ct, arity, pvs, true, true) => {
                write!(f, "call_with_default_policy {}/{}, {}", ct, arity, pvs)
            }
            &ControlInstruction::CallClause(ref ct, arity, pvs, false, true) => {
                write!(f, "execute_with_default_policy {}/{}, {}", ct, arity, pvs)
            }
            &ControlInstruction::CallClause(ref ct, arity, pvs, true, false) => {
                write!(f, "execute {}/{}, {}", ct, arity, pvs)
            }
            &ControlInstruction::CallClause(ref ct, arity, pvs, false, false) => {
                write!(f, "call {}/{}, {}", ct, arity, pvs)
            }
            &ControlInstruction::Deallocate => write!(f, "deallocate"),
            &ControlInstruction::JmpBy(arity, offset, pvs, false) => {
                write!(f, "jmp_by_call {}/{}, {}", offset, arity, pvs)
            }
            &ControlInstruction::JmpBy(arity, offset, pvs, true) => {
                write!(f, "jmp_by_execute {}/{}, {}", offset, arity, pvs)
            }
            &ControlInstruction::RevJmpBy(offset) => {
                write!(f, "rev_jmp_by {}", offset)
            }
            &ControlInstruction::Proceed => write!(f, "proceed"),
        }
    }
}

impl fmt::Display for IndexedChoiceInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IndexedChoiceInstruction::Try(offset) => write!(f, "try {}", offset),
            &IndexedChoiceInstruction::Retry(offset) => write!(f, "retry {}", offset),
            &IndexedChoiceInstruction::Trust(offset) => write!(f, "trust {}", offset),
        }
    }
}

impl fmt::Display for ChoiceInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ChoiceInstruction::DynamicElse(offset, Death::Infinity, NextOrFail::Next(i)) => {
                write!(f, "dynamic_else {}, {}, {}", offset, "inf", i)
            }
            &ChoiceInstruction::DynamicElse(offset, Death::Infinity, NextOrFail::Fail(i)) => {
                write!(f, "dynamic_else {}, {}, fail({})", offset, "inf", i)
            }
            &ChoiceInstruction::DynamicElse(offset, Death::Finite(d), NextOrFail::Next(i)) => {
                write!(f, "dynamic_else {}, {}, {}", offset, d, i)
            }
            &ChoiceInstruction::DynamicElse(offset, Death::Finite(d), NextOrFail::Fail(i)) => {
                write!(f, "dynamic_else {}, {}, fail({})", offset, d, i)
            }
            &ChoiceInstruction::DynamicInternalElse(offset, Death::Infinity, NextOrFail::Next(i)) => {
                write!(f, "dynamic_internal_else {}, {}, {}", offset, "inf", i)
            }
            &ChoiceInstruction::DynamicInternalElse(offset, Death::Infinity, NextOrFail::Fail(i)) => {
                write!(f, "dynamic_internal_else {}, {}, fail({})", offset, "inf", i)
            }
            &ChoiceInstruction::DynamicInternalElse(offset, Death::Finite(d), NextOrFail::Next(i)) => {
                write!(f, "dynamic_internal_else {}, {}, {}", offset, d, i)
            }
            &ChoiceInstruction::DynamicInternalElse(offset, Death::Finite(d), NextOrFail::Fail(i)) => {
                write!(f, "dynamic_internal_else {}, {}, fail({})", offset, d, i)
            }
            &ChoiceInstruction::TryMeElse(offset) =>
                write!(f, "try_me_else {}", offset),
            &ChoiceInstruction::DefaultRetryMeElse(offset) => {
                write!(f, "retry_me_else_by_default {}", offset)
            }
            &ChoiceInstruction::RetryMeElse(offset) =>
                write!(f, "retry_me_else {}", offset),
            &ChoiceInstruction::DefaultTrustMe(_) =>
                write!(f, "trust_me_by_default"),
            &ChoiceInstruction::TrustMe(_) =>
                write!(f, "trust_me"),
        }
    }
}

impl fmt::Display for IndexingCodePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IndexingCodePtr::DynamicExternal(o) => {
                write!(f, "IndexingCodePtr::DynamicExternal({})", o)
            }
            &IndexingCodePtr::External(o) => {
                write!(f, "IndexingCodePtr::External({})", o)
            }
            &IndexingCodePtr::Fail => {
                write!(f, "IndexingCodePtr::Fail")
            }
            &IndexingCodePtr::Internal(o) => {
                write!(f, "IndexingCodePtr::Internal({})", o)
            }
        }
    }
}

impl fmt::Display for IndexingInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IndexingInstruction::SwitchOnTerm(a, v, c, l, s) => {
                write!(f, "switch_on_term {}, {}, {}, {}, {}", a, v, c, l, s)
            }
            &IndexingInstruction::SwitchOnConstant(ref constants) => {
                write!(f, "switch_on_constant {}", constants.len())
            }
            &IndexingInstruction::SwitchOnStructure(ref structures) => {
                write!(f, "switch_on_structure {}", structures.len())
            }
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SessionError::ExistenceError(ref err) => {
                write!(f, "{}", err)
            }
            // &SessionError::CannotOverwriteBuiltIn(ref msg) => {
            //     write!(f, "cannot overwrite {}", msg)
            // }
            // &SessionError::CannotOverwriteImport(ref msg) => {
            //     write!(f, "cannot overwrite import {}", msg)
            // }
            // &SessionError::InvalidFileName(ref filename) => {
            //     write!(f, "filename {} is invalid", filename)
            // }
            &SessionError::ModuleDoesNotContainExport(ref module, ref key) => {
                write!(
                    f,
                    "module {} does not contain claimed export {}/{}",
                    module,
                    key.0,
                    key.1,
                )
            }
            &SessionError::OpIsInfixAndPostFix(_) => {
                write!(f, "cannot define an op to be both postfix and infix.")
            }
            &SessionError::NamelessEntry => {
                write!(f, "the predicate head is not an atom or clause.")
            }
            &SessionError::CompilationError(ref e) => {
                write!(f, "syntax_error({:?})", e)
            }
            &SessionError::QueryCannotBeDefinedAsFact => {
                write!(f, "queries cannot be defined as facts.")
            }
            &SessionError::ModuleCannotImportSelf(ref module_name) => {
                write!(f, "modules ({}, in this case) cannot import themselves.",
                       module_name)
            }
            &SessionError::PredicateNotMultifileOrDiscontiguous(ref compilation_target, ref key) => {
                write!(f, "module {} does not define {}/{} as multifile or discontiguous.",
                       compilation_target.module_name(), key.0, key.1)
            }
        }
    }
}

impl fmt::Display for ExistenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExistenceError::Module(ref module_name) => {
                write!(f, "the module {} does not exist", module_name)
            }
            &ExistenceError::ModuleSource(ref module_source) => {
                write!(f, "the source/sink {} does not exist", module_source)
            }
            &ExistenceError::Procedure(ref name, arity) => {
                write!(f, "the procedure {}/{} does not exist", name, arity)
            }
            &ExistenceError::SourceSink(ref addr) => {
                write!(f, "the source/sink {} does not exist", addr)
            }
            &ExistenceError::Stream(ref addr) => {
                write!(f, "the stream at {} does not exist", addr)
            }
        }
    }
}

impl fmt::Display for ModuleSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ModuleSource::File(ref file) => {
                write!(f, "at the file {}", file)
            }
            &ModuleSource::Library(ref library) => {
                write!(f, "at library({})", library)
            }
        }
    }
}

impl fmt::Display for IndexingLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &IndexingLine::Indexing(ref indexing_instr) => {
                write!(f, "{}", indexing_instr)
            }
            &IndexingLine::IndexedChoice(ref indexed_choice_instrs) => {
                for indexed_choice_instr in indexed_choice_instrs {
                    write!(f, "{}", indexed_choice_instr)?;
                }

                Ok(())
            }
            &IndexingLine::DynamicIndexedChoice(ref indexed_choice_instrs) => {
                for indexed_choice_instr in indexed_choice_instrs {
                    write!(f, "dynamic({})", indexed_choice_instr)?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Line::Arithmetic(ref arith_instr) => write!(f, "{}", arith_instr),
            &Line::Choice(ref choice_instr) => write!(f, "{}", choice_instr),
            &Line::Control(ref control_instr) => write!(f, "{}", control_instr),
            &Line::Cut(ref cut_instr) => write!(f, "{}", cut_instr),
            &Line::Fact(ref fact_instr) => write!(f, "{}", fact_instr),
            &Line::IndexingCode(ref indexing_instrs) => {
                for indexing_instr in indexing_instrs {
                    write!(f, "{}", indexing_instr)?;
                }

                Ok(())
            }
            &Line::IndexedChoice(ref indexed_choice_instr) => write!(f, "{}", indexed_choice_instr),
            &Line::DynamicIndexedChoice(ref indexed_choice_instr) => write!(f, "{}", indexed_choice_instr),
            &Line::Query(ref query_instr) => write!(f, "{}", query_instr),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Number::Fixnum(n) => write!(f, "{}", n),
            &Number::Float(fl) => write!(f, "{}", fl),
            &Number::Integer(ref bi) => write!(f, "{}", bi),
            &Number::Rational(ref r) => write!(f, "{}", r),
        }
    }
}

impl fmt::Display for ArithmeticTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ArithmeticTerm::Reg(r) => write!(f, "{}", r),
            &ArithmeticTerm::Interm(i) => write!(f, "@{}", i),
            &ArithmeticTerm::Number(ref n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for ArithmeticInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ArithmeticInstruction::Abs(ref a1, ref t) => write!(f, "abs {}, @{}", a1, t),
            &ArithmeticInstruction::Add(ref a1, ref a2, ref t) => {
                write!(f, "add {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Sub(ref a1, ref a2, ref t) => {
                write!(f, "sub {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Mul(ref a1, ref a2, ref t) => {
                write!(f, "mul {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Pow(ref a1, ref a2, ref t) => {
                write!(f, "** {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::IntPow(ref a1, ref a2, ref t) => {
                write!(f, "^ {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Div(ref a1, ref a2, ref t) => {
                write!(f, "div {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::IDiv(ref a1, ref a2, ref t) => {
                write!(f, "idiv {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Max(ref a1, ref a2, ref t) => {
                write!(f, "max {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Min(ref a1, ref a2, ref t) => {
                write!(f, "min {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::IntFloorDiv(ref a1, ref a2, ref t) => {
                write!(f, "int_floor_div {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::RDiv(ref a1, ref a2, ref t) => {
                write!(f, "rdiv {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Gcd(ref a1, ref a2, ref t) => {
                write!(f, "gcd {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Shl(ref a1, ref a2, ref t) => {
                write!(f, "shl {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Shr(ref a1, ref a2, ref t) => {
                write!(f, "shr {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Xor(ref a1, ref a2, ref t) => {
                write!(f, "xor {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::And(ref a1, ref a2, ref t) => {
                write!(f, "and {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Or(ref a1, ref a2, ref t) => {
                write!(f, "or {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Mod(ref a1, ref a2, ref t) => {
                write!(f, "mod {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Rem(ref a1, ref a2, ref t) => {
                write!(f, "rem {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::ATan2(ref a1, ref a2, ref t) => {
                write!(f, "atan2 {}, {}, @{}", a1, a2, t)
            }
            &ArithmeticInstruction::Plus(ref a, ref t) => write!(f, "plus {}, @{}", a, t),
            &ArithmeticInstruction::Sign(ref a, ref t) => write!(f, "sign {}, @{}", a, t),
            &ArithmeticInstruction::Neg(ref a, ref t) => write!(f, "neg {}, @{}", a, t),
            &ArithmeticInstruction::Cos(ref a, ref t) => write!(f, "cos {}, @{}", a, t),
            &ArithmeticInstruction::Sin(ref a, ref t) => write!(f, "sin {}, @{}", a, t),
            &ArithmeticInstruction::Tan(ref a, ref t) => write!(f, "tan {}, @{}", a, t),
            &ArithmeticInstruction::ATan(ref a, ref t) => write!(f, "atan {}, @{}", a, t),
            &ArithmeticInstruction::ASin(ref a, ref t) => write!(f, "asin {}, @{}", a, t),
            &ArithmeticInstruction::ACos(ref a, ref t) => write!(f, "acos {}, @{}", a, t),
            &ArithmeticInstruction::Log(ref a, ref t) => write!(f, "log {}, @{}", a, t),
            &ArithmeticInstruction::Exp(ref a, ref t) => write!(f, "exp {}, @{}", a, t),
            &ArithmeticInstruction::Sqrt(ref a, ref t) => write!(f, "sqrt {}, @{}", a, t),
            &ArithmeticInstruction::BitwiseComplement(ref a, ref t) => {
                write!(f, "bitwise_complement {}, @{}", a, t)
            }
            &ArithmeticInstruction::Truncate(ref a, ref t) => write!(f, "truncate {}, @{}", a, t),
            &ArithmeticInstruction::Round(ref a, ref t) => write!(f, "round {}, @{}", a, t),
            &ArithmeticInstruction::Ceiling(ref a, ref t) => write!(f, "ceiling {}, @{}", a, t),
            &ArithmeticInstruction::Floor(ref a, ref t) => write!(f, "floor {}, @{}", a, t),
            &ArithmeticInstruction::Float(ref a, ref t) => write!(f, "float {}, @{}", a, t),
        }
    }
}

impl fmt::Display for CutInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CutInstruction::Cut(r) => write!(f, "cut {}", r),
            &CutInstruction::NeckCut => write!(f, "neck_cut"),
            &CutInstruction::GetLevel(r) => write!(f, "get_level {}", r),
            &CutInstruction::GetLevelAndUnify(r) => write!(f, "get_level_and_unify {}", r),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Level::Root | &Level::Shallow => write!(f, "A"),
            &Level::Deep => write!(f, "X"),
        }
    }
}
