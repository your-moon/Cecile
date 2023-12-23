#![cfg(feature = "repl")]

use std::io::{self};

use anyhow::{Context, Result};
use reedline::{
    EditCommand, Emacs, FileBackedHistory, KeyCode, KeyModifiers, PromptEditMode,
    PromptHistorySearch, Reedline, ReedlineEvent, ValidationResult,
};
use std::borrow::Cow;

use crate::{
    allocator::allocation::CeAllocation,
    vm::{compiler::Compiler, error::report_errors, VM},
};

pub fn run(trace: bool, debug: bool, optimized: bool) -> Result<()> {
    let mut color = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);
    let mut editor = editor().context("could not start REPL")?;

    let mut allocator = CeAllocation::new();
    let mut compiler = Compiler::new(&mut allocator, debug);
    let mut vm = VM::new(&mut allocator, trace, optimized);

    let stdout = &mut io::stdout().lock();
    let stderr = &mut io::stderr().lock();

    loop {
        let line = editor.read_line(&Prompt);
        editor
            .sync_history()
            .context("could not sync history file")?;

        match line {
            Ok(reedline::Signal::Success(line)) => {
                if let Err(errors) = vm.run(&line, &mut color, false, stdout, &mut compiler) {
                    report_errors(stderr, &vm.source, &errors)
                }
            }
            Ok(reedline::Signal::CtrlC) => eprintln!("^C"),
            Ok(reedline::Signal::CtrlD) => break,
            Err(e) => {
                eprintln!("error: {e:?}");
                break;
            }
        }
    }

    Ok(())
}

fn editor() -> Result<Reedline> {
    let mut keybindings = reedline::default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );
    let edit_mode = Box::new(Emacs::new(keybindings));

    // let highlighter = Box::new(Highlighter::new()?);

    let data_dir = dirs::data_dir().context("could not find data directory")?;
    let history_path = data_dir.join("loxcraft/history.txt");
    let history = Box::new(
        FileBackedHistory::with_file(10000, history_path.clone())
            .with_context(|| format!("could not open history file: {}", history_path.display()))?,
    );

    let validator = Box::new(Validator);

    let editor = Reedline::create()
        .with_edit_mode(edit_mode)
        // .with_highlighter(highlighter)
        .with_history(history)
        .with_validator(validator);
    Ok(editor)
}

#[derive(Debug)]
struct Validator;

impl reedline::Validator for Validator {
    fn validate(&self, line: &str) -> ValidationResult {
        if crate::cc_parser::is_complete(line) {
            ValidationResult::Complete
        } else {
            ValidationResult::Incomplete
        }
    }
}

#[derive(Debug)]
pub struct Prompt;

impl reedline::Prompt for Prompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed(">>> ")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<str> {
        Cow::Borrowed("")
    }
}
