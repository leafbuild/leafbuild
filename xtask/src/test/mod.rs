use itertools::Itertools;
use path_calculate::Calculate;
use proc_macro2::{Ident, Span};
use quote::__private::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::{io, mem};

use crate::format::format_file;
use crate::{workspace_root, ws_path, ws_path_str};
use cmd::cargo_cmd;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TestKind {
    ShouldParse,
    ShouldError,
}

pub struct ParseTest {
    kind: TestKind,
    name: String,
    input: String,
    file: PathBuf,
    line: u32,
}

pub fn generate_parser_tests() -> anyhow::Result<()> {
    let blocks = line_comment_blocks(
        workspace_root(),
        "crates/leafbuild-syntax/src/parser/",
        true,
    )?;
    let mut tests = vec![];

    for line_comment_block in blocks {
        if !line_comment_block.lines[0].starts_with("test ") {
            continue;
        }

        let mut s = &line_comment_block.lines[0]["test ".len()..];
        let mut kind = TestKind::ShouldParse;

        if s.starts_with("err ") {
            kind = TestKind::ShouldError;
            s = &s["err ".len()..];
        }

        let name = s.to_string();

        let input = line_comment_block.lines[1..].join("\n");
        tests.push(ParseTest {
            name,
            kind,
            input,
            file: line_comment_block.path,
            line: line_comment_block.first_line.try_into().unwrap(),
        })
    }

    install_tests(tests)?;

    Ok(())
}

