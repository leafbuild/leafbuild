// auto-generated: "lalrpop 0.19.0"
// sha256: b1dde03eef2ac5359d1747df5da13cb5af987f85b1e95a9d8af2169452e1
use std::str::FromStr;
use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 35, 0, 0, -22, 36, 0, -22, 0, -22, -22, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 4
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 5
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 6
        0, 0, 0, 4, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 8
        0, 0, 0, 0, 43, 0, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 35, 0, 0, -21, 36, 0, -21, 0, -21, -21, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, -57, 0, 0, 33, 0, -57, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, -44, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, -40, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 14
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 15
        0, 0, 0, 4, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 16
        0, 0, 0, 0, -54, 0, 0, 33, 0, -54, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 18
        0, 0, 0, 4, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 19
        0, 0, 0, 0, -34, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -35, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 22
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 23
        0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 24
        0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 25
        0, -71, 0, 0, -71, -71, 0, -71, 0, -71, -71, 0, -53, -71, 0, 0, 0, 0, 0,
        // State 26
        0, -72, 0, 7, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 0, 0, 0, 0,
        // State 27
        0, -73, 0, 0, -73, -73, 0, -73, 0, -73, -73, 0, -52, -73, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -26, 0, 0, -26, -26, 0, -26, 0, -26, -26, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 30
        0, -48, 0, -48, -48, -48, 0, -48, 0, -48, -48, 0, -48, -48, 0, -48, 0, 0, 0,
        // State 31
        0, -69, 0, 0, -69, -69, 0, -69, 0, -69, -69, 0, -50, -69, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 33
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 34
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 35
        0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 36
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 37
        0, -25, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0, 0, -25, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -72, 0, 7, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 14, 0, 0, 0,
        // State 40
        0, -49, 0, 0, -49, -49, 0, -49, 0, -49, -49, 0, -49, -49, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -70, 0, 0, -70, -70, 0, -70, 0, -70, -70, 0, 0, -70, 0, 0, 0, 0, 0,
        // State 43
        0, -47, 0, 0, -47, -47, 0, -47, 0, -47, -47, 0, -47, -47, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -45, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -41, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0,
        // State 47
        0, 0, 0, 0, -6, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -36, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -37, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -74,
        // State 2
        -22,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        -21,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -71,
        // State 26
        -72,
        // State 27
        -73,
        // State 28
        0,
        // State 29
        -26,
        // State 30
        -48,
        // State 31
        -69,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -25,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -49,
        // State 41
        0,
        // State 42
        -70,
        // State 43
        -47,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                19 => 51,
                20 => 52,
                _ => 44,
            },
            6 => 45,
            10 => match state {
                0 => 1,
                3 => 8,
                13 => 16,
                _ => 10,
            },
            11 => 4,
            12 => match state {
                4 => 9,
                _ => 2,
            },
            13 => 5,
            14 => 38,
            15 => match state {
                7 => 40,
                _ => 25,
            },
            16 => match state {
                6 | 15 | 18 => 39,
                7 => 41,
                14 | 17 | 21..=24 => 46,
                _ => 26,
            },
            17 => 27,
            18 => 28,
            19 => match state {
                6 => 11,
                15 => 19,
                18 => 20,
                17 | 23..=24 => 49,
                _ => 47,
            },
            21 => match state {
                15 => 48,
                18 => 50,
                _ => 12,
            },
            27 => match state {
                5 => 37,
                _ => 29,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Box<Expr>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                // __Expr = Expr => ActionFn(4);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __FuncArgs = FuncArgs => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __IdExpression = IdExpression => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __NamedFuncArg = NamedFuncArg => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 33)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__FuncArgs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 1
        0, 0, 0, 0, -57, 0, 0, 33, 0, -57, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 35, 0, 0, -22, 36, 0, -22, 0, -22, -22, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, -44, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, -40, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 6
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 7
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 8
        0, 0, 0, 6, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 9
        0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 11
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 12
        0, 0, 0, 6, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 13
        0, 0, 0, 0, 48, 0, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 35, 0, 0, -21, 36, 0, -21, 0, -21, -21, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, -54, 0, 0, 33, 0, -54, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 17
        0, 0, 0, 6, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 18
        0, 0, 0, 0, -34, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, -35, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 21
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 22
        0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 23
        0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -71, 0, 0, -71, -71, 0, -71, 0, -71, -71, 0, -53, -71, 0, 0, 0, 0, 0,
        // State 26
        0, -72, 0, 9, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 10, 0, 0, 0,
        // State 27
        0, -73, 0, 0, -73, -73, 0, -73, 0, -73, -73, 0, -52, -73, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -26, 0, 0, -26, -26, 0, -26, 0, -26, -26, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 30
        0, -48, 0, -48, -48, -48, 0, -48, 0, -48, -48, 0, -48, -48, 0, -48, 0, 0, 0,
        // State 31
        0, -69, 0, 0, -69, -69, 0, -69, 0, -69, -69, 0, -50, -69, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 33
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 34
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 35
        0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 36
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 37
        0, 0, 0, 0, -45, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, -41, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -72, 0, 9, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 0, 0, 0, 0,
        // State 40
        0, -25, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0, 0, -25, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -49, 0, 0, -49, -49, 0, -49, 0, -49, -49, 0, -49, -49, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -6, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -70, 0, 0, -70, -70, 0, -70, 0, -70, -70, 0, 0, -70, 0, 0, 0, 0, 0,
        // State 48
        0, -47, 0, 0, -47, -47, 0, -47, 0, -47, -47, 0, -47, -47, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -36, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -37, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -46,
        // State 1
        -57,
        // State 2
        -22,
        // State 3
        -44,
        // State 4
        -40,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -42,
        // State 12
        -38,
        // State 13
        0,
        // State 14
        -21,
        // State 15
        -54,
        // State 16
        -43,
        // State 17
        -39,
        // State 18
        -34,
        // State 19
        -35,
        // State 20
        -30,
        // State 21
        -31,
        // State 22
        -32,
        // State 23
        -33,
        // State 24
        -75,
        // State 25
        -71,
        // State 26
        -72,
        // State 27
        -73,
        // State 28
        0,
        // State 29
        -26,
        // State 30
        -48,
        // State 31
        -69,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -45,
        // State 38
        -41,
        // State 39
        -72,
        // State 40
        -25,
        // State 41
        0,
        // State 42
        -49,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -6,
        // State 46
        -11,
        // State 47
        -70,
        // State 48
        -47,
        // State 49
        -7,
        // State 50
        -12,
        // State 51
        -36,
        // State 52
        -37,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                18 => 51,
                19 => 52,
                _ => 37,
            },
            6 => 38,
            10 => match state {
                5 => 13,
                9 => 15,
                _ => 1,
            },
            11 => 6,
            12 => match state {
                6 => 14,
                _ => 2,
            },
            13 => 7,
            14 => match state {
                8 => 41,
                _ => 24,
            },
            15 => match state {
                10 => 42,
                _ => 25,
            },
            16 => match state {
                5..=7 | 9 => 39,
                10 => 43,
                11 | 16 | 20..=23 => 44,
                _ => 26,
            },
            17 => 27,
            18 => 28,
            19 => match state {
                12 => 18,
                17 => 19,
                11 | 20..=21 => 45,
                16 | 22..=23 => 49,
                _ => 3,
            },
            21 => match state {
                12 => 46,
                17 => 50,
                _ => 4,
            },
            27 => match state {
                7 => 40,
                _ => 29,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = AstFuncCallArgs;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct FuncArgsParser {
        _priv: (),
    }

    impl FuncArgsParser {
        pub fn new() -> FuncArgsParser {
            FuncArgsParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<AstFuncCallArgs, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<AstFuncCallArgs,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            74 => {
                // __FuncArgs = FuncArgs => ActionFn(1);
                let __sym0 = __pop_Variant12(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __IdExpression = IdExpression => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __NamedFuncArg = NamedFuncArg => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 33)
    }
}
pub use self::__parse__FuncArgs::FuncArgsParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__IdExpression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -76,
        // State 2
        -48,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            16 => 1,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = String;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct IdExpressionParser {
        _priv: (),
    }

    impl IdExpressionParser {
        pub fn new() -> IdExpressionParser {
            IdExpressionParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<String, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<String,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            75 => {
                // __IdExpression = IdExpression => ActionFn(5);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __FuncArgs = FuncArgs => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __NamedFuncArg = NamedFuncArg => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 33)
    }
}
pub use self::__parse__IdExpression::IdExpressionParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__NamedFuncArg {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 1
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 2
        0, 0, 0, 0, -54, 0, 0, 34, 0, -54, 35, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 36, 0, 0, -22, 37, 0, -22, 0, -22, -22, 0, 0, 38, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 5
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 6
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 7
        0, 0, 0, 5, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 9
        0, 0, 0, 0, 44, 0, 0, 34, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 36, 0, 0, -21, 37, 0, -21, 0, -21, -21, 0, 0, 38, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, -57, 0, 0, 34, 0, -57, 35, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, -44, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, -40, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 15
        0, 0, 0, 5, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 16
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 17
        0, 0, 0, 5, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 33,
        // State 18
        0, 0, 0, 0, -34, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, -35, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 21
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 22
        0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 23
        0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -48, 0, -48, -48, -48, 0, -48, 0, -48, -48, 0, -48, -48, 0, -48, 0, 0, 0,
        // State 27
        0, -71, 0, 0, -71, -71, 0, -71, 0, -71, -71, 0, -53, -71, 0, 0, 0, 0, 0,
        // State 28
        0, -72, 0, 8, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 0, 0, 0, 0,
        // State 29
        0, -73, 0, 0, -73, -73, 0, -73, 0, -73, -73, 0, -52, -73, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -26, 0, 0, -26, -26, 0, -26, 0, -26, -26, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 32
        0, -69, 0, 0, -69, -69, 0, -69, 0, -69, -69, 0, -50, -69, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 34
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 35
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 36
        0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 37
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 38
        0, -25, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0, 0, -25, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -72, 0, 8, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 2, 0, 0, 0,
        // State 41
        0, -49, 0, 0, -49, -49, 0, -49, 0, -49, -49, 0, -49, -49, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -70, 0, 0, -70, -70, 0, -70, 0, -70, -70, 0, 0, -70, 0, 0, 0, 0, 0,
        // State 44
        0, -47, 0, 0, -47, -47, 0, -47, 0, -47, -47, 0, -47, -47, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -45, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, -41, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, -6, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -36, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -37, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -54,
        // State 3
        -22,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -21,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -77,
        // State 26
        -48,
        // State 27
        -71,
        // State 28
        -72,
        // State 29
        -73,
        // State 30
        0,
        // State 31
        -26,
        // State 32
        -69,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -25,
        // State 39
        0,
        // State 40
        0,
        // State 41
        -49,
        // State 42
        0,
        // State 43
        -70,
        // State 44
        -47,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                18 => 51,
                19 => 52,
                _ => 45,
            },
            6 => 46,
            10 => match state {
                1 => 2,
                4 => 9,
                _ => 11,
            },
            11 => 5,
            12 => match state {
                5 => 10,
                _ => 3,
            },
            13 => 6,
            14 => 39,
            15 => match state {
                8 => 41,
                _ => 27,
            },
            16 => match state {
                1 | 4..=6 => 28,
                7 | 15 | 17 => 40,
                8 => 42,
                _ => 24,
            },
            17 => 29,
            18 => 30,
            19 => match state {
                7 => 12,
                15 => 18,
                17 => 19,
                0 => 25,
                16 | 22..=23 => 49,
                _ => 47,
            },
            21 => match state {
                15 => 48,
                17 => 50,
                _ => 13,
            },
            27 => match state {
                6 => 38,
                _ => 31,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = AstNamedArg;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct NamedFuncArgParser {
        _priv: (),
    }

    impl NamedFuncArgParser {
        pub fn new() -> NamedFuncArgParser {
            NamedFuncArgParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<AstNamedArg, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<AstNamedArg,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            76 => {
                // __NamedFuncArg = NamedFuncArg => ActionFn(3);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __FuncArgs = FuncArgs => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __IdExpression = IdExpression => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 33)
    }
}
pub use self::__parse__NamedFuncArg::NamedFuncArgParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__PositionalFuncArg {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 1
        0, 0, 0, 0, -57, 0, 0, 33, 0, -57, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 35, 0, 0, -22, 36, 0, -22, 0, -22, -22, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 4
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 5
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 6
        0, 0, 0, 4, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 8
        0, 0, 0, 0, 43, 0, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 35, 0, 0, -21, 36, 0, -21, 0, -21, -21, 0, 0, 37, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, -44, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, -40, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 13
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 14
        0, 0, 0, 4, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 15
        0, 0, 0, 0, -54, 0, 0, 33, 0, -54, 34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 17
        0, 0, 0, 4, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32,
        // State 18
        0, 0, 0, 0, -34, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, -35, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 21
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 22
        0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 23
        0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0,
        // State 24
        0, -71, 0, 0, -71, -71, 0, -71, 0, -71, -71, 0, -53, -71, 0, 0, 0, 0, 0,
        // State 25
        0, -72, 0, 7, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 0, 0, 0, 0,
        // State 26
        0, -73, 0, 0, -73, -73, 0, -73, 0, -73, -73, 0, -52, -73, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -26, 0, 0, -26, -26, 0, -26, 0, -26, -26, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 30
        0, -48, 0, -48, -48, -48, 0, -48, 0, -48, -48, 0, -48, -48, 0, -48, 0, 0, 0,
        // State 31
        0, -69, 0, 0, -69, -69, 0, -69, 0, -69, -69, 0, -50, -69, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 33
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 34
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 35
        0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 36
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 37
        0, -25, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0, 0, -25, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -72, 0, 7, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 13, 0, 0, 0,
        // State 40
        0, -49, 0, 0, -49, -49, 0, -49, 0, -49, -49, 0, -49, -49, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -70, 0, 0, -70, -70, 0, -70, 0, -70, -70, 0, 0, -70, 0, 0, 0, 0, 0,
        // State 43
        0, -47, 0, 0, -47, -47, 0, -47, 0, -47, -47, 0, -47, -47, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -45, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -41, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0,
        // State 47
        0, 0, 0, 0, -6, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -36, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -37, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -57,
        // State 2
        -22,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        -21,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        -71,
        // State 25
        -72,
        // State 26
        -73,
        // State 27
        0,
        // State 28
        -78,
        // State 29
        -26,
        // State 30
        -48,
        // State 31
        -69,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -25,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -49,
        // State 41
        0,
        // State 42
        -70,
        // State 43
        -47,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                18 => 51,
                19 => 52,
                _ => 44,
            },
            6 => 45,
            10 => match state {
                3 => 8,
                12 => 15,
                _ => 1,
            },
            11 => 4,
            12 => match state {
                4 => 9,
                _ => 2,
            },
            13 => 5,
            14 => 38,
            15 => match state {
                7 => 40,
                _ => 24,
            },
            16 => match state {
                6 | 14 | 17 => 39,
                7 => 41,
                13 | 16 | 20..=23 => 46,
                _ => 25,
            },
            17 => 26,
            18 => 27,
            19 => match state {
                6 => 10,
                14 => 18,
                17 => 19,
                16 | 22..=23 => 49,
                _ => 47,
            },
            21 => match state {
                0 => 28,
                14 => 48,
                17 => 50,
                _ => 11,
            },
            27 => match state {
                5 => 37,
                _ => 29,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Box<Expr>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct PositionalFuncArgParser {
        _priv: (),
    }

    impl PositionalFuncArgParser {
        pub fn new() -> PositionalFuncArgParser {
            PositionalFuncArgParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            77 => {
                // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __FuncArgs = FuncArgs => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __IdExpression = IdExpression => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __NamedFuncArg = NamedFuncArg => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 33)
    }
}
pub use self::__parse__PositionalFuncArg::PositionalFuncArgParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::{
	grammar::{
		lexer,
		ast::{
			Atom,
			Expr,
			Opcode,
			AstFuncCall,
			AstFuncCallArgs,
			AstPositionalArg,
			AstNamedArg,
			AstMethodCall,
			AstStatement,
			AstAssignment,
			AstAtrOp,
		},
	},
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Tok),
        Variant1(String),
        Variant2(i32),
        Variant3(::std::option::Option<lexer::Tok>),
        Variant4(AstNamedArg),
        Variant5(::std::vec::Vec<AstNamedArg>),
        Variant6(Box<Expr>),
        Variant7(::std::vec::Vec<Box<Expr>>),
        Variant8(()),
        Variant9(AstAssignment),
        Variant10(AstAtrOp),
        Variant11(Opcode),
        Variant12(AstFuncCallArgs),
        Variant13(AstFuncCall),
        Variant14(AstMethodCall),
        Variant15(Vec<AstNamedArg>),
        Variant16(Vec<AstPositionalArg>),
        Variant17(::std::vec::Vec<AstStatement>),
        Variant18(AstStatement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36,
        // State 1
        0, 0, 37, 5, 0, 0, 38, 0, 39, 0, 0, 40, -51, 0, 41, 0, 42, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36,
        // State 3
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 4
        0, 0, 0, 9, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 54, 0, 0, 0, 0, 0, 0, -14, -14,
        // State 7
        0, 55, 0, 0, -22, 56, 0, -22, 0, -22, -22, 0, 0, 57, 0, 0, 0, -22, -22,
        // State 8
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 9
        0, 0, 0, 0, -57, 0, 0, 53, 0, -57, 54, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, -44, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, -40, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 13
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 14
        0, 0, 0, 0, 62, 0, 0, 53, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 16
        0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 17
        0, 0, 0, 9, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 18
        0, 55, 0, 0, -21, 56, 0, -21, 0, -21, -21, 0, 0, 57, 0, 0, 0, -21, -21,
        // State 19
        0, 0, 0, 0, -54, 0, 0, 53, 0, -54, 54, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 21
        0, 0, 0, 9, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 48,
        // State 22
        0, 0, 0, 0, -34, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, -35, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 25
        0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 26
        0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 27
        0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, -64,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, -62, -62,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, -63, -63,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, -67,
        // State 34
        0, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48, -48,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20,
        // State 37
        0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18,
        // State 38
        0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16,
        // State 39
        0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17,
        // State 40
        0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19,
        // State 41
        0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68,
        // State 43
        0, -71, 0, 0, -71, -71, 0, -71, 0, -71, -71, 0, -53, -71, 0, 0, 0, -71, -71,
        // State 44
        0, -72, 0, 5, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 0, 0, -72, -72,
        // State 45
        0, -73, 0, 0, -73, -73, 0, -73, 0, -73, -73, 0, -52, -73, 0, 0, 0, -73, -73,
        // State 46
        0, -26, 0, 0, -26, -26, 0, -26, 0, -26, -26, 0, 0, -26, 0, 0, 0, -26, -26,
        // State 47
        0, -69, 0, 0, -69, -69, 0, -69, 0, -69, -69, 0, -50, -69, 0, 0, 0, -69, -69,
        // State 48
        0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, -72, 0, 5, -72, -72, 0, -72, 0, -72, -72, 0, -51, -72, 0, 16, 0, 0, 0,
        // State 50
        0, -49, 0, 0, -49, -49, 0, -49, 0, -49, -49, 0, -49, -49, 0, 0, 0, -49, -49,
        // State 51
        0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23,
        // State 53
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24,
        // State 54
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29,
        // State 55
        0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27,
        // State 56
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28,
        // State 57
        0, -47, 0, 0, -47, -47, 0, -47, 0, -47, -47, 0, -47, -47, 0, 0, 0, -47, -47,
        // State 58
        0, 0, 0, 0, -45, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, -41, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -25, 0, 0, -25, -25, 0, -25, 0, -25, -25, 0, 0, -25, 0, 0, 0, -25, -25,
        // State 61
        0, -70, 0, 0, -70, -70, 0, -70, 0, -70, -70, 0, 0, -70, 0, 0, 0, -70, -70,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0,
        // State 63
        0, 0, 0, 0, -6, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, -36, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, -37, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -60,
        // State 1
        0,
        // State 2
        -61,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -14,
        // State 7
        -22,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -21,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -64,
        // State 29
        -62,
        // State 30
        -63,
        // State 31
        0,
        // State 32
        -79,
        // State 33
        -67,
        // State 34
        -48,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -68,
        // State 43
        -71,
        // State 44
        -72,
        // State 45
        -73,
        // State 46
        -26,
        // State 47
        -69,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -49,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        -47,
        // State 58
        0,
        // State 59
        0,
        // State 60
        -25,
        // State 61
        -70,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                22 => 67,
                23 => 68,
                _ => 58,
            },
            6 => 59,
            8 => 28,
            9 => 3,
            10 => match state {
                3 => 6,
                8 => 14,
                15 => 19,
                _ => 9,
            },
            11 => 12,
            12 => match state {
                12 => 18,
                _ => 7,
            },
            13 => 13,
            14 => 48,
            15 => match state {
                0 | 2 => 29,
                5 => 50,
                _ => 43,
            },
            16 => match state {
                0 | 2 => 1,
                4 | 17 | 21 => 49,
                5 => 51,
                16 | 20 | 24..=27 => 62,
                _ => 44,
            },
            17 => match state {
                0 | 2 => 30,
                _ => 45,
            },
            18 => 31,
            19 => match state {
                4 => 10,
                17 => 22,
                21 => 23,
                20 | 26..=27 => 65,
                _ => 63,
            },
            21 => match state {
                17 => 64,
                21 => 66,
                _ => 11,
            },
            23 => 32,
            24 => match state {
                2 => 42,
                _ => 33,
            },
            26 => 2,
            27 => match state {
                13 => 60,
                _ => 46,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""\n""###,
            r###""%""###,
            r###""%=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""*=""###,
            r###""+""###,
            r###""+=""###,
            r###"",""###,
            r###""-""###,
            r###""-=""###,
            r###"".""###,
            r###""/""###,
            r###""/=""###,
            r###"":""###,
            r###""=""###,
            r###""id""###,
            r###""num""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = lexer::LexicalError;
        type Token = lexer::Tok;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = ::std::vec::Vec<AstStatement>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            lexer::Tok::Newline if true => Some(0),
            lexer::Tok::Mod if true => Some(1),
            lexer::Tok::ModEq if true => Some(2),
            lexer::Tok::POPEN if true => Some(3),
            lexer::Tok::PCLOSE if true => Some(4),
            lexer::Tok::Mul if true => Some(5),
            lexer::Tok::MulEq if true => Some(6),
            lexer::Tok::Add if true => Some(7),
            lexer::Tok::AddEq if true => Some(8),
            lexer::Tok::Comma if true => Some(9),
            lexer::Tok::Sub if true => Some(10),
            lexer::Tok::SubEq if true => Some(11),
            lexer::Tok::Dot if true => Some(12),
            lexer::Tok::Div if true => Some(13),
            lexer::Tok::DivEq if true => Some(14),
            lexer::Tok::Colon if true => Some(15),
            lexer::Tok::Eq if true => Some(16),
            lexer::Tok::Identifier(_) if true => Some(17),
            lexer::Tok::Number(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: lexer::Tok,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => __Symbol::Variant0(__token),
            17 => match __token {
                lexer::Tok::Identifier(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            18 => match __token {
                lexer::Tok::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<::std::vec::Vec<AstStatement>, __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<::std::vec::Vec<AstStatement>,__lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            78 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAssignment, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstAtrOp, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstFuncCallArgs, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstMethodCall, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstNamedArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, AstStatement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant18(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<AstPositionalArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Tok, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<lexer::Tok>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstNamedArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<AstStatement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>) = ",", NamedFuncArg => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)* = ("," <NamedFuncArg>)+ => ActionFn(47);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ",", NamedFuncArg => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <NamedFuncArg>)+ = ("," <NamedFuncArg>)+, ",", NamedFuncArg => ActionFn(70);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>) = ",", PositionalFuncArg => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)* = ("," <PositionalFuncArg>)+ => ActionFn(50);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ",", PositionalFuncArg => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <PositionalFuncArg>)+ = ("," <PositionalFuncArg>)+, ",", PositionalFuncArg => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Assignment = IdExpression, AtrOp, Expr => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "=" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "+=" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "-=" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "*=" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "/=" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // AtrOp = "%=" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(20);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(24);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FactorOp = "%" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, "," => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, "," => ActionFn(87);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(88);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(89);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (6, 14)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg => ActionFn(91);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(92);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 14)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, ",", NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(93);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 14)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action94::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+, "," => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg => ActionFn(96);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action96::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, "," => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+, "," => ActionFn(83);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg => ActionFn(84);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncArgs =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // FuncCall = IdExpression, "(", FuncArgs, ")" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // IdExpression = "id" => ActionFn(45);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCall = MethodCallBase, ".", FuncCall => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = "num" => ActionFn(33);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = IdExpression => ActionFn(34);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = MethodCall => ActionFn(35);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // MethodCallBase = FuncCall => ActionFn(36);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArg = IdExpression, ":", Expr => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // NamedFuncArgs = NamedFuncArg, ("," <NamedFuncArg>)+ => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArg = Expr => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg => ActionFn(75);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // PositionalFuncArgs = PositionalFuncArg, ("," <PositionalFuncArg>)+ => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(99);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = FuncCall => ActionFn(7);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = MethodCall => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Assignment => ActionFn(9);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 25)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(56);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(57);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "num" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = FuncCall => ActionFn(30);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IdExpression => ActionFn(31);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = MethodCall => ActionFn(32);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __FuncArgs = FuncArgs => ActionFn(1);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __IdExpression = IdExpression => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __NamedFuncArg = NamedFuncArg => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __PositionalFuncArg = PositionalFuncArg => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 32)
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (usize, ::std::vec::Vec<AstStatement>, usize),
) -> ::std::vec::Vec<AstStatement>
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, AstFuncCallArgs, usize),
) -> AstFuncCallArgs
{
    __0
}

fn __action2<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action3<
>(
    (_, __0, _): (usize, AstNamedArg, usize),
) -> AstNamedArg
{
    __0
}

fn __action4<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action5<
>(
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

fn __action6<
>(
    (_, __0, _): (usize, ::std::vec::Vec<AstStatement>, usize),
) -> ::std::vec::Vec<AstStatement>
{
    __0
}

fn __action7<
>(
    (_, __0, _): (usize, AstFuncCall, usize),
) -> AstStatement
{
    AstStatement::FuncCall(__0)
}

fn __action8<
>(
    (_, __0, _): (usize, AstMethodCall, usize),
) -> AstStatement
{
    AstStatement::MethodCall(__0)
}

fn __action9<
>(
    (_, __0, _): (usize, AstAssignment, usize),
) -> AstStatement
{
    AstStatement::Assignment(__0)
}

fn __action10<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, args, _): (usize, AstFuncCallArgs, usize),
    (_, _, _): (usize, lexer::Tok, usize),
) -> AstFuncCall
{
    AstFuncCall::new(name, args)
}

fn __action11<
>(
    (_, pos_args, _): (usize, Vec<AstPositionalArg>, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, named_args, _): (usize, Vec<AstNamedArg>, usize),
    (_, _, _): (usize, ::std::option::Option<lexer::Tok>, usize),
) -> AstFuncCallArgs
{
    AstFuncCallArgs::new(pos_args,named_args)
}

fn __action12<
>(
    (_, pos_args, _): (usize, Vec<AstPositionalArg>, usize),
    (_, _, _): (usize, ::std::option::Option<lexer::Tok>, usize),
) -> AstFuncCallArgs
{
    AstFuncCallArgs::new_only_positional(pos_args)
}

fn __action13<
>(
    (_, named_args, _): (usize, Vec<AstNamedArg>, usize),
    (_, _, _): (usize, ::std::option::Option<lexer::Tok>, usize),
) -> AstFuncCallArgs
{
    AstFuncCallArgs::new_only_named(named_args)
}

fn __action14<
>(
    (_, __0, _): (usize, (), usize),
) -> AstFuncCallArgs
{
    AstFuncCallArgs::empty()
}

fn __action15<
>(
    (_, arg1, _): (usize, Box<Expr>, usize),
    (_, rest, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<AstPositionalArg>
{
    {let mut v = vec![arg1]; v.extend(rest); v.into_iter().map(|box_ : Box<Expr>| AstPositionalArg::from(box_)).collect()}
}

fn __action16<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action17<
>(
    (_, arg1, _): (usize, AstNamedArg, usize),
    (_, rest, _): (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> Vec<AstNamedArg>
{
    {let mut v = vec![arg1]; v.extend(rest); v}
}

fn __action18<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, value, _): (usize, Box<Expr>, usize),
) -> AstNamedArg
{
    AstNamedArg::from((name, value))
}

fn __action19<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

fn __action20<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action21<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> Opcode
{
    Opcode::Add
}

fn __action22<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> Opcode
{
    Opcode::Sub
}

fn __action23<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Opcode, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(__0, __1, __2))
}

fn __action24<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action25<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> Opcode
{
    Opcode::Mul
}

fn __action26<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> Opcode
{
    Opcode::Div
}

fn __action27<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> Opcode
{
    Opcode::Mod
}

fn __action28<
>(
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Atom(Atom::Number(__0)))
}

fn __action29<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, lexer::Tok, usize),
) -> Box<Expr>
{
    __0
}

