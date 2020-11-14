use clap::Clap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum CFileLocationPos {
    Absolute { line: u32, column: Option<u32> },
    Symbol { name: Box<str>, index: u32 },
}

impl std::fmt::Display for CFileLocationPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Absolute { line, column: None } => write!(f, ":{}", line),
            Self::Absolute {
                line,
                column: Some(column),
            } => write!(f, ":{}:{}", line, column),
            Self::Symbol { name, index } if *index == 0 => write!(f, "#{}", name),
            Self::Symbol { name, index } => write!(f, "#{}#{}", name, index),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CFileLocation {
    pub file: PathBuf,
    pub pos: CFileLocationPos,
}

impl CFileLocation {
    pub fn find<'tu>(
        &self,
        tu: clang::Entity<'tu>, // index: &'tu clang::Index<'tu>,
        symbol: clang::EntityKind,
    ) -> Option<clang::Entity<'tu>> {
        use clang::*;

        // let tu = index.parser(&self.file).parse().unwrap().get_entity(); // TODO don't unwrap

        let mut ret = None;

        match &self.pos {
            CFileLocationPos::Absolute { line, column } => {
                tu.visit_children(|c, _| {
                    if c.get_kind() != symbol {
                        return EntityVisitResult::Recurse;
                    }

                    let (_, c_line, c_column) = c.get_location().unwrap().get_presumed_location();

                    if *line == c_line {
                        if let Some(column) = column {
                            if *column == c_column {
                                ret = Some(c);
                                EntityVisitResult::Break
                            } else {
                                EntityVisitResult::Recurse
                            }
                        } else {
                            ret = Some(c);
                            EntityVisitResult::Break
                        }
                    } else {
                        EntityVisitResult::Recurse
                    }
                });
            }
            CFileLocationPos::Symbol { name, index } => {
                let name = Some(name.to_string());

                ret = tu
                    .get_children()
                    .into_iter()
                    .filter(|c| c.get_name() == name)
                    .nth(*index as _);
            }
        }

        ret
    }
}

impl std::fmt::Display for CFileLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.to_string_lossy(), self.pos)
    }
}

#[derive(Clone, Debug)]
pub enum CFileLocationParseError {
    InvalidFormat,
    InvalidNumber,
}

impl std::fmt::Display for CFileLocationParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidFormat => "Invalid file location format. Refer to help for examples.",
            Self::InvalidNumber => "Couldn't parse what we expected to be a number",
        })
    }
}

impl FromStr for CFileLocation {
    type Err = CFileLocationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_u32<'a>(
            mut iter: impl Iterator<Item = &'a str>,
        ) -> Result<Option<u32>, CFileLocationParseError> {
            let next = iter.next();

            Ok(if let Some(next) = next {
                Some(
                    next.parse()
                        .map_err(|_| CFileLocationParseError::InvalidNumber)?,
                )
            } else {
                None
            })
        }

        let mut parts = s.split('#').collect::<Vec<_>>().into_iter();

        if parts.len() >= 2 {
            return Ok(CFileLocation {
                file: parts.next().unwrap().into(),
                pos: CFileLocationPos::Symbol {
                    name: String::from(parts.next().unwrap()).into_boxed_str(),
                    index: parse_u32(&mut parts)?.unwrap_or_default(),
                },
            });
        }

        let mut parts = s.split(':').collect::<Vec<_>>().into_iter();

        if parts.len() >= 2 {
            return Ok(CFileLocation {
                file: parts.next().unwrap().into(),
                pos: CFileLocationPos::Absolute {
                    line: parse_u32(&mut parts)?.unwrap_or_default(),
                    column: parse_u32(&mut parts)?,
                },
            });
        }

        return Err(CFileLocationParseError::InvalidFormat);
    }
}

lazy_static::lazy_static! {
    static ref VERSION_WITH_CLANG: &'static str = Box::leak(format!(
        "{}, powered by {}",
        clap::crate_version!(),
        clang::get_version(),
    ).into_boxed_str());
}

#[derive(Clap, Debug)]
#[clap(version = *VERSION_WITH_CLANG, author = clap::crate_authors!(",\n"))]
pub struct Args {
    /// C code entry point or Makefile
    #[clap(long, short)]
    pub entry: Option<PathBuf>,

    /// Config file path or directory containing a `polite-c.toml`
    #[clap(long, short)]
    pub config: Option<PathBuf>,

    #[cfg(feature = "dot")]
    /// Program used to display dot graphs, spawned with `/dev/stdin` argument
    #[clap(long)]
    pub dot_viewer: Option<String>,

    #[cfg(feature = "dot")]
    /// Analyse the given function, and display its _Control Flow Graph_. Syntax:
    ///   * `file.c#function`
    ///   * `file.c#function#2` if there are more than one signature
    ///   * `file.c:42`
    ///   * `file.c:42:10`
    #[clap(long)]
    pub analyse: Option<CFileLocation>,
}

impl Args {
    pub fn parse() -> Self {
        Clap::parse()
    }
}
