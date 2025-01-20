extern crate syntect;

use std::env;
use std::path::PathBuf;

use self::syntect::parsing::{SyntaxReference, SyntaxSet};
use self::syntect::highlighting::{Theme, ThemeSet};
use self::syntect::html::highlighted_html_for_string;
use highlight::syntect::Error;

pub fn highlighted_html_for(snippet: &String, lang: Option<String>) -> String {

    match lang {
        Some(lang) => {
          let html_result = highlighted_html_for_language(&snippet, lang);
          match html_result {
            Ok(html) => html,
            Err(error) => panic!("Problem highlighting html: {:?}", error),
          }
        }
        None => snippet.to_string()
    }
}

pub fn highlighted_html_for_language(snippet: &String, attributes: String) -> Result<String, Error> {
    lazy_static! {
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_from_folder(syntax_path()).unwrap();
        static ref THEME: Theme = ThemeSet::get_theme(theme_path().as_path()).unwrap();
        static ref RUBY_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("rb").unwrap();
        static ref RUST_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("rs").unwrap();
        static ref C_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("c").unwrap();
        static ref JAVA_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("java").unwrap();
        static ref SQL_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("sql").unwrap();
    }
    if attributes.contains("ruby") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUBY_SYNTAX, &THEME)
    } else if attributes.contains("rust") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUST_SYNTAX, &THEME)
    } else if attributes.contains("java") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &JAVA_SYNTAX, &THEME)
    } else if attributes.contains("sql") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &SQL_SYNTAX, &THEME)
    } else if attributes.contains("type=\"c\"") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &C_SYNTAX, &THEME)
    } else {
        Ok(format!("<pre{}>{}</pre>", attributes, snippet.to_string()))
    }
}

fn theme_path() -> PathBuf {
    let dir = env::current_dir().unwrap();
    dir.join("theme").join("pat.tmTheme")
}

fn syntax_path() -> PathBuf {
    env::current_dir().unwrap().join("syntax")
}