fn __action30<
>(
    (_, __0, _): (usize, AstFuncCall, usize),
) -> Box<Expr>
{
    Box::new(Expr::FuncCall(__0))
}

fn __action31<
>(
    (_, __0, _): (usize, String, usize),
) -> Box<Expr>
{
    Box::new(Expr::Atom(Atom::Id(__0)))
}

fn __action32<
>(
    (_, __0, _): (usize, AstMethodCall, usize),
) -> Box<Expr>
{
    Box::new(Expr::MethodCall(__0))
}

fn __action33<
>(
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Atom(Atom::Number(__0)))
}

fn __action34<
>(
    (_, __0, _): (usize, String, usize),
) -> Box<Expr>
{
    Box::new(Expr::Atom(Atom::Id(__0)))
}

fn __action35<
>(
    (_, __0, _): (usize, AstMethodCall, usize),
) -> Box<Expr>
{
    Box::new(Expr::MethodCall(__0))
}

fn __action36<
>(
    (_, __0, _): (usize, AstFuncCall, usize),
) -> Box<Expr>
{
    Box::new(Expr::FuncCall(__0))
}

fn __action37<
>(
    (_, base, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, lexer::Tok, usize),
    (_, call, _): (usize, AstFuncCall, usize),
) -> AstMethodCall
{
    AstMethodCall::new(base, call)
}

