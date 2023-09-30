// auto-generated: "lalrpop 0.20.0"
// sha3: 12297cc98f797602fe3d92bb4dcc1432c18727b134356041f87749419d2a232c
use crate::cc_lexer;
use crate::cc_lexer::LexicalError;
use crate::cc_lexer::Token;
use crate::cc_parser::ast::Expression;
use crate::cc_parser::ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__OpPrefix {

    use crate::cc_lexer;
    use crate::cc_lexer::LexicalError;
    use crate::cc_lexer::Token;
    use crate::cc_parser::ast::Expression;
    use crate::cc_parser::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(f64),
        Variant3(ast::Spanned<ast::Expression>),
        Variant4(alloc::vec::Vec<ast::Spanned<ast::Expression>>),
        Variant5(alloc::vec::Vec<String>),
        Variant6(ast::Type),
        Variant7(core::option::Option<ast::Type>),
        Variant8(core::option::Option<ast::Spanned<ast::Expression>>),
        Variant9(()),
        Variant10(usize),
        Variant11(Vec<ast::ExprS>),
        Variant12(ast::Statement),
        Variant13(ast::Spanned<ast::Statement>),
        Variant14(alloc::vec::Vec<ast::Spanned<ast::Statement>>),
        Variant15(ast::Expression),
        Variant16(Option<ast::StatementS>),
        Variant17(ast::StatementFun),
        Variant18(ast::OpInfix),
        Variant19(ast::OpPrefix),
        Variant20(Vec<String>),
        Variant21(ast::Program),
        Variant22(ast::StatementBlock),
    }
    const __ACTION: &[i16] = &[
        // State 0
        3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 39 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -136,
        // State 2
        -89,
        // State 3
        -88,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            48 => 1,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!""###,
        r###""!=""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""/""###,
        r###"":""###,
        r###"";""###,
        r###""<""###,
        r###""<=""###,
        r###""=""###,
        r###""==""###,
        r###"">""###,
        r###"">=""###,
        r###""and""###,
        r###""else""###,
        r###""false""###,
        r###""fn""###,
        r###""for""###,
        r###""if""###,
        r###""let""###,
        r###""nil""###,
        r###""or""###,
        r###""print""###,
        r###""println""###,
        r###""return""###,
        r###""struct""###,
        r###""true""###,
        r###""while""###,
        r###""{""###,
        r###""}""###,
        r###"identifier"###,
        r###"number"###,
        r###"string"###,
        r###"type_int"###,
        r###"type_string"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i16],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = ast::OpPrefix;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
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
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 39 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
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
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Bang if true => Some(0),
            Token::BangEqual if true => Some(1),
            Token::LeftParen if true => Some(2),
            Token::RightParen if true => Some(3),
            Token::Star if true => Some(4),
            Token::Plus if true => Some(5),
            Token::Comma if true => Some(6),
            Token::Minus if true => Some(7),
            Token::Slash if true => Some(8),
            Token::Colon if true => Some(9),
            Token::Semicolon if true => Some(10),
            Token::Less if true => Some(11),
            Token::LessEqual if true => Some(12),
            Token::Equal if true => Some(13),
            Token::EqualEqual if true => Some(14),
            Token::Greater if true => Some(15),
            Token::GreaterEqual if true => Some(16),
            Token::And if true => Some(17),
            Token::Else if true => Some(18),
            Token::False if true => Some(19),
            Token::Fn if true => Some(20),
            Token::For if true => Some(21),
            Token::If if true => Some(22),
            Token::Let if true => Some(23),
            Token::Nil if true => Some(24),
            Token::Or if true => Some(25),
            Token::Print if true => Some(26),
            Token::PrintLn if true => Some(27),
            Token::Return if true => Some(28),
            Token::Struct if true => Some(29),
            Token::True if true => Some(30),
            Token::While if true => Some(31),
            Token::LeftBrace if true => Some(32),
            Token::RightBrace if true => Some(33),
            Token::Identifier(_) if true => Some(34),
            Token::Number(_) if true => Some(35),
            Token::String(_) if true => Some(36),
            Token::TypeInt if true => Some(37),
            Token::TypeString if true => Some(38),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 37 | 38 => __Symbol::Variant0(__token),
            34 | 36 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            35 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 28,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 29,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 33,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 35,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 39,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 40,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 42,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 50,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 67,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 70,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 70,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 70,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 71,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 73,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 74,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 75,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 75,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 76,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 76,
                }
            }
            135 => __state_machine::SimulatedReduce::Accept,
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 78,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct OpPrefixParser {
        _priv: (),
    }

    impl OpPrefixParser {
        pub fn new() -> OpPrefixParser {
            OpPrefixParser {
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
        ) -> Result<ast::OpPrefix, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<ast::OpPrefix,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                __reduce94(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            95 => {
                __reduce95(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            96 => {
                __reduce96(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            97 => {
                __reduce97(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            98 => {
                __reduce98(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            99 => {
                __reduce99(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            100 => {
                __reduce100(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            101 => {
                __reduce101(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            102 => {
                __reduce102(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            103 => {
                __reduce103(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            104 => {
                __reduce104(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            105 => {
                __reduce105(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            106 => {
                __reduce106(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            107 => {
                __reduce107(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            108 => {
                __reduce108(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            109 => {
                __reduce109(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            110 => {
                __reduce110(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            111 => {
                __reduce111(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            112 => {
                __reduce112(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            113 => {
                __reduce113(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            114 => {
                __reduce114(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            115 => {
                __reduce115(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            116 => {
                __reduce116(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            117 => {
                __reduce117(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            118 => {
                __reduce118(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            119 => {
                __reduce119(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            120 => {
                __reduce120(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            121 => {
                __reduce121(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            122 => {
                __reduce122(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            123 => {
                __reduce123(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            124 => {
                __reduce124(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            125 => {
                __reduce125(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            126 => {
                __reduce126(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            127 => {
                __reduce127(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            128 => {
                __reduce128(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            129 => {
                __reduce129(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            130 => {
                __reduce130(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            131 => {
                __reduce131(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            132 => {
                __reduce132(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            133 => {
                __reduce133(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            134 => {
                __reduce134(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            135 => {
                // __OpPrefix = OpPrefix => ActionFn(1);
                let __sym0 = __pop_Variant19(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            136 => {
                __reduce136(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<ast::StatementS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::ExprS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpInfix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpPrefix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementBlock, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementFun, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>) = ",", ExprS => ActionFn(79);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action79::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(78);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action78::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action127::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(128);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action128::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>) = ",", identifier => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action83::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)* =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)* = ("," <identifier>)+ => ActionFn(82);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action82::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)+ = ",", identifier => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action131::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)+ = ("," <identifier>)+, ",", identifier => ActionFn(132);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action132::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>) = ":", Types => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action109::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? = ":", Types => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action135::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? =  => ActionFn(108);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action108::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>) = "=", ExprS => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(138);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action138::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action105::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(80);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action80::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action126::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args = ExprS => ActionFn(129);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action129::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action130::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args =  => ActionFn(143);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action143::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = Stmt => ActionFn(4);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = DeclVar => ActionFn(5);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = DeclFun => ActionFn(6);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclFun = "fn", Function => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action7::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS = Spanned<Decl> => ActionFn(3);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS* =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS* = DeclS+ => ActionFn(112);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action112::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS => ActionFn(113);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action113::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS+, DeclS => ActionFn(114);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action114::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(139);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action139::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(140);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action140::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(141);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action141::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ";" => ActionFn(142);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action142::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAssign = identifier, "=", ExprS => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action39::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAssign = ExprLogicOr => ActionFn(40);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprComparison = ExprInfix<ExprComparison, OpComparison, ExprTerm> => ActionFn(48);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprEquality = ExprInfix<ExprEquality, OpEquality, ExprComparison> => ActionFn(45);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprFactor = ExprInfix<ExprFactor, OpFactor, ExprPrefix> => ActionFn(56);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(88);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(89);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action89::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 25)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(91);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action91::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(85);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(93);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action93::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(94);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action94::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 28)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(95);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action95::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(86);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 29)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(87);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action87::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprLogicAnd = ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> => ActionFn(43);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprLogicOr = ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> => ActionFn(41);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(74);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action74::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 32)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprPrimary => ActionFn(75);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "nil" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action63::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(64);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(65);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(66);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 33)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS = Spanned<Expression> => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(97);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action97::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 35)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprTerm = ExprInfix<ExprTerm, OpTerm, ExprFactor> => ActionFn(53);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(73);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ExprAssign => ActionFn(38);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action177::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 39)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(178);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action178::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(179);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action179::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(180);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action180::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 40)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<DeclVar> => ActionFn(21);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<StmtExpr> => ActionFn(22);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = ";" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(68);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant22(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action68::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (5, 42)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = ">" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = ">=" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = "<" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = "<=" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action52::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpEquality = "==" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce82<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpEquality = "!=" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce83<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpFactor = "*" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce84<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpFactor = "/" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce85<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpLogicAnd = "and" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce86<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpLogicOr = "or" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce87<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpPrefix = "-" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce88<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpPrefix = "!" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce89<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpTerm = "+" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce90<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpTerm = "-" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce91<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier => ActionFn(133);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action133::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce92<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier, ("," <identifier>)+ => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action134::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(144);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action144::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 50)
    }
    pub(crate) fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(173);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action173::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(174);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtExpr => ActionFn(26);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtBlock => ActionFn(27);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrint => ActionFn(28);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce99<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrintLn => ActionFn(29);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce100<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtReturn => ActionFn(30);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce101<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(159);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action159::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce102<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(160);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action160::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(161);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action161::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(162);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action162::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(163);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(164);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action164::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(165);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(166);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action166::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(167);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(168);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action168::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(169);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(170);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(171);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action171::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(172);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action172::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(175);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action175::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 67)
    }
    pub(crate) fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(176);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action176::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (3, 67)
    }
    pub(crate) fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = StmtOpen => ActionFn(11);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = StmtClosed => ActionFn(12);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtBlock = StatementBlockIn => ActionFn(34);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 69)
    }
    pub(crate) fn __reduce120<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtClosed> => ActionFn(17);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 70)
    }
    pub(crate) fn __reduce121<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "while", "(", ExprS, ")", Spanned<StmtClosed> => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 70)
    }
    pub(crate) fn __reduce122<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtClosed> => ActionFn(19);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 70)
    }
    pub(crate) fn __reduce123<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = SmallStmt => ActionFn(20);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 70)
    }
    pub(crate) fn __reduce124<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtExpr = ExprS, ";" => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action36::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 71)
    }
    pub(crate) fn __reduce125<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtOpen> => ActionFn(13);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 72)
    }
    pub(crate) fn __reduce126<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<Stmt> => ActionFn(14);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 72)
    }
    pub(crate) fn __reduce127<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtOpen> => ActionFn(15);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 72)
    }
    pub(crate) fn __reduce128<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "while", "(", ExprS, ")", Spanned<StmtOpen> => ActionFn(16);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 72)
    }
    pub(crate) fn __reduce129<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtPrint = "print", ExprS, ";" => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 73)
    }
    pub(crate) fn __reduce130<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtPrintLn = "println", ExprS, ";" => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 74)
    }
    pub(crate) fn __reduce131<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 75)
    }
    pub(crate) fn __reduce132<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(182);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action182::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 75)
    }
    pub(crate) fn __reduce133<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Types = type_string => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Types = type_int => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action10::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce136<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant21(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 78)
    }
}
pub use self::__parse__OpPrefix::OpPrefixParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__Program {

    use crate::cc_lexer;
    use crate::cc_lexer::LexicalError;
    use crate::cc_lexer::Token;
    use crate::cc_parser::ast::Expression;
    use crate::cc_parser::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(String),
        Variant2(f64),
        Variant3(ast::Spanned<ast::Expression>),
        Variant4(alloc::vec::Vec<ast::Spanned<ast::Expression>>),
        Variant5(alloc::vec::Vec<String>),
        Variant6(ast::Type),
        Variant7(core::option::Option<ast::Type>),
        Variant8(core::option::Option<ast::Spanned<ast::Expression>>),
        Variant9(()),
        Variant10(usize),
        Variant11(Vec<ast::ExprS>),
        Variant12(ast::Statement),
        Variant13(ast::Spanned<ast::Statement>),
        Variant14(alloc::vec::Vec<ast::Spanned<ast::Statement>>),
        Variant15(ast::Expression),
        Variant16(Option<ast::StatementS>),
        Variant17(ast::StatementFun),
        Variant18(ast::OpInfix),
        Variant19(ast::OpPrefix),
        Variant20(Vec<String>),
        Variant21(ast::Program),
        Variant22(ast::StatementBlock),
    }
    const __ACTION: &[i16] = &[
        // State 0
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 11, 82, 83, 84, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 1
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 11, 82, 83, 84, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 2
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 97, 0, 0, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 102, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 106, 0, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0,
        // State 11
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 12
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 13
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 115, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 14
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 11, 82, 83, 84, 85, 0, 12, 13, 14, 0, 86, 87, 15, 116, 88, 89, 90, 0, 0,
        // State 15
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 16
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 17
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 18
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 19
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 20
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 95, 89, 90, 0, 0,
        // State 21
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 84, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 22
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 23
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 24
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 11, 82, 83, 84, 85, 0, 12, 13, 14, 0, 86, 87, 15, 135, 88, 89, 90, 0, 0,
        // State 25
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 26
        0, -41, 0, -41, 0, 106, 0, 107, 0, 0, -41, -41, -41, 0, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43, 96, 97, 0, -43, 98, 99, -43, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 100, 0, -47, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 101, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, -49, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 104, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -51, 0, -51, 102, -51, 0, -51, 103, 0, -51, -51, -51, 0, -51, -51, -51, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 32
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 139, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 141, 142,
        // State 34
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 35
        0, 0, 0, -92, 0, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        79, 0, 10, -73, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 37
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 82, 83, 0, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 38
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 82, 83, 0, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 40
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 86, 0, 0, 0, 88, 89, 90, 0, 0,
        // State 41
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 82, 83, 0, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 42
        79, 0, 10, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 82, 83, 0, 85, 0, 12, 13, 14, 0, 86, 87, 15, 0, 88, 89, 90, 0, 0,
        // State 43
        -102, 0, -102, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, -102, -102, -102, -102, -102, 0, -102, -102, -102, 0, -102, -102, -102, -102, -102, -102, -102, 0, 0,
        // State 44
        -25, 0, -25, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, 0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, 0, 0,
        // State 45
        -30, 0, -30, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, -30, -30, -30, -30, 0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, 0, 0,
        // State 46
        -24, 0, -24, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, 0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, 0, 0,
        // State 47
        0, 0, 0, -69, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44, -104, -104, 0, -44, -104, -104, -44, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, -105, 0, -48, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, -105, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, -52, 0, -52, -106, -52, 0, -52, -106, 0, -52, -52, -52, 0, -52, -52, -52, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38, -38, -38, 0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -40, 0, -40, -40, -40, 0, -40, -40, 0, -40, -40, -40, 0, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, -54, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, -67, 0, -67, 0, -67, 0, -67, 0, 0, -67, -67, -67, 0, -67, -67, -67, -67, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, -50, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, -37, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, -46, 0, -46, -46, -46, 0, -46, -46, 0, -46, -46, -46, 0, -46, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -56, 0, -56, -56, -56, 0, -56, -56, 0, -56, -56, -56, 0, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -42, 0, -42, 0, -110, 0, -110, 0, 0, -42, -42, -42, 0, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -62, 0, -62, -62, -62, 0, -62, -62, 0, -62, -62, -62, 0, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, -111, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        -124, 0, -124, 0, 0, 0, 0, -124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -124, -124, -124, -124, -124, -124, -124, 0, -124, -124, -124, 0, -124, -124, -124, -124, -124, -124, -124, 0, 0,
        // State 67
        -27, 0, -27, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, 0,
        // State 68
        0, 0, 0, -64, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        -120, 0, -120, 0, 0, 0, 0, -120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, -120, -120, -120, -120, -120, -120, 0, -120, -120, -120, 0, -120, -120, -120, -120, -120, -120, -120, 0, 0,
        // State 70
        -23, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, 0, 0,
        // State 71
        -98, 0, -98, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, -98, -98, -98, -98, -98, -98, 0, -98, -98, -98, 0, -98, -98, -98, -98, -98, -98, -98, 0, 0,
        // State 72
        -119, 0, -119, 0, 0, 0, 0, -119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -119, -119, -119, -119, -119, -119, 0, -119, -119, -119, 0, -119, -119, -119, -119, -119, -119, -119, 0, 0,
        // State 73
        -97, 0, -97, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, -97, -97, -97, -97, -97, -97, 0, -97, -97, -97, 0, -97, -97, -97, -97, -97, -97, -97, 0, 0,
        // State 74
        -118, 0, -118, 0, 0, 0, 0, -118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -118, -118, -118, -118, -118, -118, 0, -118, -118, -118, 0, -118, -118, -118, -118, -118, -118, -118, 0, 0,
        // State 75
        -99, 0, -99, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, -99, -99, -99, -99, -99, -99, 0, -99, -99, -99, 0, -99, -99, -99, -99, -99, -99, -99, 0, 0,
        // State 76
        -100, 0, -100, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, -100, -100, -100, -100, -100, -100, 0, -100, -100, -100, 0, -100, -100, -100, -100, -100, -100, -100, 0, 0,
        // State 77
        -101, 0, -101, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, -101, -101, -101, -101, -101, -101, 0, -101, -101, -101, 0, -101, -101, -101, -101, -101, -101, -101, 0, 0,
        // State 78
        -89, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, 0, 0, 0, -89, -89, -89, 0, 0,
        // State 79
        -88, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, -88, 0, 0, 0, -88, -88, -88, 0, 0,
        // State 80
        0, -58, 0, -58, -58, -58, 0, -58, -58, 0, -58, -58, -58, 0, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 0,
        // State 84
        0, -57, 0, -57, -57, -57, 0, -57, -57, 0, -57, -57, -57, 0, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, -59, 0, -59, -59, -59, 0, -59, -59, 0, -59, -59, -59, 0, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, -68, 0, -68, -68, -68, 0, -68, -68, 0, -68, -68, -68, 26, -68, -68, -68, -68, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, -61, 0, -61, -61, -61, 0, -61, -61, 0, -61, -61, -61, 0, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, -60, 0, -60, -60, -60, 0, -60, -60, 0, -60, -60, -60, 0, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        -31, 0, -31, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, -31, -31, -31, -31, 0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, 0, 0,
        // State 91
        -125, 0, -125, 0, 0, 0, 0, -125, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, -125, -125, -125, -125, -125, -125, -125, 0, -125, -125, -125, 0, -125, -125, -125, -125, -125, -125, -125, 0, 0,
        // State 92
        0, -109, 0, -109, -109, -109, 0, -109, -109, 0, -109, -109, -109, 0, -109, -109, -109, -109, 0, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, -55, 0, -55, -55, -55, 0, -55, -55, 0, -55, -55, -55, 0, -55, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, -68, 0, -68, -68, -68, 0, -68, -68, 0, -68, -68, -68, 0, -68, -68, -68, -68, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        -80, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, -80, 0, 0, 0, -80, -80, -80, 0, 0,
        // State 96
        -81, 0, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, -81, 0, 0, 0, -81, -81, -81, 0, 0,
        // State 97
        -78, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, -78, 0, 0, 0, -78, -78, -78, 0, 0,
        // State 98
        -79, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, -79, 0, 0, 0, -79, -79, -79, 0, 0,
        // State 99
        -83, 0, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, -83, -83, -83, 0, 0,
        // State 100
        -82, 0, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, -82, 0, 0, 0, -82, -82, -82, 0, 0,
        // State 101
        -84, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, -84, 0, 0, 0, -84, -84, -84, 0, 0,
        // State 102
        -85, 0, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, -85, -85, -85, 0, 0,
        // State 103
        -86, 0, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, -86, -86, -86, 0, 0,
        // State 104
        -87, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, -87, 0, 0, 0, -87, -87, -87, 0, 0,
        // State 105
        -90, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, 0, -90, -90, -90, 0, 0,
        // State 106
        -91, 0, -91, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, 0, 0, 0, -91, -91, -91, 0, 0,
        // State 107
        0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        -26, 0, -26, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, 0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, 0,
        // State 109
        0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 130, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        -133, 0, -133, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, -133, -133, -133, -133, -133, -133, 0, -133, -133, -133, 0, -133, -133, -133, -133, -133, -133, -133, 0, 0,
        // State 115
        -116, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, -116, -116, -116, -116, -116, -116, 0, -116, -116, -116, 0, -116, -116, -116, -116, -116, -116, -116, 0, 0,
        // State 116
        0, -110, 0, -110, 0, -110, 0, -110, 0, 0, -110, -110, -110, 0, -110, -110, -110, -110, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, -104, 0, -104, 0, 0, 0, 0, 0, 0, -104, -104, -104, 0, -104, -104, -104, -104, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, -45, 0, -45, -45, -45, 0, -45, -45, 0, -45, -45, -45, 0, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, -105, 0, -105, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, -105, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, -107, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, -106, 0, -106, -106, -106, 0, -106, -106, 0, -106, -106, -106, 0, -106, -106, -106, -106, 0, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, -63, 0, -63, -63, -63, 0, -63, -63, 0, -63, -63, -63, 0, -63, -63, -63, -63, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        -103, 0, -103, 0, 0, 0, 0, -103, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, -103, 0, 0, 0, -103, -103, -103, 0, 0,
        // State 124
        -74, 0, -74, 0, 0, 0, 0, -74, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, -74, 0, 0, 0, -74, -74, -74, 0, 0,
        // State 125
        -75, 0, -75, 0, 0, 0, 0, -75, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, -75, 0, 0, 0, -75, -75, -75, 0, 0,
        // State 126
        -114, 0, -114, 0, 0, 0, 0, -114, 0, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, -114, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, -114, 0, 0, 0, -114, -114, -114, 0, 0,
        // State 127
        -76, 0, -76, 0, 0, 0, 0, -76, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, -76, 0, 0, 0, -76, -76, -76, 0, 0,
        // State 128
        0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        -35, 0, -35, 0, 0, 0, 0, -35, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, 0, 0,
        // State 130
        -130, 0, -130, 0, 0, 0, 0, -130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -130, -130, -130, -130, -130, -130, -130, 0, -130, -130, -130, 0, -130, -130, -130, -130, -130, -130, -130, 0, 0,
        // State 131
        -131, 0, -131, 0, 0, 0, 0, -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, -131, -131, -131, -131, -131, -131, 0, -131, -131, -131, 0, -131, -131, -131, -131, -131, -131, -131, 0, 0,
        // State 132
        -132, 0, -132, 0, 0, 0, 0, -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, -132, -132, -132, -132, -132, -132, 0, -132, -132, -132, 0, -132, -132, -132, -132, -132, -132, -132, 0, 0,
        // State 133
        0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        -117, 0, -117, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, -117, -117, -117, -117, -117, -117, 0, -117, -117, -117, 0, -117, -117, -117, -117, -117, -117, -117, 0, 0,
        // State 135
        0, 0, 0, -36, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 137
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 138
        -71, 0, -71, -71, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, -71, 0, 0, 0, -71, -71, -71, 0, 0,
        // State 139
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -135, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, 0, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 142
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 154, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        0, 0, 0, -93, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 161, 0, 0, 0, 0,
        // State 145
        -70, 0, -70, -70, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, -70, 0, 0, 0, -70, -70, -70, 0, 0,
        // State 146
        0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        -127, 0, -127, 0, 0, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -127, -127, -127, -127, -127, -127, 0, -127, -127, -127, 0, -127, -127, -127, -127, -127, -127, -127, 0, 0,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        -112, 0, -112, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -112, -112, -112, -112, -112, -112, 0, -112, -112, -112, 0, -112, -112, -112, -112, -112, -112, -112, 0, 0,
        // State 151
        -119, 0, -119, 0, 0, 0, 0, -119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -119, -119, -119, -119, -119, -119, 0, -119, -119, -119, 0, -119, -119, -119, -119, -119, -119, -119, 0, 0,
        // State 152
        -33, 0, -33, 0, 0, 0, 0, -33, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, 0,
        // State 153
        -34, 0, -34, 0, 0, 0, 0, -34, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, 0, 0,
        // State 154
        -122, 0, -122, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, -122, -122, -122, -122, -122, -122, 0, -122, -122, -122, 0, -122, -122, -122, -122, -122, -122, -122, 0, 0,
        // State 155
        -129, 0, -129, 0, 0, 0, 0, -129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, -129, -129, -129, -129, -129, 0, -129, -129, -129, 0, -129, -129, -129, -129, -129, -129, -129, 0, 0,
        // State 156
        -113, 0, -113, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -113, -113, -113, -113, -113, -113, -113, 0, -113, -113, -113, 0, -113, -113, -113, -113, -113, -113, -113, 0, 0,
        // State 157
        -115, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, -115, -115, -115, -115, -115, 0, -115, -115, -115, 0, -115, -115, -115, -115, -115, -115, -115, 0, 0,
        // State 158
        -77, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, -77, -77, -77, -77, -77, 0, -77, -77, -77, 0, -77, -77, -77, -77, -77, -77, -77, 0, 0,
        // State 159
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 0, 0, 0, 0,
        // State 160
        0, 0, 0, -9, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 161
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 162
        0, 0, 0, -10, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 163
        -123, 0, -123, 0, 0, 0, 0, -123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, -123, -123, -123, -123, -123, -123, 0, -123, -123, -123, 0, -123, -123, -123, -123, -123, -123, -123, 0, 0,
        // State 164
        -126, 0, -126, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, -126, -126, -126, -126, -126, 0, -126, -126, -126, 0, -126, -126, -126, -126, -126, -126, -126, 0, 0,
        // State 165
        -121, 0, -121, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, -121, -121, -121, -121, -121, -121, 0, -121, -121, -121, 0, -121, -121, -121, -121, -121, -121, -121, 0, 0,
        // State 166
        -128, 0, -128, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, -128, -128, -128, -128, -128, 0, -128, -128, -128, 0, -128, -128, -128, -128, -128, -128, -128, 0, 0,
        // State 167
        -32, 0, -32, 0, 0, 0, 0, -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, -32, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 39 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -95,
        // State 1
        -96,
        // State 2
        0,
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
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
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
        0,
        // State 43
        -102,
        // State 44
        -25,
        // State 45
        -30,
        // State 46
        -24,
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
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        -137,
        // State 66
        -124,
        // State 67
        -27,
        // State 68
        0,
        // State 69
        -120,
        // State 70
        -23,
        // State 71
        -98,
        // State 72
        -119,
        // State 73
        -97,
        // State 74
        -118,
        // State 75
        -99,
        // State 76
        -100,
        // State 77
        -101,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        -31,
        // State 91
        -125,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        -26,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        0,
        // State 113
        0,
        // State 114
        -133,
        // State 115
        -116,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        0,
        // State 121
        0,
        // State 122
        0,
        // State 123
        0,
        // State 124
        0,
        // State 125
        0,
        // State 126
        0,
        // State 127
        0,
        // State 128
        0,
        // State 129
        -35,
        // State 130
        -130,
        // State 131
        -131,
        // State 132
        -132,
        // State 133
        0,
        // State 134
        -117,
        // State 135
        0,
        // State 136
        0,
        // State 137
        0,
        // State 138
        0,
        // State 139
        0,
        // State 140
        0,
        // State 141
        0,
        // State 142
        0,
        // State 143
        0,
        // State 144
        0,
        // State 145
        0,
        // State 146
        0,
        // State 147
        0,
        // State 148
        -127,
        // State 149
        0,
        // State 150
        -112,
        // State 151
        -119,
        // State 152
        -33,
        // State 153
        -34,
        // State 154
        -122,
        // State 155
        -129,
        // State 156
        -113,
        // State 157
        -115,
        // State 158
        -77,
        // State 159
        0,
        // State 160
        0,
        // State 161
        0,
        // State 162
        0,
        // State 163
        -123,
        // State 164
        -126,
        // State 165
        -121,
        // State 166
        -128,
        // State 167
        -32,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            5 => 143,
            14 => 43,
            15 => 44,
            16 => match state {
                1 | 24 => 90,
                _ => 45,
            },
            18 => match state {
                14 => 24,
                _ => 1,
            },
            19 => match state {
                21 => 123,
                _ => 46,
            },
            20 => 47,
            21 => match state {
                16 => 117,
                _ => 48,
            },
            22 => match state {
                18 => 119,
                _ => 49,
            },
            23 => match state {
                20 => 121,
                _ => 50,
            },
            24 => 51,
            25 => 52,
            26 => 53,
            27 => 54,
            28 => 55,
            29 => 56,
            30 => match state {
                19 => 120,
                _ => 57,
            },
            31 => 58,
            32 => match state {
                2 | 17 => 92,
                _ => 59,
            },
            33 => 60,
            34 => match state {
                11 => 111,
                12 => 112,
                13 => 113,
                22 => 128,
                23 => 133,
                25 => 135,
                32 => 137,
                34 => 142,
                36 => 146,
                40 => 161,
                _ => 61,
            },
            36 => match state {
                15 => 116,
                _ => 62,
            },
            37 => 63,
            38 => match state {
                9 => 107,
                _ => 64,
            },
            39 => 36,
            40 => 147,
            41 => 32,
            42 => 108,
            43 => 15,
            44 => 16,
            45 => 17,
            46 => 18,
            47 => 19,
            48 => 2,
            49 => 20,
            50 => 136,
            51 => 65,
            52 => 66,
            53 => 67,
            54 => 124,
            55 => match state {
                16 => 27,
                _ => 3,
            },
            56 => match state {
                18 => 28,
                _ => 4,
            },
            57 => match state {
                20 => 30,
                _ => 5,
            },
            58 => match state {
                19 => 29,
                _ => 6,
            },
            59 => 7,
            60 => match state {
                17 => 118,
                _ => 93,
            },
            61 => match state {
                15 => 26,
                _ => 8,
            },
            62 => 68,
            63 => 148,
            64 => match state {
                38 => 154,
                41 => 163,
                42 => 165,
                _ => 149,
            },
            65 => 125,
            66 => match state {
                41 => 164,
                42 => 166,
                _ => 155,
            },
            67 => match state {
                39 => 158,
                _ => 69,
            },
            68 => match state {
                37 => 150,
                _ => 70,
            },
            69 => 71,
            70 => match state {
                37 => 151,
                38 | 41..=42 => 156,
                _ => 72,
            },
            71 => match state {
                21 => 126,
                _ => 73,
            },
            72 => match state {
                38 | 41..=42 => 157,
                _ => 74,
            },
            73 => 75,
            74 => 76,
            75 => 77,
            76 => 139,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!""###,
        r###""!=""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""/""###,
        r###"":""###,
        r###"";""###,
        r###""<""###,
        r###""<=""###,
        r###""=""###,
        r###""==""###,
        r###"">""###,
        r###"">=""###,
        r###""and""###,
        r###""else""###,
        r###""false""###,
        r###""fn""###,
        r###""for""###,
        r###""if""###,
        r###""let""###,
        r###""nil""###,
        r###""or""###,
        r###""print""###,
        r###""println""###,
        r###""return""###,
        r###""struct""###,
        r###""true""###,
        r###""while""###,
        r###""{""###,
        r###""}""###,
        r###"identifier"###,
        r###"number"###,
        r###"string"###,
        r###"type_int"###,
        r###"type_string"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i16],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<>
    where 
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = ast::Program;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
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
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 39 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
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
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Bang if true => Some(0),
            Token::BangEqual if true => Some(1),
            Token::LeftParen if true => Some(2),
            Token::RightParen if true => Some(3),
            Token::Star if true => Some(4),
            Token::Plus if true => Some(5),
            Token::Comma if true => Some(6),
            Token::Minus if true => Some(7),
            Token::Slash if true => Some(8),
            Token::Colon if true => Some(9),
            Token::Semicolon if true => Some(10),
            Token::Less if true => Some(11),
            Token::LessEqual if true => Some(12),
            Token::Equal if true => Some(13),
            Token::EqualEqual if true => Some(14),
            Token::Greater if true => Some(15),
            Token::GreaterEqual if true => Some(16),
            Token::And if true => Some(17),
            Token::Else if true => Some(18),
            Token::False if true => Some(19),
            Token::Fn if true => Some(20),
            Token::For if true => Some(21),
            Token::If if true => Some(22),
            Token::Let if true => Some(23),
            Token::Nil if true => Some(24),
            Token::Or if true => Some(25),
            Token::Print if true => Some(26),
            Token::PrintLn if true => Some(27),
            Token::Return if true => Some(28),
            Token::Struct if true => Some(29),
            Token::True if true => Some(30),
            Token::While if true => Some(31),
            Token::LeftBrace if true => Some(32),
            Token::RightBrace if true => Some(33),
            Token::Identifier(_) if true => Some(34),
            Token::Number(_) if true => Some(35),
            Token::String(_) if true => Some(36),
            Token::TypeInt if true => Some(37),
            Token::TypeString if true => Some(38),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 37 | 38 => __Symbol::Variant0(__token),
            34 | 36 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            35 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 28,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 29,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 32,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 33,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 35,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 39,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 40,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 42,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 50,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 67,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 67,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 70,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 70,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 70,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 71,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 73,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 74,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 75,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 75,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 76,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 76,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 77,
                }
            }
            136 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
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
        ) -> Result<ast::Program, __lalrpop_util::ParseError<usize, Token, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<ast::Program,__lalrpop_util::ParseError<usize, Token, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            72 => {
                __reduce72(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            73 => {
                __reduce73(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            74 => {
                __reduce74(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            75 => {
                __reduce75(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            76 => {
                __reduce76(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            77 => {
                __reduce77(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            78 => {
                __reduce78(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            79 => {
                __reduce79(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            80 => {
                __reduce80(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            81 => {
                __reduce81(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            82 => {
                __reduce82(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            83 => {
                __reduce83(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            84 => {
                __reduce84(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            85 => {
                __reduce85(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            86 => {
                __reduce86(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            87 => {
                __reduce87(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            88 => {
                __reduce88(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            89 => {
                __reduce89(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            90 => {
                __reduce90(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            91 => {
                __reduce91(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            92 => {
                __reduce92(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            93 => {
                __reduce93(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            94 => {
                __reduce94(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            95 => {
                __reduce95(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            96 => {
                __reduce96(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            97 => {
                __reduce97(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            98 => {
                __reduce98(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            99 => {
                __reduce99(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            100 => {
                __reduce100(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            101 => {
                __reduce101(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            102 => {
                __reduce102(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            103 => {
                __reduce103(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            104 => {
                __reduce104(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            105 => {
                __reduce105(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            106 => {
                __reduce106(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            107 => {
                __reduce107(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            108 => {
                __reduce108(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            109 => {
                __reduce109(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            110 => {
                __reduce110(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            111 => {
                __reduce111(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            112 => {
                __reduce112(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            113 => {
                __reduce113(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            114 => {
                __reduce114(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            115 => {
                __reduce115(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            116 => {
                __reduce116(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            117 => {
                __reduce117(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            118 => {
                __reduce118(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            119 => {
                __reduce119(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            120 => {
                __reduce120(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            121 => {
                __reduce121(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            122 => {
                __reduce122(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            123 => {
                __reduce123(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            124 => {
                __reduce124(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            125 => {
                __reduce125(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            126 => {
                __reduce126(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            127 => {
                __reduce127(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            128 => {
                __reduce128(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            129 => {
                __reduce129(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            130 => {
                __reduce130(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            131 => {
                __reduce131(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            132 => {
                __reduce132(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            133 => {
                __reduce133(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            134 => {
                __reduce134(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            135 => {
                __reduce135(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            136 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant21(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
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
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<ast::StatementS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::ExprS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpInfix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpPrefix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementBlock, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementFun, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>) = ",", ExprS => ActionFn(79);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action79::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* =  => ActionFn(77);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action77::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(78);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action78::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action127::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(128);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action128::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>) = ",", identifier => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action83::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)* =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)* = ("," <identifier>)+ => ActionFn(82);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action82::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)+ = ",", identifier => ActionFn(131);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action131::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("," <identifier>)+ = ("," <identifier>)+, ",", identifier => ActionFn(132);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action132::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>) = ":", Types => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action109::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? = ":", Types => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action135::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce12<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? =  => ActionFn(108);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action108::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>) = "=", ExprS => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(138);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action138::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce15<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action105::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(80);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action80::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action126::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce18<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce19<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args = ExprS => ActionFn(129);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action129::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce20<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action130::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce21<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Args =  => ActionFn(143);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action143::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = Stmt => ActionFn(4);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = DeclVar => ActionFn(5);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Decl = DeclFun => ActionFn(6);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce25<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclFun = "fn", Function => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action7::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce26<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS = Spanned<Decl> => ActionFn(3);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS* =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce28<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS* = DeclS+ => ActionFn(112);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action112::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce29<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS => ActionFn(113);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action113::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS+, DeclS => ActionFn(114);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action114::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce31<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(139);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant3(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action139::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce32<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(140);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action140::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce33<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(141);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action141::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce34<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ";" => ActionFn(142);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action142::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce35<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAssign = identifier, "=", ExprS => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action39::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce36<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprAssign = ExprLogicOr => ActionFn(40);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce37<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprComparison = ExprInfix<ExprComparison, OpComparison, ExprTerm> => ActionFn(48);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprEquality = ExprInfix<ExprEquality, OpEquality, ExprComparison> => ActionFn(45);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce39<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprFactor = ExprInfix<ExprFactor, OpFactor, ExprPrefix> => ActionFn(56);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(88);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action88::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce41<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(89);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action89::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(90);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action90::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 25)
    }
    pub(crate) fn __reduce43<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(91);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action91::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce44<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action84::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce45<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(85);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce46<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(92);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action92::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce47<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(93);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action93::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce48<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(94);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action94::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 28)
    }
    pub(crate) fn __reduce49<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(95);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action95::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce50<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(86);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action86::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 29)
    }
    pub(crate) fn __reduce51<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(87);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action87::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce52<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprLogicAnd = ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> => ActionFn(43);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprLogicOr = ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> => ActionFn(41);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce54<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(74);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action74::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 32)
    }
    pub(crate) fn __reduce55<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprPrimary => ActionFn(75);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action75::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "nil" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce58<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action63::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(64);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(65);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(66);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 33)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS = Spanned<Expression> => ActionFn(37);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(97);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action97::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce65<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(98);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action98::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 35)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprTerm = ExprInfix<ExprTerm, OpTerm, ExprFactor> => ActionFn(53);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(73);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce68<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ExprAssign => ActionFn(38);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action177::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 39)
    }
    pub(crate) fn __reduce70<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(178);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action178::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(179);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action179::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(180);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action180::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 40)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<DeclVar> => ActionFn(21);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<StmtExpr> => ActionFn(22);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForInit = ";" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce76<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(68);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant22(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action68::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (5, 42)
    }
    pub(crate) fn __reduce77<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = ">" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = ">=" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce79<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = "<" => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce80<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpComparison = "<=" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action52::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce81<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpEquality = "==" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce82<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpEquality = "!=" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce83<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpFactor = "*" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce84<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpFactor = "/" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce85<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpLogicAnd = "and" => ActionFn(44);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce86<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpLogicOr = "or" => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce87<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpPrefix = "-" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce88<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpPrefix = "!" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce89<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpTerm = "+" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce90<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // OpTerm = "-" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce91<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier => ActionFn(133);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action133::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce92<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier, ("," <identifier>)+ => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action134::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(144);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action144::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 50)
    }
    pub(crate) fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(173);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action173::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(174);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtExpr => ActionFn(26);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtBlock => ActionFn(27);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrint => ActionFn(28);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce99<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrintLn => ActionFn(29);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce100<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtReturn => ActionFn(30);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce101<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(159);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action159::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce102<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(160);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action160::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(161);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action161::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(162);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action162::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(163);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(164);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action164::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(165);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(166);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action166::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(167);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(168);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action168::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(169);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(170);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(171);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action171::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(172);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action172::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(175);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action175::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 67)
    }
    pub(crate) fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(176);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action176::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (3, 67)
    }
    pub(crate) fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = StmtOpen => ActionFn(11);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Stmt = StmtClosed => ActionFn(12);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtBlock = StatementBlockIn => ActionFn(34);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 69)
    }
    pub(crate) fn __reduce120<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtClosed> => ActionFn(17);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 70)
    }
    pub(crate) fn __reduce121<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "while", "(", ExprS, ")", Spanned<StmtClosed> => ActionFn(18);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 70)
    }
    pub(crate) fn __reduce122<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtClosed> => ActionFn(19);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 70)
    }
    pub(crate) fn __reduce123<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtClosed = SmallStmt => ActionFn(20);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action20::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 70)
    }
    pub(crate) fn __reduce124<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtExpr = ExprS, ";" => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action36::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 71)
    }
    pub(crate) fn __reduce125<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtOpen> => ActionFn(13);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 72)
    }
    pub(crate) fn __reduce126<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<Stmt> => ActionFn(14);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 72)
    }
    pub(crate) fn __reduce127<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtOpen> => ActionFn(15);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant13(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 72)
    }
    pub(crate) fn __reduce128<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtOpen = "while", "(", ExprS, ")", Spanned<StmtOpen> => ActionFn(16);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant13(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (5, 72)
    }
    pub(crate) fn __reduce129<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtPrint = "print", ExprS, ";" => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 73)
    }
    pub(crate) fn __reduce130<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtPrintLn = "println", ExprS, ";" => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 74)
    }
    pub(crate) fn __reduce131<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 75)
    }
    pub(crate) fn __reduce132<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(182);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action182::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 75)
    }
    pub(crate) fn __reduce133<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Types = type_string => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Types = type_int => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action10::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce135<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __OpPrefix = OpPrefix => ActionFn(1);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 77)
    }
}
pub use self::__parse__Program::ProgramParser;

#[allow(clippy::too_many_arguments)]
fn __action0<
>(
    (_, __0, _): (usize, ast::Program, usize),
) -> ast::Program
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action1<
>(
    (_, __0, _): (usize, ast::OpPrefix, usize),
) -> ast::OpPrefix
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action2<
>(
    (_, statements, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> ast::Program
{
    ast::Program { statements }
}

#[allow(clippy::too_many_arguments)]
fn __action3<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Spanned<ast::Statement>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action4<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action5<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action6<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action7<
>(
    (_, _, _): (usize, Token, usize),
    (_, function, _): (usize, ast::StatementFun, usize),
) -> ast::Statement
{
    ast::Statement::Fun(function)
}

#[allow(clippy::too_many_arguments)]
fn __action8<
>(
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, String, usize),
    (_, type_, _): (usize, core::option::Option<ast::Type>, usize),
    (_, value, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Var(ast::StatementVar {
        var: ast::Var { name, type_: type_},
        value ,
      })
}

#[allow(clippy::too_many_arguments)]
fn __action9<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Type
{
    ast::Type::String
}

#[allow(clippy::too_many_arguments)]
fn __action10<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Type
{
    ast::Type::Int
}

#[allow(clippy::too_many_arguments)]
fn __action11<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action12<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action13<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, init, _): (usize, Option<ast::StatementS>, usize),
    (_, cond, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, update, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::For(Box::new(ast::StatementFor { init, cond, update, body }))
}

#[allow(clippy::too_many_arguments)]
fn __action14<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, then_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::If(Box::new(ast::StatementIf { cond, then_branch, else_branch: None }))
}

#[allow(clippy::too_many_arguments)]
fn __action15<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, then_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
    (_, _, _): (usize, Token, usize),
    (_, else_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::If(Box::new(ast::StatementIf { cond, then_branch, else_branch: Some(else_branch)}))
}

#[allow(clippy::too_many_arguments)]
fn __action16<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::While(Box::new(ast::StatementWhile { cond, body }))
}

#[allow(clippy::too_many_arguments)]
fn __action17<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, then_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
    (_, _, _): (usize, Token, usize),
    (_, else_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::If(Box::new(ast::StatementIf { cond, then_branch, else_branch: Some(else_branch) }))
}

#[allow(clippy::too_many_arguments)]
fn __action18<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::While(Box::new(ast::StatementWhile { cond, body }))
}

#[allow(clippy::too_many_arguments)]
fn __action19<
>(
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, init, _): (usize, Option<ast::StatementS>, usize),
    (_, cond, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, update, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::For(Box::new(ast::StatementFor { init, cond, update, body }))
}

#[allow(clippy::too_many_arguments)]
fn __action20<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action21<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> Option<ast::StatementS>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action22<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> Option<ast::StatementS>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action23<
>(
    (_, __0, _): (usize, Token, usize),
) -> Option<ast::StatementS>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action24<
>(
    (_, __0, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action25<
>(
    (_, __0, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action26<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action27<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action28<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action29<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action30<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action31<
>(
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Return(ast::StatementReturn { value })
}

#[allow(clippy::too_many_arguments)]
fn __action32<
>(
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::PrintLn(ast::StatementPrintLn { value })
}

#[allow(clippy::too_many_arguments)]
fn __action33<
>(
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Print(ast::StatementPrint { value })
}

#[allow(clippy::too_many_arguments)]
fn __action34<
>(
    (_, __0, _): (usize, ast::StatementBlock, usize),
) -> ast::Statement
{
    ast::Statement::Block(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action35<
>(
    (_, _, _): (usize, Token, usize),
    (_, statements, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::StatementBlock
{
    ast::StatementBlock { statements }
}

#[allow(clippy::too_many_arguments)]
fn __action36<
>(
    (_, expr, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Expression(ast::StatementExpr { expr })
}

#[allow(clippy::too_many_arguments)]
fn __action37<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action38<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action39<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Assign(Box::new(ast::ExprAssign {
            lhs: ast::Var { name, type_: None },
            rhs: value,
        }))
}

#[allow(clippy::too_many_arguments)]
fn __action40<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action41<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action42<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LogicOr
}

#[allow(clippy::too_many_arguments)]
fn __action43<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action44<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LogicAnd
}

#[allow(clippy::too_many_arguments)]
fn __action45<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action46<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Equal
}

#[allow(clippy::too_many_arguments)]
fn __action47<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::NotEqual
}

#[allow(clippy::too_many_arguments)]
fn __action48<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action49<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Greater
}

#[allow(clippy::too_many_arguments)]
fn __action50<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::GreaterEqual
}

#[allow(clippy::too_many_arguments)]
fn __action51<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Less
}

#[allow(clippy::too_many_arguments)]
fn __action52<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LessEqual
}

#[allow(clippy::too_many_arguments)]
fn __action53<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action54<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Add
}

#[allow(clippy::too_many_arguments)]
fn __action55<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Sub
}

#[allow(clippy::too_many_arguments)]
fn __action56<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action57<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Mul
}

#[allow(clippy::too_many_arguments)]
fn __action58<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Div
}

#[allow(clippy::too_many_arguments)]
fn __action59<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpPrefix
{
    ast::OpPrefix::Negate
}

#[allow(clippy::too_many_arguments)]
fn __action60<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::OpPrefix
{
    ast::OpPrefix::Not
}

#[allow(clippy::too_many_arguments)]
fn __action61<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Nil)
}

#[allow(clippy::too_many_arguments)]
fn __action62<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(false))
}

#[allow(clippy::too_many_arguments)]
fn __action63<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(true))
}

#[allow(clippy::too_many_arguments)]
fn __action64<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::String(__0))
}

#[allow(clippy::too_many_arguments)]
fn __action65<
>(
    (_, __0, _): (usize, f64, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Number(__0))
}

#[allow(clippy::too_many_arguments)]
fn __action66<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action67<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action68<
>(
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, params, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::StatementBlock, usize),
) -> ast::StatementFun
{
    ast::StatementFun { name, params, body }
}

#[allow(clippy::too_many_arguments)]
fn __action69<
>(
    (_, first, _): (usize, String, usize),
    (_, mut params, _): (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    {
        params.insert(0, first);
        params
    }
}

#[allow(clippy::too_many_arguments)]
fn __action70<
>(
    (_, __0, _): (usize, (), usize),
) -> Vec<String>
{
    Vec::new()
}

#[allow(clippy::too_many_arguments)]
fn __action71<
>(
    (_, first, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, mut args, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> Vec<ast::ExprS>
{
    {
        args.insert(0, first);
        args
    }
}

#[allow(clippy::too_many_arguments)]
fn __action72<
>(
    (_, __0, _): (usize, (), usize),
) -> Vec<ast::ExprS>
{
    Vec::new()
}

#[allow(clippy::too_many_arguments)]
fn __action73<
>(
    (_, name, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Var(ast::ExprVar { var: ast::Var {name, type_: None} })
}

#[allow(clippy::too_many_arguments)]
fn __action74<
>(
    (_, op, _): (usize, ast::OpPrefix, usize),
    (_, rt, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Prefix(Box::new(ast::ExprPrefix { op, rt }))
}

#[allow(clippy::too_many_arguments)]
fn __action75<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action76<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action77<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action78<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action79<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action80<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
)
{
}

#[allow(clippy::too_many_arguments)]
fn __action81<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action82<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action83<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action84<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action85<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action86<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action87<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action88<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action89<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action90<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action91<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action92<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action93<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action94<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action95<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action96<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action97<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action98<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action99<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action100<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action101<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action102<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action103<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action104<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action105<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action106<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action107<
>(
    (_, __0, _): (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action108<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Type>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action109<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action110<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action111<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action112<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action113<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action114<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action115<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action116<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action117<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action118<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action119<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action120<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action121<
>(
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action122<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action123<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action124<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    { let mut v = v; v.push(e); v }
}

fn __action125<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookbehind
}

fn __action126<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookahead
}

#[allow(clippy::too_many_arguments)]
fn __action127<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action79(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action123(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action128<
>(
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action79(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action124(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action129<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action77(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action130<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action78(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action131<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action83(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action132<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, Token, usize),
    __2: (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action83(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action133<
>(
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action81(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action134<
>(
    __0: (usize, String, usize),
    __1: (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action82(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action135<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action109(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action136<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Type, usize),
    __4: (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    __5: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action135(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __0,
        __1,
        __temp0,
        __4,
        __5,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action137<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    __3: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action108(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action138<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action106(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action139<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Type, usize),
    __4: (usize, Token, usize),
    __5: (usize, ast::Spanned<ast::Expression>, usize),
    __6: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __4.0;
    let __end0 = __5.2;
    let __temp0 = __action138(
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action136(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __6,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action140<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Type, usize),
    __4: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action105(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action136(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action141<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Spanned<ast::Expression>, usize),
    __4: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action138(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action137(
        __0,
        __1,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action142<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action105(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action137(
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action143<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::ExprS>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action80(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action144<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action80(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action145<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action146<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action147<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action148<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action149<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action150<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action116(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action151<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action115(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action152<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action153<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action119(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action154<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action155<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action156<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action157<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action158<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action126(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action159<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action160<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action161<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action147(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action162<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action148(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action163<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action149(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action164<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action150(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action165<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action151(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action166<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action152(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action167<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action153(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action168<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action154(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action169<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action170<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action171<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action157(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action172<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action158(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action173<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ast::Program
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action111(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action174<
>(
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> ast::Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action112(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action175<
>(
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action111(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __0,
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action176<
>(
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    __2: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action112(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __0,
        __temp0,
        __2,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action177<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action97(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action178<
>(
    __0: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action98(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action179<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action97(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action180<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action98(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action181<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action97(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __0,
        __temp0,
        __2,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action182<
>(
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action98(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __0,
        __temp0,
        __1,
    )
}
#[allow(clippy::type_complexity)]

pub trait __ToTriple<>
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>>;
}

impl<> __ToTriple<> for (usize, Token, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), LexicalError>
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
