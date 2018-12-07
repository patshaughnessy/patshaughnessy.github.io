extern crate syntect;

use std::path::Path;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_snippet_for_file;

fn main() {

    let ss = SyntaxSet::load_defaults_nonewlines(); // TODO: maybe just load the Ruby syntax set alone?



    let theme_file : String = "/Users/pat/apps/rust_blog3/pat.tmTheme".to_string();
    let tm_path = Path::new(&theme_file);
    let theme = ThemeSet::get_theme(tm_path).unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please pass in a file to highlight");
        return;
    }

    // This checks which syntax set I'm using.

    //let syntax = ss.find_syntax_for_file(&args[1])
        //.unwrap() // for IO errors, you may want to use try!() or another plain text fallback
        //.unwrap_or_else(|| ss.find_syntax_plain_text());
    //assert_eq!(syntax.name, "HTML (Rails)");

    let html = highlighted_snippet_for_file(&args[1], &ss, &theme).unwrap();
    println!("<html>
                <head>
                  <link rel=\"stylesheet\" href=\"http://localhost/assets/css/main.css\"href=\"/assets/css/main.css\" type=\"text/css\" media=\"screen\" >
                  <title>Testing...!</title>
                </head>
                <body>
                  {}
                </body>
              </html>", html);

}