fn __action38<
>(
    (_, var_name, _): (usize, String, usize),
    (_, op, _): (usize, AstAtrOp, usize),
    (_, val, _): (usize, Box<Expr>, usize),
) -> AstAssignment
{
    AstAssignment::new(var_name, op, val)
}

fn __action39<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::Atr
}

fn __action40<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::AddAtr
}

fn __action41<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::SubAtr
}

fn __action42<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::MulAtr
}

fn __action43<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::DivAtr
}

fn __action44<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> AstAtrOp
{
    AstAtrOp::ModAtr
}

fn __action45<
>(
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

fn __action46<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<AstNamedArg>
{
    vec![]
}

fn __action47<
>(
    (_, v, _): (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> ::std::vec::Vec<AstNamedArg>
{
    v
}

fn __action48<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, AstNamedArg, usize),
) -> AstNamedArg
{
    __0
}

fn __action49<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![]
}

fn __action50<
>(
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    v
}

fn __action51<
>(
    (_, _, _): (usize, lexer::Tok, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

fn __action52<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ()
{
    ()
}

fn __action53<
>(
    (_, __0, _): (usize, lexer::Tok, usize),
) -> ::std::option::Option<lexer::Tok>
{
    Some(__0)
}

fn __action54<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<lexer::Tok>
{
    None
}

fn __action55<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<AstStatement>
{
    vec![]
}

fn __action56<
>(
    (_, v, _): (usize, ::std::vec::Vec<AstStatement>, usize),
) -> ::std::vec::Vec<AstStatement>
{
    v
}

fn __action57<
>(
    (_, __0, _): (usize, AstStatement, usize),
) -> ::std::vec::Vec<AstStatement>
{
    vec![__0]
}

fn __action58<
>(
    (_, v, _): (usize, ::std::vec::Vec<AstStatement>, usize),
    (_, e, _): (usize, AstStatement, usize),
) -> ::std::vec::Vec<AstStatement>
{
    { let mut v = v; v.push(e); v }
}

fn __action59<
>(
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    vec![__0]
}

fn __action60<
>(
    (_, v, _): (usize, ::std::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

fn __action61<
>(
    (_, __0, _): (usize, AstNamedArg, usize),
) -> ::std::vec::Vec<AstNamedArg>
{
    vec![__0]
}

fn __action62<
>(
    (_, v, _): (usize, ::std::vec::Vec<AstNamedArg>, usize),
    (_, e, _): (usize, AstNamedArg, usize),
) -> ::std::vec::Vec<AstNamedArg>
{
    { let mut v = v; v.push(e); v }
}

fn __action63<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, Vec<AstNamedArg>, usize),
    __3: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action53(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action64<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action65<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __0,
        __temp0,
    )
}

fn __action66<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __0,
        __temp0,
    )
}

