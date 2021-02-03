use crate::prolog_parser_rebis::ast::*;

use crate::clause_types::*;
use crate::forms::*;
use crate::indexing::IndexingCodePtr;
use crate::machine::heap::*;
use crate::machine::machine_errors::MachineStub;
use crate::machine::machine_indices::*;
use crate::rug::Integer;

use crate::indexmap::IndexMap;

use slice_deque::SliceDeque;

use std::rc::Rc;

fn reg_type_into_functor(r: RegType) -> MachineStub {
    match r {
        RegType::Temp(r) => functor!("x", [integer(r)]),
        RegType::Perm(r) => functor!("y", [integer(r)]),
    }
}

impl Level {
    fn into_functor(self) -> MachineStub {
        match self {
            Level::Root => functor!("level", [atom("root")]),
            Level::Shallow => functor!("level", [atom("shallow")]),
            Level::Deep => functor!("level", [atom("deep")]),
        }
    }
}

impl ArithmeticTerm {
    fn to_functor(&self) -> MachineStub {
        match *self {
            Self::Reg(r) => reg_type_into_functor(r),
            Self::Interm(i) => {
                functor!("intermediate", [integer(i)])
            }
            Self::Number(ref n) => {
                vec![n.clone().into()]
            }
        }
    }
}

#[derive(Debug)]
pub enum ChoiceInstruction {
    DefaultRetryMeElse(usize),
    DefaultTrustMe(usize),
    RetryMeElse(usize),
    TrustMe(usize),
    TryMeElse(usize),
}

impl ChoiceInstruction {
    pub fn to_functor(&self) -> MachineStub {
        match *self {
            Self::TryMeElse(offset) => {
                functor!("try_me_else", [integer(offset)])
            }
            Self::RetryMeElse(offset) => {
                functor!("retry_me_else", [integer(offset)])
            }
            Self::TrustMe(offset) => {
                functor!("trust_me", [integer(offset)])
            }
            Self::DefaultRetryMeElse(offset) => {
                functor!("default_retry_me_else", [integer(offset)])
            }
            Self::DefaultTrustMe(offset) => {
                functor!("default_trust_me", [integer(offset)])
            }
        }
    }
}

#[derive(Debug)]
pub enum CutInstruction {
    Cut(RegType),
    GetLevel(RegType),
    GetLevelAndUnify(RegType),
    NeckCut,
}

