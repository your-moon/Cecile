// auto-generated: "lalrpop 0.20.0"
// sha3: b458c334f667cacf2612b38913333de54ba40aecc0ddeeca338ad6fa19112b82
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
        3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -139,
        // State 2
        -91,
        // State 3
        -90,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            49 => 1,
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
        r###"".""###,
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
            __action(state, 40 - 1)
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
            Token::Dot if true => Some(8),
            Token::Slash if true => Some(9),
            Token::Colon if true => Some(10),
            Token::Semicolon if true => Some(11),
            Token::Less if true => Some(12),
            Token::LessEqual if true => Some(13),
            Token::Equal if true => Some(14),
            Token::EqualEqual if true => Some(15),
            Token::Greater if true => Some(16),
            Token::GreaterEqual if true => Some(17),
            Token::And if true => Some(18),
            Token::Else if true => Some(19),
            Token::False if true => Some(20),
            Token::Fn if true => Some(21),
            Token::For if true => Some(22),
            Token::If if true => Some(23),
            Token::Let if true => Some(24),
            Token::Nil if true => Some(25),
            Token::Or if true => Some(26),
            Token::Print if true => Some(27),
            Token::PrintLn if true => Some(28),
            Token::Return if true => Some(29),
            Token::Struct if true => Some(30),
            Token::True if true => Some(31),
            Token::While if true => Some(32),
            Token::LeftBrace if true => Some(33),
            Token::RightBrace if true => Some(34),
            Token::Identifier(_) if true => Some(35),
            Token::Number(_) if true => Some(36),
            Token::String(_) if true => Some(37),
            Token::TypeInt if true => Some(38),
            Token::TypeString if true => Some(39),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 38 | 39 => __Symbol::Variant0(__token),
            35 | 37 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            36 => match __token {
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
                    states_to_pop: 4,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
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
                    states_to_pop: 3,
                    nonterminal_produced: 30,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
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
                    states_to_pop: 2,
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
                    nonterminal_produced: 34,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
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
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 36,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
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
                    states_to_pop: 2,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 41,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
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
                    nonterminal_produced: 46,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
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
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    nonterminal_produced: 53,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
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
                    nonterminal_produced: 53,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 2,
                    nonterminal_produced: 69,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 69,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
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
                    states_to_pop: 1,
                    nonterminal_produced: 72,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 73,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 74,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 74,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 74,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 74,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 75,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 76,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 77,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 77,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 78,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 78,
                }
            }
            138 => __state_machine::SimulatedReduce::Accept,
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 80,
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
                __reduce135(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            136 => {
                __reduce136(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            137 => {
                __reduce137(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            138 => {
                // __OpPrefix = OpPrefix => ActionFn(1);
                let __sym0 = __pop_Variant19(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            139 => {
                __reduce139(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        // ("," <ExprS>) = ",", ExprS => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(__sym0, __sym1);
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
        // ("," <ExprS>)* =  => ActionFn(78);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action78::<>(&__start, &__end);
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
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(79);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action79::<>(__sym0);
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
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action130::<>(__sym0, __sym1);
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
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(131);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action131::<>(__sym0, __sym1, __sym2);
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
        // ("," <identifier>) = ",", identifier => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(__sym0, __sym1);
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
        // ("," <identifier>)* =  => ActionFn(82);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82::<>(&__start, &__end);
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
        // ("," <identifier>)* = ("," <identifier>)+ => ActionFn(83);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(__sym0);
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
        // ("," <identifier>)+ = ",", identifier => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action134::<>(__sym0, __sym1);
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
        // ("," <identifier>)+ = ("," <identifier>)+, ",", identifier => ActionFn(135);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action135::<>(__sym0, __sym1, __sym2);
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
        // (":" <Types>) = ":", Types => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action112::<>(__sym0, __sym1);
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
        // (":" <Types>)? = ":", Types => ActionFn(138);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action138::<>(__sym0, __sym1);
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
        // (":" <Types>)? =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(&__start, &__end);
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
        // ("=" <ExprS>) = "=", ExprS => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action109::<>(__sym0, __sym1);
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
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action141::<>(__sym0, __sym1);
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
        // ("=" <ExprS>)? =  => ActionFn(108);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action108::<>(&__start, &__end);
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
        // () =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(&__start, &__end);
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
        // @L =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(&__start, &__end);
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
        // @R =  => ActionFn(124);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action124::<>(&__start, &__end);
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
        // Args = ExprS => ActionFn(132);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action132::<>(__sym0);
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
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action133::<>(__sym0, __sym1);
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
        // Args =  => ActionFn(146);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action146::<>(&__start, &__end);
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
        // DeclS* =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action114::<>(&__start, &__end);
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
        // DeclS* = DeclS+ => ActionFn(115);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action115::<>(__sym0);
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
        // DeclS+ = DeclS => ActionFn(116);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action116::<>(__sym0);
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
        // DeclS+ = DeclS+, DeclS => ActionFn(117);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action117::<>(__sym0, __sym1);
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
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(142);
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
        let __nt = super::__action142::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
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
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(143);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action143::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
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
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(144);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action144::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
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
        // DeclVar = "let", identifier, ";" => ActionFn(145);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action145::<>(__sym0, __sym1, __sym2);
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
        // ExprCall = Spanned<ExprCall>, "(", Args, ")" => ActionFn(63);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action63::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (4, 21)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprCall = ExprPrimary => ActionFn(64);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce39<
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
        (1, 22)
    }
    pub(crate) fn __reduce40<
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
        (1, 23)
    }
    pub(crate) fn __reduce41<
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
        (1, 24)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(92);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action92::<>(__sym0);
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
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(93);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(94);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action94::<>(__sym0);
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
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(87);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(88);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action88::<>(__sym0);
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
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(96);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action96::<>(__sym0);
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
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(97);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action97::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(98);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(__sym0);
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
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(89);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 30)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce54<
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
        (1, 31)
    }
    pub(crate) fn __reduce55<
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
        (1, 32)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(61);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action61::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprCall => ActionFn(62);
        let __sym0 = __pop_Variant15(__symbols);
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
        // ExprPrimary = "nil" => ActionFn(65);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(67);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action67::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(68);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action68::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(69);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action69::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(70);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(71);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce65<
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
        (1, 35)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action101::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 36)
    }
    pub(crate) fn __reduce68<
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
        (1, 37)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce70<
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
        (1, 39)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(182);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action182::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 40)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(183);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action183::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(184);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action184::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(185);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action185::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 41)
    }
    pub(crate) fn __reduce75<
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
        (1, 42)
    }
    pub(crate) fn __reduce76<
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
        (1, 42)
    }
    pub(crate) fn __reduce77<
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
        (1, 42)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(72);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant22(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action72::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce79<
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
        (1, 44)
    }
    pub(crate) fn __reduce80<
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
        (1, 44)
    }
    pub(crate) fn __reduce81<
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
        (1, 44)
    }
    pub(crate) fn __reduce82<
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
        (1, 44)
    }
    pub(crate) fn __reduce83<
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
        (1, 45)
    }
    pub(crate) fn __reduce84<
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
        (1, 45)
    }
    pub(crate) fn __reduce85<
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
        (1, 46)
    }
    pub(crate) fn __reduce86<
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
        (1, 46)
    }
    pub(crate) fn __reduce87<
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
        (1, 47)
    }
    pub(crate) fn __reduce88<
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
        (1, 48)
    }
    pub(crate) fn __reduce89<
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
        (1, 49)
    }
    pub(crate) fn __reduce90<
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
        (1, 49)
    }
    pub(crate) fn __reduce91<
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
        (1, 50)
    }
    pub(crate) fn __reduce92<
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
        (1, 50)
    }
    pub(crate) fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier => ActionFn(136);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action136::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier, ("," <identifier>)+ => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action137::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 51)
    }
    pub(crate) fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(147);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action147::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(178);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action178::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(179);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action179::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
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
        (1, 53)
    }
    pub(crate) fn __reduce99<
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
        (1, 53)
    }
    pub(crate) fn __reduce100<
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
        (1, 53)
    }
    pub(crate) fn __reduce101<
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
        (1, 53)
    }
    pub(crate) fn __reduce102<
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
        (1, 53)
    }
    pub(crate) fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(163);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(164);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action164::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprCall> = ExprCall => ActionFn(165);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(166);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action166::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(167);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(168);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action168::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(169);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(170);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(171);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action171::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(172);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action172::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(173);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action173::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(174);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(175);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action175::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(176);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action176::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(177);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action177::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(180);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action180::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 69)
    }
    pub(crate) fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (3, 69)
    }
    pub(crate) fn __reduce120<
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
        (1, 70)
    }
    pub(crate) fn __reduce121<
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
        (1, 70)
    }
    pub(crate) fn __reduce122<
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
        (1, 71)
    }
    pub(crate) fn __reduce123<
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
        (7, 72)
    }
    pub(crate) fn __reduce124<
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
        (5, 72)
    }
    pub(crate) fn __reduce125<
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
        (7, 72)
    }
    pub(crate) fn __reduce126<
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
        (1, 72)
    }
    pub(crate) fn __reduce127<
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
        (2, 73)
    }
    pub(crate) fn __reduce128<
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
        (7, 74)
    }
    pub(crate) fn __reduce129<
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
        (5, 74)
    }
    pub(crate) fn __reduce130<
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
        (7, 74)
    }
    pub(crate) fn __reduce131<
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
        (5, 74)
    }
    pub(crate) fn __reduce132<
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
        (3, 75)
    }
    pub(crate) fn __reduce133<
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
        (3, 76)
    }
    pub(crate) fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(186);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action186::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 77)
    }
    pub(crate) fn __reduce135<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(187);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action187::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 77)
    }
    pub(crate) fn __reduce136<
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
        (1, 78)
    }
    pub(crate) fn __reduce137<
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
        (1, 78)
    }
    pub(crate) fn __reduce139<
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
        (1, 80)
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
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 11, 88, 89, 90, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 1
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 11, 88, 89, 90, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 2
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 103, 0, 0, 104, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 108, 0, 0, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 112, 0, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 0, 0, 0, 0,
        // State 11
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 12
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 13
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 14
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 11, 88, 89, 90, 91, 0, 12, 13, 14, 0, 92, 93, 15, 122, 94, 95, 96, 0, 0,
        // State 15
        85, 0, 10, -22, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 16
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 17
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 18
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 19
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 20
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 21
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 101, 95, 96, 0, 0,
        // State 22
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 90, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 23
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 24
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 25
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 11, 88, 89, 90, 91, 0, 12, 13, 14, 0, 92, 93, 15, 142, 94, 95, 96, 0, 0,
        // State 26
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 27
        0, 0, 0, -20, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -43, 0, -43, 0, 112, -43, 113, 0, 0, 0, -43, -43, -43, 0, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -45, 0, -45, 0, 0, -45, 0, 0, 0, 0, -45, 102, 103, 0, -45, 104, 105, -45, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 106, 0, -49, 0, 0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 107, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, -51, 0, 0, -51, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -53, 0, -53, 108, -53, -53, -53, 0, 109, 0, -53, -53, -53, 0, -53, -53, -53, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0,
        // State 34
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 148, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 151,
        // State 36
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 37
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 38
        0, 0, 0, -94, 0, 0, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        85, 0, 10, -75, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 40
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 88, 89, 0, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 41
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 88, 89, 0, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 42
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 44
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 0, 91, 0, 0, 0, 0, 0, 92, 0, 0, 0, 94, 95, 96, 0, 0,
        // State 45
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 88, 89, 0, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 46
        85, 0, 10, 0, 0, 0, 0, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 0, 88, 89, 0, 91, 0, 12, 13, 14, 0, 92, 93, 15, 0, 94, 95, 96, 0, 0,
        // State 47
        -104, 0, -104, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, -104, -104, -104, -104, -104, 0, -104, -104, -104, 0, -104, -104, -104, -104, -104, -104, -104, 0, 0,
        // State 48
        -25, 0, -25, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, 0, -25, -25, -25, 0, -25, -25, -25, -25, -25, -25, -25, 0, 0,
        // State 49
        -30, 0, -30, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, -30, -30, -30, -30, 0, -30, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, 0, 0,
        // State 50
        -24, 0, -24, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, 0, -24, -24, -24, 0, -24, -24, -24, -24, -24, -24, -24, 0, 0,
        // State 51
        0, 0, 0, -71, 0, 0, -71, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -58, -106, -58, -58, -58, -58, -58, 0, -58, 0, -58, -58, -58, 0, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -46, 0, -46, 0, 0, -46, 0, 0, 0, 0, -46, -107, -107, 0, -46, -107, -107, -46, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, -108, 0, -50, 0, 0, -50, 0, 0, 0, 0, -50, 0, 0, 0, -108, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, -54, 0, -54, -109, -54, -54, -54, 0, -109, 0, -54, -54, -54, 0, -54, -54, -54, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, -40, 0, -40, 0, 0, -40, 0, 0, 0, 0, -40, -40, -40, 0, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, -41, 0, -41, 0, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, -42, 0, -42, -42, -42, -42, -42, 0, -42, 0, -42, -42, -42, 0, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, -55, 0, 0, -55, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, -56, 0, 0, -56, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, -69, 0, -69, 0, -69, -69, -69, 0, 0, 0, -69, -69, -69, 0, -69, -69, -69, -69, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, -52, 0, 0, -52, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, -37, 0, 0, -37, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, -48, 0, -48, -48, -48, -48, -48, 0, -48, 0, -48, -48, -48, 0, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, -39, -39, -39, 0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -44, 0, -44, 0, -113, -44, -113, 0, 0, 0, -44, -44, -44, 0, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, -64, -64, -64, -64, -64, -64, -64, 0, -64, 0, -64, -64, -64, 0, -64, -64, -64, -64, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, -114, 0, 0, -114, 0, 0, 0, 0, -114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        -127, 0, -127, 0, 0, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -127, -127, -127, -127, -127, -127, -127, 0, -127, -127, -127, 0, -127, -127, -127, -127, -127, -127, -127, 0, 0,
        // State 72
        -27, 0, -27, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, 0, -27, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, 0,
        // State 73
        0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, -66, 0, 0, -66, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        -123, 0, -123, 0, 0, 0, 0, -123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -123, -123, -123, -123, -123, -123, -123, 0, -123, -123, -123, 0, -123, -123, -123, -123, -123, -123, -123, 0, 0,
        // State 76
        -23, 0, -23, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, 0, -23, -23, -23, 0, -23, -23, -23, -23, -23, -23, -23, 0, 0,
        // State 77
        -100, 0, -100, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, -100, -100, -100, -100, -100, -100, 0, -100, -100, -100, 0, -100, -100, -100, -100, -100, -100, -100, 0, 0,
        // State 78
        -122, 0, -122, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, -122, -122, -122, -122, -122, 0, -122, -122, -122, 0, -122, -122, -122, -122, -122, -122, -122, 0, 0,
        // State 79
        -99, 0, -99, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, -99, -99, -99, -99, -99, -99, 0, -99, -99, -99, 0, -99, -99, -99, -99, -99, -99, -99, 0, 0,
        // State 80
        -121, 0, -121, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, -121, -121, -121, -121, -121, 0, -121, -121, -121, 0, -121, -121, -121, -121, -121, -121, -121, 0, 0,
        // State 81
        -101, 0, -101, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, -101, -101, -101, -101, -101, -101, 0, -101, -101, -101, 0, -101, -101, -101, -101, -101, -101, -101, 0, 0,
        // State 82
        -102, 0, -102, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, -102, -102, -102, -102, -102, -102, 0, -102, -102, -102, 0, -102, -102, -102, -102, -102, -102, -102, 0, 0,
        // State 83
        -103, 0, -103, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, -103, -103, -103, -103, -103, -103, 0, -103, -103, -103, 0, -103, -103, -103, -103, -103, -103, -103, 0, 0,
        // State 84
        -91, 0, -91, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, 0, 0, 0, -91, -91, -91, 0, 0,
        // State 85
        -90, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, 0, -90, -90, -90, 0, 0,
        // State 86
        0, -60, -60, -60, -60, -60, -60, -60, 0, -60, 0, -60, -60, -60, 0, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 117, 0, 0, 0, 0,
        // State 90
        0, -59, -59, -59, -59, -59, -59, -59, 0, -59, 0, -59, -59, -59, 0, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, -61, -61, -61, -61, -61, -61, -61, 0, -61, 0, -61, -61, -61, 0, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, -70, -70, -70, -70, -70, -70, -70, 0, -70, 0, -70, -70, -70, 27, -70, -70, -70, -70, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, -63, -63, -63, -63, -63, -63, -63, 0, -63, 0, -63, -63, -63, 0, -63, -63, -63, -63, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, -62, -62, -62, -62, -62, -62, -62, 0, -62, 0, -62, -62, -62, 0, -62, -62, -62, -62, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        -31, 0, -31, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, -31, -31, -31, -31, 0, -31, -31, -31, 0, -31, -31, -31, -31, -31, -31, -31, 0, 0,
        // State 97
        -128, 0, -128, 0, 0, 0, 0, -128, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, -128, -128, -128, -128, -128, -128, -128, 0, -128, -128, -128, 0, -128, -128, -128, -128, -128, -128, -128, 0, 0,
        // State 98
        0, -112, 0, -112, -112, -112, -112, -112, 0, -112, 0, -112, -112, -112, 0, -112, -112, -112, -112, 0, 0, 0, 0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, -57, 0, -57, -57, -57, -57, -57, 0, -57, 0, -57, -57, -57, 0, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, -70, -70, -70, -70, -70, -70, -70, 0, -70, 0, -70, -70, -70, 0, -70, -70, -70, -70, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        -82, 0, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, -82, 0, 0, 0, -82, -82, -82, 0, 0,
        // State 102
        -83, 0, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, -83, -83, -83, 0, 0,
        // State 103
        -80, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, -80, 0, 0, 0, -80, -80, -80, 0, 0,
        // State 104
        -81, 0, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, -81, 0, 0, 0, -81, -81, -81, 0, 0,
        // State 105
        -85, 0, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, -85, -85, -85, 0, 0,
        // State 106
        -84, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, -84, 0, 0, 0, -84, -84, -84, 0, 0,
        // State 107
        -86, 0, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, -86, 0, 0, 0, -86, -86, -86, 0, 0,
        // State 108
        -87, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, -87, 0, 0, 0, -87, -87, -87, 0, 0,
        // State 109
        -88, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, -88, 0, 0, 0, -88, -88, -88, 0, 0,
        // State 110
        -89, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, 0, 0, 0, -89, -89, -89, 0, 0,
        // State 111
        -92, 0, -92, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, -92, 0, 0, 0, -92, -92, -92, 0, 0,
        // State 112
        -93, 0, -93, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, -93, 0, 0, 0, -93, -93, -93, 0, 0,
        // State 113
        0, 0, 0, 130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        -26, 0, -26, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, 0, -26, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, 0,
        // State 115
        0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 137, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        -136, 0, -136, 0, 0, 0, 0, -136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -136, -136, -136, -136, -136, -136, -136, 0, -136, -136, -136, 0, -136, -136, -136, -136, -136, -136, -136, 0, 0,
        // State 121
        -119, 0, -119, 0, 0, 0, 0, -119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -119, -119, -119, -119, -119, -119, -119, 0, -119, -119, -119, 0, -119, -119, -119, -119, -119, -119, -119, 0, 0,
        // State 122
        0, 0, 0, 144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, -113, 0, -113, 0, -113, -113, -113, 0, 0, 0, -113, -113, -113, 0, -113, -113, -113, -113, 0, 0, 0, 0, 0, 0, 0, -113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, -107, 0, -107, 0, 0, -107, 0, 0, 0, 0, -107, -107, -107, 0, -107, -107, -107, -107, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        0, -47, 0, -47, -47, -47, -47, -47, 0, -47, 0, -47, -47, -47, 0, -47, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        0, -108, 0, -108, 0, 0, -108, 0, 0, 0, 0, -108, 0, 0, 0, -108, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 0, -110, 0, 0, -110, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, -110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, -109, 0, -109, -109, -109, -109, -109, 0, -109, 0, -109, -109, -109, 0, -109, -109, -109, -109, 0, 0, 0, 0, 0, 0, 0, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, -65, -65, -65, -65, -65, -65, -65, 0, -65, 0, -65, -65, -65, 0, -65, -65, -65, -65, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        -105, 0, -105, 0, 0, 0, 0, -105, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, -105, 0, 0, 0, -105, -105, -105, 0, 0,
        // State 131
        -76, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, -76, 0, 0, 0, -76, -76, -76, 0, 0,
        // State 132
        -77, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, -77, 0, 0, 0, -77, -77, -77, 0, 0,
        // State 133
        -117, 0, -117, 0, 0, 0, 0, -117, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, -117, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, -117, 0, 0, 0, -117, -117, -117, 0, 0,
        // State 134
        -78, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, -78, 0, 0, 0, -78, -78, -78, 0, 0,
        // State 135
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        -35, 0, -35, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, 0, 0,
        // State 137
        -133, 0, -133, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, -133, -133, -133, -133, -133, -133, 0, -133, -133, -133, 0, -133, -133, -133, -133, -133, -133, -133, 0, 0,
        // State 138
        -134, 0, -134, 0, 0, 0, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, -134, -134, -134, -134, -134, -134, 0, -134, -134, -134, 0, -134, -134, -134, -134, -134, -134, -134, 0, 0,
        // State 139
        -135, 0, -135, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -135, -135, -135, -135, -135, -135, -135, 0, -135, -135, -135, 0, -135, -135, -135, -135, -135, -135, -135, 0, 0,
        // State 140
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        -120, 0, -120, 0, 0, 0, 0, -120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, -120, -120, -120, -120, -120, -120, 0, -120, -120, -120, 0, -120, -120, -120, -120, -120, -120, -120, 0, 0,
        // State 142
        0, 0, 0, -36, 0, 0, -36, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        0, -38, -38, -38, -38, -38, -38, -38, 0, -38, 0, -38, -38, -38, 0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 144
        0, 0, 0, -21, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        -73, 0, -73, -73, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, -73, 0, 0, 0, -73, -73, -73, 0, 0,
        // State 148
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -138, 0, 0, -138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -137, 0, 0, -137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 151
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 153
        0, 0, 0, -95, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 0, 0, 0, 0,
        // State 155
        -72, 0, -72, -72, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, -72, 0, 0, 0, -72, -72, -72, 0, 0,
        // State 156
        0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 157
        0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        -130, 0, -130, 0, 0, 0, 0, -130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -130, -130, -130, -130, -130, -130, 0, -130, -130, -130, 0, -130, -130, -130, -130, -130, -130, -130, 0, 0,
        // State 159
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 160
        -115, 0, -115, 0, 0, 0, 0, -115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, -115, -115, -115, -115, -115, 0, -115, -115, -115, 0, -115, -115, -115, -115, -115, -115, -115, 0, 0,
        // State 161
        -122, 0, -122, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, -122, -122, -122, -122, -122, -122, 0, -122, -122, -122, 0, -122, -122, -122, -122, -122, -122, -122, 0, 0,
        // State 162
        -33, 0, -33, 0, 0, 0, 0, -33, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, 0,
        // State 163
        -34, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, 0, 0,
        // State 164
        -125, 0, -125, 0, 0, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125, -125, -125, -125, -125, -125, -125, 0, -125, -125, -125, 0, -125, -125, -125, -125, -125, -125, -125, 0, 0,
        // State 165
        -132, 0, -132, 0, 0, 0, 0, -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, -132, -132, -132, -132, -132, 0, -132, -132, -132, 0, -132, -132, -132, -132, -132, -132, -132, 0, 0,
        // State 166
        -116, 0, -116, 0, 0, 0, 0, -116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, -116, -116, -116, -116, -116, -116, 0, -116, -116, -116, 0, -116, -116, -116, -116, -116, -116, -116, 0, 0,
        // State 167
        -118, 0, -118, 0, 0, 0, 0, -118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -118, -118, -118, -118, -118, -118, 0, -118, -118, -118, 0, -118, -118, -118, -118, -118, -118, -118, 0, 0,
        // State 168
        0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 169
        -79, 0, -79, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, -79, -79, -79, -79, -79, 0, -79, -79, -79, 0, -79, -79, -79, -79, -79, -79, -79, 0, 0,
        // State 170
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 174, 0, 0, 0, 0,
        // State 171
        0, 0, 0, -9, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 172
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 173
        0, 0, 0, -10, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 174
        -126, 0, -126, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, -126, -126, -126, -126, -126, -126, 0, -126, -126, -126, 0, -126, -126, -126, -126, -126, -126, -126, 0, 0,
        // State 175
        -129, 0, -129, 0, 0, 0, 0, -129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, -129, -129, -129, -129, -129, 0, -129, -129, -129, 0, -129, -129, -129, -129, -129, -129, -129, 0, 0,
        // State 176
        -124, 0, -124, 0, 0, 0, 0, -124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -124, -124, -124, -124, -124, -124, -124, 0, -124, -124, -124, 0, -124, -124, -124, -124, -124, -124, -124, 0, 0,
        // State 177
        -131, 0, -131, 0, 0, 0, 0, -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, -131, -131, -131, -131, -131, 0, -131, -131, -131, 0, -131, -131, -131, -131, -131, -131, -131, 0, 0,
        // State 178
        -32, 0, -32, 0, 0, 0, 0, -32, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, -32, -32, -32, 0, -32, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 40 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -97,
        // State 1
        -98,
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
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -104,
        // State 48
        -25,
        // State 49
        -30,
        // State 50
        -24,
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
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        -140,
        // State 71
        -127,
        // State 72
        -27,
        // State 73
        0,
        // State 74
        0,
        // State 75
        -123,
        // State 76
        -23,
        // State 77
        -100,
        // State 78
        -122,
        // State 79
        -99,
        // State 80
        -121,
        // State 81
        -101,
        // State 82
        -102,
        // State 83
        -103,
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
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        -31,
        // State 97
        -128,
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
        0,
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
        -26,
        // State 115
        0,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        -136,
        // State 121
        -119,
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
        0,
        // State 130
        0,
        // State 131
        0,
        // State 132
        0,
        // State 133
        0,
        // State 134
        0,
        // State 135
        0,
        // State 136
        -35,
        // State 137
        -133,
        // State 138
        -134,
        // State 139
        -135,
        // State 140
        0,
        // State 141
        -120,
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
        0,
        // State 149
        0,
        // State 150
        0,
        // State 151
        0,
        // State 152
        0,
        // State 153
        0,
        // State 154
        0,
        // State 155
        0,
        // State 156
        0,
        // State 157
        0,
        // State 158
        -130,
        // State 159
        0,
        // State 160
        -115,
        // State 161
        -122,
        // State 162
        -33,
        // State 163
        -34,
        // State 164
        -125,
        // State 165
        -132,
        // State 166
        -116,
        // State 167
        -118,
        // State 168
        0,
        // State 169
        -79,
        // State 170
        0,
        // State 171
        0,
        // State 172
        0,
        // State 173
        0,
        // State 174
        -126,
        // State 175
        -129,
        // State 176
        -124,
        // State 177
        -131,
        // State 178
        -32,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            2 => 144,
            5 => 153,
            13 => 122,
            14 => 47,
            15 => 48,
            16 => match state {
                1 | 25 => 96,
                _ => 49,
            },
            18 => match state {
                14 => 25,
                _ => 1,
            },
            19 => match state {
                22 => 130,
                _ => 50,
            },
            20 => 51,
            21 => 52,
            22 => match state {
                17 => 124,
                _ => 53,
            },
            23 => match state {
                19 => 126,
                _ => 54,
            },
            24 => match state {
                21 => 128,
                _ => 55,
            },
            25 => 56,
            26 => 57,
            27 => 58,
            28 => 59,
            29 => 60,
            30 => 61,
            31 => match state {
                20 => 127,
                _ => 62,
            },
            32 => 63,
            33 => match state {
                2 | 18 => 98,
                _ => 64,
            },
            34 => 65,
            35 => match state {
                15 => 27,
                11 => 117,
                12 => 118,
                13 => 119,
                23 => 135,
                24 => 140,
                26 => 142,
                34 => 146,
                36 => 151,
                37 => 152,
                39 => 156,
                42 => 168,
                44 => 172,
                _ => 66,
            },
            37 => match state {
                16 => 123,
                _ => 67,
            },
            38 => 68,
            39 => match state {
                9 => 113,
                _ => 69,
            },
            40 => 39,
            41 => 157,
            42 => 34,
            43 => 114,
            44 => 16,
            45 => 17,
            46 => 18,
            47 => 19,
            48 => 20,
            49 => 2,
            50 => 21,
            51 => 145,
            52 => 70,
            53 => 71,
            54 => 72,
            55 => 131,
            56 => 73,
            57 => match state {
                17 => 29,
                _ => 3,
            },
            58 => match state {
                19 => 30,
                _ => 4,
            },
            59 => match state {
                21 => 32,
                _ => 5,
            },
            60 => match state {
                20 => 31,
                _ => 6,
            },
            61 => 7,
            62 => match state {
                18 => 125,
                _ => 99,
            },
            63 => match state {
                16 => 28,
                _ => 8,
            },
            64 => 74,
            65 => 158,
            66 => match state {
                41 => 164,
                45 => 174,
                46 => 176,
                _ => 159,
            },
            67 => 132,
            68 => match state {
                45 => 175,
                46 => 177,
                _ => 165,
            },
            69 => match state {
                43 => 169,
                _ => 75,
            },
            70 => match state {
                40 => 160,
                _ => 76,
            },
            71 => 77,
            72 => match state {
                40 => 161,
                41 | 45..=46 => 166,
                _ => 78,
            },
            73 => match state {
                22 => 133,
                _ => 79,
            },
            74 => match state {
                41 | 45..=46 => 167,
                _ => 80,
            },
            75 => 81,
            76 => 82,
            77 => 83,
            78 => 148,
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
        r###"".""###,
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
            __action(state, 40 - 1)
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
            Token::Dot if true => Some(8),
            Token::Slash if true => Some(9),
            Token::Colon if true => Some(10),
            Token::Semicolon if true => Some(11),
            Token::Less if true => Some(12),
            Token::LessEqual if true => Some(13),
            Token::Equal if true => Some(14),
            Token::EqualEqual if true => Some(15),
            Token::Greater if true => Some(16),
            Token::GreaterEqual if true => Some(17),
            Token::And if true => Some(18),
            Token::Else if true => Some(19),
            Token::False if true => Some(20),
            Token::Fn if true => Some(21),
            Token::For if true => Some(22),
            Token::If if true => Some(23),
            Token::Let if true => Some(24),
            Token::Nil if true => Some(25),
            Token::Or if true => Some(26),
            Token::Print if true => Some(27),
            Token::PrintLn if true => Some(28),
            Token::Return if true => Some(29),
            Token::Struct if true => Some(30),
            Token::True if true => Some(31),
            Token::While if true => Some(32),
            Token::LeftBrace if true => Some(33),
            Token::RightBrace if true => Some(34),
            Token::Identifier(_) if true => Some(35),
            Token::Number(_) if true => Some(36),
            Token::String(_) if true => Some(37),
            Token::TypeInt if true => Some(38),
            Token::TypeString if true => Some(39),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 38 | 39 => __Symbol::Variant0(__token),
            35 | 37 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            36 => match __token {
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
                    states_to_pop: 4,
                    nonterminal_produced: 21,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
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
                    states_to_pop: 3,
                    nonterminal_produced: 30,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
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
                    states_to_pop: 2,
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
                    nonterminal_produced: 34,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
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
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 36,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
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
                    states_to_pop: 2,
                    nonterminal_produced: 40,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 41,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
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
                    nonterminal_produced: 46,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
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
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 51,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 51,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    nonterminal_produced: 53,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
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
                    nonterminal_produced: 53,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 2,
                    nonterminal_produced: 69,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 69,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 72,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 72,
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
                    states_to_pop: 1,
                    nonterminal_produced: 72,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 73,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 74,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 74,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 74,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 74,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 75,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 76,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 77,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 77,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 78,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 78,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 79,
                }
            }
            139 => __state_machine::SimulatedReduce::Accept,
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
                __reduce136(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            137 => {
                __reduce137(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            138 => {
                __reduce138(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            139 => {
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
        // ("," <ExprS>) = ",", ExprS => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(__sym0, __sym1);
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
        // ("," <ExprS>)* =  => ActionFn(78);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action78::<>(&__start, &__end);
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
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(79);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action79::<>(__sym0);
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
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(130);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action130::<>(__sym0, __sym1);
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
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(131);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action131::<>(__sym0, __sym1, __sym2);
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
        // ("," <identifier>) = ",", identifier => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(__sym0, __sym1);
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
        // ("," <identifier>)* =  => ActionFn(82);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action82::<>(&__start, &__end);
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
        // ("," <identifier>)* = ("," <identifier>)+ => ActionFn(83);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(__sym0);
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
        // ("," <identifier>)+ = ",", identifier => ActionFn(134);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action134::<>(__sym0, __sym1);
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
        // ("," <identifier>)+ = ("," <identifier>)+, ",", identifier => ActionFn(135);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action135::<>(__sym0, __sym1, __sym2);
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
        // (":" <Types>) = ":", Types => ActionFn(112);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action112::<>(__sym0, __sym1);
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
        // (":" <Types>)? = ":", Types => ActionFn(138);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action138::<>(__sym0, __sym1);
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
        // (":" <Types>)? =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(&__start, &__end);
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
        // ("=" <ExprS>) = "=", ExprS => ActionFn(109);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action109::<>(__sym0, __sym1);
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
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action141::<>(__sym0, __sym1);
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
        // ("=" <ExprS>)? =  => ActionFn(108);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action108::<>(&__start, &__end);
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
        // () =  => ActionFn(81);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action81::<>(&__start, &__end);
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
        // @L =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(&__start, &__end);
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
        // @R =  => ActionFn(124);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action124::<>(&__start, &__end);
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
        // Args = ExprS => ActionFn(132);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action132::<>(__sym0);
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
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action133::<>(__sym0, __sym1);
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
        // Args =  => ActionFn(146);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action146::<>(&__start, &__end);
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
        // DeclS* =  => ActionFn(114);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action114::<>(&__start, &__end);
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
        // DeclS* = DeclS+ => ActionFn(115);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action115::<>(__sym0);
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
        // DeclS+ = DeclS => ActionFn(116);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action116::<>(__sym0);
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
        // DeclS+ = DeclS+, DeclS => ActionFn(117);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action117::<>(__sym0, __sym1);
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
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(142);
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
        let __nt = super::__action142::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
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
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(143);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action143::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
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
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(144);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action144::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
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
        // DeclVar = "let", identifier, ";" => ActionFn(145);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action145::<>(__sym0, __sym1, __sym2);
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
        // ExprCall = Spanned<ExprCall>, "(", Args, ")" => ActionFn(63);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action63::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (4, 21)
    }
    pub(crate) fn __reduce38<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprCall = ExprPrimary => ActionFn(64);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce39<
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
        (1, 22)
    }
    pub(crate) fn __reduce40<
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
        (1, 23)
    }
    pub(crate) fn __reduce41<
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
        (1, 24)
    }
    pub(crate) fn __reduce42<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action91::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(92);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action92::<>(__sym0);
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
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(93);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action93::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(94);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action94::<>(__sym0);
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
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(87);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action87::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(88);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action88::<>(__sym0);
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
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(95);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action95::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(96);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action96::<>(__sym0);
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
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(97);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action97::<>(__sym0, __sym1, __sym2);
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
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(98);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(__sym0);
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
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(89);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action89::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 30)
    }
    pub(crate) fn __reduce53<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(90);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce54<
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
        (1, 31)
    }
    pub(crate) fn __reduce55<
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
        (1, 32)
    }
    pub(crate) fn __reduce56<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(61);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action61::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce57<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprCall => ActionFn(62);
        let __sym0 = __pop_Variant15(__symbols);
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
        // ExprPrimary = "nil" => ActionFn(65);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce59<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce60<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(67);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action67::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce61<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(68);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action68::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce62<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(69);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action69::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce63<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(70);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce64<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(71);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant15(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce65<
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
        (1, 35)
    }
    pub(crate) fn __reduce66<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce67<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action101::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 36)
    }
    pub(crate) fn __reduce68<
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
        (1, 37)
    }
    pub(crate) fn __reduce69<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce70<
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
        (1, 39)
    }
    pub(crate) fn __reduce71<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(182);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action182::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 40)
    }
    pub(crate) fn __reduce72<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(183);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action183::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce73<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(184);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action184::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(185);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action185::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 41)
    }
    pub(crate) fn __reduce75<
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
        (1, 42)
    }
    pub(crate) fn __reduce76<
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
        (1, 42)
    }
    pub(crate) fn __reduce77<
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
        (1, 42)
    }
    pub(crate) fn __reduce78<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(72);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant22(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action72::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce79<
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
        (1, 44)
    }
    pub(crate) fn __reduce80<
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
        (1, 44)
    }
    pub(crate) fn __reduce81<
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
        (1, 44)
    }
    pub(crate) fn __reduce82<
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
        (1, 44)
    }
    pub(crate) fn __reduce83<
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
        (1, 45)
    }
    pub(crate) fn __reduce84<
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
        (1, 45)
    }
    pub(crate) fn __reduce85<
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
        (1, 46)
    }
    pub(crate) fn __reduce86<
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
        (1, 46)
    }
    pub(crate) fn __reduce87<
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
        (1, 47)
    }
    pub(crate) fn __reduce88<
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
        (1, 48)
    }
    pub(crate) fn __reduce89<
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
        (1, 49)
    }
    pub(crate) fn __reduce90<
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
        (1, 49)
    }
    pub(crate) fn __reduce91<
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
        (1, 50)
    }
    pub(crate) fn __reduce92<
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
        (1, 50)
    }
    pub(crate) fn __reduce93<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier => ActionFn(136);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action136::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params = identifier, ("," <identifier>)+ => ActionFn(137);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action137::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (2, 51)
    }
    pub(crate) fn __reduce95<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(147);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action147::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (0, 51)
    }
    pub(crate) fn __reduce96<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(178);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action178::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 52)
    }
    pub(crate) fn __reduce97<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(179);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action179::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
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
        (1, 53)
    }
    pub(crate) fn __reduce99<
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
        (1, 53)
    }
    pub(crate) fn __reduce100<
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
        (1, 53)
    }
    pub(crate) fn __reduce101<
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
        (1, 53)
    }
    pub(crate) fn __reduce102<
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
        (1, 53)
    }
    pub(crate) fn __reduce103<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(163);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce104<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(164);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action164::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce105<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprCall> = ExprCall => ActionFn(165);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce106<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(166);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action166::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce107<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(167);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce108<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(168);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action168::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce109<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(169);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action169::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce110<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(170);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action170::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce111<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(171);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action171::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce112<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(172);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action172::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce113<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(173);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action173::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce114<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(174);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce115<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(175);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action175::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce116<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(176);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action176::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce117<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(177);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action177::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce118<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(180);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action180::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (2, 69)
    }
    pub(crate) fn __reduce119<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(181);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action181::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (3, 69)
    }
    pub(crate) fn __reduce120<
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
        (1, 70)
    }
    pub(crate) fn __reduce121<
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
        (1, 70)
    }
    pub(crate) fn __reduce122<
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
        (1, 71)
    }
    pub(crate) fn __reduce123<
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
        (7, 72)
    }
    pub(crate) fn __reduce124<
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
        (5, 72)
    }
    pub(crate) fn __reduce125<
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
        (7, 72)
    }
    pub(crate) fn __reduce126<
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
        (1, 72)
    }
    pub(crate) fn __reduce127<
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
        (2, 73)
    }
    pub(crate) fn __reduce128<
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
        (7, 74)
    }
    pub(crate) fn __reduce129<
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
        (5, 74)
    }
    pub(crate) fn __reduce130<
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
        (7, 74)
    }
    pub(crate) fn __reduce131<
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
        (5, 74)
    }
    pub(crate) fn __reduce132<
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
        (3, 75)
    }
    pub(crate) fn __reduce133<
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
        (3, 76)
    }
    pub(crate) fn __reduce134<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(186);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action186::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 77)
    }
    pub(crate) fn __reduce135<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(187);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action187::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 77)
    }
    pub(crate) fn __reduce136<
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
        (1, 78)
    }
    pub(crate) fn __reduce137<
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
        (1, 78)
    }
    pub(crate) fn __reduce138<
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
        (1, 79)
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
    (_, op, _): (usize, ast::OpPrefix, usize),
    (_, rt, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Prefix(Box::new(ast::ExprPrefix { op, rt }))
}

