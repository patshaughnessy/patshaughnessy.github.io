extern crate syntect;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use self::syntect::parsing::{SyntaxReference, SyntaxSet};
use self::syntect::highlighting::{Theme, ThemeSet};
use self::syntect::html::highlighted_html_for_string;

pub fn highlighted_html_for(snippet: &String) -> String {
    lazy_static! {
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_from_folder(syntax_path()).unwrap();
        static ref THEME: Theme = ThemeSet::get_theme(theme_path().as_path()).unwrap();
        static ref SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("rb").unwrap();
    }

    // TODO Load the appropriate one
    //let syntax = SYNTAX_SET.find_syntax_by_extension("rb").unwrap();
    let html = highlighted_html_for_string(&snippet, &SYNTAX_SET, &SYNTAX, &THEME);
    println!("Highlighted!");
    html
}

fn theme_path() -> PathBuf {
    let dir = env::current_dir().unwrap();
    dir.join("theme").join("pat.tmTheme")
}

fn syntax_path() -> PathBuf {
    env::current_dir().unwrap().join("syntax")
}