impl CutInstruction {
    pub fn to_functor(&self, h: usize) -> MachineStub {
        match *self {
            Self::Cut(r) => {
                let rt_stub = reg_type_into_functor(r);
                functor!("cut", [aux(h, 0)], [rt_stub])
            }
            Self::GetLevel(r) => {
                let rt_stub = reg_type_into_functor(r);
                functor!("get_level", [aux(h, 0)], [rt_stub])
            }
            Self::GetLevelAndUnify(r) => {
                let rt_stub = reg_type_into_functor(r);
                functor!("get_level_and_unify", [aux(h, 0)], [rt_stub])
            }
            Self::NeckCut => {
                functor!("neck_cut")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IndexedChoiceInstruction {
    Retry(usize),
    Trust(usize),
    Try(usize),
}

impl IndexedChoiceInstruction {
    pub fn offset(&self) -> usize {
        match *self {
            Self::Retry(offset) => offset,
            Self::Trust(offset) => offset,
            Self::Try(offset) => offset,
        }
    }

    pub fn to_functor(&self) -> MachineStub {
        match *self {
            Self::Try(offset) => {
                functor!("try", [integer(offset)])
            }
            Self::Trust(offset) => {
                functor!("trust", [integer(offset)])
            }
            Self::Retry(offset) => {
                functor!("retry", [integer(offset)])
            }
        }
    }
}

/// A `Line` is an instruction (cf. page 98 of wambook).
#[derive(Debug)]
pub enum IndexingLine {
    Indexing(IndexingInstruction),
    IndexedChoice(SliceDeque<IndexedChoiceInstruction>),
}

impl From<IndexingInstruction> for IndexingLine {
    #[inline]
    fn from(instr: IndexingInstruction) -> Self {
        IndexingLine::Indexing(instr)
    }
}

impl From<SliceDeque<IndexedChoiceInstruction>> for IndexingLine {
    #[inline]
    fn from(instrs: SliceDeque<IndexedChoiceInstruction>) -> Self {
        IndexingLine::IndexedChoice(instrs)
    }
}

#[derive(Debug)]
pub enum Line {
    Arithmetic(ArithmeticInstruction),
    Choice(ChoiceInstruction),
    Control(ControlInstruction),
    Cut(CutInstruction),
    Fact(FactInstruction),
    IndexingCode(Vec<IndexingLine>),
    IndexedChoice(IndexedChoiceInstruction),
    Query(QueryInstruction),
}

impl Line {
    #[inline]
    pub fn is_head_instr(&self) -> bool {
        matches!(&self, Self::Cut(_) | Self::Fact(_) | Self::Query(_))
    }

    pub fn enqueue_functors(&self, mut h: usize, functors: &mut Vec<MachineStub>) {
        match *self {
            Self::Arithmetic(ref arith_instr) => functors.push(arith_instr.to_functor(h)),
            Self::Choice(ref choice_instr) => functors.push(choice_instr.to_functor()),
            Self::Control(ref control_instr) => functors.push(control_instr.to_functor()),
            Self::Cut(ref cut_instr) => functors.push(cut_instr.to_functor(h)),
            Self::Fact(ref fact_instr) => functors.push(fact_instr.to_functor(h)),
            Self::IndexingCode(ref indexing_instrs) => {
                for indexing_instr in indexing_instrs {
                    match indexing_instr {
                        IndexingLine::Indexing(indexing_instr) => {
                            let section = indexing_instr.to_functor(h);
                            h += section.len();
                            functors.push(section);
                        }
                        IndexingLine::IndexedChoice(indexed_choice_instrs) => {
                            for indexed_choice_instr in indexed_choice_instrs {
                                let section = indexed_choice_instr.to_functor();
                                h += section.len();
                                functors.push(section);
                            }
                        }
                    }
                }
            }
            Self::IndexedChoice(ref indexed_choice_instr) => {
                functors.push(indexed_choice_instr.to_functor())
            }
            Self::Query(ref query_instr) => functors.push(query_instr.to_functor(h)),
        }
    }
}

#[inline]
pub fn to_indexing_line_mut(line: &mut Line) -> Option<&mut Vec<IndexingLine>> {
    if let Line::IndexingCode(ref mut indexing_code) = line {
        Some(indexing_code)
    } else {
        None
    }
}

#[inline]
pub fn to_indexing_line(line: &Line) -> Option<&Vec<IndexingLine>> {
    if let Line::IndexingCode(ref indexing_code) = line {
        Some(indexing_code)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub enum ArithmeticInstruction {
    Add(ArithmeticTerm, ArithmeticTerm, usize),
    Sub(ArithmeticTerm, ArithmeticTerm, usize),
    Mul(ArithmeticTerm, ArithmeticTerm, usize),
    Pow(ArithmeticTerm, ArithmeticTerm, usize),
    IntPow(ArithmeticTerm, ArithmeticTerm, usize),
    IDiv(ArithmeticTerm, ArithmeticTerm, usize),
    Max(ArithmeticTerm, ArithmeticTerm, usize),
    Min(ArithmeticTerm, ArithmeticTerm, usize),
    IntFloorDiv(ArithmeticTerm, ArithmeticTerm, usize),
    RDiv(ArithmeticTerm, ArithmeticTerm, usize),
    Div(ArithmeticTerm, ArithmeticTerm, usize),
    Shl(ArithmeticTerm, ArithmeticTerm, usize),
    Shr(ArithmeticTerm, ArithmeticTerm, usize),
    Xor(ArithmeticTerm, ArithmeticTerm, usize),
    And(ArithmeticTerm, ArithmeticTerm, usize),
    Or(ArithmeticTerm, ArithmeticTerm, usize),
    Mod(ArithmeticTerm, ArithmeticTerm, usize),
    Rem(ArithmeticTerm, ArithmeticTerm, usize),
    Gcd(ArithmeticTerm, ArithmeticTerm, usize),
    Sign(ArithmeticTerm, usize),
    Cos(ArithmeticTerm, usize),
    Sin(ArithmeticTerm, usize),
    Tan(ArithmeticTerm, usize),
    Log(ArithmeticTerm, usize),
    Exp(ArithmeticTerm, usize),
    ACos(ArithmeticTerm, usize),
    ASin(ArithmeticTerm, usize),
    ATan(ArithmeticTerm, usize),
    ATan2(ArithmeticTerm, ArithmeticTerm, usize),
    Sqrt(ArithmeticTerm, usize),
    Abs(ArithmeticTerm, usize),
    Float(ArithmeticTerm, usize),
    Truncate(ArithmeticTerm, usize),
    Round(ArithmeticTerm, usize),
    Ceiling(ArithmeticTerm, usize),
    Floor(ArithmeticTerm, usize),
    Neg(ArithmeticTerm, usize),
    Plus(ArithmeticTerm, usize),
    BitwiseComplement(ArithmeticTerm, usize),
}

fn arith_instr_unary_functor(
    h: usize,
    name: &'static str,
    at: &ArithmeticTerm,
    t: usize,
) -> MachineStub {
    let at_stub = at.to_functor();

    functor!(name, [aux(h, 0), integer(t)], [at_stub])
}

fn arith_instr_bin_functor(
    h: usize,
    name: &'static str,
    at_1: &ArithmeticTerm,
    at_2: &ArithmeticTerm,
    t: usize,
) -> MachineStub {
    let at_1_stub = at_1.to_functor();
    let at_2_stub = at_2.to_functor();

    functor!(
        name,
        [aux(h, 0), aux(h, 1), integer(t)],
        [at_1_stub, at_2_stub]
    )
}

impl ArithmeticInstruction {
    pub fn to_functor(&self, h: usize) -> MachineStub {
        match *self {
            Self::Add(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "add", at_1, at_2, t),
            Self::Sub(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "sub", at_1, at_2, t),
            Self::Mul(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "mul", at_1, at_2, t),
            Self::IntPow(ref at_1, ref at_2, t) => {
                arith_instr_bin_functor(h, "int_pow", at_1, at_2, t)
            }
            Self::Pow(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "pow", at_1, at_2, t),
            Self::IDiv(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "idiv", at_1, at_2, t),
            Self::Max(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "max", at_1, at_2, t),
            Self::Min(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "min", at_1, at_2, t),
            Self::IntFloorDiv(ref at_1, ref at_2, t) => {
                arith_instr_bin_functor(h, "int_floor_div", at_1, at_2, t)
            }
            Self::RDiv(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "rdiv", at_1, at_2, t),
            Self::Div(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "div", at_1, at_2, t),
            Self::Shl(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "shl", at_1, at_2, t),
            Self::Shr(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "shr", at_1, at_2, t),
            Self::Xor(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "xor", at_1, at_2, t),
            Self::And(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "and", at_1, at_2, t),
            Self::Or(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "or", at_1, at_2, t),
            Self::Mod(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "mod", at_1, at_2, t),
            Self::Rem(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "rem", at_1, at_2, t),
            Self::ATan2(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "rem", at_1, at_2, t),
            Self::Gcd(ref at_1, ref at_2, t) => arith_instr_bin_functor(h, "gcd", at_1, at_2, t),
            Self::Sign(ref at, t) => arith_instr_unary_functor(h, "sign", at, t),
            Self::Cos(ref at, t) => arith_instr_unary_functor(h, "cos", at, t),
            Self::Sin(ref at, t) => arith_instr_unary_functor(h, "sin", at, t),
            Self::Tan(ref at, t) => arith_instr_unary_functor(h, "tan", at, t),
            Self::Log(ref at, t) => arith_instr_unary_functor(h, "log", at, t),
            Self::Exp(ref at, t) => arith_instr_unary_functor(h, "exp", at, t),
            Self::ACos(ref at, t) => arith_instr_unary_functor(h, "acos", at, t),
            Self::ASin(ref at, t) => arith_instr_unary_functor(h, "asin", at, t),
            Self::ATan(ref at, t) => arith_instr_unary_functor(h, "atan", at, t),
            Self::Sqrt(ref at, t) => arith_instr_unary_functor(h, "sqrt", at, t),
            Self::Abs(ref at, t) => arith_instr_unary_functor(h, "abs", at, t),
            Self::Float(ref at, t) => arith_instr_unary_functor(h, "float", at, t),
            Self::Truncate(ref at, t) => arith_instr_unary_functor(h, "truncate", at, t),
            Self::Round(ref at, t) => arith_instr_unary_functor(h, "round", at, t),
            Self::Ceiling(ref at, t) => arith_instr_unary_functor(h, "ceiling", at, t),
            Self::Floor(ref at, t) => arith_instr_unary_functor(h, "floor", at, t),
            Self::Neg(ref at, t) => arith_instr_unary_functor(h, "-", at, t),
            Self::Plus(ref at, t) => arith_instr_unary_functor(h, "+", at, t),
            Self::BitwiseComplement(ref at, t) => arith_instr_unary_functor(h, "\\", at, t),
        }
    }
}

#[derive(Debug)]
pub enum ControlInstruction {
    Allocate(usize), // num_frames.
    // name, arity, perm_vars after threshold, last call, use default call policy.
    CallClause(ClauseType, usize, usize, bool, bool),
    Deallocate,
    JmpBy(usize, usize, usize, bool), // arity, global_offset, perm_vars after threshold, last call.
    RevJmpBy(usize),                  // notice the lack of context change as in
    // JmpBy. RevJmpBy is used only to patch extensible
    // predicates together.
    Proceed,
}

impl ControlInstruction {
    pub fn perm_vars(&self) -> Option<usize> {
        match self {
            ControlInstruction::CallClause(_, _, num_cells, ..) => Some(*num_cells),
            ControlInstruction::JmpBy(_, _, num_cells, ..) => Some(*num_cells),
            _ => None,
        }
    }

    pub fn to_functor(&self) -> MachineStub {
        match *self {
            Self::Allocate(num_frames) => {
                functor!("allocate", [integer(num_frames)])
            }
            Self::CallClause(ref ct, arity, _, false, _) => {
                functor!("call", [clause_name(ct.name()), integer(arity)])
            }
            Self::CallClause(ref ct, arity, _, true, _) => {
                functor!("execute", [clause_name(ct.name()), integer(arity)])
            }
            Self::Deallocate => {
                functor!("deallocate")
            }
            Self::JmpBy(_, offset, ..) => {
                functor!("jmp_by", [integer(offset)])
            }
            Self::RevJmpBy(offset) => {
                functor!("rev_jmp_by", [integer(offset)])
            }
            Self::Proceed => {
                functor!("proceed")
            }
        }
    }
}

/// `IndexingInstruction` cf. page 110 of wambook.
#[derive(Debug)]
pub enum IndexingInstruction {
    // The first index is the optimal argument being indexed.
    SwitchOnTerm(
        usize,
        usize,
        IndexingCodePtr,
        IndexingCodePtr,
        IndexingCodePtr,
    ),
    SwitchOnConstant(IndexMap<Constant, IndexingCodePtr>),
    SwitchOnStructure(IndexMap<(ClauseName, usize), IndexingCodePtr>),
}

impl IndexingInstruction {
    pub fn to_functor(&self, mut h: usize) -> MachineStub {
        match *self {
            Self::SwitchOnTerm(arg, vars, constants, lists, structures) => {
                functor!(
                    "switch_on_term",
                    [
                        integer(arg),
                        integer(vars),
                        indexing_code_ptr(h, constants),
                        indexing_code_ptr(h, lists),
                        indexing_code_ptr(h, structures)
                    ]
                )
            }
            Self::SwitchOnConstant(ref constants) => {
                let mut key_value_list_stub = vec![];
                let orig_h = h;

                h += 2; // skip the 2-cell "switch_on_constant" functor.

                for (c, ptr) in constants.iter() {
                    let key_value_pair = functor!(
                        ":",
                        SharedOpDesc::new(600, XFY),
                        [constant(c), indexing_code_ptr(h + 3, *ptr)]
                    );

                    key_value_list_stub.push(HeapCellValue::Addr(Addr::Lis(h + 1)));
                    key_value_list_stub.push(HeapCellValue::Addr(Addr::Str(h + 3)));
                    key_value_list_stub.push(HeapCellValue::Addr(Addr::HeapCell(
                        h + 3 + key_value_pair.len(),
                    )));

                    h += key_value_pair.len() + 3;
                    key_value_list_stub.extend(key_value_pair.into_iter());
                }

                key_value_list_stub.push(HeapCellValue::Addr(Addr::EmptyList));

                functor!(
                    "switch_on_constant",
                    [aux(orig_h, 0)],
                    [key_value_list_stub]
                )
            }
            Self::SwitchOnStructure(ref structures) => {
                let mut key_value_list_stub = vec![];
                let orig_h = h;

                h += 2; // skip the 2-cell "switch_on_constant" functor.

                for ((name, arity), ptr) in structures.iter() {
                    let predicate_indicator_stub = functor!(
                        "/",
                        SharedOpDesc::new(400, YFX),
                        [clause_name(name.clone()), integer(*arity)]
                    );

                    let key_value_pair = functor!(
                        ":",
                        SharedOpDesc::new(600, XFY),
                        [aux(h + 3, 0), indexing_code_ptr(h + 3, *ptr)],
                        [predicate_indicator_stub]
                    );

                    key_value_list_stub.push(HeapCellValue::Addr(Addr::Lis(h + 1)));
                    key_value_list_stub.push(HeapCellValue::Addr(Addr::Str(h + 3)));
                    key_value_list_stub.push(HeapCellValue::Addr(Addr::HeapCell(
                        h + 3 + key_value_pair.len(),
                    )));

                    h += key_value_pair.len() + 3;
                    key_value_list_stub.extend(key_value_pair.into_iter());
                }

                key_value_list_stub.push(HeapCellValue::Addr(Addr::EmptyList));

                functor!(
                    "switch_on_structure",
                    [aux(orig_h, 0)],
                    [key_value_list_stub]
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FactInstruction {
    GetConstant(Level, Constant, RegType),
    GetList(Level, RegType),
    GetPartialString(Level, String, RegType, bool),
    GetStructure(ClauseType, usize, RegType),
    GetValue(RegType, usize),
    GetVariable(RegType, usize),
    UnifyConstant(Constant),
    UnifyLocalValue(RegType),
    UnifyVariable(RegType),
    UnifyValue(RegType),
    UnifyVoid(usize),
}

impl FactInstruction {
    pub fn to_functor(&self, h: usize) -> MachineStub {
        match *self {
            Self::GetConstant(lvl, ref c, r) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "get_constant",
                    [aux(h, 0), constant(h, c), aux(h, 1)],
                    [lvl_stub, rt_stub]
                )
            }
            Self::GetList(lvl, r) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!("get_list", [aux(h, 0), aux(h, 1)], [lvl_stub, rt_stub])
            }
            Self::GetPartialString(lvl, ref s, r, has_tail) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "get_partial_string",
                    [aux(h, 0), string(h, s), aux(h, 1), boolean(has_tail)],
                    [lvl_stub, rt_stub]
                )
            }
            Self::GetStructure(ref ct, arity, r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "get_structure",
                    [clause_name(ct.name()), integer(arity), aux(h, 0)],
                    [rt_stub]
                )
            }
            Self::GetValue(r, arg) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("get_value", [aux(h, 0), integer(arg)], [rt_stub])
            }
            Self::GetVariable(r, arg) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("get_variable", [aux(h, 0), integer(arg)], [rt_stub])
            }
            Self::UnifyConstant(ref c) => {
                functor!("unify_constant", [constant(h, c)], [])
            }
            Self::UnifyLocalValue(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("unify_local_value", [aux(h, 0)], [rt_stub])
            }
            Self::UnifyVariable(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("unify_variable", [aux(h, 0)], [rt_stub])
            }
            Self::UnifyValue(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("unify_value", [aux(h, 0)], [rt_stub])
            }
            Self::UnifyVoid(vars) => {
                functor!("unify_void", [integer(vars)])
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum QueryInstruction {
    GetVariable(RegType, usize),
    PutConstant(Level, Constant, RegType),
    PutList(Level, RegType),
    PutPartialString(Level, String, RegType, bool),
    PutStructure(ClauseType, usize, RegType),
    PutUnsafeValue(usize, usize),
    PutValue(RegType, usize),
    PutVariable(RegType, usize),
    SetConstant(Constant),
    SetLocalValue(RegType),
    SetVariable(RegType),
    SetValue(RegType),
    SetVoid(usize),
}

impl QueryInstruction {
    pub fn to_functor(&self, h: usize) -> MachineStub {
        match *self {
            Self::PutUnsafeValue(norm, arg) => {
                functor!("put_unsafe_value", [integer(norm), integer(arg)])
            }
            Self::PutConstant(lvl, ref c, r) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "put_constant",
                    [aux(h, 0), constant(h, c), aux(h, 1)],
                    [lvl_stub, rt_stub]
                )
            }
            Self::PutList(lvl, r) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!("put_list", [aux(h, 0), aux(h, 1)], [lvl_stub, rt_stub])
            }
            Self::PutPartialString(lvl, ref s, r, has_tail) => {
                let lvl_stub = lvl.into_functor();
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "put_partial_string",
                    [aux(h, 0), string(h, s), aux(h, 1), boolean(has_tail)],
                    [lvl_stub, rt_stub]
                )
            }
            Self::PutStructure(ref ct, arity, r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!(
                    "put_structure",
                    [clause_name(ct.name()), integer(arity), aux(h, 0)],
                    [rt_stub]
                )
            }
            Self::PutValue(r, arg) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("put_value", [aux(h, 0), integer(arg)], [rt_stub])
            }
            Self::GetVariable(r, arg) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("get_variable", [aux(h, 0), integer(arg)], [rt_stub])
            }
            Self::PutVariable(r, arg) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("put_variable", [aux(h, 0), integer(arg)], [rt_stub])
            }
            Self::SetConstant(ref c) => {
                functor!("set_constant", [constant(h, c)], [])
            }
            Self::SetLocalValue(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("set_local_value", [aux(h, 0)], [rt_stub])
            }
            Self::SetVariable(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("set_variable", [aux(h, 0)], [rt_stub])
            }
            Self::SetValue(r) => {
                let rt_stub = reg_type_into_functor(r);

                functor!("set_value", [aux(h, 0)], [rt_stub])
            }
            Self::SetVoid(vars) => {
                functor!("set_void", [integer(vars)])
            }
        }
    }
}

pub type CompiledFact = Vec<FactInstruction>;

pub type Code = Vec<Line>;