#[allow(clippy::too_many_arguments)]
fn __action62<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action63<
>(
    (_, callee, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<ast::ExprS>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Call(Box::new(ast::ExprCall { callee, args }))
}

#[allow(clippy::too_many_arguments)]
fn __action64<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action65<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Nil)
}

#[allow(clippy::too_many_arguments)]
fn __action66<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(false))
}

#[allow(clippy::too_many_arguments)]
fn __action67<
>(
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(true))
}

#[allow(clippy::too_many_arguments)]
fn __action68<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::String(__0))
}

#[allow(clippy::too_many_arguments)]
fn __action69<
>(
    (_, __0, _): (usize, f64, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Number(__0))
}

#[allow(clippy::too_many_arguments)]
fn __action70<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action71<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action72<
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
fn __action73<
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
fn __action74<
>(
    (_, __0, _): (usize, (), usize),
) -> Vec<String>
{
    Vec::new()
}

#[allow(clippy::too_many_arguments)]
fn __action75<
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
fn __action76<
>(
    (_, __0, _): (usize, (), usize),
) -> Vec<ast::ExprS>
{
    Vec::new()
}

#[allow(clippy::too_many_arguments)]
fn __action77<
>(
    (_, name, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Var(ast::ExprVar { var: ast::Var {name, type_: None} })
}

#[allow(clippy::too_many_arguments)]
fn __action78<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action79<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action80<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action81<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
)
{
}