pub fn install_tests(tests: Vec<ParseTest>) -> anyhow::Result<()> {
    struct ParseTestVec(Vec<ParseTest>);

    struct Tests {
        passing_tests: ParseTestVec,
        error_tests: ParseTestVec,
    }

    impl ToTokens for ParseTest {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let name = Ident::new(self.name.as_str(), Span::call_site());
            let code: TokenStream = raw_str_literal(self.input.as_str(), 8);
            let file = self.file.as_path().to_str().unwrap();
            let line = self.line;
            tokens.append_all(quote::quote! {
                parse_test_full!(#name, #code, #file, #line);
            })
        }
    }
    impl ToTokens for ParseTestVec {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.0.iter().map(|it| {
                quote::quote! {
                    #it
                }
            }));
        }
    }

    impl ToTokens for Tests {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Tests {
                passing_tests,
                error_tests,
            } = self;

            let parser_tests_snap_path =
                ws_path_str!("crates" / "leafbuild-syntax" / "parser_tests_snaps");

            tokens.append_all(quote::quote! {
                mod parse_error {
                    use crate::parser::parse;

                    use unindent::Unindent;

                    macro_rules! parse_test {
                        ($name:ident, $s:expr) => {
                            let p = parse($s);
                            assert_ne!(p.errors, vec![]);
                        };
                    }

                    macro_rules! parse_test_full {
                        ($name:ident, $s:expr, $file:expr, $line:expr) => {
                            #[test]
                            fn $name() {
                                parse_test!($name, $s);
                            }
                        };
                    }

                    #error_tests
                }

                #[allow(clippy::unseparated_literal_suffix)]
                mod parse_output {
                    use crate::parser::parse;
                    use crate::syn_tree::{self, AstNode, CastableFromSyntaxNode, Root, SyntaxElement};
                    use crate::LeafbuildLanguage;

                    use unindent::Unindent;

                    macro_rules! parse_test {
                        ($name:ident, $s:expr, $file:expr, $line:expr) => {
                            let mut s = String::new();
                            let p = parse($s);
                            assert_eq!(p.errors, vec![]);
                            let node = rowan::SyntaxNode::<LeafbuildLanguage>::new_root(p.green_node);
                            let node: Root = Root::cast(node).unwrap();
                            syn_tree::test_dbg(0, SyntaxElement::Node(node.syntax().clone()), &mut s);

                            let mut settings = insta::Settings::clone_current();
                            settings.set_snapshot_path(#parser_tests_snap_path);
                            settings.bind(|| {
                                insta::_macro_support::assert_snapshot(
                                    stringify!($name).into(),
                                    s.as_str(),
                                    env!("CARGO_MANIFEST_DIR"),
                                    module_path!(),
                                    $file,
                                    $line,
                                    $s,
                                )
                                .unwrap();
                            });
                        };
                    }

                    macro_rules! parse_test_full {
                        ($name:ident, $s:expr) => {
                            parse_test_full!($name, $s, file!(), line!());
                        };
                        ($name:ident, $s:expr, $file:expr, $line:expr) => {
                            #[test]
                            fn $name() {
                                parse_test!($name, $s, $file, $line);
                            }
                        };
                    }

                    #passing_tests
                }
            });
        }
    }

    let (passing_tests, error_tests) = tests
        .into_iter()
        .partition(|it| it.kind == TestKind::ShouldParse);

    let tests = Tests {
        passing_tests: ParseTestVec(passing_tests),
        error_tests: ParseTestVec(error_tests),
    };

    let s = (quote::quote! {#tests}).to_string();

    let tests_file = ws_path!("crates" / "leafbuild-syntax" / "src" / "tests.rs");

    std::fs::write(
        &tests_file,
        &format!("{}\n{}", "// @generated by xtask generate-parser-tests", s),
    )?;
    format_file(&tests_file)?;
    Ok(())
}

pub struct LineCommentBlock {
    first_line: usize,
    lines: Vec<String>,
    path: PathBuf,
}

pub fn line_comment_blocks<R: AsRef<Path>, P: AsRef<Path>>(
    root: R,
    p: P,
    allow_empty: bool,
) -> io::Result<Vec<LineCommentBlock>> {
    let p = p.as_ref();
    let mut v = vec![];

    for file in walkdir::WalkDir::new(&p).into_iter() {
        let file = file?;

        if !file.file_name().to_str().unwrap().ends_with(".rs") {
            continue;
        }

        let path = file.path();
        let rel_path = path.related_to(root.as_ref())?;
        let file_contents = std::fs::read_to_string(path)?;

        v.extend(line_comment_blocks_in_file(file_contents, allow_empty, rel_path).into_iter());
    }

    Ok(v)
}

pub fn line_comment_blocks_in_file(
    file: String,
    allow_empty: bool,
    rel_path: Cow<Path>,
) -> Vec<LineCommentBlock> {
    let mut v = vec![];
    let prefix = "// ";

    let mut block = (0, vec![]);
    for (line_index, line) in file.lines().map(str::trim_start).enumerate() {
        if line == "//" && allow_empty {
            block.1.push(String::new());
            continue;
        }

        let is_comment = line.starts_with(prefix);
        if is_comment {
            block.1.push(line[prefix.len()..].to_string());
        } else {
            if !block.1.is_empty() {
                v.push(LineCommentBlock {
                    first_line: block.0,
                    path: rel_path.clone().into_owned(),
                    lines: mem::take(&mut block.1),
                })
            }
            block.0 = line_index + 2;
        }
    }

    if !block.1.is_empty() {
        v.push(LineCommentBlock {
            first_line: block.0,
            path: rel_path.into_owned(),
            lines: mem::take(&mut block.1),
        })
    }

    v
}

pub fn run_tests(args: Vec<String>) -> anyhow::Result<()> {
    generate_parser_tests()?;
    Ok(cargo_cmd!(format!("test {}", args.join(" ")))?)
}

fn raw_str_literal(s: &'_ str, indent: usize) -> ::proc_macro2::TokenStream {
    let mut depth = 0;
    let mut cur_depth = None;
    let mut chars = s.chars().peekable();
    loop {
        match (cur_depth, chars.peek()) {
            (_, Some('#')) => {}
            (Some(d), _) => depth = depth.max(d),
            _ => {}
        }
        match (chars.next(), cur_depth) {
            (None, _) => break,
            (Some('"'), _) => cur_depth = Some(0),
            (Some('#'), Some(ref mut d)) => *d += 1,
            (Some(_), _) => cur_depth = None,
        }
    }
    format!(
        "&r{0:#^raw_depth$}\"\n{wrapped}\"{0:#^raw_depth$}.unindent()",
        "",
        wrapped = s
            .lines()
            .chain(std::iter::once(""))
            .map(|it| format!("{:indent$}{val}", "", indent = indent, val = it))
            .join("\n"),
        raw_depth = depth + 1,
    )
    .parse()
    .unwrap()
}
