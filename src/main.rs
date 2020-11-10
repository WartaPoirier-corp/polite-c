use clang::{Clang, EntityKind, EntityVisitResult, Index};
use clap::Clap;
use std::ops::Range;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!(",\n"))]
struct Args {
    /// C code entry point or Makefile
    #[clap(long, short)]
    entry: Option<PathBuf>,

    /// Config file path or directory containing a `polite-c.toml`
    #[clap(long, short)]
    config: Option<PathBuf>,
}

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

    assert_eq!(lines.len(), 1);

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

fn main() {
    let args = Args::parse();

    println!("polite-c");
    println!(" * Preparing digital transmission");

    // Concurrency guard
    let clang = Clang::new().unwrap();

    let index = Index::new(&clang, false, false);
    let tu = index.parser("example.c").parse().unwrap();

    println!(" * Initiate starter process");

    // FIXME completely temporary structure
    let mut errors = Vec::new();

    println!(" * GO !\n");

    tu.get_entity().visit_children(|e, _| {
        match e.get_kind() {
            EntityKind::FunctionDecl => {
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
                        start_line..(end_line + 1),
                        start_col..end_col,
                        "using Rust naming conventions in C is absolutely illegal",
                        Some(format!(
                            "note: try renaming `self` to `p_my{}` instead",
                            typ,
                        )),
                    ))
                }
            }
            _ => (),
        }

        EntityVisitResult::Recurse
    });

    for error in errors {
        print_rusty_error(error);
    }
}