fn __action67<
>(
    __0: (usize, Vec<AstNamedArg>, usize),
    __1: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
    )
}

fn __action68<
>(
    __0: (usize, Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
    )
}

fn __action69<
>(
    __0: (usize, lexer::Tok, usize),
    __1: (usize, AstNamedArg, usize),
) -> ::std::vec::Vec<AstNamedArg>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __temp0,
    )
}

fn __action70<
>(
    __0: (usize, ::std::vec::Vec<AstNamedArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
) -> ::std::vec::Vec<AstNamedArg>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __0,
        __temp0,
    )
}

fn __action71<
>(
    __0: (usize, AstNamedArg, usize),
) -> Vec<AstNamedArg>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        __0,
        __temp0,
    )
}

fn __action72<
>(
    __0: (usize, AstNamedArg, usize),
    __1: (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> Vec<AstNamedArg>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        __0,
        __temp0,
    )
}

fn __action73<
>(
    __0: (usize, lexer::Tok, usize),
    __1: (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action51(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        __temp0,
    )
}

fn __action74<
>(
    __0: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, Box<Expr>, usize),
) -> ::std::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action51(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        __0,
        __temp0,
    )
}

fn __action75<
>(
    __0: (usize, Box<Expr>, usize),
) -> Vec<AstPositionalArg>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        __0,
        __temp0,
    )
}

