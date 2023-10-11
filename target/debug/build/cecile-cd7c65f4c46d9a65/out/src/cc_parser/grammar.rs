// auto-generated: "lalrpop 0.20.0"
// sha3: adcc1f60ca6c9fca6c7cc358bcf7383917632ae8f4f795b7517bf80da336ceda
use crate::cc_lexer;
use crate::vm::error::ErrorS;
use crate::cc_lexer::Token;
use crate::cc_parser::ast::Expression;
use crate::cc_parser::ast;
use hashbrown::HashMap;
use crate::cc_parser::ast::Fn;
use lalrpop_util::ParseError;
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
    use crate::vm::error::ErrorS;
    use crate::cc_lexer::Token;
    use crate::cc_parser::ast::Expression;
    use crate::cc_parser::ast;
    use hashbrown::HashMap;
    use crate::cc_parser::ast::Fn;
    use lalrpop_util::ParseError;
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
        Variant3(core::option::Option<Token>),
        Variant4(ast::Spanned<ast::Expression>),
        Variant5(alloc::vec::Vec<ast::Spanned<ast::Expression>>),
        Variant6((String, Option<ast::Type>)),
        Variant7(alloc::vec::Vec<(String, Option<ast::Type>)>),
        Variant8(ast::Type),
        Variant9(core::option::Option<ast::Type>),
        Variant10(core::option::Option<ast::Spanned<ast::Expression>>),
        Variant11(()),
        Variant12(ast::Field),
        Variant13(alloc::vec::Vec<ast::Field>),
        Variant14(usize),
        Variant15(Vec<ast::ExprS>),
        Variant16(ast::Statement),
        Variant17(ast::Spanned<ast::Statement>),
        Variant18(alloc::vec::Vec<ast::Spanned<ast::Statement>>),
        Variant19(ast::Expression),
        Variant20(Option<ast::StatementS>),
        Variant21(ast::StatementFun),
        Variant22(ast::OpInfix),
        Variant23(ast::OpPrefix),
        Variant24(HashMap<String, Option<ast::Type>>),
        Variant25(ast::Program),
        Variant26(ast::StatementBlock),
        Variant27(Vec<ast::Field>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 42 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        -161,
        // State 2
        -105,
        // State 3
        -104,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            56 => 1,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!""###,
        r###""!=""###,
        r###""%""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""->""###,
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
        'err,
    >(
        __states: &[i16],
        _: core::marker::PhantomData<(&'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'err>
    where 
    {
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __phantom: core::marker::PhantomData<(&'err ())>,
    }
    impl<'err> __state_machine::ParserDefinition for __StateMachine<'err>
    where 
    {
        type Location = usize;
        type Error = ErrorS;
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
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 42 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
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
                self.errors,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'err,
    >(
        __token: &Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Bang if true => Some(0),
            Token::BangEqual if true => Some(1),
            Token::Percent if true => Some(2),
            Token::LeftParen if true => Some(3),
            Token::RightParen if true => Some(4),
            Token::Star if true => Some(5),
            Token::Plus if true => Some(6),
            Token::Comma if true => Some(7),
            Token::Minus if true => Some(8),
            Token::Arrow if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Slash if true => Some(11),
            Token::Colon if true => Some(12),
            Token::Semicolon if true => Some(13),
            Token::Less if true => Some(14),
            Token::LessEqual if true => Some(15),
            Token::Equal if true => Some(16),
            Token::EqualEqual if true => Some(17),
            Token::Greater if true => Some(18),
            Token::GreaterEqual if true => Some(19),
            Token::And if true => Some(20),
            Token::Else if true => Some(21),
            Token::False if true => Some(22),
            Token::Fn if true => Some(23),
            Token::For if true => Some(24),
            Token::If if true => Some(25),
            Token::Let if true => Some(26),
            Token::Nil if true => Some(27),
            Token::Or if true => Some(28),
            Token::Print if true => Some(29),
            Token::PrintLn if true => Some(30),
            Token::Return if true => Some(31),
            Token::Struct if true => Some(32),
            Token::True if true => Some(33),
            Token::While if true => Some(34),
            Token::LeftBrace if true => Some(35),
            Token::RightBrace if true => Some(36),
            Token::Identifier(_) if true => Some(37),
            Token::Number(_) if true => Some(38),
            Token::String(_) if true => Some(39),
            Token::TypeInt if true => Some(40),
            Token::TypeString if true => Some(41),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'err,
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 40 | 41 => __Symbol::Variant0(__token),
            37 | 39 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            38 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'err,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'err>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
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
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 19,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 23,
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
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 26,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
                    nonterminal_produced: 35,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 37,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 40,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
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
                    states_to_pop: 3,
                    nonterminal_produced: 41,
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
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 43,
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
                    nonterminal_produced: 45,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 47,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 48,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
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
                    states_to_pop: 7,
                    nonterminal_produced: 50,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 50,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
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
                    states_to_pop: 1,
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
                    nonterminal_produced: 54,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
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
                    nonterminal_produced: 57,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 58,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 59,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 59,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 60,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 61,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 61,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 72,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 73,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 74,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 75,
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
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 78,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 78,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 79,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 79,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 80,
                }
            }
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 81,
                }
            }
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 81,
                }
            }
            143 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 81,
                }
            }
            144 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 81,
                }
            }
            145 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 82,
                }
            }
            146 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 83,
                }
            }
            147 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 83,
                }
            }
            148 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 83,
                }
            }
            149 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 83,
                }
            }
            150 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 84,
                }
            }
            151 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 85,
                }
            }
            152 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 86,
                }
            }
            153 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 86,
                }
            }
            154 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 87,
                }
            }
            155 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 87,
                }
            }
            156 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 87,
                }
            }
            157 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            158 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            159 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            160 => __state_machine::SimulatedReduce::Accept,
            161 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 90,
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
            'err,
            __TOKEN: __ToTriple<'err, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
            __tokens0: __TOKENS,
        ) -> Result<ast::OpPrefix, __lalrpop_util::ParseError<usize, Token, ErrorS>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'err,
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'err ())>,
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
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
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
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<Result<ast::OpPrefix,__lalrpop_util::ParseError<usize, Token, ErrorS>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                __reduce94(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            95 => {
                __reduce95(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            96 => {
                __reduce96(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            97 => {
                __reduce97(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            98 => {
                __reduce98(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            99 => {
                __reduce99(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            100 => {
                __reduce100(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            101 => {
                __reduce101(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            102 => {
                __reduce102(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            103 => {
                __reduce103(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            104 => {
                __reduce104(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            105 => {
                __reduce105(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            106 => {
                __reduce106(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            107 => {
                __reduce107(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            108 => {
                __reduce108(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            109 => {
                __reduce109(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            110 => {
                __reduce110(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            111 => {
                __reduce111(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            112 => {
                __reduce112(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            113 => {
                __reduce113(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            114 => {
                __reduce114(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            115 => {
                __reduce115(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            116 => {
                __reduce116(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            117 => {
                __reduce117(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            118 => {
                __reduce118(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            119 => {
                __reduce119(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            120 => {
                __reduce120(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            121 => {
                __reduce121(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            122 => {
                __reduce122(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            123 => {
                __reduce123(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            124 => {
                __reduce124(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            125 => {
                __reduce125(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            126 => {
                __reduce126(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            127 => {
                __reduce127(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            128 => {
                __reduce128(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            129 => {
                __reduce129(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            130 => {
                __reduce130(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            131 => {
                __reduce131(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            132 => {
                __reduce132(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            133 => {
                __reduce133(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            134 => {
                __reduce134(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            135 => {
                __reduce135(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            136 => {
                __reduce136(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            137 => {
                __reduce137(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            138 => {
                __reduce138(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            139 => {
                __reduce139(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            140 => {
                __reduce140(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            141 => {
                __reduce141(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            142 => {
                __reduce142(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            143 => {
                __reduce143(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            144 => {
                __reduce144(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            145 => {
                __reduce145(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            146 => {
                __reduce146(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            147 => {
                __reduce147(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            148 => {
                __reduce148(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            149 => {
                __reduce149(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            150 => {
                __reduce150(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            151 => {
                __reduce151(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            152 => {
                __reduce152(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            153 => {
                __reduce153(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            154 => {
                __reduce154(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            155 => {
                __reduce155(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            156 => {
                __reduce156(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            157 => {
                __reduce157(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            158 => {
                __reduce158(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            159 => {
                __reduce159(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            160 => {
                // __OpPrefix = OpPrefix => ActionFn(1);
                let __sym0 = __pop_Variant23(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(errors, __sym0);
                return Some(Ok(__nt));
            }
            161 => {
                __reduce161(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
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
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (String, Option<ast::Type>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, HashMap<String, Option<ast::Type>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<ast::StatementS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::ExprS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Field, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpInfix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpPrefix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementBlock, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementFun, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(123);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action123::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(124);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action124::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>) = ",", ExprS => ActionFn(88);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action88::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* =  => ActionFn(86);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action86::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(87);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action87::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(150);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action150::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(151);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action151::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>) = ",", Param => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action91::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)* =  => ActionFn(89);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)* = ("," <Param>)+ => ActionFn(90);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)+ = ",", Param => ActionFn(154);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action154::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)+ = ("," <Param>)+, ",", Param => ActionFn(155);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action155::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>) = "->", Types => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action94::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>)? = "->", Types => ActionFn(158);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action158::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>)? =  => ActionFn(93);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action93::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>) = ":", Types => ActionFn(122);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action122::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? = ":", Types => ActionFn(161);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action161::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? =  => ActionFn(121);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action121::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce18<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>) = "=", ExprS => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action119::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(166);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action166::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? =  => ActionFn(118);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action118::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce21<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>) = SingleField => ActionFn(128);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action128::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)* =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action126::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce24<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)* = (<SingleField>)+ => ActionFn(127);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action127::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce25<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)+ = SingleField => ActionFn(174);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce26<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)+ = (<SingleField>)+, SingleField => ActionFn(175);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action175::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce27<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(143);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action143::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce28<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(142);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action142::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce29<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args = ExprS => ActionFn(152);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action152::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce30<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action153::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce31<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args =  => ActionFn(171);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action171::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce32<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = Stmt => ActionFn(4);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce33<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclVar => ActionFn(5);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce34<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclFun => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclStruct => ActionFn(7);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce36<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclFun = "fn", Function => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action12::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce37<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS = Spanned<Decl> => ActionFn(3);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS* =  => ActionFn(130);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action130::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce39<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS* = DeclS+ => ActionFn(131);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action131::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS => ActionFn(132);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action132::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce41<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS+, DeclS => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action133::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce42<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclStruct = "struct", identifier, StructFields, ";" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant27(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 25)
    }
    pub(crate) fn __reduce43<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(167);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action167::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 26)
    }
    pub(crate) fn __reduce44<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(168);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action168::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 26)
    }
    pub(crate) fn __reduce45<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(169);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action169::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 26)
    }
    pub(crate) fn __reduce46<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ";" => ActionFn(170);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action170::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce47<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprAssign = identifier, "=", ExprS => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce48<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprAssign = ExprLogicOr => ActionFn(43);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce49<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprCall = Spanned<ExprCall>, "(", Args, ")" => ActionFn(67);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant15(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action67::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 28)
    }
    pub(crate) fn __reduce50<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprCall = ExprPrimary => ActionFn(68);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action68::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce51<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprComparison = ExprInfix<ExprComparison, OpComparison, ExprTerm> => ActionFn(51);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce52<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprEquality = ExprInfix<ExprEquality, OpEquality, ExprComparison> => ActionFn(48);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce53<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprFactor = ExprInfix<ExprFactor, OpFactor, ExprPrefix> => ActionFn(59);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce54<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(101);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action101::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce55<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(102);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action102::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce56<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(103);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action103::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 33)
    }
    pub(crate) fn __reduce57<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(104);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action104::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce58<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(97);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action97::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce59<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(98);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce60<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(105);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action105::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 35)
    }
    pub(crate) fn __reduce61<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(106);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action106::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce62<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action107::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 36)
    }
    pub(crate) fn __reduce63<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(108);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action108::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce64<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(99);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action99::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 37)
    }
    pub(crate) fn __reduce65<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(100);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprLogicAnd = ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> => ActionFn(46);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce67<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprLogicOr = ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce68<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action65::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 40)
    }
    pub(crate) fn __reduce69<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprCall => ActionFn(66);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce70<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "nil" => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action69::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce71<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(70);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce72<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(71);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action71::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(72);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action72::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(73);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(74);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action74::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce76<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(75);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action75::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 41)
    }
    pub(crate) fn __reduce77<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS = Spanned<Expression> => ActionFn(40);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce78<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(110);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action110::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce79<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 43)
    }
    pub(crate) fn __reduce80<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprTerm = ExprInfix<ExprTerm, OpTerm, ExprFactor> => ActionFn(56);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce81<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(85);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce82<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Expression = ExprAssign => ActionFn(41);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce83<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(212);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action212::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 47)
    }
    pub(crate) fn __reduce84<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(213);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action213::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce85<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(214);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action214::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce86<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(215);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action215::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 48)
    }
    pub(crate) fn __reduce87<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<DeclVar> => ActionFn(24);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce88<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<StmtExpr> => ActionFn(25);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce89<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = ";" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce90<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", "->", Types, StatementBlockIn => ActionFn(159);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant26(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action159::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (7, 50)
    }
    pub(crate) fn __reduce91<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(160);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant26(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action160::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (5, 50)
    }
    pub(crate) fn __reduce92<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = ">" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action52::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce93<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = ">=" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = "<" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce95<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = "<=" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpEquality = "==" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce97<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpEquality = "!=" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "*" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce99<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "/" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce100<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "%" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce101<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpLogicAnd = "and" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce102<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpLogicOr = "or" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce103<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpPrefix = "-" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action63::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce104<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpPrefix = "!" => ActionFn(64);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce105<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpTerm = "+" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce106<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpTerm = "-" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce107<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Param = identifier, ":", Types => ActionFn(164);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action164::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 58)
    }
    pub(crate) fn __reduce108<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Param = identifier => ActionFn(165);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce109<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params = Param => ActionFn(156);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action156::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce110<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params = Param, ("," <Param>)+ => ActionFn(157);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action157::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce111<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(172);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action172::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 59)
    }
    pub(crate) fn __reduce112<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(208);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action208::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 60)
    }
    pub(crate) fn __reduce113<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(209);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action209::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce114<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SingleField = identifier, ":", Types, "," => ActionFn(148);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action148::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 61)
    }
    pub(crate) fn __reduce115<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SingleField = identifier, ":", Types => ActionFn(149);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action149::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 61)
    }
    pub(crate) fn __reduce116<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtExpr => ActionFn(29);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce117<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtBlock => ActionFn(30);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce118<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrint => ActionFn(31);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce119<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrintLn => ActionFn(32);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce120<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtReturn => ActionFn(33);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce121<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(193);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action193::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce122<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(194);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action194::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce123<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprCall> = ExprCall => ActionFn(195);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action195::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce124<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(196);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action196::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce125<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(197);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action197::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce126<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(198);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action198::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce127<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(199);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action199::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 69)
    }
    pub(crate) fn __reduce128<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(200);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action200::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 70)
    }
    pub(crate) fn __reduce129<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(201);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action201::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 71)
    }
    pub(crate) fn __reduce130<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(202);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action202::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 72)
    }
    pub(crate) fn __reduce131<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(203);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action203::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 73)
    }
    pub(crate) fn __reduce132<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(204);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action204::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 74)
    }
    pub(crate) fn __reduce133<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(205);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action205::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 75)
    }
    pub(crate) fn __reduce134<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(206);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action206::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce135<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(207);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action207::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 77)
    }
    pub(crate) fn __reduce136<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action210::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 78)
    }
    pub(crate) fn __reduce137<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(211);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action211::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (3, 78)
    }
    pub(crate) fn __reduce138<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = StmtOpen => ActionFn(14);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 79)
    }
    pub(crate) fn __reduce139<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = StmtClosed => ActionFn(15);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 79)
    }
    pub(crate) fn __reduce140<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtBlock = StatementBlockIn => ActionFn(37);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 80)
    }
    pub(crate) fn __reduce141<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtClosed> => ActionFn(20);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action20::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 81)
    }
    pub(crate) fn __reduce142<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "while", "(", ExprS, ")", Spanned<StmtClosed> => ActionFn(21);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action21::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 81)
    }
    pub(crate) fn __reduce143<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtClosed> => ActionFn(22);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action22::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 81)
    }
    pub(crate) fn __reduce144<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = SmallStmt => ActionFn(23);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 81)
    }
    pub(crate) fn __reduce145<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtExpr = ExprS, ";" => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action39::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 82)
    }
    pub(crate) fn __reduce146<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtOpen> => ActionFn(16);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action16::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 83)
    }
    pub(crate) fn __reduce147<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<Stmt> => ActionFn(17);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action17::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 83)
    }
    pub(crate) fn __reduce148<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtOpen> => ActionFn(18);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action18::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 83)
    }
    pub(crate) fn __reduce149<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "while", "(", ExprS, ")", Spanned<StmtOpen> => ActionFn(19);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action19::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 83)
    }
    pub(crate) fn __reduce150<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtPrint = "print", ExprS, ";" => ActionFn(36);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action36::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 84)
    }
    pub(crate) fn __reduce151<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtPrintLn = "println", ExprS, ";" => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 85)
    }
    pub(crate) fn __reduce152<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(216);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action216::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 86)
    }
    pub(crate) fn __reduce153<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(217);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action217::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 86)
    }
    pub(crate) fn __reduce154<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields = "{", "}" => ActionFn(176);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action176::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (2, 87)
    }
    pub(crate) fn __reduce155<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields = "{", (<SingleField>)+, "}" => ActionFn(177);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action177::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (3, 87)
    }
    pub(crate) fn __reduce156<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields =  => ActionFn(173);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action173::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 87)
    }
    pub(crate) fn __reduce157<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = type_string => ActionFn(82);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action82::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce158<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = type_int => ActionFn(83);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce159<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = "fn" => ActionFn(84);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action84::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce161<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 90)
    }
}
pub use self::__parse__OpPrefix::OpPrefixParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__Program {

    use crate::cc_lexer;
    use crate::vm::error::ErrorS;
    use crate::cc_lexer::Token;
    use crate::cc_parser::ast::Expression;
    use crate::cc_parser::ast;
    use hashbrown::HashMap;
    use crate::cc_parser::ast::Fn;
    use lalrpop_util::ParseError;
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
        Variant3(core::option::Option<Token>),
        Variant4(ast::Spanned<ast::Expression>),
        Variant5(alloc::vec::Vec<ast::Spanned<ast::Expression>>),
        Variant6((String, Option<ast::Type>)),
        Variant7(alloc::vec::Vec<(String, Option<ast::Type>)>),
        Variant8(ast::Type),
        Variant9(core::option::Option<ast::Type>),
        Variant10(core::option::Option<ast::Spanned<ast::Expression>>),
        Variant11(()),
        Variant12(ast::Field),
        Variant13(alloc::vec::Vec<ast::Field>),
        Variant14(usize),
        Variant15(Vec<ast::ExprS>),
        Variant16(ast::Statement),
        Variant17(ast::Spanned<ast::Statement>),
        Variant18(alloc::vec::Vec<ast::Spanned<ast::Statement>>),
        Variant19(ast::Expression),
        Variant20(Option<ast::StatementS>),
        Variant21(ast::StatementFun),
        Variant22(ast::OpInfix),
        Variant23(ast::OpPrefix),
        Variant24(HashMap<String, Option<ast::Type>>),
        Variant25(ast::Program),
        Variant26(ast::StatementBlock),
        Variant27(Vec<ast::Field>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 11, 98, 99, 100, 101, 0, 12, 13, 14, 102, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 1
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 11, 98, 99, 100, 101, 0, 12, 13, 14, 102, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 2
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 113, 114, 0, 0, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 119, 0, 0, 120, 0, 0, 0, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 124, 0, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0,
        // State 11
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 12
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 13
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 133, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 14
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 11, 98, 99, 100, 101, 0, 12, 13, 14, 102, 103, 104, 15, 134, 105, 106, 107, 0, 0,
        // State 15
        95, 0, 0, 10, -32, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 16
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 17
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 18
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 19
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 20
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 21
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 112, 106, 107, 0, 0,
        // State 22
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 147, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 100, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 23
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 25
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 26
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 11, 98, 99, 100, 101, 0, 12, 13, 14, 102, 103, 104, 15, 155, 105, 106, 107, 0, 0,
        // State 27
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 28
        0, 0, 0, 0, -30, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -55, 0, 0, -55, 0, 124, -55, 125, 0, 0, 0, 0, -55, -55, -55, 0, -55, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -57, 0, 0, -57, 0, 0, -57, 0, 0, 0, 0, 0, -57, 113, 114, 0, -57, 115, 116, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 117, 0, 0, -61, 0, 0, -61, 0, 0, 0, 0, 0, -61, 0, 0, 0, 118, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, -63, 0, 0, -63, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 122, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -65, 119, 0, -65, 120, -65, -65, -65, 0, 0, 121, 0, -65, -65, -65, 0, -65, -65, -65, -65, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, -112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0,
        // State 35
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 162, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 166,
        // State 37
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 171, 0, 0, 0, 0,
        // State 39
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 40
        0, 0, 0, 0, -110, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        95, 0, 0, 10, -87, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 42
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 98, 99, 0, 101, 0, 12, 13, 14, 0, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 171, 0, 0, 0, 0,
        // State 44
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 98, 99, 0, 101, 0, 12, 13, 14, 0, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 45
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 166,
        // State 49
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 0, 0, 0, 101, 0, 0, 0, 0, 0, 103, 0, 0, 0, 105, 106, 107, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 166,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 166,
        // State 53
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 98, 99, 0, 101, 0, 12, 13, 14, 0, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 54
        95, 0, 0, 10, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 0, 98, 99, 0, 101, 0, 12, 13, 14, 0, 103, 104, 15, 0, 105, 106, 107, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 56
        -122, 0, 0, -122, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, -122, -122, -122, -122, -122, 0, -122, -122, -122, -122, -122, -122, -122, -122, -122, -122, -122, 0, 0,
        // State 57
        -35, 0, 0, -35, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, 0,
        // State 58
        -41, 0, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0,
        // State 59
        -36, 0, 0, -36, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, 0,
        // State 60
        -34, 0, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, 0,
        // State 61
        0, 0, 0, 0, -83, 0, 0, -83, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -70, -70, -124, -70, -70, -70, -70, -70, 0, 0, -70, 0, -70, -70, -70, 0, -70, -70, -70, -70, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -58, 0, 0, -58, 0, 0, -58, 0, 0, 0, 0, 0, -58, -125, -125, 0, -58, -125, -125, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, -126, 0, 0, -62, 0, 0, -62, 0, 0, 0, 0, 0, -62, 0, 0, 0, -126, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -66, -127, 0, -66, -127, -66, -66, -66, 0, 0, -127, 0, -66, -66, -66, 0, -66, -66, -66, -66, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, -52, 0, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, -52, -52, 0, -52, -52, -52, -52, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -53, 0, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, -53, 0, 0, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, -54, -54, 0, -54, -54, -54, -54, -54, 0, 0, -54, 0, -54, -54, -54, 0, -54, -54, -54, -54, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, -67, 0, 0, -67, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, -68, 0, 0, -68, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, -81, 0, 0, -81, 0, -81, -81, -81, 0, 0, 0, 0, -81, -81, -81, 0, -81, -81, -81, -81, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, -64, 0, 0, -64, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, -49, 0, 0, -49, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, -60, -60, 0, -60, -60, -60, -60, -60, 0, 0, -60, 0, -60, -60, -60, 0, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, -51, -51, -51, -51, -51, -51, -51, -51, 0, 0, -51, 0, -51, -51, -51, 0, -51, -51, -51, -51, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, -56, 0, 0, -56, 0, -131, -56, -131, 0, 0, 0, 0, -56, -56, -56, 0, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, -76, -76, -76, -76, -76, -76, -76, -76, 0, 0, -76, 0, -76, -76, -76, 0, -76, -76, -76, -76, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, -132, 0, 0, -132, 0, 0, 0, 0, 0, -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -145, 0, 0, -145, 0, 0, 0, 0, -145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -145, -145, -145, -145, -145, -145, -145, 0, -145, -145, -145, -145, -145, -145, -145, -145, -145, -145, -145, 0, 0,
        // State 82
        -38, 0, 0, -38, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, 0, 0,
        // State 83
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, -78, 0, 0, -78, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        -141, 0, 0, -141, 0, 0, 0, 0, -141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -141, -141, -141, -141, -141, -141, -141, 0, -141, -141, -141, -141, -141, -141, -141, -141, -141, -141, -141, 0, 0,
        // State 86
        -33, 0, 0, -33, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, 0, 0,
        // State 87
        -118, 0, 0, -118, 0, 0, 0, 0, -118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -118, -118, -118, -118, -118, -118, -118, 0, -118, -118, -118, -118, -118, -118, -118, -118, -118, -118, -118, 0, 0,
        // State 88
        -140, 0, 0, -140, 0, 0, 0, 0, -140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -140, -140, -140, -140, -140, -140, 0, -140, -140, -140, -140, -140, -140, -140, -140, -140, -140, -140, 0, 0,
        // State 89
        -117, 0, 0, -117, 0, 0, 0, 0, -117, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -117, -117, -117, -117, -117, -117, -117, 0, -117, -117, -117, -117, -117, -117, -117, -117, -117, -117, -117, 0, 0,
        // State 90
        -139, 0, 0, -139, 0, 0, 0, 0, -139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -139, -139, -139, -139, -139, -139, 0, -139, -139, -139, -139, -139, -139, -139, -139, -139, -139, -139, 0, 0,
        // State 91
        -119, 0, 0, -119, 0, 0, 0, 0, -119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -119, -119, -119, -119, -119, -119, -119, 0, -119, -119, -119, -119, -119, -119, -119, -119, -119, -119, -119, 0, 0,
        // State 92
        -120, 0, 0, -120, 0, 0, 0, 0, -120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -120, -120, -120, -120, -120, -120, -120, 0, -120, -120, -120, -120, -120, -120, -120, -120, -120, -120, -120, 0, 0,
        // State 93
        -121, 0, 0, -121, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, -121, -121, -121, -121, -121, -121, 0, -121, -121, -121, -121, -121, -121, -121, -121, -121, -121, -121, 0, 0,
        // State 94
        -105, 0, 0, -105, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -105, 0, 0, 0, 0, -105, 0, 0, 0, 0, 0, -105, 0, 0, 0, -105, -105, -105, 0, 0,
        // State 95
        -104, 0, 0, -104, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -104, 0, 0, 0, 0, -104, 0, 0, 0, 0, 0, -104, 0, 0, 0, -104, -104, -104, 0, 0,
        // State 96
        0, -72, -72, -72, -72, -72, -72, -72, -72, 0, 0, -72, 0, -72, -72, -72, 0, -72, -72, -72, -72, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 129, 0, 0, 0, 0,
        // State 100
        0, -71, -71, -71, -71, -71, -71, -71, -71, 0, 0, -71, 0, -71, -71, -71, 0, -71, -71, -71, -71, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0,
        // State 102
        0, -73, -73, -73, -73, -73, -73, -73, -73, 0, 0, -73, 0, -73, -73, -73, 0, -73, -73, -73, -73, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, -82, -82, -82, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 28, -82, -82, -82, -82, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        0, -75, -75, -75, -75, -75, -75, -75, -75, 0, 0, -75, 0, -75, -75, -75, 0, -75, -75, -75, -75, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, -74, -74, -74, -74, -74, -74, -74, -74, 0, 0, -74, 0, -74, -74, -74, 0, -74, -74, -74, -74, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        -42, 0, 0, -42, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42, -42, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0,
        // State 108
        -146, 0, 0, -146, 0, 0, 0, 0, -146, 0, 0, 0, 0, -146, 0, 0, 0, 0, 0, 0, 0, -146, -146, -146, -146, -146, -146, -146, 0, -146, -146, -146, -146, -146, -146, -146, -146, -146, -146, -146, 0, 0,
        // State 109
        0, -130, -130, 0, -130, -130, -130, -130, -130, 0, 0, -130, 0, -130, -130, -130, 0, -130, -130, -130, -130, 0, 0, 0, 0, 0, 0, 0, -130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, -69, -69, 0, -69, -69, -69, -69, -69, 0, 0, -69, 0, -69, -69, -69, 0, -69, -69, -69, -69, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, -82, -82, -82, -82, -82, -82, -82, -82, 0, 0, -82, 0, -82, -82, -82, 0, -82, -82, -82, -82, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        -95, 0, 0, -95, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -95, 0, 0, 0, 0, -95, 0, 0, 0, 0, 0, -95, 0, 0, 0, -95, -95, -95, 0, 0,
        // State 113
        -96, 0, 0, -96, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, -96, 0, 0, 0, -96, -96, -96, 0, 0,
        // State 114
        -93, 0, 0, -93, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, -93, 0, 0, 0, -93, -93, -93, 0, 0,
        // State 115
        -94, 0, 0, -94, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -94, 0, 0, 0, 0, -94, 0, 0, 0, 0, 0, -94, 0, 0, 0, -94, -94, -94, 0, 0,
        // State 116
        -98, 0, 0, -98, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -98, 0, 0, 0, 0, -98, 0, 0, 0, 0, 0, -98, 0, 0, 0, -98, -98, -98, 0, 0,
        // State 117
        -97, 0, 0, -97, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, -97, 0, 0, 0, -97, -97, -97, 0, 0,
        // State 118
        -101, 0, 0, -101, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -101, 0, 0, 0, 0, -101, 0, 0, 0, 0, 0, -101, 0, 0, 0, -101, -101, -101, 0, 0,
        // State 119
        -99, 0, 0, -99, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -99, 0, 0, 0, 0, -99, 0, 0, 0, 0, 0, -99, 0, 0, 0, -99, -99, -99, 0, 0,
        // State 120
        -100, 0, 0, -100, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -100, 0, 0, 0, 0, -100, 0, 0, 0, 0, 0, -100, 0, 0, 0, -100, -100, -100, 0, 0,
        // State 121
        -102, 0, 0, -102, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -102, 0, 0, 0, 0, -102, 0, 0, 0, 0, 0, -102, 0, 0, 0, -102, -102, -102, 0, 0,
        // State 122
        -103, 0, 0, -103, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -103, 0, 0, 0, 0, -103, 0, 0, 0, 0, 0, -103, 0, 0, 0, -103, -103, -103, 0, 0,
        // State 123
        -106, 0, 0, -106, 0, 0, 0, 0, -106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -106, 0, 0, 0, 0, -106, 0, 0, 0, 0, 0, -106, 0, 0, 0, -106, -106, -106, 0, 0,
        // State 124
        -107, 0, 0, -107, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -107, 0, 0, 0, 0, -107, 0, 0, 0, 0, 0, -107, 0, 0, 0, -107, -107, -107, 0, 0,
        // State 125
        0, 0, 0, 0, 142, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        -37, 0, 0, -37, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, -37, -37, -37, -37, 0, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, 0, 0,
        // State 127
        0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 149, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 130
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 132
        -154, 0, 0, -154, 0, 0, 0, 0, -154, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -154, -154, -154, -154, -154, -154, -154, 0, -154, -154, -154, -154, -154, -154, -154, -154, -154, -154, -154, 0, 0,
        // State 133
        -137, 0, 0, -137, 0, 0, 0, 0, -137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -137, -137, -137, -137, -137, -137, -137, 0, -137, -137, -137, -137, -137, -137, -137, -137, -137, -137, -137, 0, 0,
        // State 134
        0, 0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 135
        0, -131, 0, 0, -131, 0, -131, -131, -131, 0, 0, 0, 0, -131, -131, -131, 0, -131, -131, -131, -131, 0, 0, 0, 0, 0, 0, 0, -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        0, -125, 0, 0, -125, 0, 0, -125, 0, 0, 0, 0, 0, -125, -125, -125, 0, -125, -125, -125, -125, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 137
        0, -59, -59, 0, -59, -59, -59, -59, -59, 0, 0, -59, 0, -59, -59, -59, 0, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 138
        0, -126, 0, 0, -126, 0, 0, -126, 0, 0, 0, 0, 0, -126, 0, 0, 0, -126, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 139
        0, 0, 0, 0, -128, 0, 0, -128, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 140
        0, -127, -127, 0, -127, -127, -127, -127, -127, 0, 0, -127, 0, -127, -127, -127, 0, -127, -127, -127, -127, 0, 0, 0, 0, 0, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, -77, -77, -77, -77, -77, -77, -77, -77, 0, 0, -77, 0, -77, -77, -77, 0, -77, -77, -77, -77, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 142
        -123, 0, 0, -123, 0, 0, 0, 0, -123, 0, 0, 0, 0, -123, 0, 0, 0, 0, 0, 0, 0, 0, -123, 0, 0, 0, 0, -123, 0, 0, 0, 0, 0, -123, 0, 0, 0, -123, -123, -123, 0, 0,
        // State 143
        -88, 0, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, -88, 0, 0, 0, 0, -88, 0, 0, 0, 0, 0, -88, 0, 0, 0, -88, -88, -88, 0, 0,
        // State 144
        -89, 0, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, 0, 0, 0, -89, 0, 0, 0, 0, -89, 0, 0, 0, 0, 0, -89, 0, 0, 0, -89, -89, -89, 0, 0,
        // State 145
        -135, 0, 0, -135, 0, 0, 0, 0, -135, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, -135, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, -135, 0, 0, 0, -135, -135, -135, 0, 0,
        // State 146
        -90, 0, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, 0, -90, -90, -90, 0, 0,
        // State 147
        0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 148
        -47, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, -47, -47, -47, -47, -47, -47, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, -47, 0, 0,
        // State 149
        -151, 0, 0, -151, 0, 0, 0, 0, -151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -151, -151, -151, -151, -151, -151, -151, 0, -151, -151, -151, -151, -151, -151, -151, -151, -151, -151, -151, 0, 0,
        // State 150
        -152, 0, 0, -152, 0, 0, 0, 0, -152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -152, -152, -152, -152, -152, -152, -152, 0, -152, -152, -152, -152, -152, -152, -152, -152, -152, -152, -152, 0, 0,
        // State 151
        -153, 0, 0, -153, 0, 0, 0, 0, -153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -153, -153, -153, -153, -153, -153, -153, 0, -153, -153, -153, -153, -153, -153, -153, -153, -153, -153, -153, 0, 0,
        // State 152
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 153
        0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 154
        -138, 0, 0, -138, 0, 0, 0, 0, -138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -138, -138, -138, -138, -138, -138, -138, 0, -138, -138, -138, -138, -138, -138, -138, -138, -138, -138, -138, 0, 0,
        // State 155
        0, 0, 0, 0, -48, 0, 0, -48, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, -50, -50, -50, -50, -50, -50, -50, -50, 0, 0, -50, 0, -50, -50, -50, 0, -50, -50, -50, -50, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 157
        0, 0, 0, 0, -31, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 159
        0, 0, 0, 0, -109, 0, 0, -109, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 160
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 161
        -85, 0, 0, -85, -85, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, -85, 0, 0, 0, -85, -85, -85, 0, 0,
        // State 162
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 181, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 163
        0, 0, 0, 0, -160, 0, 0, -160, 0, 0, 0, 0, 0, -160, 0, 0, -160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -160, -160, -160, 0, 0, 0, 0,
        // State 164
        0, 0, 0, 0, -159, 0, 0, -159, 0, 0, 0, 0, 0, -159, 0, 0, -159, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -159, -159, -159, 0, 0, 0, 0,
        // State 165
        0, 0, 0, 0, -158, 0, 0, -158, 0, 0, 0, 0, 0, -158, 0, 0, -158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -158, -158, -158, 0, 0, 0, 0,
        // State 166
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 167
        -43, 0, 0, -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, -43, -43, -43, -43, -43, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0,
        // State 168
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, 0, 0, 0, 0,
        // State 169
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 170
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 171
        0, 0, 0, 0, -6, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 172
        0, 0, 0, 0, -111, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 173
        -84, 0, 0, -84, -84, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, -84, 0, 0, 0, -84, -84, -84, 0, 0,
        // State 174
        0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 175
        0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 176
        -148, 0, 0, -148, 0, 0, 0, 0, -148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -148, -148, -148, -148, -148, -148, 0, -148, -148, -148, -148, -148, -148, -148, -148, -148, -148, -148, 0, 0,
        // State 177
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 178
        -133, 0, 0, -133, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, -133, -133, -133, -133, -133, 0, -133, -133, -133, -133, -133, -133, -133, -133, -133, -133, -133, 0, 0,
        // State 179
        -140, 0, 0, -140, 0, 0, 0, 0, -140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, -140, -140, -140, -140, -140, -140, 0, -140, -140, -140, -140, -140, -140, -140, -140, -140, -140, -140, 0, 0,
        // State 180
        -45, 0, 0, -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, -45, -45, -45, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, 0, 0,
        // State 181
        -46, 0, 0, -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, -46, -46, -46, -46, 0, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, -46, 0, 0,
        // State 182
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, 0, 0, 0,
        // State 183
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 184
        -143, 0, 0, -143, 0, 0, 0, 0, -143, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -143, -143, -143, -143, -143, -143, -143, 0, -143, -143, -143, -143, -143, -143, -143, -143, -143, -143, -143, 0, 0,
        // State 185
        -150, 0, 0, -150, 0, 0, 0, 0, -150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -150, -150, -150, -150, -150, -150, 0, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, 0, 0,
        // State 186
        -134, 0, 0, -134, 0, 0, 0, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -134, -134, -134, -134, -134, -134, -134, 0, -134, -134, -134, -134, -134, -134, -134, -134, -134, -134, -134, 0, 0,
        // State 187
        -136, 0, 0, -136, 0, 0, 0, 0, -136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -136, -136, -136, -136, -136, -136, 0, -136, -136, -136, -136, -136, -136, -136, -136, -136, -136, -136, 0, 0,
        // State 188
        0, 0, 0, 0, -7, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 189
        0, 0, 0, 0, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 190
        -92, 0, 0, -92, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, -92, -92, -92, -92, -92, 0, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, -92, 0, 0,
        // State 191
        0, 0, 0, 0, -108, 0, 0, -108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 192
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 193
        0, 0, 0, 0, 0, 0, 0, 201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -116, -116, 0, 0, 0, 0,
        // State 194
        0, 0, 0, 0, -12, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 195
        -144, 0, 0, -144, 0, 0, 0, 0, -144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -144, -144, -144, -144, -144, -144, -144, 0, -144, -144, -144, -144, -144, -144, -144, -144, -144, -144, -144, 0, 0,
        // State 196
        -147, 0, 0, -147, 0, 0, 0, 0, -147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -147, -147, -147, -147, -147, -147, 0, -147, -147, -147, -147, -147, -147, -147, -147, -147, -147, -147, 0, 0,
        // State 197
        -142, 0, 0, -142, 0, 0, 0, 0, -142, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -142, -142, -142, -142, -142, -142, -142, 0, -142, -142, -142, -142, -142, -142, -142, -142, -142, -142, -142, 0, 0,
        // State 198
        -149, 0, 0, -149, 0, 0, 0, 0, -149, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -149, -149, -149, -149, -149, -149, 0, -149, -149, -149, -149, -149, -149, -149, -149, -149, -149, -149, 0, 0,
        // State 199
        -44, 0, 0, -44, 0, 0, 0, 0, -44, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, -44, -44, -44, -44, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0,
        // State 200
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -115, -115, 0, 0, 0, 0,
        // State 201
        -91, 0, 0, -91, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, -91, -91, -91, -91, -91, 0, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, -91, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 42 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -113,
        // State 1
        -114,
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
        -122,
        // State 57
        -35,
        // State 58
        -41,
        // State 59
        -36,
        // State 60
        -34,
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
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        -162,
        // State 81
        -145,
        // State 82
        -38,
        // State 83
        0,
        // State 84
        0,
        // State 85
        -141,
        // State 86
        -33,
        // State 87
        -118,
        // State 88
        -140,
        // State 89
        -117,
        // State 90
        -139,
        // State 91
        -119,
        // State 92
        -120,
        // State 93
        -121,
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
        -42,
        // State 108
        -146,
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
        0,
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
        -37,
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
        -154,
        // State 133
        -137,
        // State 134
        0,
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
        -47,
        // State 149
        -151,
        // State 150
        -152,
        // State 151
        -153,
        // State 152
        0,
        // State 153
        0,
        // State 154
        -138,
        // State 155
        0,
        // State 156
        0,
        // State 157
        0,
        // State 158
        0,
        // State 159
        0,
        // State 160
        0,
        // State 161
        0,
        // State 162
        0,
        // State 163
        0,
        // State 164
        0,
        // State 165
        0,
        // State 166
        0,
        // State 167
        -43,
        // State 168
        0,
        // State 169
        0,
        // State 170
        0,
        // State 171
        0,
        // State 172
        0,
        // State 173
        0,
        // State 174
        0,
        // State 175
        0,
        // State 176
        -148,
        // State 177
        0,
        // State 178
        -133,
        // State 179
        -140,
        // State 180
        -45,
        // State 181
        -46,
        // State 182
        0,
        // State 183
        0,
        // State 184
        -143,
        // State 185
        -150,
        // State 186
        -134,
        // State 187
        -136,
        // State 188
        0,
        // State 189
        0,
        // State 190
        -92,
        // State 191
        0,
        // State 192
        0,
        // State 193
        0,
        // State 194
        0,
        // State 195
        -144,
        // State 196
        -147,
        // State 197
        -142,
        // State 198
        -149,
        // State 199
        -44,
        // State 200
        0,
        // State 201
        -91,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            3 => 157,
            6 => 172,
            16 => 43,
            19 => 134,
            20 => 56,
            21 => 57,
            22 => match state {
                1 | 26 => 107,
                _ => 58,
            },
            24 => match state {
                14 => 26,
                _ => 1,
            },
            25 => 59,
            26 => match state {
                22 => 142,
                _ => 60,
            },
            27 => 61,
            28 => 62,
            29 => match state {
                17 => 136,
                _ => 63,
            },
            30 => match state {
                19 => 138,
                _ => 64,
            },
            31 => match state {
                21 => 140,
                _ => 65,
            },
            32 => 66,
            33 => 67,
            34 => 68,
            35 => 69,
            36 => 70,
            37 => 71,
            38 => match state {
                20 => 139,
                _ => 72,
            },
            39 => 73,
            40 => match state {
                2 | 18 => 109,
                _ => 74,
            },
            41 => 75,
            42 => match state {
                15 => 28,
                11 => 129,
                12 => 130,
                13 => 131,
                23 => 147,
                25 => 153,
                27 => 155,
                35 => 160,
                37 => 166,
                39 => 171,
                41 => 174,
                45 => 188,
                49 => 192,
                _ => 76,
            },
            44 => match state {
                16 => 135,
                _ => 77,
            },
            45 => 78,
            46 => match state {
                9 => 125,
                _ => 79,
            },
            47 => 41,
            48 => 175,
            49 => 35,
            50 => 126,
            51 => 16,
            52 => 17,
            53 => 18,
            54 => 19,
            55 => 20,
            56 => 2,
            57 => 21,
            58 => match state {
                46 => 189,
                51 => 194,
                _ => 40,
            },
            59 => 158,
            60 => 80,
            61 => match state {
                43 => 182,
                _ => 168,
            },
            62 => 81,
            63 => 82,
            64 => 143,
            65 => 83,
            66 => match state {
                17 => 30,
                _ => 3,
            },
            67 => match state {
                19 => 31,
                _ => 4,
            },
            68 => match state {
                21 => 33,
                _ => 5,
            },
            69 => match state {
                20 => 32,
                _ => 6,
            },
            70 => 7,
            71 => match state {
                18 => 137,
                _ => 110,
            },
            72 => match state {
                16 => 29,
                _ => 8,
            },
            73 => 84,
            74 => 176,
            75 => match state {
                44 => 184,
                53 => 195,
                54 => 197,
                _ => 177,
            },
            76 => 144,
            77 => match state {
                53 => 196,
                54 => 198,
                _ => 185,
            },
            78 => match state {
                47 => 190,
                55 => 201,
                _ => 85,
            },
            79 => match state {
                42 => 178,
                _ => 86,
            },
            80 => 87,
            81 => match state {
                42 => 179,
                44 | 53..=54 => 186,
                _ => 88,
            },
            82 => match state {
                22 => 145,
                _ => 89,
            },
            83 => match state {
                44 | 53..=54 => 187,
                _ => 90,
            },
            84 => 91,
            85 => 92,
            86 => 93,
            87 => 152,
            88 => match state {
                36 => 162,
                48 => 191,
                50 => 193,
                _ => 55,
            },
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""!""###,
        r###""!=""###,
        r###""%""###,
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""->""###,
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
        'err,
    >(
        __states: &[i16],
        _: core::marker::PhantomData<(&'err ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'err>
    where 
    {
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __phantom: core::marker::PhantomData<(&'err ())>,
    }
    impl<'err> __state_machine::ParserDefinition for __StateMachine<'err>
    where 
    {
        type Location = usize;
        type Error = ErrorS;
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
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 42 - 1)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
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
                self.errors,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'err,
    >(
        __token: &Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Bang if true => Some(0),
            Token::BangEqual if true => Some(1),
            Token::Percent if true => Some(2),
            Token::LeftParen if true => Some(3),
            Token::RightParen if true => Some(4),
            Token::Star if true => Some(5),
            Token::Plus if true => Some(6),
            Token::Comma if true => Some(7),
            Token::Minus if true => Some(8),
            Token::Arrow if true => Some(9),
            Token::Dot if true => Some(10),
            Token::Slash if true => Some(11),
            Token::Colon if true => Some(12),
            Token::Semicolon if true => Some(13),
            Token::Less if true => Some(14),
            Token::LessEqual if true => Some(15),
            Token::Equal if true => Some(16),
            Token::EqualEqual if true => Some(17),
            Token::Greater if true => Some(18),
            Token::GreaterEqual if true => Some(19),
            Token::And if true => Some(20),
            Token::Else if true => Some(21),
            Token::False if true => Some(22),
            Token::Fn if true => Some(23),
            Token::For if true => Some(24),
            Token::If if true => Some(25),
            Token::Let if true => Some(26),
            Token::Nil if true => Some(27),
            Token::Or if true => Some(28),
            Token::Print if true => Some(29),
            Token::PrintLn if true => Some(30),
            Token::Return if true => Some(31),
            Token::Struct if true => Some(32),
            Token::True if true => Some(33),
            Token::While if true => Some(34),
            Token::LeftBrace if true => Some(35),
            Token::RightBrace if true => Some(36),
            Token::Identifier(_) if true => Some(37),
            Token::Number(_) if true => Some(38),
            Token::String(_) if true => Some(39),
            Token::TypeInt if true => Some(40),
            Token::TypeString if true => Some(41),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'err,
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 40 | 41 => __Symbol::Variant0(__token),
            37 | 39 => match __token {
                Token::Identifier(__tok0) | Token::String(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            38 => match __token {
                Token::Number(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'err,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'err>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 12,
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
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 0,
                    nonterminal_produced: 18,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 19,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 23,
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
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 25,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 26,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 26,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 27,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 28,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
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
                    states_to_pop: 3,
                    nonterminal_produced: 35,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 37,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 40,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
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
                    states_to_pop: 3,
                    nonterminal_produced: 41,
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
                    states_to_pop: 1,
                    nonterminal_produced: 43,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 43,
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
                    nonterminal_produced: 45,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 46,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 47,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 48,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
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
                    states_to_pop: 7,
                    nonterminal_produced: 50,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 50,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
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
                    states_to_pop: 1,
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
                    nonterminal_produced: 54,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 56,
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
                    nonterminal_produced: 57,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 58,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 59,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 59,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 60,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 61,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 61,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 67,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 68,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 69,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 70,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 71,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 72,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 73,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 74,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 75,
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
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 78,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 78,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 79,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 79,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 80,
                }
            }
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 81,
                }
            }
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 81,
                }
            }
            143 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 81,
                }
            }
            144 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 81,
                }
            }
            145 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 82,
                }
            }
            146 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 83,
                }
            }
            147 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 83,
                }
            }
            148 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 83,
                }
            }
            149 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 83,
                }
            }
            150 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 84,
                }
            }
            151 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 85,
                }
            }
            152 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 86,
                }
            }
            153 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 86,
                }
            }
            154 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 87,
                }
            }
            155 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 87,
                }
            }
            156 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 87,
                }
            }
            157 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            158 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            159 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 88,
                }
            }
            160 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 89,
                }
            }
            161 => __state_machine::SimulatedReduce::Accept,
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
            'err,
            __TOKEN: __ToTriple<'err, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
            __tokens0: __TOKENS,
        ) -> Result<ast::Program, __lalrpop_util::ParseError<usize, Token, ErrorS>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    errors,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'err,
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'err ())>,
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
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
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
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> Option<Result<ast::Program,__lalrpop_util::ParseError<usize, Token, ErrorS>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                __reduce94(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            95 => {
                __reduce95(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            96 => {
                __reduce96(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            97 => {
                __reduce97(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            98 => {
                __reduce98(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            99 => {
                __reduce99(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            100 => {
                __reduce100(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            101 => {
                __reduce101(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            102 => {
                __reduce102(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            103 => {
                __reduce103(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            104 => {
                __reduce104(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            105 => {
                __reduce105(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            106 => {
                __reduce106(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            107 => {
                __reduce107(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            108 => {
                __reduce108(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            109 => {
                __reduce109(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            110 => {
                __reduce110(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            111 => {
                __reduce111(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            112 => {
                __reduce112(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            113 => {
                __reduce113(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            114 => {
                __reduce114(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            115 => {
                __reduce115(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            116 => {
                __reduce116(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            117 => {
                __reduce117(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            118 => {
                __reduce118(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            119 => {
                __reduce119(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            120 => {
                __reduce120(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            121 => {
                __reduce121(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            122 => {
                __reduce122(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            123 => {
                __reduce123(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            124 => {
                __reduce124(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            125 => {
                __reduce125(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            126 => {
                __reduce126(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            127 => {
                __reduce127(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            128 => {
                __reduce128(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            129 => {
                __reduce129(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            130 => {
                __reduce130(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            131 => {
                __reduce131(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            132 => {
                __reduce132(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            133 => {
                __reduce133(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            134 => {
                __reduce134(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            135 => {
                __reduce135(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            136 => {
                __reduce136(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            137 => {
                __reduce137(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            138 => {
                __reduce138(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            139 => {
                __reduce139(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            140 => {
                __reduce140(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            141 => {
                __reduce141(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            142 => {
                __reduce142(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            143 => {
                __reduce143(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            144 => {
                __reduce144(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            145 => {
                __reduce145(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            146 => {
                __reduce146(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            147 => {
                __reduce147(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            148 => {
                __reduce148(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            149 => {
                __reduce149(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            150 => {
                __reduce150(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            151 => {
                __reduce151(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            152 => {
                __reduce152(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            153 => {
                __reduce153(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            154 => {
                __reduce154(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            155 => {
                __reduce155(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            156 => {
                __reduce156(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            157 => {
                __reduce157(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            158 => {
                __reduce158(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            159 => {
                __reduce159(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            160 => {
                __reduce160(errors, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            161 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant25(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(errors, __sym0);
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
    fn __pop_Variant11<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (String, Option<ast::Type>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, HashMap<String, Option<ast::Type>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Option<ast::StatementS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant15<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::ExprS>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<ast::Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Field, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpInfix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::OpPrefix, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Expression>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Spanned<ast::Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementBlock, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::StatementFun, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<Token>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, core::option::Option<ast::Type>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
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
    fn __pop_Variant14<
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ","? = "," => ActionFn(123);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action123::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ","? =  => ActionFn(124);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action124::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>) = ",", ExprS => ActionFn(88);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action88::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* =  => ActionFn(86);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action86::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)* = ("," <ExprS>)+ => ActionFn(87);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action87::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ",", ExprS => ActionFn(150);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action150::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <ExprS>)+ = ("," <ExprS>)+, ",", ExprS => ActionFn(151);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action151::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>) = ",", Param => ActionFn(91);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action91::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce8<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)* =  => ActionFn(89);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action89::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 5)
    }
    pub(crate) fn __reduce9<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)* = ("," <Param>)+ => ActionFn(90);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)+ = ",", Param => ActionFn(154);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action154::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("," <Param>)+ = ("," <Param>)+, ",", Param => ActionFn(155);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action155::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce12<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>) = "->", Types => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action94::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce13<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>)? = "->", Types => ActionFn(158);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action158::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("->" <Types>)? =  => ActionFn(93);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action93::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce15<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>) = ":", Types => ActionFn(122);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action122::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? = ":", Types => ActionFn(161);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action161::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce17<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (":" <Types>)? =  => ActionFn(121);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action121::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce18<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>) = "=", ExprS => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action119::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? = "=", ExprS => ActionFn(166);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action166::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce20<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ("=" <ExprS>)? =  => ActionFn(118);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action118::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce21<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // () =  => ActionFn(125);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action125::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>) = SingleField => ActionFn(128);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action128::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)* =  => ActionFn(126);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action126::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce24<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)* = (<SingleField>)+ => ActionFn(127);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action127::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce25<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)+ = SingleField => ActionFn(174);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action174::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce26<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // (<SingleField>)+ = (<SingleField>)+, SingleField => ActionFn(175);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action175::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce27<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(143);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action143::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce28<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(142);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action142::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 18)
    }
    pub(crate) fn __reduce29<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args = ExprS => ActionFn(152);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action152::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce30<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args = ExprS, ("," <ExprS>)+ => ActionFn(153);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action153::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce31<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Args =  => ActionFn(171);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action171::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce32<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = Stmt => ActionFn(4);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce33<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclVar => ActionFn(5);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce34<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclFun => ActionFn(6);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce35<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Decl = DeclStruct => ActionFn(7);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce36<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclFun = "fn", Function => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant21(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action12::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce37<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS = Spanned<Decl> => ActionFn(3);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce38<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS* =  => ActionFn(130);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action130::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 23)
    }
    pub(crate) fn __reduce39<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS* = DeclS+ => ActionFn(131);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action131::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce40<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS => ActionFn(132);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action132::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce41<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclS+ = DeclS+, DeclS => ActionFn(133);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action133::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce42<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclStruct = "struct", identifier, StructFields, ";" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant27(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (4, 25)
    }
    pub(crate) fn __reduce43<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, "=", ExprS, ";" => ActionFn(167);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant4(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action167::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 26)
    }
    pub(crate) fn __reduce44<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ":", Types, ";" => ActionFn(168);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action168::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 26)
    }
    pub(crate) fn __reduce45<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, "=", ExprS, ";" => ActionFn(169);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action169::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 26)
    }
    pub(crate) fn __reduce46<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // DeclVar = "let", identifier, ";" => ActionFn(170);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action170::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce47<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprAssign = identifier, "=", ExprS => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce48<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprAssign = ExprLogicOr => ActionFn(43);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce49<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprCall = Spanned<ExprCall>, "(", Args, ")" => ActionFn(67);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant15(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action67::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (4, 28)
    }
    pub(crate) fn __reduce50<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprCall = ExprPrimary => ActionFn(68);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action68::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce51<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprComparison = ExprInfix<ExprComparison, OpComparison, ExprTerm> => ActionFn(51);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce52<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprEquality = ExprInfix<ExprEquality, OpEquality, ExprComparison> => ActionFn(48);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce53<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprFactor = ExprInfix<ExprFactor, OpFactor, ExprPrefix> => ActionFn(59);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce54<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = Spanned<ExprComparison>, OpComparison, Spanned<ExprTerm> => ActionFn(101);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action101::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 32)
    }
    pub(crate) fn __reduce55<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprComparison, OpComparison, ExprTerm> = ExprTerm => ActionFn(102);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action102::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce56<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = Spanned<ExprEquality>, OpEquality, Spanned<ExprComparison> => ActionFn(103);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action103::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 33)
    }
    pub(crate) fn __reduce57<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprEquality, OpEquality, ExprComparison> = ExprComparison => ActionFn(104);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action104::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce58<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = Spanned<ExprFactor>, OpFactor, Spanned<ExprPrefix> => ActionFn(97);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action97::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce59<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprFactor, OpFactor, ExprPrefix> = ExprPrefix => ActionFn(98);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce60<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = Spanned<ExprLogicAnd>, OpLogicAnd, Spanned<ExprEquality> => ActionFn(105);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action105::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 35)
    }
    pub(crate) fn __reduce61<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> = ExprEquality => ActionFn(106);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action106::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce62<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = Spanned<ExprLogicOr>, OpLogicOr, Spanned<ExprLogicAnd> => ActionFn(107);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action107::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 36)
    }
    pub(crate) fn __reduce63<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> = ExprLogicAnd => ActionFn(108);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action108::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce64<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = Spanned<ExprTerm>, OpTerm, Spanned<ExprFactor> => ActionFn(99);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action99::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 37)
    }
    pub(crate) fn __reduce65<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprInfix<ExprTerm, OpTerm, ExprFactor> = ExprFactor => ActionFn(100);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce66<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprLogicAnd = ExprInfix<ExprLogicAnd, OpLogicAnd, ExprEquality> => ActionFn(46);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce67<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprLogicOr = ExprInfix<ExprLogicOr, OpLogicOr, ExprLogicAnd> => ActionFn(44);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce68<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrefix = OpPrefix, Spanned<ExprPrefix> => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action65::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (2, 40)
    }
    pub(crate) fn __reduce69<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrefix = ExprCall => ActionFn(66);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action66::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce70<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "nil" => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action69::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce71<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "false" => ActionFn(70);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce72<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "true" => ActionFn(71);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action71::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce73<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = string => ActionFn(72);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action72::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce74<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = number => ActionFn(73);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce75<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = ExprVar => ActionFn(74);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action74::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce76<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprPrimary = "(", Expression, ")" => ActionFn(75);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action75::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 41)
    }
    pub(crate) fn __reduce77<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS = Spanned<Expression> => ActionFn(40);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 42)
    }
    pub(crate) fn __reduce78<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS? = ExprS => ActionFn(110);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action110::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 43)
    }
    pub(crate) fn __reduce79<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprS? =  => ActionFn(111);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action111::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 43)
    }
    pub(crate) fn __reduce80<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprTerm = ExprInfix<ExprTerm, OpTerm, ExprFactor> => ActionFn(56);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce81<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ExprVar = identifier => ActionFn(85);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce82<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Expression = ExprAssign => ActionFn(41);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 46)
    }
    pub(crate) fn __reduce83<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForCond = ExprS, ";" => ActionFn(212);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action212::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 47)
    }
    pub(crate) fn __reduce84<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForCond = ";" => ActionFn(213);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action213::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce85<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForIncr = ExprS => ActionFn(214);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action214::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce86<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForIncr =  => ActionFn(215);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action215::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 48)
    }
    pub(crate) fn __reduce87<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<DeclVar> => ActionFn(24);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce88<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = Spanned<StmtExpr> => ActionFn(25);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce89<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // ForInit = ";" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce90<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", "->", Types, StatementBlockIn => ActionFn(159);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant26(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action159::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (7, 50)
    }
    pub(crate) fn __reduce91<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Function = identifier, "(", Params, ")", StatementBlockIn => ActionFn(160);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant26(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant24(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action160::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (5, 50)
    }
    pub(crate) fn __reduce92<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = ">" => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action52::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce93<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = ">=" => ActionFn(53);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce94<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = "<" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce95<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpComparison = "<=" => ActionFn(55);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce96<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpEquality = "==" => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce97<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpEquality = "!=" => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce98<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "*" => ActionFn(60);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce99<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "/" => ActionFn(61);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce100<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpFactor = "%" => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce101<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpLogicAnd = "and" => ActionFn(47);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce102<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpLogicOr = "or" => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce103<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpPrefix = "-" => ActionFn(63);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action63::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce104<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpPrefix = "!" => ActionFn(64);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 56)
    }
    pub(crate) fn __reduce105<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpTerm = "+" => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce106<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // OpTerm = "-" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce107<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Param = identifier, ":", Types => ActionFn(164);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action164::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 58)
    }
    pub(crate) fn __reduce108<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Param = identifier => ActionFn(165);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action165::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce109<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params = Param => ActionFn(156);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action156::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce110<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params = Param, ("," <Param>)+ => ActionFn(157);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action157::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (2, 59)
    }
    pub(crate) fn __reduce111<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(172);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action172::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (0, 59)
    }
    pub(crate) fn __reduce112<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(208);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action208::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 60)
    }
    pub(crate) fn __reduce113<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Program = DeclS+ => ActionFn(209);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action209::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce114<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SingleField = identifier, ":", Types, "," => ActionFn(148);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action148::<>(errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 61)
    }
    pub(crate) fn __reduce115<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SingleField = identifier, ":", Types => ActionFn(149);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action149::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 61)
    }
    pub(crate) fn __reduce116<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtExpr => ActionFn(29);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce117<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtBlock => ActionFn(30);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce118<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrint => ActionFn(31);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce119<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtPrintLn => ActionFn(32);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce120<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // SmallStmt = StmtReturn => ActionFn(33);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce121<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Decl> = Decl => ActionFn(193);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action193::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce122<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<DeclVar> = DeclVar => ActionFn(194);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action194::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce123<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprCall> = ExprCall => ActionFn(195);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action195::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce124<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprComparison> = ExprComparison => ActionFn(196);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action196::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 66)
    }
    pub(crate) fn __reduce125<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprEquality> = ExprEquality => ActionFn(197);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action197::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 67)
    }
    pub(crate) fn __reduce126<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprFactor> = ExprFactor => ActionFn(198);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action198::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 68)
    }
    pub(crate) fn __reduce127<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicAnd> = ExprLogicAnd => ActionFn(199);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action199::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 69)
    }
    pub(crate) fn __reduce128<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprLogicOr> = ExprLogicOr => ActionFn(200);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action200::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 70)
    }
    pub(crate) fn __reduce129<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprPrefix> = ExprPrefix => ActionFn(201);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action201::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 71)
    }
    pub(crate) fn __reduce130<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<ExprTerm> = ExprTerm => ActionFn(202);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action202::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 72)
    }
    pub(crate) fn __reduce131<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Expression> = Expression => ActionFn(203);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action203::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 73)
    }
    pub(crate) fn __reduce132<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<Stmt> = Stmt => ActionFn(204);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action204::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 74)
    }
    pub(crate) fn __reduce133<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtClosed> = StmtClosed => ActionFn(205);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action205::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 75)
    }
    pub(crate) fn __reduce134<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtExpr> = StmtExpr => ActionFn(206);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action206::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 76)
    }
    pub(crate) fn __reduce135<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Spanned<StmtOpen> = StmtOpen => ActionFn(207);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action207::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 77)
    }
    pub(crate) fn __reduce136<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", "}" => ActionFn(210);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action210::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (2, 78)
    }
    pub(crate) fn __reduce137<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StatementBlockIn = "{", DeclS+, "}" => ActionFn(211);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant18(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action211::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (3, 78)
    }
    pub(crate) fn __reduce138<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = StmtOpen => ActionFn(14);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 79)
    }
    pub(crate) fn __reduce139<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Stmt = StmtClosed => ActionFn(15);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 79)
    }
    pub(crate) fn __reduce140<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtBlock = StatementBlockIn => ActionFn(37);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 80)
    }
    pub(crate) fn __reduce141<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtClosed> => ActionFn(20);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action20::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 81)
    }
    pub(crate) fn __reduce142<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "while", "(", ExprS, ")", Spanned<StmtClosed> => ActionFn(21);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action21::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 81)
    }
    pub(crate) fn __reduce143<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtClosed> => ActionFn(22);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action22::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 81)
    }
    pub(crate) fn __reduce144<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtClosed = SmallStmt => ActionFn(23);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 81)
    }
    pub(crate) fn __reduce145<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtExpr = ExprS, ";" => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action39::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 82)
    }
    pub(crate) fn __reduce146<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "for", "(", ForInit, ForCond, ForIncr, ")", Spanned<StmtOpen> => ActionFn(16);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant10(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant20(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action16::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 83)
    }
    pub(crate) fn __reduce147<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<Stmt> => ActionFn(17);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action17::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 83)
    }
    pub(crate) fn __reduce148<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "if", "(", ExprS, ")", Spanned<StmtClosed>, "else", Spanned<StmtOpen> => ActionFn(18);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant17(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action18::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (7, 83)
    }
    pub(crate) fn __reduce149<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtOpen = "while", "(", ExprS, ")", Spanned<StmtOpen> => ActionFn(19);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action19::<>(errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (5, 83)
    }
    pub(crate) fn __reduce150<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtPrint = "print", ExprS, ";" => ActionFn(36);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action36::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 84)
    }
    pub(crate) fn __reduce151<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtPrintLn = "println", ExprS, ";" => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 85)
    }
    pub(crate) fn __reduce152<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ExprS, ";" => ActionFn(216);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action216::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (3, 86)
    }
    pub(crate) fn __reduce153<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StmtReturn = "return", ";" => ActionFn(217);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action217::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 86)
    }
    pub(crate) fn __reduce154<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields = "{", "}" => ActionFn(176);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action176::<>(errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (2, 87)
    }
    pub(crate) fn __reduce155<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields = "{", (<SingleField>)+, "}" => ActionFn(177);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action177::<>(errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (3, 87)
    }
    pub(crate) fn __reduce156<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // StructFields =  => ActionFn(173);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action173::<>(errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 87)
    }
    pub(crate) fn __reduce157<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = type_string => ActionFn(82);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action82::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce158<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = type_int => ActionFn(83);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce159<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // Types = "fn" => ActionFn(84);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action84::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 88)
    }
    pub(crate) fn __reduce160<
        'err,
    >(
        errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<>,usize)>,
        _: core::marker::PhantomData<(&'err ())>,
    ) -> (usize, usize)
    {
        // __OpPrefix = OpPrefix => ActionFn(1);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(errors, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 89)
    }
}
pub use self::__parse__Program::ProgramParser;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action0<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Program, usize),
) -> ast::Program
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action1<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::OpPrefix, usize),
) -> ast::OpPrefix
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action2<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, statements, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> ast::Program
{
    ast::Program { statements }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action3<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Spanned<ast::Statement>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action4<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action5<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action6<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action7<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action8<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, String, usize),
    (_, fields, _): (usize, Vec<ast::Field>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Struct(ast::StatementStruct { name, fields })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action9<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, mut fields, _): (usize, alloc::vec::Vec<ast::Field>, usize),
    (_, _, _): (usize, Token, usize),
) -> Vec<ast::Field>
{
    {
      fields
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action10<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, (), usize),
) -> Vec<ast::Field>
{
    Vec::new()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action11<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, type_, _): (usize, ast::Type, usize),
    (_, _, _): (usize, core::option::Option<Token>, usize),
) -> ast::Field
{
    ast::Field { name, type_ }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action12<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, function, _): (usize, ast::StatementFun, usize),
) -> ast::Statement
{
    ast::Statement::Fun(function)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action13<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, name, _): (usize, String, usize),
    (_, type_, _): (usize, core::option::Option<ast::Type>, usize),
    (_, value, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Var(ast::StatementVar {
        var: ast::Var { name, type_: type_},
        value,
      })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action14<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action15<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action16<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action17<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, then_branch, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::If(Box::new(ast::StatementIf { cond, then_branch, else_branch: None }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action18<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action19<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::While(Box::new(ast::StatementWhile { cond, body }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action20<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action21<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, _, _): (usize, Token, usize),
    (_, cond, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, body, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> ast::Statement
{
    ast::Statement::While(Box::new(ast::StatementWhile { cond, body }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action22<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action23<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action24<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> Option<ast::StatementS>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action25<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> Option<ast::StatementS>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action26<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> Option<ast::StatementS>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action27<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action28<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action29<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action30<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action31<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action32<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action33<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action34<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Return(ast::StatementReturn { value })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action35<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::PrintLn(ast::StatementPrintLn { value })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action36<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, value, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Print(ast::StatementPrint { value })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action37<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::StatementBlock, usize),
) -> ast::Statement
{
    ast::Statement::Block(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action38<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, statements, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::StatementBlock
{
    ast::StatementBlock { statements }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action39<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, expr, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Statement
{
    ast::Statement::Expression(ast::StatementExpr { expr })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action40<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action41<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action42<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action43<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action44<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action45<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LogicOr
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action46<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action47<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LogicAnd
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action48<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action49<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Equal
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action50<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::NotEqual
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action51<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action52<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Greater
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action53<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::GreaterEqual
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action54<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Less
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action55<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::LessEqual
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action56<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action57<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Add
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action58<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Sub
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action59<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action60<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Mul
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action61<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Div
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action62<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpInfix
{
    ast::OpInfix::Modulo
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action63<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpPrefix
{
    ast::OpPrefix::Negate
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action64<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::OpPrefix
{
    ast::OpPrefix::Not
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action65<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, op, _): (usize, ast::OpPrefix, usize),
    (_, rt, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Prefix(Box::new(ast::ExprPrefix { op, rt }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action66<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action67<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, callee, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, _, _): (usize, Token, usize),
    (_, args, _): (usize, Vec<ast::ExprS>, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Call(Box::new(ast::ExprCall { callee, args }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action68<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action69<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Nil)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action70<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(false))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action71<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Bool(true))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action72<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::String(__0))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action73<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, f64, usize),
) -> ast::Expression
{
    ast::Expression::Literal(ast::ExprLiteral::Number(__0))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action74<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action75<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, Token, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action76<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, Token, usize),
    (_, params, _): (usize, HashMap<String, Option<ast::Type>>, usize),
    (_, _, _): (usize, Token, usize),
    (_, return_type, _): (usize, core::option::Option<ast::Type>, usize),
    (_, body, _): (usize, ast::StatementBlock, usize),
) -> ast::StatementFun
{
    ast::StatementFun { name, params, return_type, body }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action77<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, first, _): (usize, (String, Option<ast::Type>), usize),
    (_, mut params, _): (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize),
) -> HashMap<String, Option<ast::Type>>
{
    {
        let mut param_map = HashMap::new();
        let (string_, type_) = first;
        param_map.insert(string_, type_);
        for (string_, type_) in params {
          param_map.insert(string_, type_);
        }
        param_map
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action78<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, (), usize),
) -> HashMap<String, Option<ast::Type>>
{
    HashMap::new()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action79<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, string_, _): (usize, String, usize),
    (_, type_, _): (usize, core::option::Option<ast::Type>, usize),
) -> (String, Option<ast::Type>)
{
    (string_,type_)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action80<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, first, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, mut args, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> Vec<ast::ExprS>
{
    {
        args.insert(0, first);
        args
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action81<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, (), usize),
) -> Vec<ast::ExprS>
{
    Vec::new()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action82<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Type
{
    ast::Type::String
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action83<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Type
{
    ast::Type::Int
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action84<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> ast::Type
{
    ast::Type::Fn(ast::Fn { return_type: Box::new(None) })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action85<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, name, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Var(ast::ExprVar { var: ast::Var {name, type_: None} })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action86<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action87<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action88<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action89<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action90<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize),
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action91<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, (String, Option<ast::Type>), usize),
) -> (String, Option<ast::Type>)
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action92<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action93<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Type>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action94<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action95<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action96<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action97<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action98<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action99<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action100<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action101<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action102<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action103<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action104<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action105<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action106<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action107<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, lhs, _): (usize, ast::Spanned<ast::Expression>, usize),
    (_, op, _): (usize, ast::OpInfix, usize),
    (_, rhs, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Expression
{
    ast::Expression::Infix(Box::new(ast::ExprInfix { lhs, op, rhs }))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action108<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action109<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action110<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action111<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action112<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action113<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action114<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action115<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action116<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action117<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action118<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action119<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> ast::Spanned<ast::Expression>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action120<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action121<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Type>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action122<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, _, _): (usize, Token, usize),
    (_, __0, _): (usize, ast::Type, usize),
) -> ast::Type
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action123<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, Token, usize),
) -> core::option::Option<Token>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action124<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Token>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action125<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
)
{
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action126<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Field>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action127<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Field>, usize),
) -> alloc::vec::Vec<ast::Field>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action128<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Field, usize),
) -> ast::Field
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action129<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Statement, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action130<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action131<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action132<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action133<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Statement>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action134<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Field, usize),
) -> alloc::vec::Vec<ast::Field>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action135<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Field>, usize),
    (_, e, _): (usize, ast::Field, usize),
) -> alloc::vec::Vec<ast::Field>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action136<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action137<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action138<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action139<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action140<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action141<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, l, _): (usize, usize, usize),
    (_, t, _): (usize, ast::Expression, usize),
    (_, r, _): (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    (t, l..r)
}

#[allow(unused_variables)]
fn __action142<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookbehind
}

#[allow(unused_variables)]
fn __action143<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    *__lookahead
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action144<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, (String, Option<ast::Type>), usize),
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action145<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize),
    (_, e, _): (usize, (String, Option<ast::Type>), usize),
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action146<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, __0, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action147<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    (_, v, _): (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    (_, e, _): (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action148<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Type, usize),
    __3: (usize, Token, usize),
) -> ast::Field
{
    let __start0 = __3.0;
    let __end0 = __3.2;
    let __temp0 = __action123(
        errors,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        errors,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action149<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Type, usize),
) -> ast::Field
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action124(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        errors,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action150<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action88(
        errors,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action151<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Spanned<ast::Expression>, usize),
) -> alloc::vec::Vec<ast::Spanned<ast::Expression>>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action88(
        errors,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action147(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action152<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action86(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action153<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Expression>>, usize),
) -> Vec<ast::ExprS>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action87(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action154<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, (String, Option<ast::Type>), usize),
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action91(
        errors,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action155<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize),
    __1: (usize, Token, usize),
    __2: (usize, (String, Option<ast::Type>), usize),
) -> alloc::vec::Vec<(String, Option<ast::Type>)>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action91(
        errors,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action156<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, (String, Option<ast::Type>), usize),
) -> HashMap<String, Option<ast::Type>>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action89(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action157<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, (String, Option<ast::Type>), usize),
    __1: (usize, alloc::vec::Vec<(String, Option<ast::Type>)>, usize),
) -> HashMap<String, Option<ast::Type>>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action90(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action158<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action94(
        errors,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action159<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, HashMap<String, Option<ast::Type>>, usize),
    __3: (usize, Token, usize),
    __4: (usize, Token, usize),
    __5: (usize, ast::Type, usize),
    __6: (usize, ast::StatementBlock, usize),
) -> ast::StatementFun
{
    let __start0 = __4.0;
    let __end0 = __5.2;
    let __temp0 = __action158(
        errors,
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action160<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, HashMap<String, Option<ast::Type>>, usize),
    __3: (usize, Token, usize),
    __4: (usize, ast::StatementBlock, usize),
) -> ast::StatementFun
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action93(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action161<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, ast::Type, usize),
) -> core::option::Option<ast::Type>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action122(
        errors,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action120(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action162<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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
    let __temp0 = __action161(
        errors,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        errors,
        __0,
        __1,
        __temp0,
        __4,
        __5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action163<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, core::option::Option<ast::Spanned<ast::Expression>>, usize),
    __3: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action121(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        errors,
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action164<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
    __1: (usize, Token, usize),
    __2: (usize, ast::Type, usize),
) -> (String, Option<ast::Type>)
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action161(
        errors,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action165<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, String, usize),
) -> (String, Option<ast::Type>)
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action121(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action166<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action119(
        errors,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action167<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
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
    let __temp0 = __action166(
        errors,
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action162(
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action168<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Type, usize),
    __4: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __3.2;
    let __end0 = __4.0;
    let __temp0 = __action118(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action162(
        errors,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action169<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
    __3: (usize, ast::Spanned<ast::Expression>, usize),
    __4: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __2.0;
    let __end0 = __3.2;
    let __temp0 = __action166(
        errors,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action163(
        errors,
        __0,
        __1,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action170<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, String, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action118(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action163(
        errors,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action171<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::ExprS>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action125(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action172<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> HashMap<String, Option<ast::Type>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action125(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action173<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::Field>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action125(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action174<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Field, usize),
) -> alloc::vec::Vec<ast::Field>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action128(
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action134(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action175<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, alloc::vec::Vec<ast::Field>, usize),
    __1: (usize, ast::Field, usize),
) -> alloc::vec::Vec<ast::Field>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action128(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action135(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action176<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> Vec<ast::Field>
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action126(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        errors,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action177<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Field>, usize),
    __2: (usize, Token, usize),
) -> Vec<ast::Field>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action127(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        errors,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action178<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action129(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action179<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action180<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action181<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action139(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action182<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action138(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action183<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action141(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action184<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action137(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action185<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action136(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action186<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action187<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action140(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action188<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action189<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action115(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action190<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action114(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action191<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action112(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action192<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
    __1: (usize, usize, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action143(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action116(
        errors,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action193<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action178(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action194<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action179(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action195<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action180(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action196<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action181(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action197<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action182(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action198<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action183(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action199<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action184(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action200<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action185(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action201<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action186(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action202<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action187(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action203<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Expression, usize),
) -> ast::Spanned<ast::Expression>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action188(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action204<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action189(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action205<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action190(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action206<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action191(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action207<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Statement, usize),
) -> ast::Spanned<ast::Statement>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action142(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action192(
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action208<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ast::Program
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action130(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action209<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
) -> ast::Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action131(
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action210<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action130(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        errors,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action211<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, alloc::vec::Vec<ast::Spanned<ast::Statement>>, usize),
    __2: (usize, Token, usize),
) -> ast::StatementBlock
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action131(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        errors,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action212<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Spanned<ast::Expression>, usize),
    __1: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action110(
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        errors,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action213<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action111(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        errors,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action214<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, ast::Spanned<ast::Expression>, usize),
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action110(
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action215<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ast::Spanned<ast::Expression>>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action111(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action216<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, ast::Spanned<ast::Expression>, usize),
    __2: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action110(
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        errors,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action217<
    'err,
>(
    errors: &'err mut Vec<ParseError<usize, cc_lexer::Token, ErrorS>>,
    __0: (usize, Token, usize),
    __1: (usize, Token, usize),
) -> ast::Statement
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action111(
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        errors,
        __0,
        __temp0,
        __1,
    )
}
#[allow(clippy::type_complexity)]

pub trait __ToTriple<'err, >
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, ErrorS>>;
}

impl<'err, > __ToTriple<'err, > for (usize, Token, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, ErrorS>> {
        Ok(value)
    }
}
impl<'err, > __ToTriple<'err, > for Result<(usize, Token, usize), ErrorS>
{
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, ErrorS>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
