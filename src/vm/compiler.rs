use std::{fmt::Display, hash::BuildHasherDefault, mem, ops::Range};

use arrayvec::ArrayVec;
use hashbrown::HashMap;
use termcolor::WriteColor;
use termcolor::{Color, ColorSpec, StandardStream};

use crate::cc_parser::ast::{ExprGet, ExprSet, Field, StatementImpl, StatementStruct};
use crate::vm::error::NameError;
use crate::{
    allocator::allocation::CeAllocation,
    cc_parser::{
        ast::{
            ExprAssign, ExprCall, ExprInfix, ExprLiteral, ExprPrefix, ExprVar, Expression, Fn,
            OpInfix, OpPrefix, Span, Statement, StatementBlock, StatementFor, StatementFun,
            StatementIf, StatementPrint, StatementPrintLn, StatementReturn, StatementVar,
            StatementWhile, Type,
        },
        parse,
    },
    vm::error::TypeError,
};

use super::error::OverflowError;
use super::object::StringObject;
use super::{
    error::{Error, ErrorS, Result, SyntaxError},
    object::ObjectFunction,
    op,
    value::Value,
};

use rustc_hash::FxHasher;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FunctionType {
    Script,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Default)]
struct Local {
    name: String,
    type_: Type,
    depth: usize,
    is_initialized: bool,
    is_captured: bool,
}

impl Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ name: {}, type_: {:?}, depth: {}, is_initialized: {} }}",
            self.name, self.type_, self.depth, self.is_initialized
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Upvalue {
    type_: Type,
    index: u8,
    is_local: bool,
}

pub struct CompilerCell {
    pub function: *mut ObjectFunction,
    type_: FunctionType,
    locals: ArrayVec<Local, 256>,
    scope_depth: usize,
    parent: Option<Box<CompilerCell>>,
    upvalues: ArrayVec<Upvalue, 256>,
}

impl CompilerCell {
    pub fn resolve_local(
        &mut self,
        name: &str,
        capture: bool,
        span: &Span,
    ) -> Result<Option<(u8, Type)>> {
        match self
            .locals
            .iter_mut()
            .enumerate()
            .rfind(|(_, local)| local.name == name)
        {
            Some((idx, local)) => {
                if local.is_initialized {
                    if capture {
                        local.is_captured = true;
                    }
                    Ok(Some((
                        idx.try_into().expect("local index overflow"),
                        local.type_.clone(),
                    )))
                } else {
                    Err((
                        Error::NameError(NameError::AccessInsideInitializer {
                            name: name.to_string(),
                        }),
                        span.clone(),
                    ))
                }
            }
            None => Ok(None),
        }
    }

    pub fn resolve_upvalue(&mut self, name: &str, span: &Span) -> Result<Option<(u8, Type)>> {
        let local = match &mut self.parent {
            Some(parent) => parent.resolve_local(name, true, span)?,
            None => return Ok(None),
        };

        if let Some((local_idx, local_type)) = local {
            let upvalue_idx = self.add_upvalue(local_idx, local_type.clone(), true, span)?;
            return Ok(Some((upvalue_idx, local_type)));
        };

        let upvalue_idx = match &mut self.parent {
            Some(parent) => parent.resolve_upvalue(name, span)?,
            None => return Ok(None),
        };

        if let Some((upvalue_idx, upvalue_type)) = upvalue_idx {
            let upvalue_idx = self.add_upvalue(upvalue_idx, upvalue_type.clone(), false, span)?;
            return Ok(Some((upvalue_idx, upvalue_type)));
        };

        Ok(None)
    }

    fn add_upvalue(&mut self, idx: u8, type_: Type, is_local: bool, span: &Span) -> Result<u8> {
        let upvalue = Upvalue {
            index: idx,
            is_local,
            type_,
        };
        let upvalue_idx = match self.upvalues.iter().position(|u| u == &upvalue) {
            Some(upvalue_idx) => upvalue_idx,
            None => {
                self.upvalues
                    .try_push(upvalue)
                    .map_err(|_| (OverflowError::TooManyUpvalues.into(), span.clone()))?;
                let upvalues = self.upvalues.len();
                unsafe {
                    (*self.function).upvalue_count =
                        upvalues.try_into().expect("upvalue index overflow")
                };
                upvalues - 1
            }
        };

        Ok(upvalue_idx.try_into().expect("upvalue index overflow"))
    }
}

#[derive(Debug, Clone)]
pub struct StructCell {
    name: *mut StringObject,
    fields: Vec<Field>,
    methods: Vec<StructMethod>,
}

#[derive(Debug, Clone)]
pub struct StructMethod {
    name: String,
    return_type: Type,
}

pub struct Compiler {
    pub globals: HashMap<String, Type, BuildHasherDefault<FxHasher>>,
    pub current_compiler: CompilerCell,
    pub current_struct: Option<String>,
    pub structs: Vec<StructCell>,
}

impl Compiler {
    pub fn find_struct(&mut self, name: &str) -> Option<&mut StructCell> {
        self.structs
            .iter_mut()
            .find(|s| unsafe { (*s.name).value } == name)
    }