fn __action76<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> Vec<AstPositionalArg>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        __0,
        __temp0,
    )
}

fn __action77<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> AstFuncCallArgs
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action52(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        __temp0,
    )
}

fn __action78<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action71(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __0,
        __1,
        __temp0,
        __3,
    )
}

fn __action79<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, ::std::vec::Vec<AstNamedArg>, usize),
    __4: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action72(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __0,
        __1,
        __temp0,
        __4,
    )
}

fn __action80<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
) -> AstFuncCallArgs
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action71(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __0,
        __1,
        __temp0,
    )
}

fn __action81<
>(
    __0: (usize, Vec<AstPositionalArg>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action72(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __0,
        __1,
        __temp0,
    )
}

fn __action82<
>(
    __0: (usize, AstNamedArg, usize),
    __1: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action71(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        __temp0,
        __1,
    )
}

fn __action83<
>(
    __0: (usize, AstNamedArg, usize),
    __1: (usize, ::std::vec::Vec<AstNamedArg>, usize),
    __2: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action72(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        __temp0,
        __2,
    )
}

fn __action84<
>(
    __0: (usize, AstNamedArg, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action71(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __temp0,
    )
}

fn __action85<
>(
    __0: (usize, AstNamedArg, usize),
    __1: (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action72(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __temp0,
    )
}

fn __action86<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        __temp0,
        __1,
        __2,
        __3,
    )
}

fn __action87<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __2: (usize, lexer::Tok, usize),
    __3: (usize, AstNamedArg, usize),
    __4: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        __temp0,
        __2,
        __3,
        __4,
    )
}

