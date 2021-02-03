use crate::prolog_parser_rebis::ast::*;

use crate::forms::Number;
use crate::machine::machine_indices::*;
use crate::rug::rand::RandState;

use crate::ref_thread_local::RefThreadLocal;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompareNumberQT {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
    Equal,
}

impl CompareNumberQT {
    fn name(self) -> &'static str {
        match self {
            CompareNumberQT::GreaterThan => ">",
            CompareNumberQT::LessThan => "<",
            CompareNumberQT::GreaterThanOrEqual => ">=",
            CompareNumberQT::LessThanOrEqual => "=<",
            CompareNumberQT::NotEqual => "=\\=",
            CompareNumberQT::Equal => "=:=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareTermQT {
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

impl CompareTermQT {
    fn name<'a>(self) -> &'a str {
        match self {
            CompareTermQT::GreaterThan => "@>",
            CompareTermQT::LessThan => "@<",
            CompareTermQT::GreaterThanOrEqual => "@>=",
            CompareTermQT::LessThanOrEqual => "@=<",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticTerm {
    Reg(RegType),
    Interm(usize),
    Number(Number),
}

impl ArithmeticTerm {
    pub fn interm_or(&self, interm: usize) -> usize {
        if let ArithmeticTerm::Interm(interm) = *self {
            interm
        } else {
            interm
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InlinedClauseType {
    CompareNumber(CompareNumberQT, ArithmeticTerm, ArithmeticTerm),
    IsAtom(RegType),
    IsAtomic(RegType),
    IsCompound(RegType),
    IsInteger(RegType),
    IsNumber(RegType),
    IsRational(RegType),
    IsFloat(RegType),
    IsNonVar(RegType),
    IsVar(RegType),
}

ref_thread_local! {
    pub static managed RANDOM_STATE: RandState<'static> = RandState::new();
}

ref_thread_local! {
    pub static managed CLAUSE_TYPE_FORMS: BTreeMap<(&'static str, usize), ClauseType> = {
        let mut m = BTreeMap::new();

        let r1 = temp_v!(1);
        let r2 = temp_v!(2);

        m.insert((">", 2),
                 ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::GreaterThan, ar_reg!(r1), ar_reg!(r2))));
        m.insert(("<", 2),
                 ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::LessThan, ar_reg!(r1), ar_reg!(r2))));
        m.insert((">=", 2), ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::GreaterThanOrEqual, ar_reg!(r1), ar_reg!(r2))));
        m.insert(("=<", 2), ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::LessThanOrEqual, ar_reg!(r1), ar_reg!(r2))));
        m.insert(("=:=", 2), ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::Equal, ar_reg!(r1), ar_reg!(r2))));
        m.insert(("=\\=", 2), ClauseType::Inlined(InlinedClauseType::CompareNumber(CompareNumberQT::NotEqual, ar_reg!(r1), ar_reg!(r2))));
        m.insert(("atom", 1), ClauseType::Inlined(InlinedClauseType::IsAtom(r1)));
        m.insert(("atomic", 1), ClauseType::Inlined(InlinedClauseType::IsAtomic(r1)));
        m.insert(("compound", 1), ClauseType::Inlined(InlinedClauseType::IsCompound(r1)));
        m.insert(("integer", 1), ClauseType::Inlined(InlinedClauseType::IsInteger(r1)));
        m.insert(("number", 1), ClauseType::Inlined(InlinedClauseType::IsNumber(r1)));
        m.insert(("rational", 1), ClauseType::Inlined(InlinedClauseType::IsRational(r1)));
        m.insert(("float", 1), ClauseType::Inlined(InlinedClauseType::IsFloat(r1)));
        m.insert(("nonvar", 1), ClauseType::Inlined(InlinedClauseType::IsNonVar(r1)));
        m.insert(("var", 1), ClauseType::Inlined(InlinedClauseType::IsVar(r1)));
        m.insert(("acyclic_term", 1), ClauseType::BuiltIn(BuiltInClauseType::AcyclicTerm));
        m.insert(("arg", 3), ClauseType::BuiltIn(BuiltInClauseType::Arg));
        m.insert(("compare", 3), ClauseType::BuiltIn(BuiltInClauseType::Compare));
        m.insert(("@>", 2), ClauseType::BuiltIn(BuiltInClauseType::CompareTerm(CompareTermQT::GreaterThan)));
        m.insert(("@<", 2), ClauseType::BuiltIn(BuiltInClauseType::CompareTerm(CompareTermQT::LessThan)));
        m.insert(("@>=", 2), ClauseType::BuiltIn(BuiltInClauseType::CompareTerm(CompareTermQT::GreaterThanOrEqual)));
        m.insert(("@=<", 2), ClauseType::BuiltIn(BuiltInClauseType::CompareTerm(CompareTermQT::LessThanOrEqual)));
        m.insert(("copy_term", 2), ClauseType::BuiltIn(BuiltInClauseType::CopyTerm));
        m.insert(("==", 2), ClauseType::BuiltIn(BuiltInClauseType::Eq));
        m.insert(("functor", 3), ClauseType::BuiltIn(BuiltInClauseType::Functor));
        m.insert(("ground", 1), ClauseType::BuiltIn(BuiltInClauseType::Ground));
        m.insert(("is", 2), ClauseType::BuiltIn(BuiltInClauseType::Is(r1, ar_reg!(r2))));
        m.insert(("keysort", 2), ClauseType::BuiltIn(BuiltInClauseType::KeySort));
        m.insert(("nl", 0), ClauseType::BuiltIn(BuiltInClauseType::Nl));
        m.insert(("\\==", 2), ClauseType::BuiltIn(BuiltInClauseType::NotEq));
        m.insert(("read", 1), ClauseType::BuiltIn(BuiltInClauseType::Read));
        m.insert(("sort", 2), ClauseType::BuiltIn(BuiltInClauseType::Sort));

        m
    };
}