#[allow(clippy::too_many_arguments)]
fn __action82<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action83<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action84<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action85<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action86<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action87<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action88<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action89<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action90<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action91<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action92<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action93<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action94<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action95<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action96<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action97<
>(
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(clippy::too_many_arguments)]
fn __action98<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action99<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action100<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action101<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
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
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action105<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action106<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action107<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action108<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action109<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action110<
>(
    (_, __0, _): (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments)]
fn __action111<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Type>
{
    None
}

#[allow(clippy::too_many_arguments)]
fn __action112<
>(
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    __0
}

#[allow(clippy::too_many_arguments)]
fn __action113<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action114<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn __action115<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    v
}

#[allow(clippy::too_many_arguments)]
fn __action116<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action117<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    { let mut v = v; v.push(e); v }
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
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action122<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(clippy::too_many_arguments)]
fn __action123<
>(
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

fn __action124<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookbehind
}

fn __action125<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookahead
}

#[allow(clippy::too_many_arguments)]
fn __action126<
>(
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action127<
>(
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action128<
>(
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments)]
fn __action129<
>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments)]
fn __action130<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action80(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action128(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action131<
>(
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action80(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action129(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action132<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action78(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action133<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action79(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action134<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action84(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action126(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action135<
>(
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, Token, usize),
    __2: (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action84(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action127(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action136<
>(
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action82(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action137<
>(
    __0: (usize, String, usize),
    __1: (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action83(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action138<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action112(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
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
    __4: (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    __5: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action138(
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
fn __action140<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    __3: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action111(
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
fn __action141<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
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
fn __action142<
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
    let __temp0 = __action141(
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action139(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __6,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action143<
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
    let __temp0 = __action108(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action139(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action144<
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
    let __temp0 = __action141(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action140(
        __0,
        __1,
        __temp0,
        __4,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action145<
>(
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action108(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action140(
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action146<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::ExprS>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action81(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action147<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action81(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action148<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action149<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
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
fn __action150<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
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
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
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
    let __temp0 = __action125(
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
fn __action153<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action123(
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
    let __temp0 = __action125(
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
fn __action155<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
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
fn __action156<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action157<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action158<
>(
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
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
fn __action159<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action105(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action160<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action161<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
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
fn __action162<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action125(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
        __temp0,
        __0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action163<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action164<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action165<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action166<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action167<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action168<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action169<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action170<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action171<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action172<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action173<
>(
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
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
fn __action174<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action159(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action175<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action160(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action176<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action161(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action177<
>(
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action124(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action162(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action178<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ast::Program
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action114(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action179<
>(
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> ast::Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action115(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action180<
>(
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action114(
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
fn __action181<
>(
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    __2: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action115(
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
fn __action182<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action100(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action183<
>(
    __0: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action101(
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
fn __action184<
>(
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action100(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action185<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action101(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments)]
fn __action186<
>(
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action100(
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
fn __action187<
>(
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action101(
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