fn __action88<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, ::std::vec::Vec<AstNamedArg>, usize),
    __4: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action89<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __2: (usize, lexer::Tok, usize),
    __3: (usize, AstNamedArg, usize),
    __4: (usize, ::std::vec::Vec<AstNamedArg>, usize),
    __5: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        __temp0,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action90<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        __temp0,
        __1,
        __2,
    )
}

fn __action91<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __2: (usize, lexer::Tok, usize),
    __3: (usize, AstNamedArg, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        __temp0,
        __2,
        __3,
    )
}

fn __action92<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, lexer::Tok, usize),
    __2: (usize, AstNamedArg, usize),
    __3: (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        __temp0,
        __1,
        __2,
        __3,
    )
}

fn __action93<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __2: (usize, lexer::Tok, usize),
    __3: (usize, AstNamedArg, usize),
    __4: (usize, ::std::vec::Vec<AstNamedArg>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        __temp0,
        __2,
        __3,
        __4,
    )
}

fn __action94<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
        __1,
    )
}

fn __action95<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
    __2: (usize, lexer::Tok, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
        __2,
    )
}

fn __action96<
>(
    __0: (usize, Box<Expr>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action75(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        __temp0,
    )
}

fn __action97<
>(
    __0: (usize, Box<Expr>, usize),
    __1: (usize, ::std::vec::Vec<Box<Expr>>, usize),
) -> AstFuncCallArgs
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action76(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        __temp0,
    )
}

fn __action98<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<AstStatement>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action55(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __temp0,
    )
}

fn __action99<
>(
    __0: (usize, ::std::vec::Vec<AstStatement>, usize),
) -> ::std::vec::Vec<AstStatement>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __temp0,
    )
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize), __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>>;
}

impl<> __ToTriple<> for (usize, lexer::Tok, usize) {
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize), __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, lexer::Tok, usize), lexer::LexicalError> {
    fn to_triple(value: Self) -> Result<(usize,lexer::Tok,usize), __lalrpop_util::ParseError<usize, lexer::Tok, lexer::LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
