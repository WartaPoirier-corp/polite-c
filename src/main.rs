mod cfg;
mod cli;
mod diagnostics;

#[cfg(feature = "dot")]
use crate::cfg::CFG;
#[cfg(feature = "dot")]
use crate::cli::Args;
use clang::documentation::{CommentChild, ParamCommand};
use clang::{Clang, EntityKind, EntityVisitResult, Index};
#[cfg(feature = "dot")]
use std::convert::TryFrom;
use std::ops::Range;

fn make_ascii_title_case(s: &mut str) {
    if let Some(s) = s.get_mut(0..1) {
        s.make_ascii_uppercase();
    }
}

// FIXME completely temporary
fn print_rusty_error<S: AsRef<str>>(
    (file, lines, cols, msg, note): (String, Range<u32>, Range<u32>, S, Option<String>),
) {
    let source = std::fs::read_to_string(&file).unwrap();

    assert_eq!(lines.len(), 0);

    println!(
        "error: {}\n  --> {}:{}:{}",
        msg.as_ref(),
        file,
        lines.start,
        cols.start
    );

    let line = source.lines().nth((lines.start - 1) as _).unwrap();
    let line_name = lines.start.to_string();
    let padding = " ".repeat(line_name.len());
    let mut arrows = " ".repeat((cols.start - 1) as _);
    arrows.push_str(&"^".repeat(cols.len()));

    println!(
        "{padding} |\n{ln} | {line}\n{padding} | {arrows}",
        line = line,
        padding = padding,
        ln = line_name,
        arrows = arrows,
    );

    if let Some(note) = note {
        println!(
            "{padding} |\n{padding} = {note}",
            padding = padding,
            note = note,
        );
    }
}

#[cfg(feature = "dot")]
fn display_dot(graph: CFG, viewer: impl AsRef<str>) {
    use std::process::*;

    let mut viewer = Command::new(viewer.as_ref())
        .stdin(Stdio::piped())
        .arg("/dev/stdin")
        .spawn()
        .expect("failed to start viewer");

    let stdin = viewer.stdin.as_mut().unwrap();
    graph.write_dot(stdin).unwrap();
}

fn main() {
    #![allow(clippy::expect_fun_call)]

    #[cfg(feature = "dot")]
    let args = Args::parse();

    println!("polite-c");
    println!(" * Preparing digital transmission");

    // Concurrency guard
    let clang = Clang::new().unwrap();

    let index = Index::new(&clang, false, false);
    let tu = index.parser("test_c_project/example.c").parse().unwrap();

    println!(" * Initiate starter process");

    // FIXME completely temporary structure
    let mut errors = Vec::new();
    // let mut tags = Vec::new();

    #[cfg(feature = "dot")]
    if let Some(analyse) = args.analyse {
        let viewer = args
            .dot_viewer
            .expect("no --dot-viewer specified when launched in analysis mode");

        let tu_ = index.parser(&analyse.file).parse().unwrap();
        let tu = tu_.get_entity();

        let function = analyse
            //.find(&index, EntityKind::FunctionDecl)
            .find(tu, EntityKind::FunctionDecl)
            .expect(&format!("{} yielded no match ;(", analyse));

        let cfg = CFG::try_from(function).unwrap();
        display_dot(cfg, viewer);
    }

    println!(" * GO !\n");

    tu.get_entity().visit_children(|e, _| {
        if let Some(comment) = e.get_parsed_comment() {
            for child in comment.get_children() {
                if let CommentChild::ParamCommand(ParamCommand { parameter, .. }) = child {
                    println!("{}", parameter);
                }
            }
        }

        if let EntityKind::FunctionDecl = e.get_kind() {
            if let Some(arg) = e
                .get_arguments()
                .unwrap_or_default()
                .iter()
                .find(|arg| arg.get_name() == Some("self".to_string()))
            {
                let loc = arg.get_range().expect("where tf is the function ?");

                let (file, start_line, start_col) = loc.get_start().get_presumed_location();
                let (_, end_line, end_col) = loc.get_end().get_presumed_location();

                let mut typ = arg.get_type().unwrap().get_display_name();
                make_ascii_title_case(&mut typ);

                errors.push((
                    file,
                    start_line..end_line,
                    start_col..end_col,
                    "using Rust naming conventions in C is absolutely illegal",
                    Some(format!(
                        "note: try renaming `self` to `p_my{}` instead",
                        typ,
                    )),
                ))
            }
        }

        EntityVisitResult::Recurse
    });

    for error in errors {
        print_rusty_error(error);
    }
}