    pub fn new(allocator: &mut CeAllocation) -> Self {
        let name = allocator.alloc("");
        Self {
            current_compiler: CompilerCell {
                function: allocator.alloc(ObjectFunction::new(name, 0, None)),
                type_: FunctionType::Script,
                locals: ArrayVec::new(),
                scope_depth: 0,
                parent: None,
                upvalues: ArrayVec::new(),
            },
            globals: HashMap::default(),
            structs: Vec::new(),
            current_struct: None,
        }
    }

    pub fn compile(
        &mut self,
        source: &str,
        allocator: &mut CeAllocation,
        stdout: &mut StandardStream,
    ) -> Result<*mut ObjectFunction, Vec<ErrorS>> {
        let program = parse(source)?;

        for statement in &program.statements {
            let type_ = self.compile_statement(statement, allocator);
            if let Err(e) = type_ {
                return Err(vec![e]);
            }
        }

        self.emit_u8(op::NIL, &(0..0));
        self.emit_u8(op::RETURN, &(0..0));
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .disassemble("script")
        };
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));

        println!("|--------------Virtual Machine--------------|");
        //reset color
        stdout.reset();
        Ok(self.current_compiler.function)
    }

    fn compile_statement(
        &mut self,
        (statement, range): &(Statement, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        match statement {
            Statement::Expression(expr) => {
                let type_ = self.cp_expression(&expr.expr, allocator)?;
                self.emit_u8(op::POP, &range);
                Ok(type_)
            }
            Statement::Print(print) => self.print_stmt((print, range), allocator),
            Statement::PrintLn(print) => self.print_ln_stmt((print, range), allocator),
            Statement::Var(var) => self.compile_var_stmt((var, range), allocator),
            Statement::Block(block) => self.compile_block_stmt((block, range), allocator),
            Statement::If(if_) => self.compile_if_stmt((if_, range), allocator),
            Statement::While(while_) => self.compile_while_stmt((while_, range), allocator),
            Statement::For(for_) => self.compile_fun_stmt((for_, range), allocator),
            Statement::Fun(func) => {
                let func_type =
                    self.compile_statement_fun((func, range), FunctionType::Function, allocator)?;
                if self.is_global() {
                    let name = allocator.alloc(&func.name).into();
                    self.globals.insert(
                        func.name.clone(),
                        Type::Fn(Fn {
                            return_type: Box::new(func.return_type.clone()),
                        }),
                    );
                    self.emit_u8(op::DEFINE_GLOBAL, &range);
                    self.write_constant(name, &range);
                } else {
                    self.declare_local(
                        &func.name,
                        &Type::Fn(Fn {
                            return_type: Box::new(func.return_type.clone()),
                        }),
                        &range,
                    )?
                }
                return Ok(func_type);
            }
            Statement::Return(return_) => {
                self.compile_statement_return((return_, range), allocator)
            }
            Statement::Struct(struct_) => self.compile_struct_stmt((struct_, range), allocator),
            Statement::Impl(impl_) => self.compile_impl_stmt((impl_, range), allocator),
            Statement::Error => todo!(),
        }
    }

    fn compile_struct_stmt(
        &mut self,
        (struct_, _range): (&StatementStruct, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let name = allocator.alloc(&struct_.name);
        let fields = struct_.fields.clone();

        self.structs.push(StructCell {
            name,
            fields,
            methods: Vec::new(),
        });

        Ok(Type::Struct(struct_.name.clone()))
    }

    fn compile_impl_stmt(
        &mut self,
        (impl_, range): (&StatementImpl, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let has_super = impl_.super_.is_some();
        self.current_struct = Some(impl_.name.clone());

        let found_struct = self.find_struct(&impl_.name);

        if !found_struct.is_some() {
            let result = Err((
                Error::NameError(NameError::StructNameNotFound {
                    name: impl_.name.clone(),
                }),
                range.clone(),
            ));
            return result;
        }

        let fields = found_struct.as_ref().unwrap().fields.clone();
        let methods = impl_.methods.clone();

        let name = allocator.alloc(&impl_.name).into();
        self.emit_u8(op::STRUCT, &range);

        self.write_constant(name, &range);
        for field in &fields {
            let name = allocator.alloc(&field.name).into();
            self.emit_u8(op::FIELD, &range);
            self.write_constant(name, &range);
        }

        if self.is_global() {
            self.globals
                .insert(impl_.name.clone(), Type::Struct(impl_.name.clone()));
            self.emit_u8(op::DEFINE_GLOBAL, range);
            self.write_constant(name, range);
        } else {
            self.declare_local(&impl_.name, &Type::Struct(impl_.name.clone()), &range)?;
        }

        if !impl_.methods.is_empty() {
            self.get_variable(&impl_.name, &range, allocator)?;
            for (method, span) in &methods {
                let method = method.as_fun().unwrap();
                let type_ = if method.name == "new" {
                    FunctionType::Initializer
                } else {
                    FunctionType::Method
                };
                self.compile_statement_fun((&method, span), type_, allocator)?;

                let name = allocator.alloc(&method.name).into();
                self.emit_u8(op::METHOD, span);
                self.write_constant(name, span);

                let strct_method = StructMethod {
                    name: method.name.clone(),
                    return_type: Type::Fn(Fn {
                        return_type: Box::new(method.return_type.clone()),
                    }),
                };

                let found_struct = self.find_struct(&impl_.name).unwrap();
                found_struct.methods.push(strct_method);
            }
            self.emit_u8(op::POP, &range);
        }

        self.current_struct = None;

        Ok(Type::Struct(impl_.name.clone()))
    }

    fn compile_statement_return(
        &mut self,
        (return_, range): (&StatementReturn, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        match self.current_compiler.type_ {
            FunctionType::Script => {
                let result = Err((
                    Error::SyntaxError(SyntaxError::ReturnInInitializer),
                    range.clone(),
                ));
                return result;
            }
            FunctionType::Function | FunctionType::Method => {
                let func_return_type = unsafe { &(*self.current_compiler.function).return_type };
                match &return_.value {
                    Some(value) => {
                        let return_stmt_type = self.cp_expression(value, allocator)?;
                        match func_return_type {
                            Some(fn_return_type) => {
                                //if function that return function else check return type and
                                //function's return type
                                if return_stmt_type.is_both_fn(fn_return_type) {
                                    self.emit_u8(op::RETURN, &range);
                                    return Ok(return_stmt_type);
                                } else if &return_stmt_type != fn_return_type {
                                    let result = Err((
                                        Error::TypeError(TypeError::ReturnTypeMismatch {
                                            expected: fn_return_type.to_string(),
                                            actual: return_stmt_type.to_string(),
                                        }),
                                        range.clone(),
                                    ));
                                    return result;
                                }
                                self.emit_u8(op::RETURN, &range);
                                return Ok(return_stmt_type);
                            }
                            None => {
                                if return_stmt_type != Type::Nil {
                                    let result = Err((
                                        Error::TypeError(TypeError::ReturnTypeMustBeNil),
                                        range.clone(),
                                    ));
                                    return result;
                                }
                                self.emit_u8(op::RETURN, &range);
                                return Ok(return_stmt_type);
                            }
                        }
                    }
                    // when return_stmt_type is nil,
                    None => {
                        match func_return_type {
                            Some(fn_return_type) => {
                                let result = Err((
                                    Error::TypeError(TypeError::ReturnTypeMismatch {
                                        expected: fn_return_type.to_string(),
                                        actual: Type::Nil.to_string(),
                                    }),
                                    range.clone(),
                                ));
                                return result;
                            }
                            // both return_stmt_type and fn_return_type is nil
                            None => {
                                self.emit_u8(op::NIL, &range);
                                self.emit_u8(op::RETURN, &range);
                            }
                        }
                        return Ok(Type::Fn(Fn {
                            return_type: Box::new(Some(Type::Nil)),
                        }));
                    }
                }
            }
            FunctionType::Initializer => match &return_.value {
                Some(value) => {
                    let return_type = self.cp_expression(value, allocator)?;
                    if return_type != Type::Nil {
                        let result = Err((
                            Error::TypeError(TypeError::ReturnTypeMustBeNil),
                            range.clone(),
                        ));
                        return result;
                    }
                    self.emit_u8(op::GET_LOCAL, &range);
                    self.emit_u8(0, &range);
                    self.emit_u8(op::RETURN, &range);
                    return Ok(return_type);
                }
                None => {
                    self.emit_u8(op::GET_LOCAL, &range);
                    self.emit_u8(0, &range);
                    self.emit_u8(op::RETURN, &range);
                    return Ok(Type::UnInitialized);
                }
            },
        }
    }

    fn compile_statement_fun(
        &mut self,
        (func, range): (&StatementFun, &Range<usize>),
        type_: FunctionType,
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let name = allocator.alloc(&func.name);
        let arity_count = func.params.len() as u8;
        println!("COMPILE FUN: {:?}", arity_count);

        let cell = CompilerCell {
            function: allocator.alloc(ObjectFunction::new(
                name,
                arity_count,
                func.return_type.clone(),
            )),
            type_,
            locals: ArrayVec::new(),
            scope_depth: self.current_compiler.scope_depth + 1,
            parent: None,
            upvalues: ArrayVec::new(),
        };
        self.begin_cell(cell);

        // match type_ {
        //     FunctionType::Initializer | FunctionType::Method => {
        //         self.declare_local("self", &Type::Self_, &range)
        //     }
        //     FunctionType::Function | FunctionType::Script => self.declare_local(
        //         &func.name,
        //         &Type::Fn(Fn {
        //             return_type: Box::new(func.return_type.clone()),
        //         }),
        //         &range,
        //     ),
        // }?;
        //

        match type_ {
            FunctionType::Script => {
                self.globals.insert(
                    func.name.clone(),
                    Type::Fn(Fn {
                        return_type: Box::new(func.return_type.clone()),
                    }),
                );
            }
            FunctionType::Function => self.declare_local(
                &func.name,
                &Type::Fn(Fn {
                    return_type: Box::new(func.return_type.clone()),
                }),
                &range,
            )?,
            FunctionType::Initializer | FunctionType::Method => {
                let current_struct = self.current_struct.clone().unwrap();
                self.declare_local("self", &Type::Struct(current_struct), &range)?;
                // self.declare_local(
                //     &func.name,
                //     &Type::Fn(Fn {
                //         return_type: Box::new(func.return_type.clone()),
                //     }),
                //     &range,
                // )?;
            }
        }
        //TODO check param type
        for (param_string, param_type) in &func.params {
            match param_type {
                Some(t) => match t {
                    // TODO Add more type
                    Type::String | Type::Int => self.declare_local(&param_string, &t, &range)?,
                    Type::Struct(name) => {
                        let found_struct = self.find_struct(&name);
                        match found_struct {
                            Some(struct_) => {
                                self.declare_local(&param_string, &t, &range)?;
                            }
                            None => {
                                let result = Err((
                                    Error::NameError(NameError::StructNameNotFound {
                                        name: name.clone(),
                                    }),
                                    range.clone(),
                                ));
                                return result;
                            }
                        }
                    }
                    _ => todo!("type not implemented"),
                },
                //TODO identify param type
                None => self.declare_local(&param_string, &Type::UnInitialized, &range)?,
            }
            self.define_local();
        }

        for statement in &func.body.statements {
            self.compile_statement(&statement, allocator)?;
        }

        if unsafe { (*self.current_compiler.function).chunk.code.last() } != Some(&op::RETURN) {
            let stmt = (Statement::Return(StatementReturn { value: None }), (0..0));
            self.compile_statement(&stmt, allocator)?;
        }

        let (function, upvalues) = self.end_cell();
        unsafe { (*function).chunk.disassemble((*name).value) };
        let function = function.into();
        self.emit_u8(op::CLOSURE, &range);
        self.write_constant(function, &range);
        for upvalue in &upvalues {
            self.emit_u8(upvalue.is_local.into(), &range);
            self.emit_u8(upvalue.index, &range);
        }

        let return_type = match &func.return_type {
            Some(t) => t.clone(),
            None => Type::Nil,
        };
        return Ok(return_type);
    }

    fn begin_cell(&mut self, cell: CompilerCell) {
        let cell = mem::replace(&mut self.current_compiler, cell);
        self.current_compiler.parent = Some(Box::new(cell));
    }

    fn end_cell(&mut self) -> (*mut ObjectFunction, ArrayVec<Upvalue, 256>) {
        let parent = self
            .current_compiler
            .parent
            .take()
            .expect("Compiler has no parent");
        let cell = mem::replace(&mut self.current_compiler, *parent);
        (cell.function, cell.upvalues)
    }

    fn compile_fun_stmt(
        &mut self,
        (for_, range): (&StatementFor, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        self.begin_scope();
        let mut compiled_type = None;
        if let Some(init) = &for_.init {
            let compile_init = self.compile_statement(&init, allocator)?;
            compiled_type = Some(compile_init);
        }

        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let mut exit_jump = None;
        if let Some(cond) = &for_.cond {
            let cond_type = self.cp_expression(cond, allocator)?;
            if cond_type != Type::Bool {
                let result = Err((
                    Error::TypeError(TypeError::LoopMustBeBoolean),
                    range.clone(),
                ));
                return result;
            }
            exit_jump = Some(self.emit_jump(op::JUMP_IF_FALSE, &range));
            self.emit_u8(op::POP, &range);
        }

        self.compile_statement(&for_.body, allocator)?;
        if let Some(increment) = &for_.update {
            self.cp_expression(increment, allocator)?;
            self.emit_u8(op::POP, &range);
        }
        self.emit_loop(loop_start, &range)?;

        if let Some(exit_jump) = exit_jump {
            self.patch_jump(exit_jump, &range)?;
            self.emit_u8(op::POP, &range);
        }
        self.end_scope(range.clone());
        if let Some(compiled_type) = compiled_type {
            return Ok(compiled_type);
        }
        return Ok(Type::UnInitialized);
    }

    fn compile_while_stmt(
        &mut self,
        (while_, range): (&StatementWhile, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let loop_start = unsafe { (*self.current_compiler.function).chunk.code.len() };
        let cond_type = self.cp_expression(&while_.cond, allocator)?;
        if cond_type != Type::Bool {
            let result = Err((
                Error::TypeError(TypeError::LoopMustBeBoolean),
                range.clone(),
            ));
            return result;
        }
        let exit_jump = self.emit_jump(op::JUMP_IF_FALSE, &range);
        self.emit_u8(op::POP, &range);
        self.compile_statement(&while_.body, allocator)?;
        self.emit_loop(loop_start, &range)?;
        self.patch_jump(exit_jump, &range)?;
        self.emit_u8(op::POP, &range);
        return Ok(Type::UnInitialized);
    }

    fn emit_loop(&mut self, loop_start: usize, span: &Span) -> Result<()> {
        self.emit_u8(op::LOOP, span);
        let offset = unsafe { (*self.current_compiler.function).chunk.code.len() - loop_start + 2 };
        if offset > u16::MAX as usize {
            return Err((
                Error::OverflowError(OverflowError::LoopTooLarge),
                span.clone(),
            ));
        }
        self.emit_u8((offset >> 8) as u8, span);
        self.emit_u8(offset as u8, span);

        Ok(())
    }

    fn compile_if_stmt(
        &mut self,
        (if_, range): (&StatementIf, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let cond_type = self.cp_expression(&if_.cond, allocator)?;
        if cond_type != Type::Bool {
            let result = Err((
                Error::TypeError(TypeError::CondMustbeBoolean),
                range.clone(),
            ));
            return result;
        }
        // If the condition is false, go to ELSE.
        let jump_to_else = self.emit_jump(op::JUMP_IF_FALSE, &range);
        // Discard the condition.
        self.emit_u8(op::POP, &range);
        // Evaluate the if branch.
        self.compile_statement(&if_.then_branch, allocator)?;
        // Go to END.
        let jump_to_end = self.emit_jump(op::JUMP, &range);

        // ELSE:
        self.patch_jump(jump_to_else, &range)?;
        self.emit_u8(op::POP, &range); // Discard the condition.
        if let Some(else_branch) = &if_.else_branch {
            self.compile_statement(else_branch, allocator)?;
        }

        // END:
        self.patch_jump(jump_to_end, &range)?;
        return Ok(Type::UnInitialized);
    }

    fn compile_block_stmt(
        &mut self,
        (block, range): (&StatementBlock, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        self.begin_scope();
        for statement in &block.statements {
            self.compile_statement(&statement, allocator)?;
        }
        self.end_scope(range.clone());
        return Ok(Type::UnInitialized);
    }

    fn print_ln_stmt(
        &mut self,
        (print, range): (&StatementPrintLn, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let expr_type = self.cp_expression(&print.value, allocator)?;
        self.emit_u8(op::PRINT_LN, range);
        return Ok(expr_type);
    }

    fn print_stmt(
        &mut self,
        (print, range): (&StatementPrint, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let expr_type = self.cp_expression(&print.value, allocator)?;
        self.emit_u8(op::PRINT, range);
        Ok(expr_type)
    }

    fn cp_expression(
        &mut self,
        expr: &(Expression, Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let (expr, range) = expr;
        let expr_type = match expr {
            Expression::Assign(assign) => self.compile_assign((assign, range), allocator),
            Expression::Prefix(prefix) => self.compile_prefix((prefix, range), allocator),
            Expression::Infix(infix) => self.compile_infix((infix, range), allocator),
            Expression::Literal(value) => self.compile_literal((value, range), allocator),
            Expression::Var(var) => self.compile_expression_var((var, range), allocator),
            Expression::Call(call) => self.compile_call((call, range), allocator),
            Expression::Get(get) => self.compile_get((get, range), allocator),
            Expression::Set(set) => self.compile_set((set, range), allocator),
        }?;
        Ok(expr_type)
    }

    fn find_struct_field_or_method_type(
        &mut self,
        struct_name: &Type,
        check_field: String,
        range: &Span,
    ) -> Result<Type> {
        let result_type = match struct_name {
            Type::Struct(strct) => {
                let found_struct = self.find_struct(&strct);
                if !found_struct.is_some() {
                    let result = Err((
                        Error::NameError(NameError::StructNameNotFound {
                            name: strct.clone(),
                        }),
                        range.clone(),
                    ));
                    return result;
                }
                let found_struct = found_struct.unwrap();
                let field = found_struct
                    .fields
                    .iter()
                    .find(|f| (*f.name) == check_field);
                let strct_type = match field {
                    Some(field) => field.type_.clone(),
                    None => {
                        let method = found_struct.methods.iter().find(|m| m.name == check_field);
                        let method_type = match method {
                            Some(method) => method.return_type.clone(),
                            None => {
                                let result = Err((
                                    Error::NameError(NameError::StructMethodNotFound {
                                        name: check_field.clone(),
                                        struct_name: strct.clone(),
                                    }),
                                    range.clone(),
                                ));
                                return result;
                            }
                        };
                        method_type
                    }
                };
                strct_type
            }
            _ => todo!("Only struct can use get"),
        };
        return Ok(result_type);
    }

    fn compile_set(
        &mut self,
        (set, range): (&Box<ExprSet>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let value_type = self.cp_expression(&set.value, allocator)?;
        let object_type = self.cp_expression(&set.object, allocator)?;

        let field_type =
            self.find_struct_field_or_method_type(&object_type, set.name.clone(), range)?;

        if field_type != value_type {
            let result = Err((
                Error::TypeError(TypeError::SetTypeMisMatch {
                    expected: object_type.to_string(),
                    actual: value_type.to_string(),
                }),
                range.clone(),
            ));
            return result;
        }
        let name = allocator.alloc(&set.name);
        self.emit_u8(op::SET_FIELD, &range);
        self.write_constant(name.into(), &range);
        Ok(field_type)
    }

    fn compile_get(
        &mut self,
        (get, range): (&Box<ExprGet>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let var_type = self.cp_expression(&get.object, allocator)?;

        let field_type =
            self.find_struct_field_or_method_type(&var_type, get.name.clone(), range)?;
        println!("GET OBJECT TYPE: {:?}", field_type);

        let name = allocator.alloc(&get.name);
        self.emit_u8(op::GET_FIELD, &range);
        self.emit_constant_w(name.into(), &range)?;
        return Ok(field_type);
    }

    fn compile_call(
        &mut self,
        (call, range): (&Box<ExprCall>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let arg_count = call.args.len();
        if arg_count > u8::MAX as usize {
            Err((
                Error::OverflowError(OverflowError::TooManyArguments),
                range.clone(),
            ))?;
        }
        let callee_type = self.cp_expression(&call.callee, allocator)?;
        let callee_type = if callee_type.is_fn() {
            let mut callee_type = callee_type.as_fn().unwrap();
            let callee_type = callee_type.return_type.as_mut().clone();
            let callee_type = match callee_type {
                Some(type_) => type_,
                None => Type::Nil,
            };
            callee_type
        } else {
            callee_type
        };

        println!("CALLEE TYPE: {:?}", callee_type);
        // if callee_type != Type::Object {
        //     todo!("Can only call functions and classes");
        // }
        for arg in &call.args {
            self.cp_expression(&arg, allocator)?;
        }

        let ops = unsafe { &mut (*self.current_compiler.function).chunk.code };

        match ops.len().checked_sub(2) {
            Some(idx) if ops[idx] == op::GET_FIELD => ops[idx] = op::INVOKE,
            // Some(idx) if ops[idx] == op::GET_SUPER => ops[idx] = op::SUPER_INVOKE,
            Some(_) | None => self.emit_u8(op::CALL, &range),
        }
        self.emit_u8(arg_count as u8, &range);
        Ok(callee_type)
    }

    fn compile_assign(
        &mut self,
        (assign, range): (&Box<ExprAssign>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let var_type = self.cp_expression(&assign.rhs, allocator)?;
        if let Ok(Some((local_index, _local_type))) =
            self.current_compiler
                .resolve_local(&assign.lhs.name, false, &range)
        {
            self.emit_u8(op::SET_LOCAL, &range);
            self.emit_u8(local_index, &range);
        } else if let Ok(Some((upvalue, _upvalue_type))) = self
            .current_compiler
            .resolve_upvalue(&assign.lhs.name, &range)
        {
            self.emit_u8(op::SET_UPVALUE, &range);
            self.emit_u8(upvalue, &range);
        } else {
            let name = allocator.alloc(&assign.lhs.name);
            self.emit_u8(op::SET_GLOBAL, &range);
            self.write_constant(name.into(), &range);
        }
        return Ok(var_type);
    }

    fn compile_var_stmt(
        &mut self,
        (var, range): (&StatementVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        // Variable declaration left hand side expression, if it's not have expression variable
        // type is UnInitialized
        let value_var_type = var
            .value
            .as_ref()
            .map_or(Ok(Type::UnInitialized), |value| {
                let val_type = self.cp_expression(value, allocator)?;
                Ok(val_type)
            })?;

        if var.var.type_.is_some() {
            let var_type = var.var.type_.as_ref().unwrap();
            match var_type {
                Type::String | Type::Int | Type::Nil | Type::Bool => {}
                Type::Struct(strct) => {
                    let found_struct = self.find_struct(strct);
                    if !found_struct.is_some() {
                        let result = Err((
                            Error::NameError(NameError::StructNameNotFound {
                                name: strct.clone(),
                            }),
                            range.clone(),
                        ));
                        return result;
                    }
                }
                _ => todo!("type not implemented"),
            }
            println!("var type: {:?}", var_type);
            if var_type != &value_var_type {
                let result = Err((
                    Error::TypeError(TypeError::VariableTypeMismatch {
                        expected: var_type.to_string(),
                        actual: value_var_type.to_string(),
                    }),
                    range.clone(),
                ));
                return result;
            }
        }

        //Check if variable type is declared
        match &var.var.type_ {
            Some(type_) => match type_ {
                Type::String | Type::Int | Type::Nil | Type::Bool => {
                    let name = &var.var.name;
                    if self.is_global() {
                        let string = allocator.alloc(name);
                        self.globals.insert(name.clone(), type_.clone());
                        self.emit_u8(op::DEFINE_GLOBAL, &range);
                        self.write_constant(string.into(), &range);
                        // return Ok(Type::String);
                    } else {
                        self.declare_local(name, &value_var_type, &range)?;
                        self.define_local();
                    }
                }
                Type::Struct(strct) => {
                    let found_struct = self.find_struct(strct);
                    if !found_struct.is_some() {
                        let result = Err((
                            Error::NameError(NameError::StructNameNotFound {
                                name: strct.clone(),
                            }),
                            range.clone(),
                        ));
                        return result;
                    }
                    let name = &var.var.name;
                    if self.is_global() {
                        let string = allocator.alloc(name);
                        self.globals.insert(name.clone(), type_.clone());
                        self.emit_u8(op::DEFINE_GLOBAL, &range);
                        self.write_constant(string.into(), &range);
                        // return Ok(Type::String);
                    } else {
                        self.declare_local(name, &value_var_type, &range)?;
                        self.define_local();
                    }
                }
                _ => todo!("type not implemented"),
            },
            None => {
                let name = &var.var.name;
                let string = allocator.alloc(name);
                //If variable type is not declared, left hand side expression type will be variable type
                if self.is_global() {
                    self.globals.insert(name.clone(), value_var_type.clone());
                    self.emit_u8(op::DEFINE_GLOBAL, &range);
                    self.write_constant(string.into(), &range);
                } else {
                    //If variable type is not declared, left hand side expression type will be variable type
                    self.declare_local(name, &value_var_type, &range)?;
                    self.define_local();
                }
            }
        }
        Ok(value_var_type)
    }

    fn get_variable(&mut self, name: &str, span: &Span, gc: &mut CeAllocation) -> Result<Type> {
        if name == "self" && self.current_struct.is_none() {
            return Err((SyntaxError::SelfOutsideClass.into(), span.clone()));
        }
        if let Some((local_idx, local_type)) =
            self.current_compiler.resolve_local(name, false, span)?
        {
            self.emit_u8(op::GET_LOCAL, span);
            self.emit_u8(local_idx, span);
            return Ok(local_type);
        } else if let Some((upvalue_idx, upvalue_type)) =
            self.current_compiler.resolve_upvalue(name, span)?
        {
            self.emit_u8(op::GET_UPVALUE, span);
            self.emit_u8(upvalue_idx, span);
            return Ok(upvalue_type);
        } else {
            let allocated_name = gc.alloc(name);
            self.emit_u8(op::GET_GLOBAL, span);
            self.write_constant(allocated_name.into(), span);
            let type_ = self.globals.get(&name.to_string());

            let expr_var_type = match type_ {
                Some(type_) => Ok(type_.clone()),
                None => Err((
                    Error::NameError(NameError::IdentifierNotDefined { name: name.into() }),
                    span.clone(),
                )),
            };
            return expr_var_type;
        }
    }

    fn compile_expression_var(
        &mut self,
        (prefix, range): (&ExprVar, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let name = &prefix.var.name;
        // if self.current_compiler.type_ == FunctionType::Function {
        //     let function_ = unsafe { &mut (*self.current_compiler.function) };
        //     let arity = function_.arity.as_ref().unwrap();
        //     for param in arity {
        //         if unsafe { (*param.name).value } == name {
        //             return param.type_.clone();
        //         }
        //     }
        // }
        self.get_variable(name, &range, allocator)
    }

    fn compile_prefix(
        &mut self,
        prefix: (&Box<ExprPrefix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let (prefix, range) = prefix;
        let rt_type = self.cp_expression(&(prefix.rt), allocator)?;
        if rt_type != Type::Int || rt_type != Type::Bool {
            let spanned_error = Err((
                Error::TypeError(TypeError::UnsupportedOperandPrefix {
                    op: prefix.op.to_string(),
                    rt_type: rt_type.to_string(),
                }),
                range.clone(),
            ));
            return spanned_error;
        }
        match prefix.op {
            OpPrefix::Negate => self.emit_u8(op::NEG, &range),
            OpPrefix::Not => self.emit_u8(op::NOT, &range),
        }
        return Ok(rt_type);
    }

    fn compile_infix(
        &mut self,
        infix: (&Box<ExprInfix>, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        let (infix, range) = infix;
        let lhs_type = self.cp_expression(&(infix.lhs), allocator)?;
        println!("LHS TYPE: {:?}", lhs_type);
        let rhs_type = self.cp_expression(&(infix.rhs), allocator)?;
        println!("RHS TYPE: {:?}", rhs_type);

        let return_type = lhs_type.clone();

        // if lhs_type != rhs_type {
        //     todo!("type mismatch");
        // }

        match infix.op {
            OpInfix::Modulo => {
                self.emit_u8(op::MODULO, &range);
                return Ok(return_type);
            }
            OpInfix::Add => {
                if lhs_type == Type::String && rhs_type == Type::String {
                    self.emit_u8(op::CONCAT, &range);
                    return Ok(Type::String);
                } else {
                    self.emit_u8(op::ADD, &range);
                }
                return Ok(return_type);
            }
            OpInfix::Sub => {
                self.emit_u8(op::SUB, &range);
                return Ok(return_type);
            }
            OpInfix::Mul => {
                self.emit_u8(op::MUL, &range);
                return Ok(return_type);
            }
            OpInfix::Div => {
                self.emit_u8(op::DIV, &range);
                return Ok(return_type);
            }
            OpInfix::Equal => {
                self.emit_u8(op::EQUAL, &range);
                return Ok(Type::Bool);
            }
            OpInfix::NotEqual => {
                self.emit_u8(op::NOT_EQUAL, &range);
                return Ok(Type::Bool);
            }
            OpInfix::Greater => {
                self.emit_u8(op::GREATER_THAN, &range);
                return Ok(Type::Bool);
            }
            OpInfix::GreaterEqual => {
                self.emit_u8(op::GREATER_THAN_EQUAL, &range);
                return Ok(Type::Bool);
            }
            OpInfix::Less => {
                self.emit_u8(op::LESS_THAN, &range);
                return Ok(Type::Bool);
            }
            OpInfix::LessEqual => {
                self.emit_u8(op::LESS_THAN_EQUAL, &range);
                return Ok(Type::Bool);
            }
            OpInfix::LogicOr => {
                self.emit_u8(op::OR, &range);
                return Ok(Type::Bool);
            }
            OpInfix::LogicAnd => {
                self.emit_u8(op::AND, &range);
                return Ok(Type::Bool);
            }
        }
    }

    fn compile_literal(
        &mut self,
        (literal, range): (&ExprLiteral, &Range<usize>),
        allocator: &mut CeAllocation,
    ) -> Result<Type> {
        match literal {
            ExprLiteral::Number(value) => {
                let value = (*value).into();
                self.emit_constant(value, &range);
                Ok(Type::Int)
            }
            ExprLiteral::String(string) => {
                let string = allocator.alloc(string);
                unsafe { (*string).main.is_marked = true };
                self.emit_constant(string.into(), &range);
                Ok(Type::String)
            }
            ExprLiteral::Bool(value) => {
                match value {
                    true => self.emit_u8(op::TRUE, &range),
                    false => self.emit_u8(op::FALSE, &range),
                };
                Ok(Type::Bool)
            }
            ExprLiteral::Nil => {
                self.emit_u8(op::NIL, &range);
                Ok(Type::Nil)
            }
        }
    }

    fn begin_scope(&mut self) {
        self.current_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self, span: Range<usize>) {
        self.current_compiler.scope_depth -= 1;

        // Remove all locals that are no longer in scope.
        while let Some(local) = self.current_compiler.locals.last() {
            if local.depth > self.current_compiler.scope_depth {
                if local.is_captured {
                    self.emit_u8(op::CLOSE_UPVALUE, &span);
                } else {
                    self.emit_u8(op::POP, &span);
                }
                self.current_compiler.locals.pop();
            } else {
                break;
            }
        }
    }

    fn declare_local(&mut self, name: &str, type_: &Type, span: &Span) -> Result<()> {
        let is_initialized = match type_ {
            Type::UnInitialized => false,
            _ => true,
        };
        for local in self.current_compiler.locals.iter().rev() {
            if local.depth != self.current_compiler.scope_depth {
                break;
            }

            if local.name == name {
                return Err((
                    Error::SyntaxError(SyntaxError::AlreadyDeclared {
                        name: name.to_string(),
                    }),
                    span.clone(),
                ));
            }
        }
        let local = Local {
            name: name.to_string(),
            type_: type_.clone(),
            depth: self.current_compiler.scope_depth,
            is_initialized,
            is_captured: false,
        };
        self.current_compiler.locals.push(local);
        Ok(())
    }

    fn define_local(&mut self) {
        let local = self
            .current_compiler
            .locals
            .last_mut()
            .expect("Define local called without locals");
        local.is_initialized = true;
    }

    fn is_global(&self) -> bool {
        self.current_compiler.scope_depth == 0
    }

    fn emit_jump(&mut self, instruction: u8, span: &Span) -> usize {
        self.emit_u8(instruction, span);
        self.emit_u8(0xff, span);
        self.emit_u8(0xff, span);
        unsafe { (*self.current_compiler.function).chunk.code.len() - 2 }
    }

    fn patch_jump(&mut self, offset_idx: usize, span: &Span) -> Result<()> {
        let offset = unsafe { (*self.current_compiler.function).chunk.code.len() - offset_idx - 2 };
        if offset > u16::MAX as usize {
            Err((
                Error::OverflowError(OverflowError::TooManyArguments),
                span.clone(),
            ))?;
        }
        unsafe {
            (*self.current_compiler.function).chunk.code[offset_idx] = (offset >> 8) as u8;
            (*self.current_compiler.function).chunk.code[offset_idx + 1] = offset as u8;
        }

        Ok(())
    }

    fn emit_constant_w(&mut self, value: Value, span: &Span) -> Result<()> {
        let constant_idx = unsafe {
            (*self.current_compiler.function)
                .chunk
                .write_constant_w(value, span)?
        };
        self.emit_u8(constant_idx, span);
        Ok(())
    }

    fn emit_u8(&mut self, byte: u8, span: &Span) {
        unsafe {
            (*self.current_compiler.function).chunk.write_u8(byte, span);
        }
    }

    fn emit_constant(&mut self, value: Value, span: &Span) {
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .emit_constant(value, span);
        }
    }

    fn write_constant(&mut self, value: Value, span: &Span) {
        unsafe {
            (*self.current_compiler.function)
                .chunk
                .write_constant(value, span);
        }
    }
}