impl InlinedClauseType {
    pub fn name(&self) -> &'static str {
        match *self {
            InlinedClauseType::CompareNumber(qt, ..) => qt.name(),
            InlinedClauseType::IsAtom(..) => "atom",
            InlinedClauseType::IsAtomic(..) => "atomic",
            InlinedClauseType::IsCompound(..) => "compound",
            InlinedClauseType::IsNumber(..) => "number",
            InlinedClauseType::IsInteger(..) => "integer",
            InlinedClauseType::IsRational(..) => "rational",
            InlinedClauseType::IsFloat(..) => "float",
            InlinedClauseType::IsNonVar(..) => "nonvar",
            InlinedClauseType::IsVar(..) => "var",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SystemClauseType {
    // AbolishClause,
    // AbolishModuleClause,
    // AssertDynamicPredicateToBack,
    // AssertDynamicPredicateToFront,
    // AtEndOfExpansion,
    AtomChars,
    AtomCodes,
    AtomLength,
    BindFromRegister,
    CallContinuation,
    CharCode,
    CharType,
    CharsToNumber,
    ClearAttributeGoals,
    CloneAttributeGoals,
    CodesToNumber,
    CopyTermWithoutAttrVars,
    CheckCutPoint,
    Close,
    CopyToLiftedHeap,
    CreatePartialString,
    CurrentHostname,
    CurrentInput,
    CurrentOutput,
    DirectoryFiles,
    FileSize,
    FileExists,
    DirectoryExists,
    DirectorySeparator,
    MakeDirectory,
    DeleteFile,
    WorkingDirectory,
    PathCanonical,
    FileTime,
    DeleteAttribute,
    DeleteHeadAttribute,
    DynamicModuleResolution(usize),
    EnqueueAttributeGoal,
    EnqueueAttributedVar,
    //  ExpandGoal,
    //  ExpandTerm,
    FetchGlobalVar,
    FetchGlobalVarWithOffset,
    FirstStream,
    FlushOutput,
    GetByte,
    GetChar,
    GetNChars,
    GetCode,
    GetSingleChar,
    ResetAttrVarState,
    TruncateIfNoLiftedHeapGrowthDiff,
    TruncateIfNoLiftedHeapGrowth,
    GetAttributedVariableList,
    GetAttrVarQueueDelimiter,
    GetAttrVarQueueBeyond,
    GetBValue,
    //  GetClause,
    GetContinuationChunk,
    //  GetModuleClause,
    GetNextDBRef,
    GetNextOpDBRef,
    IsPartialString,
    LookupDBRef,
    LookupOpDBRef,
    Halt,
    //  ModuleHeadIsDynamic,
    GetLiftedHeapFromOffset,
    GetLiftedHeapFromOffsetDiff,
    GetSCCCleaner,
    HeadIsDynamic,
    InstallSCCCleaner,
    InstallInferenceCounter,
    LiftedHeapLength,
    LoadLibraryAsStream,
    // ModuleAssertDynamicPredicateToFront,
    // ModuleAssertDynamicPredicateToBack,
    ModuleExists,
    // ModuleRetractClause,
    NextEP,
    NoSuchPredicate,
    NumberToChars,
    NumberToCodes,
    OpDeclaration,
    Open,
    NextStream,
    PartialStringTail,
    PeekByte,
    PeekChar,
    PeekCode,
    PointsToContinuationResetMarker,
    PutByte,
    PutChar,
    PutChars,
    PutCode,
    REPL(REPLCodePtr),
    ReadQueryTerm,
    ReadTerm,
    RedoAttrVarBinding,
    RemoveCallPolicyCheck,
    RemoveInferenceCounter,
    ResetContinuationMarker,
    ResetGlobalVarAtKey,
    ResetGlobalVarAtOffset,
    // RetractClause,
    RestoreCutPolicy,
    SetCutPoint(RegType),
    SetInput,
    SetOutput,
    StoreGlobalVar,
    StoreGlobalVarWithOffset,
    StreamProperty,
    SetStreamPosition,
    InferenceLevel,
    CleanUpBlock,
    EraseBall,
    Fail,
    GetBall,
    GetCurrentBlock,
    GetCutPoint,
    GetDoubleQuotes,
    InstallNewBlock,
    Maybe,
    CpuNow,
    CurrentTime,
    QuotedToken,
    ReadTermFromChars,
    ResetBlock,
    ReturnFromVerifyAttr,
    SetBall,
    SetCutPointByDefault(RegType),
    SetDoubleQuotes,
    SetSeed,
    SkipMaxList,
    Sleep,
    SocketClientOpen,
    SocketServerOpen,
    SocketServerAccept,
    SocketServerClose,
    Succeed,
    TermAttributedVariables,
    TermVariables,
    TruncateLiftedHeapTo,
    UnifyWithOccursCheck,
    UnwindEnvironments,
    UnwindStack,
    Variant,
    WAMInstructions,
    WriteTerm,
    WriteTermToChars,
    ScryerPrologVersion,
    CryptoRandomByte,
    CryptoDataHash,
    CryptoDataHKDF,
    CryptoPasswordHash,
    CryptoDataEncrypt,
    CryptoDataDecrypt,
    CryptoCurveScalarMult,
    Ed25519Sign,
    Ed25519Verify,
    Ed25519NewKeyPair,
    Ed25519KeyPairPublicKey,
    Curve25519ScalarMult,
    LoadHTML,
    LoadXML,
    GetEnv,
    SetEnv,
    UnsetEnv,
    CharsBase64,
}

impl SystemClauseType {
    pub fn name(&self) -> ClauseName {
        match *self {
            // Self::AbolishClause => clause_name!("$abolish_clause"),
            // Self::AbolishModuleClause => clause_name!("$abolish_module_clause"),
            // Self::AssertDynamicPredicateToBack => clause_name!("$assertz"),
            // Self::AssertDynamicPredicateToFront => clause_name!("$asserta"),
            // Self::AtEndOfExpansion => clause_name!("$at_end_of_expansion"),
            Self::AtomChars => clause_name!("$atom_chars"),
            Self::AtomCodes => clause_name!("$atom_codes"),
            Self::AtomLength => clause_name!("$atom_length"),
            Self::BindFromRegister => clause_name!("$bind_from_register"),
            Self::CallContinuation => clause_name!("$call_continuation"),
            Self::CharCode => clause_name!("$char_code"),
            Self::CharType => clause_name!("$char_type"),
            Self::CharsToNumber => clause_name!("$chars_to_number"),
            Self::CheckCutPoint => clause_name!("$check_cp"),
            Self::ClearAttributeGoals => clause_name!("$clear_attribute_goals"),
            Self::CloneAttributeGoals => clause_name!("$clone_attribute_goals"),
            Self::CodesToNumber => clause_name!("$codes_to_number"),
            Self::CopyTermWithoutAttrVars => {
                clause_name!("$copy_term_without_attr_vars")
            }
            Self::CreatePartialString => clause_name!("$create_partial_string"),
            Self::CurrentInput => clause_name!("$current_input"),
            Self::CurrentHostname => clause_name!("$current_hostname"),
            Self::CurrentOutput => clause_name!("$current_output"),
            Self::DirectoryFiles => clause_name!("$directory_files"),
            Self::FileSize => clause_name!("$file_size"),
            Self::FileExists => clause_name!("$file_exists"),
            Self::DirectoryExists => clause_name!("$directory_exists"),
            Self::DirectorySeparator => clause_name!("$directory_separator"),
            Self::MakeDirectory => clause_name!("$make_directory"),
            Self::DeleteFile => clause_name!("$delete_file"),
            Self::WorkingDirectory => clause_name!("$working_directory"),
            Self::PathCanonical => clause_name!("$path_canonical"),
            Self::FileTime => clause_name!("$file_time"),
            Self::REPL(REPLCodePtr::AddDynamicPredicate) => {
                clause_name!("$add_dynamic_predicate")
            }
            Self::REPL(REPLCodePtr::AddGoalExpansionClause) => {
                clause_name!("$add_goal_expansion_clause")
            }
            Self::REPL(REPLCodePtr::AddTermExpansionClause) => {
                clause_name!("$add_term_expansion_clause")
            }
            Self::REPL(REPLCodePtr::ClauseToEvacuable) => {
                clause_name!("$clause_to_evacuable")
            }
            Self::REPL(REPLCodePtr::ConcludeLoad) => clause_name!("$conclude_load"),
            Self::REPL(REPLCodePtr::DeclareModule) => clause_name!("$declare_module"),
            Self::REPL(REPLCodePtr::LoadCompiledLibrary) => {
                clause_name!("$load_compiled_library")
            }
            Self::REPL(REPLCodePtr::PushLoadStatePayload) => {
                clause_name!("$push_load_state_payload")
            }
            Self::REPL(REPLCodePtr::UserAsserta) => clause_name!("$asserta"),
            Self::REPL(REPLCodePtr::UserAssertz) => clause_name!("$assertz"),
            Self::REPL(REPLCodePtr::UserRetract) => clause_name!("$retract_clause"),
            Self::REPL(REPLCodePtr::UseModule) => clause_name!("$use_module"),
            Self::REPL(REPLCodePtr::PushLoadContext) => {
                clause_name!("$push_load_context")
            }
            Self::REPL(REPLCodePtr::PopLoadContext) => {
                clause_name!("$pop_load_context")
            }
            Self::REPL(REPLCodePtr::PopLoadStatePayload) => {
                clause_name!("$pop_load_state_payload")
            }
            Self::REPL(REPLCodePtr::LoadContextSource) => {
                clause_name!("$prolog_lc_source")
            }
            Self::REPL(REPLCodePtr::LoadContextFile) => {
                clause_name!("$prolog_lc_file")
            }
            Self::REPL(REPLCodePtr::LoadContextDirectory) => {
                clause_name!("$prolog_lc_dir")
            }
            Self::REPL(REPLCodePtr::LoadContextModule) => {
                clause_name!("$prolog_lc_module")
            }
            Self::REPL(REPLCodePtr::LoadContextStream) => {
                clause_name!("$prolog_lc_stream")
            }
            Self::REPL(REPLCodePtr::MetaPredicateProperty) => {
                clause_name!("$cpp_meta_predicate_property")
            }
            Self::REPL(REPLCodePtr::BuiltInProperty) => {
                clause_name!("$cpp_built_in_property")
            }
            Self::REPL(REPLCodePtr::CompilePendingPredicates) => {
                clause_name!("$compile_pending_predicates")
            }
            Self::Close => clause_name!("$close"),
            Self::CopyToLiftedHeap => clause_name!("$copy_to_lh"),
            Self::DeleteAttribute => clause_name!("$del_attr_non_head"),
            Self::DeleteHeadAttribute => clause_name!("$del_attr_head"),
            Self::DynamicModuleResolution(_) => clause_name!("$module_call"),
            Self::EnqueueAttributeGoal => clause_name!("$enqueue_attribute_goal"),
            Self::EnqueueAttributedVar => clause_name!("$enqueue_attr_var"),
            Self::FetchGlobalVar => clause_name!("$fetch_global_var"),
            Self::FetchGlobalVarWithOffset => {
                clause_name!("$fetch_global_var_with_offset")
            }
            Self::FirstStream => clause_name!("$first_stream"),
            Self::FlushOutput => clause_name!("$flush_output"),
            Self::GetByte => clause_name!("$get_byte"),
            Self::GetChar => clause_name!("$get_char"),
            Self::GetNChars => clause_name!("$get_n_chars"),
            Self::GetCode => clause_name!("$get_code"),
            Self::GetSingleChar => clause_name!("$get_single_char"),
            Self::ResetAttrVarState => clause_name!("$reset_attr_var_state"),
            Self::TruncateIfNoLiftedHeapGrowth => {
                clause_name!("$truncate_if_no_lh_growth")
            }
            Self::TruncateIfNoLiftedHeapGrowthDiff => {
                clause_name!("$truncate_if_no_lh_growth_diff")
            }
            Self::GetAttributedVariableList => clause_name!("$get_attr_list"),
            Self::GetAttrVarQueueDelimiter => {
                clause_name!("$get_attr_var_queue_delim")
            }
            Self::GetAttrVarQueueBeyond => clause_name!("$get_attr_var_queue_beyond"),
            Self::GetContinuationChunk => clause_name!("$get_cont_chunk"),
            Self::GetLiftedHeapFromOffset => clause_name!("$get_lh_from_offset"),
            Self::GetLiftedHeapFromOffsetDiff => {
                clause_name!("$get_lh_from_offset_diff")
            }
            Self::GetBValue => clause_name!("$get_b_value"),
            //          Self::GetClause => clause_name!("$get_clause"),
            Self::GetNextDBRef => clause_name!("$get_next_db_ref"),
            Self::GetNextOpDBRef => clause_name!("$get_next_op_db_ref"),
            Self::LookupDBRef => clause_name!("$lookup_db_ref"),
            Self::LookupOpDBRef => clause_name!("$lookup_op_db_ref"),
            Self::GetDoubleQuotes => clause_name!("$get_double_quotes"),
            //          Self::GetModuleClause => clause_name!("$get_module_clause"),
            Self::GetSCCCleaner => clause_name!("$get_scc_cleaner"),
            Self::Halt => clause_name!("$halt"),
            Self::HeadIsDynamic => clause_name!("$head_is_dynamic"),
            Self::Open => clause_name!("$open"),
            Self::OpDeclaration => clause_name!("$op"),
            Self::InstallSCCCleaner => clause_name!("$install_scc_cleaner"),
            Self::InstallInferenceCounter => {
                clause_name!("$install_inference_counter")
            }
            Self::IsPartialString => clause_name!("$is_partial_string"),
            Self::PartialStringTail => clause_name!("$partial_string_tail"),
            Self::PeekByte => clause_name!("$peek_byte"),
            Self::PeekChar => clause_name!("$peek_char"),
            Self::PeekCode => clause_name!("$peek_code"),
            Self::LiftedHeapLength => clause_name!("$lh_length"),
            Self::Maybe => clause_name!("maybe"),
            Self::CpuNow => clause_name!("$cpu_now"),
            Self::CurrentTime => clause_name!("$current_time"),
            // Self::ModuleAssertDynamicPredicateToFront => {
            //     clause_name!("$module_asserta")
            // }
            // Self::ModuleAssertDynamicPredicateToBack => {
            //     clause_name!("$module_assertz")
            // }
            //          Self::ModuleHeadIsDynamic => clause_name!("$module_head_is_dynamic"),
            Self::ModuleExists => clause_name!("$module_exists"),
            Self::NextStream => clause_name!("$next_stream"),
            Self::NoSuchPredicate => clause_name!("$no_such_predicate"),
            Self::NumberToChars => clause_name!("$number_to_chars"),
            Self::NumberToCodes => clause_name!("$number_to_codes"),
            Self::PointsToContinuationResetMarker => {
                clause_name!("$points_to_cont_reset_marker")
            }
            Self::PutByte => {
                clause_name!("$put_byte")
            }
            Self::PutChar => {
                clause_name!("$put_char")
            }
            Self::PutChars => {
                clause_name!("$put_chars")
            }
            Self::PutCode => {
                clause_name!("$put_code")
            }
            Self::QuotedToken => {
                clause_name!("$quoted_token")
            }
            Self::RedoAttrVarBinding => clause_name!("$redo_attr_var_binding"),
            Self::RemoveCallPolicyCheck => clause_name!("$remove_call_policy_check"),
            Self::RemoveInferenceCounter => clause_name!("$remove_inference_counter"),
            Self::RestoreCutPolicy => clause_name!("$restore_cut_policy"),
            Self::SetCutPoint(_) => clause_name!("$set_cp"),
            Self::SetInput => clause_name!("$set_input"),
            Self::SetOutput => clause_name!("$set_output"),
            Self::SetSeed => clause_name!("$set_seed"),
            Self::StreamProperty => clause_name!("$stream_property"),
            Self::SetStreamPosition => clause_name!("$set_stream_position"),
            Self::StoreGlobalVar => clause_name!("$store_global_var"),
            Self::StoreGlobalVarWithOffset => {
                clause_name!("$store_global_var_with_offset")
            }
            Self::InferenceLevel => clause_name!("$inference_level"),
            Self::CleanUpBlock => clause_name!("$clean_up_block"),
            Self::EraseBall => clause_name!("$erase_ball"),
            Self::Fail => clause_name!("$fail"),
            Self::GetBall => clause_name!("$get_ball"),
            Self::GetCutPoint => clause_name!("$get_cp"),
            Self::GetCurrentBlock => clause_name!("$get_current_block"),
            Self::InstallNewBlock => clause_name!("$install_new_block"),
            // Self::ModuleRetractClause => clause_name!("$module_retract_clause"),
            Self::NextEP => clause_name!("$nextEP"),
            Self::ReadQueryTerm => clause_name!("$read_query_term"),
            Self::ReadTerm => clause_name!("$read_term"),
            Self::ReadTermFromChars => clause_name!("$read_term_from_chars"),
            Self::ResetGlobalVarAtKey => clause_name!("$reset_global_var_at_key"),
            Self::ResetGlobalVarAtOffset => {
                clause_name!("$reset_global_var_at_offset")
            }
            Self::ResetBlock => clause_name!("$reset_block"),
            Self::ResetContinuationMarker => clause_name!("$reset_cont_marker"),
            Self::ReturnFromVerifyAttr => clause_name!("$return_from_verify_attr"),
            Self::SetBall => clause_name!("$set_ball"),
            Self::SetCutPointByDefault(_) => clause_name!("$set_cp_by_default"),
            Self::SetDoubleQuotes => clause_name!("$set_double_quotes"),
            Self::SkipMaxList => clause_name!("$skip_max_list"),
            Self::Sleep => clause_name!("$sleep"),
            Self::SocketClientOpen => clause_name!("$socket_client_open"),
            Self::SocketServerOpen => clause_name!("$socket_server_open"),
            Self::SocketServerAccept => clause_name!("$socket_server_accept"),
            Self::SocketServerClose => clause_name!("$socket_server_close"),
            Self::Succeed => clause_name!("$succeed"),
            Self::TermAttributedVariables => {
                clause_name!("$term_attributed_variables")
            }
            Self::TermVariables => clause_name!("$term_variables"),
            Self::TruncateLiftedHeapTo => clause_name!("$truncate_lh_to"),
            Self::UnifyWithOccursCheck => clause_name!("$unify_with_occurs_check"),
            Self::UnwindEnvironments => clause_name!("$unwind_environments"),
            Self::UnwindStack => clause_name!("$unwind_stack"),
            Self::Variant => clause_name!("$variant"),
            Self::WAMInstructions => clause_name!("$wam_instructions"),
            Self::WriteTerm => clause_name!("$write_term"),
            Self::WriteTermToChars => clause_name!("$write_term_to_chars"),
            Self::ScryerPrologVersion => clause_name!("$scryer_prolog_version"),
            Self::CryptoRandomByte => clause_name!("$crypto_random_byte"),
            Self::CryptoDataHash => clause_name!("$crypto_data_hash"),
            Self::CryptoDataHKDF => clause_name!("$crypto_data_hkdf"),
            Self::CryptoPasswordHash => clause_name!("$crypto_password_hash"),
            Self::CryptoDataEncrypt => clause_name!("$crypto_data_encrypt"),
            Self::CryptoDataDecrypt => clause_name!("$crypto_data_decrypt"),
            Self::CryptoCurveScalarMult => clause_name!("$crypto_curve_scalar_mult"),
            Self::Ed25519Sign => clause_name!("$ed25519_sign"),
            Self::Ed25519Verify => clause_name!("$ed25519_verify"),
            Self::Ed25519NewKeyPair => clause_name!("$ed25519_new_keypair"),
            Self::Ed25519KeyPairPublicKey => {
                clause_name!("$ed25519_keypair_public_key")
            }
            Self::Curve25519ScalarMult => clause_name!("$curve25519_scalar_mult"),
            Self::LoadHTML => clause_name!("$load_html"),
            Self::LoadXML => clause_name!("$load_xml"),
            Self::GetEnv => clause_name!("$getenv"),
            Self::SetEnv => clause_name!("$setenv"),
            Self::UnsetEnv => clause_name!("$unsetenv"),
            Self::CharsBase64 => clause_name!("$chars_base64"),
            Self::LoadLibraryAsStream => clause_name!("$load_library_as_stream"),
        }
    }

    pub fn from(name: &str, arity: usize) -> Option<SystemClauseType> {
        match (name, arity) {
            // ("$abolish_clause", 2) => Some(SystemClauseType::AbolishClause),
            ("$add_dynamic_predicate", 3) => {
                Some(SystemClauseType::REPL(REPLCodePtr::AddDynamicPredicate))
            }
            ("$add_goal_expansion_clause", 4) => {
                Some(SystemClauseType::REPL(REPLCodePtr::AddGoalExpansionClause))
            }
            ("$add_term_expansion_clause", 3) => {
                Some(SystemClauseType::REPL(REPLCodePtr::AddTermExpansionClause))
            }
            // ("$at_end_of_expansion", 0) => Some(SystemClauseType::AtEndOfExpansion),
            ("$atom_chars", 2) => Some(SystemClauseType::AtomChars),
            ("$atom_codes", 2) => Some(SystemClauseType::AtomCodes),
            ("$atom_length", 2) => Some(SystemClauseType::AtomLength),
            // ("$abolish_module_clause", 3) => Some(SystemClauseType::AbolishModuleClause),
            ("$bind_from_register", 2) => Some(SystemClauseType::BindFromRegister),
            // ("$module_asserta", 5) => Some(SystemClauseType::ModuleAssertDynamicPredicateToFront),
            // ("$module_assertz", 5) => Some(SystemClauseType::ModuleAssertDynamicPredicateToBack),
            ("$call_continuation", 1) => Some(SystemClauseType::CallContinuation),
            ("$char_code", 2) => Some(SystemClauseType::CharCode),
            ("$char_type", 2) => Some(SystemClauseType::CharType),
            ("$chars_to_number", 2) => Some(SystemClauseType::CharsToNumber),
            ("$clear_attribute_goals", 0) => Some(SystemClauseType::ClearAttributeGoals),
            ("$clone_attribute_goals", 1) => Some(SystemClauseType::CloneAttributeGoals),
            ("$codes_to_number", 2) => Some(SystemClauseType::CodesToNumber),
            ("$copy_term_without_attr_vars", 2) => Some(SystemClauseType::CopyTermWithoutAttrVars),
            ("$create_partial_string", 3) => Some(SystemClauseType::CreatePartialString),
            ("$check_cp", 1) => Some(SystemClauseType::CheckCutPoint),
            ("$copy_to_lh", 2) => Some(SystemClauseType::CopyToLiftedHeap),
            ("$close", 2) => Some(SystemClauseType::Close),
            ("$current_hostname", 1) => Some(SystemClauseType::CurrentHostname),
            ("$current_input", 1) => Some(SystemClauseType::CurrentInput),
            ("$current_output", 1) => Some(SystemClauseType::CurrentOutput),
            ("$first_stream", 1) => Some(SystemClauseType::FirstStream),
            ("$next_stream", 2) => Some(SystemClauseType::NextStream),
            ("$flush_output", 1) => Some(SystemClauseType::FlushOutput),
            ("$del_attr_non_head", 1) => Some(SystemClauseType::DeleteAttribute),
            ("$del_attr_head", 1) => Some(SystemClauseType::DeleteHeadAttribute),
            ("$get_next_db_ref", 2) => Some(SystemClauseType::GetNextDBRef),
            ("$get_next_op_db_ref", 2) => Some(SystemClauseType::GetNextOpDBRef),
            ("$lookup_db_ref", 3) => Some(SystemClauseType::LookupDBRef),
            ("$lookup_op_db_ref", 4) => Some(SystemClauseType::LookupOpDBRef),
            ("$module_call", _) => Some(SystemClauseType::DynamicModuleResolution(arity - 2)),
            ("$enqueue_attribute_goal", 1) => Some(SystemClauseType::EnqueueAttributeGoal),
            ("$enqueue_attr_var", 1) => Some(SystemClauseType::EnqueueAttributedVar),
            ("$partial_string_tail", 2) => Some(SystemClauseType::PartialStringTail),
            ("$peek_byte", 2) => Some(SystemClauseType::PeekByte),
            ("$peek_char", 2) => Some(SystemClauseType::PeekChar),
            ("$peek_code", 2) => Some(SystemClauseType::PeekCode),
            ("$is_partial_string", 1) => Some(SystemClauseType::IsPartialString),
            //          ("$expand_term", 2) => Some(SystemClauseType::ExpandTerm),
            //          ("$expand_goal", 2) => Some(SystemClauseType::ExpandGoal),
            ("$fetch_global_var", 2) => Some(SystemClauseType::FetchGlobalVar),
            ("$fetch_global_var_with_offset", 3) => {
                Some(SystemClauseType::FetchGlobalVarWithOffset)
            }
            ("$get_byte", 2) => Some(SystemClauseType::GetByte),
            ("$get_char", 2) => Some(SystemClauseType::GetChar),
            ("$get_n_chars", 3) => Some(SystemClauseType::GetNChars),
            ("$get_code", 2) => Some(SystemClauseType::GetCode),
            ("$get_single_char", 1) => Some(SystemClauseType::GetSingleChar),
            ("$points_to_cont_reset_marker", 1) => {
                Some(SystemClauseType::PointsToContinuationResetMarker)
            }
            ("$put_byte", 2) => Some(SystemClauseType::PutByte),
            ("$put_char", 2) => Some(SystemClauseType::PutChar),
            ("$put_chars", 2) => Some(SystemClauseType::PutChars),
            ("$put_code", 2) => Some(SystemClauseType::PutCode),
            ("$reset_attr_var_state", 0) => Some(SystemClauseType::ResetAttrVarState),
            ("$truncate_if_no_lh_growth", 1) => {
                Some(SystemClauseType::TruncateIfNoLiftedHeapGrowth)
            }
            ("$truncate_if_no_lh_growth_diff", 2) => {
                Some(SystemClauseType::TruncateIfNoLiftedHeapGrowthDiff)
            }
            ("$get_attr_list", 2) => Some(SystemClauseType::GetAttributedVariableList),
            ("$get_b_value", 1) => Some(SystemClauseType::GetBValue),
            //          ("$get_clause", 2) => Some(SystemClauseType::GetClause),
            //          ("$get_module_clause", 3) => Some(SystemClauseType::GetModuleClause),
            ("$get_lh_from_offset", 2) => Some(SystemClauseType::GetLiftedHeapFromOffset),
            ("$get_lh_from_offset_diff", 3) => Some(SystemClauseType::GetLiftedHeapFromOffsetDiff),
            ("$get_double_quotes", 1) => Some(SystemClauseType::GetDoubleQuotes),
            ("$get_scc_cleaner", 1) => Some(SystemClauseType::GetSCCCleaner),
            ("$halt", 1) => Some(SystemClauseType::Halt),
            ("$head_is_dynamic", 2) => Some(SystemClauseType::HeadIsDynamic),
            ("$install_scc_cleaner", 2) => Some(SystemClauseType::InstallSCCCleaner),
            ("$install_inference_counter", 3) => Some(SystemClauseType::InstallInferenceCounter),
            ("$lh_length", 1) => Some(SystemClauseType::LiftedHeapLength),
            ("$maybe", 0) => Some(SystemClauseType::Maybe),
            ("$cpu_now", 1) => Some(SystemClauseType::CpuNow),
            ("$current_time", 1) => Some(SystemClauseType::CurrentTime),
            ("$module_exists", 1) => Some(SystemClauseType::ModuleExists),
            // ("$module_retract_clause", 5) => Some(SystemClauseType::ModuleRetractClause),
            // ("$module_head_is_dynamic", 2) => Some(SystemClauseType::ModuleHeadIsDynamic),
            ("$no_such_predicate", 2) => Some(SystemClauseType::NoSuchPredicate),
            ("$number_to_chars", 2) => Some(SystemClauseType::NumberToChars),
            ("$number_to_codes", 2) => Some(SystemClauseType::NumberToCodes),
            ("$op", 3) => Some(SystemClauseType::OpDeclaration),
            ("$open", 7) => Some(SystemClauseType::Open),
            ("$redo_attr_var_binding", 2) => Some(SystemClauseType::RedoAttrVarBinding),
            ("$remove_call_policy_check", 1) => Some(SystemClauseType::RemoveCallPolicyCheck),
            ("$remove_inference_counter", 2) => Some(SystemClauseType::RemoveInferenceCounter),
            ("$restore_cut_policy", 0) => Some(SystemClauseType::RestoreCutPolicy),
            ("$set_cp", 1) => Some(SystemClauseType::SetCutPoint(temp_v!(1))),
            ("$set_input", 1) => Some(SystemClauseType::SetInput),
            ("$set_output", 1) => Some(SystemClauseType::SetOutput),
            ("$stream_property", 3) => Some(SystemClauseType::StreamProperty),
            ("$set_stream_position", 2) => Some(SystemClauseType::SetStreamPosition),
            ("$inference_level", 2) => Some(SystemClauseType::InferenceLevel),
            ("$clean_up_block", 1) => Some(SystemClauseType::CleanUpBlock),
            ("$erase_ball", 0) => Some(SystemClauseType::EraseBall),
            ("$fail", 0) => Some(SystemClauseType::Fail),
            ("$get_attr_var_queue_beyond", 2) => Some(SystemClauseType::GetAttrVarQueueBeyond),
            ("$get_attr_var_queue_delim", 1) => Some(SystemClauseType::GetAttrVarQueueDelimiter),
            ("$get_ball", 1) => Some(SystemClauseType::GetBall),
            ("$get_cont_chunk", 3) => Some(SystemClauseType::GetContinuationChunk),
            ("$get_current_block", 1) => Some(SystemClauseType::GetCurrentBlock),
            ("$get_cp", 1) => Some(SystemClauseType::GetCutPoint),
            ("$install_new_block", 1) => Some(SystemClauseType::InstallNewBlock),
            ("$quoted_token", 1) => Some(SystemClauseType::QuotedToken),
            ("$nextEP", 3) => Some(SystemClauseType::NextEP),
            ("$read_query_term", 5) => Some(SystemClauseType::ReadQueryTerm),
            ("$read_term", 5) => Some(SystemClauseType::ReadTerm),
            ("$read_term_from_chars", 2) => Some(SystemClauseType::ReadTermFromChars),
            ("$reset_block", 1) => Some(SystemClauseType::ResetBlock),
            ("$reset_cont_marker", 0) => Some(SystemClauseType::ResetContinuationMarker),
            ("$reset_global_var_at_key", 1) => Some(SystemClauseType::ResetGlobalVarAtKey),
            ("$reset_global_var_at_offset", 3) => Some(SystemClauseType::ResetGlobalVarAtOffset),
            // ("$retract_clause", 4) => Some(SystemClauseType::RetractClause),
            ("$return_from_verify_attr", 0) => Some(SystemClauseType::ReturnFromVerifyAttr),
            ("$set_ball", 1) => Some(SystemClauseType::SetBall),
            ("$set_cp_by_default", 1) => Some(SystemClauseType::SetCutPointByDefault(temp_v!(1))),
            ("$set_double_quotes", 1) => Some(SystemClauseType::SetDoubleQuotes),
            ("$set_seed", 1) => Some(SystemClauseType::SetSeed),
            ("$skip_max_list", 4) => Some(SystemClauseType::SkipMaxList),
            ("$sleep", 1) => Some(SystemClauseType::Sleep),
            ("$socket_client_open", 8) => Some(SystemClauseType::SocketClientOpen),
            ("$socket_server_open", 3) => Some(SystemClauseType::SocketServerOpen),
            ("$socket_server_accept", 7) => Some(SystemClauseType::SocketServerAccept),
            ("$socket_server_close", 1) => Some(SystemClauseType::SocketServerClose),
            ("$store_global_var", 2) => Some(SystemClauseType::StoreGlobalVar),
            ("$store_global_var_with_offset", 2) => {
                Some(SystemClauseType::StoreGlobalVarWithOffset)
            }
            ("$term_attributed_variables", 2) => Some(SystemClauseType::TermAttributedVariables),
            ("$term_variables", 2) => Some(SystemClauseType::TermVariables),
            ("$truncate_lh_to", 1) => Some(SystemClauseType::TruncateLiftedHeapTo),
            ("$unwind_environments", 0) => Some(SystemClauseType::UnwindEnvironments),
            ("$unwind_stack", 0) => Some(SystemClauseType::UnwindStack),
            ("$unify_with_occurs_check", 2) => Some(SystemClauseType::UnifyWithOccursCheck),
            ("$directory_files", 2) => Some(SystemClauseType::DirectoryFiles),
            ("$file_size", 2) => Some(SystemClauseType::FileSize),
            ("$file_exists", 1) => Some(SystemClauseType::FileExists),
            ("$directory_exists", 1) => Some(SystemClauseType::DirectoryExists),
            ("$directory_separator", 1) => Some(SystemClauseType::DirectorySeparator),
            ("$make_directory", 1) => Some(SystemClauseType::MakeDirectory),
            ("$delete_file", 1) => Some(SystemClauseType::DeleteFile),
            ("$working_directory", 2) => Some(SystemClauseType::WorkingDirectory),
            ("$path_canonical", 2) => Some(SystemClauseType::PathCanonical),
            ("$file_time", 3) => Some(SystemClauseType::FileTime),
            ("$clause_to_evacuable", 3) => {
                Some(SystemClauseType::REPL(REPLCodePtr::ClauseToEvacuable))
            }
            ("$conclude_load", 1) => Some(SystemClauseType::REPL(REPLCodePtr::ConcludeLoad)),
            ("$use_module", 3) => Some(SystemClauseType::REPL(REPLCodePtr::UseModule)),
            ("$declare_module", 3) => Some(SystemClauseType::REPL(REPLCodePtr::DeclareModule)),
            ("$load_compiled_library", 2) => {
                Some(SystemClauseType::REPL(REPLCodePtr::LoadCompiledLibrary))
            }
            ("$push_load_state_payload", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::PushLoadStatePayload))
            }
            ("$asserta", 4) => Some(SystemClauseType::REPL(REPLCodePtr::UserAsserta)),
            ("$assertz", 4) => Some(SystemClauseType::REPL(REPLCodePtr::UserAssertz)),
            ("$retract_clause", 3) => Some(SystemClauseType::REPL(REPLCodePtr::UserRetract)),
            ("$variant", 2) => Some(SystemClauseType::Variant),
            ("$wam_instructions", 4) => Some(SystemClauseType::WAMInstructions),
            ("$write_term", 7) => Some(SystemClauseType::WriteTerm),
            ("$write_term_to_chars", 7) => Some(SystemClauseType::WriteTermToChars),
            ("$scryer_prolog_version", 1) => Some(SystemClauseType::ScryerPrologVersion),
            ("$crypto_random_byte", 1) => Some(SystemClauseType::CryptoRandomByte),
            ("$crypto_data_hash", 4) => Some(SystemClauseType::CryptoDataHash),
            ("$crypto_data_hkdf", 7) => Some(SystemClauseType::CryptoDataHKDF),
            ("$crypto_password_hash", 4) => Some(SystemClauseType::CryptoPasswordHash),
            ("$crypto_data_encrypt", 7) => Some(SystemClauseType::CryptoDataEncrypt),
            ("$crypto_data_decrypt", 6) => Some(SystemClauseType::CryptoDataDecrypt),
            ("$crypto_curve_scalar_mult", 5) => Some(SystemClauseType::CryptoCurveScalarMult),
            ("$ed25519_sign", 4) => Some(SystemClauseType::Ed25519Sign),
            ("$ed25519_verify", 4) => Some(SystemClauseType::Ed25519Verify),
            ("$ed25519_new_keypair", 1) => Some(SystemClauseType::Ed25519NewKeyPair),
            ("$ed25519_keypair_public_key", 2) => Some(SystemClauseType::Ed25519KeyPairPublicKey),
            ("$curve25519_scalar_mult", 3) => Some(SystemClauseType::Curve25519ScalarMult),
            ("$load_html", 3) => Some(SystemClauseType::LoadHTML),
            ("$load_xml", 3) => Some(SystemClauseType::LoadXML),
            ("$getenv", 2) => Some(SystemClauseType::GetEnv),
            ("$setenv", 2) => Some(SystemClauseType::SetEnv),
            ("$unsetenv", 1) => Some(SystemClauseType::UnsetEnv),
            ("$chars_base64", 4) => Some(SystemClauseType::CharsBase64),
            ("$load_library_as_stream", 3) => Some(SystemClauseType::LoadLibraryAsStream),
            ("$push_load_context", 2) => Some(SystemClauseType::REPL(REPLCodePtr::PushLoadContext)),
            ("$pop_load_state_payload", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::PopLoadStatePayload))
            }
            ("$pop_load_context", 0) => Some(SystemClauseType::REPL(REPLCodePtr::PopLoadContext)),
            ("$prolog_lc_source", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::LoadContextSource))
            }
            ("$prolog_lc_file", 1) => Some(SystemClauseType::REPL(REPLCodePtr::LoadContextFile)),
            ("$prolog_lc_dir", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::LoadContextDirectory))
            }
            ("$prolog_lc_module", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::LoadContextModule))
            }
            ("$prolog_lc_stream", 1) => {
                Some(SystemClauseType::REPL(REPLCodePtr::LoadContextStream))
            }
            ("$cpp_meta_predicate_property", 4) => {
                Some(SystemClauseType::REPL(REPLCodePtr::MetaPredicateProperty))
            }
            ("$cpp_built_in_property", 2) => {
                Some(SystemClauseType::REPL(REPLCodePtr::BuiltInProperty))
            }
            ("$compile_pending_predicates", 1) => Some(SystemClauseType::REPL(
                REPLCodePtr::CompilePendingPredicates,
            )),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BuiltInClauseType {
    AcyclicTerm,
    Arg,
    Compare,
    CompareTerm(CompareTermQT),
    CopyTerm,
    Eq,
    Functor,
    Ground,
    Is(RegType, ArithmeticTerm),
    KeySort,
    Nl,
    NotEq,
    Read,
    Sort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseType {
    BuiltIn(BuiltInClauseType),
    CallN,
    Inlined(InlinedClauseType),
    Named(ClauseName, usize, CodeIndex), // name, arity, index.
    Op(ClauseName, SharedOpDesc, CodeIndex),
    System(SystemClauseType),
}

impl BuiltInClauseType {
    pub fn name(&self) -> ClauseName {
        match *self {
            Self::AcyclicTerm => clause_name!("acyclic_term"),
            Self::Arg => clause_name!("arg"),
            Self::Compare => clause_name!("compare"),
            Self::CompareTerm(qt) => clause_name!(qt.name()),
            Self::CopyTerm => clause_name!("copy_term"),
            Self::Eq => clause_name!("=="),
            Self::Functor => clause_name!("functor"),
            Self::Ground => clause_name!("ground"),
            Self::Is(..) => clause_name!("is"),
            Self::KeySort => clause_name!("keysort"),
            Self::Nl => clause_name!("nl"),
            Self::NotEq => clause_name!("\\=="),
            Self::Read => clause_name!("read"),
            Self::Sort => clause_name!("sort"),
        }
    }

    pub fn arity(&self) -> usize {
        match *self {
            Self::AcyclicTerm => 1,
            Self::Arg => 3,
            Self::Compare => 2,
            Self::CompareTerm(_) => 2,
            Self::CopyTerm => 2,
            Self::Eq => 2,
            Self::Functor => 3,
            Self::Ground => 1,
            Self::Is(..) => 2,
            Self::KeySort => 2,
            Self::NotEq => 2,
            Self::Nl => 0,
            Self::Read => 1,
            Self::Sort => 2,
        }
    }
}

impl ClauseType {
    pub fn spec(&self) -> Option<SharedOpDesc> {
        match *self {
            Self::Op(_, ref spec, _) => Some(spec.clone()),
            Self::Inlined(InlinedClauseType::CompareNumber(..))
            | Self::BuiltIn(BuiltInClauseType::Is(..))
            | Self::BuiltIn(BuiltInClauseType::CompareTerm(_))
            | Self::BuiltIn(BuiltInClauseType::NotEq)
            | Self::BuiltIn(BuiltInClauseType::Eq) => Some(SharedOpDesc::new(700, XFX)),
            _ => None,
        }
    }

    pub fn name(&self) -> ClauseName {
        match *self {
            Self::BuiltIn(ref built_in) => built_in.name(),
            Self::CallN => clause_name!("call"),
            Self::Inlined(ref inlined) => clause_name!(inlined.name()),
            Self::Op(ref name, ..) => name.clone(),
            Self::Named(ref name, ..) => name.clone(),
            Self::System(ref system) => system.name(),
        }
    }

    pub fn from(name: ClauseName, arity: usize, spec: Option<SharedOpDesc>) -> Self {
        CLAUSE_TYPE_FORMS
            .borrow()
            .get(&(name.as_str(), arity))
            .cloned()
            .unwrap_or_else(|| {
                SystemClauseType::from(name.as_str(), arity)
                    .map(ClauseType::System)
                    .unwrap_or_else(|| {
                        if let Some(spec) = spec {
                            ClauseType::Op(name, spec, CodeIndex::default())
                        } else if name.as_str() == "call" {
                            ClauseType::CallN
                        } else {
                            ClauseType::Named(name, arity, CodeIndex::default())
                        }
                    })
            })
    }
}

impl From<InlinedClauseType> for ClauseType {
    fn from(inlined_ct: InlinedClauseType) -> Self {
        ClauseType::Inlined(inlined_ct)
    }
}
