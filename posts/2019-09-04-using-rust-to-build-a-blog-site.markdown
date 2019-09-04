title: "Using Rust to Build a Blog Site"
date: 2019/09/04
tag: Rust

<div style="float: left; padding: 8px 30px 20px 0px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2019/9/4/batteries.jpg"><br/>
<i> Rust comes with batteries included<br/>
    <small>(source: <a href="https://commons.wikimedia.org/wiki/File:Neo-Geo-Pocket-Color-w-batteries.jpg">Wikimedia Commons</a>)</small></i>
</div>

After “Hello World,” blog sites are the world’s second most unneeded
application. If you want to write a blog, use Medium, Wordpress or just
Twitter. The world doesn’t need another blog app.

However, like Hello World, building a static site generator is a great way to
get your feet wet in a new programming language. Recently I rewrote [the script
I use to generate this web
site](https://github.com/patshaughnessy/patshaughnessy.github.io/blob/master/src/lib.rs)
using Rust: I needed to update and fix my script, but really I was looking for
an excuse to write Rust.  Despite its reputation as a difficult to learn,
expert level language,  Rust turned out to be a great choice for the simple
task of generating a few HTML files, quickly and reliably. Why? Not because of
its sophisticated borrow checker or support for safe concurrency.

Rust was a great choice for me because I didn’t have to write most of the code.
Rust’s dependency management and build tool,
[Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), allowed me to
glue together open source Rust libraries called “crates” which do most of the
work. The Rust community’s crate registry, [crates.io](https://crates.io), has
over 29,000 crates available.  Downloading, compiling and using them is dead
simple. And writing a blog site using Rust turned out to be simple too.

## My Cargo.toml File

I needed a few important features to generate this web site. I wanted my script
to work like this for each blog post:

<img src="http://patshaughnessy.net/assets/2019/9/4/flowchart.png"/>

For each blog post, My new Rust script had to: parse the markdown source file
and convert it to HTML markup, highlight the syntax of my code snippets using
&lt;style> tags and CSS, and use a template to insert the HTML for each post
into the surrounding web layout/design. Sounds like a lot of work, right?

Wrong. Other Rust developers smarter than me had already implemented all of
this. All I had to do was find the crates I needed and add them to my
Cargo.toml file:

<pre type="console">
[dependencies]
maud = "*"
pulldown-cmark = "*"
syntect = "3.0"
</pre>

[Pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) is a markdown
parser crate, [Syntect](https://github.com/trishume/syntect) is a color syntax
highlighting crate, and [Maud](https://github.com/lfairy/maud) is an HTML
template crate. Actually, to be honest I ended up adding a few other crates to
get my script to work:

<pre type="console">
[dependencies]
maud = "\*"
pulldown-cmark = "\*"
regex = "\*"
lazy_static = "\*"
syntect = "3.0"
chrono = "\*"
clap = "\*"
ordinal = "\*"
</pre>

I’m not sure why, but the Rust standard library is very minimal. Features that
are included in other languages, like regular expressions or date/time parsing,
are handled by crates (e.g. regex and chrono).

In any case, all I had to do was build my project and Cargo downloaded
everything I needed:

<pre type="console">
$ cargo build --release
    Updating crates.io index
  Downloaded chrono v0.4.7
  Downloaded clap v2.33.0
  Downloaded maud v0.19.0
  Downloaded lazy_static v1.2.0
  Downloaded pulldown-cmark v0.2.0
  Downloaded ordinal v0.2.2
  Downloaded regex v1.1.0
  Downloaded syntect v3.0.2
  Downloaded libc v0.2.44
  Downloaded num-integer v0.1.41
  Downloaded num-traits v0.2.8
  Downloaded time v0.1.42

etc…
   Compiling syntect v3.0.2
   Compiling blogc v0.1.0 (/Users/pat/apps/patshaughnessy.github.io)
    Finished release [optimized] target(s) in 2m 27s
</pre>

It couldn’t be easier! During the rest of this post, I’ll show you how I used
these three crates: Pulldown-cmark, Syntect and Maud.

## Pulldown-cmark

Now that my blog app included the Pulldown-mark crate, using it was just a
matter of pasting in a few of lines of code from the [helpful example on
docs.rs](https://docs.rs/pulldown-cmark/0.5.3/pulldown_cmark/html/fn.push_html.html):

<pre type="rust">
let parser = Parser::new(&markdown);
let mut html = String::new();
html::push_html(&mut html, parser);
</pre>

The first line created a <span class="code">Parser</span> struct, passing in a
reference to my markdown string. Then I created an empty, mutable target
string, called <span class="code">html</span>. Last, I called the <span
class="code">push_html</span> function which parsed the markdown source,
converted it to HTML and saved it into <span class="code">html</span>. I didn’t
have to do any work whatsoever.

In fact, the only real work for me had to do with “header” strings present at
the top of each post source file. For example, the
[2017-12-15-looking-inside-postgres-at-a-gist-index.markdown](https://raw.githubusercontent.com/patshaughnessy/patshaughnessy.github.io/master/posts/2017-12-15-looking-inside-postgres-at-a-gist-index.markdown)
file starts with:

<pre type="console">
title: "Looking Inside Postgres at a GiST Index"
date: 2017/12/15
tag: the Postgres LTREE Extension

etc…
</pre>

Here the first three lines are metadata values about the post and not part of
the post content. So before calling Pulldown-mark, my script parses and
removes these header lines:

<pre type="rust">
fn other_lines(lines: &Vec<String>) -> Vec<String> {
  lines.iter().skip_while(|l| is_header(l)).map(|l| l.to_string()).collect()
}
</pre>

Above the <span class="code">lines</span> parameter is an array of strings,
each a single line of text in the markdown source file. (More precisely, it’s a
reference to a <span class="code">Vec&lt;String></span>, not an array.) The code
is fairly readable: <span class="code">other_lines</span> creates an iterator
over the lines, skips the first few header lines, and then collects the
remaining lines into a second array which the function returns.

Here’s the complete <span class="code">html_from_markdown</span> function,
which calls <span class="code">other_lines</span>, joins them together into a
single large string, and then passes that to Pulldown-mark:

<pre type="rust">
fn html_from_markdown(lines: &Vec<String>) -> Result<String, InvalidPostError> {
  let mut markdown = String::new();
  for line in other_lines(lines) {
    markdown.push_str(&line);
    markdown.push('\n');
  }
  let parser = Parser::new(&markdown);
  let mut html = String::new();
  html::push_html(&mut html, parser);
  Ok(with_delim_removed(with_highlighted_code_snippets(&html)))
}
</pre>

## Syntect

If you read the code above carefully, you’ll notice <span
class="code">html_from_markdown</span> calls <span
class="code">with_highlighted_code_snippets</span> before returning the HTML
for each post. This function performs color syntax highlighting.

The code snippets in each of my blog posts appear inside of &lt;pre>…&lt;/pre>
tags.  And I use a “type” attribute to indicate the programming language of the
snippet. For example:

<pre type="console">
&lt;pre type="ruby">
puts "This is Ruby code I’m writing about…"
&lt;/pre>
</pre>

Like parsing markdown, syntax highlighting is a very complex task: The Syntect
crate has to parse the given code snippet, determine the semantic meaning of
each keyword in the snippet based on the provided programming language, and
then insert the proper color information. Thank goodness I didn’t have to write
that code!

But using Syntect was easy:

<pre type="rust">
pub fn highlighted_html_for_language(snippet: &String, attributes: String) -> String {
  lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_from_folder(syntax_path()).unwrap();
    static ref THEME: Theme = ThemeSet::get_theme(theme_path().as_path()).unwrap();
    static ref RUBY_SYNTAX: &'static SyntaxReference =
      SYNTAX_SET.find_syntax_by_extension("rb").unwrap();
    static ref RUST_SYNTAX: &'static SyntaxReference =
      SYNTAX_SET.find_syntax_by_extension("rs").unwrap();

etc...

  }
  if attributes.contains("ruby") {
    highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUBY_SYNTAX, &THEME)
  } else if attributes.contains("rust") {
    highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUST_SYNTAX, &THEME)

etc...
</pre>

First I used a <span class="code">lazy_static</span> block to initialize a few
constant values.
([lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) is another
crate I didn’t have to write!) Rust executes this block once the first time
it’s encountered and then never again. The values are:

* <span class="code">SYNTAX_SET</span>: These are the Sublime syntax files
  describing each programming language I need to colorize. vim is my editor,
  but I use Sublime for color syntax highlighting :) I just downloaded these
  files for the languages I needed and checked them into my app.

* <span class="code">THEME</span>: These are the Sublime “theme” files, which
  select the colors to use for each type of code keyword. I found and adapted
  one of these files. They play the role of a CSS file, but use XML syntax.
  Weird, but not hard to figure out.

* <span class="code">RUBY_SYNTAX</span>, <span class="code">RUST_SYNTAX</span>,
  etc. The lazy block also looks up the syntax language file for each language.

Later my <span class="code">highlighted_html_for_language</span> function
checks which programming language my post displays, and calls <span
class="code">syntect::html::highlighted_html_for_string</span> with the proper
values:

<pre type="rust">
  if attributes.contains("ruby") {
    highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUBY_SYNTAX, &THEME)
  } else if attributes.contains("rust") {
    highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUST_SYNTAX, &THEME)

etc...
</pre>

<span class="code">attributes</span> is the array of HTML attributes from the
&lt;pre> tag surrounding the code snippet in my post source. My app uses
regular expressions to find the &lt;pre>…&lt;/pre> HTML blocks, parses the
attributes and provides them to <span
class="code">highlighted_html_for_language</span>.

## Maud

Now my script has HTML for each blog post. All I have to do now is save it in a
series of HTML files. But first I needed a template engine for Rust, like ERB
for Ruby or Mustache for node.js.

This turned out to be one of the most fun parts of this project. I rewrote [my
HTML
markup](https://github.com/patshaughnessy/patshaughnessy.github.io/tree/master/src/layout)
using Maud <span class="code">@</span> directives, like this:

<pre type="rust">
@if let Some(ref t) = post.tag {
  div class="header" {
    "More on " (t)
  }
  div class="links" {
    ul {
      @for (link_url, link_title) in recent_links {
        li {
          a href={ "/" (link_url) } {
            (link_title)
          }
        }
      }
    }
  }
}
</pre>

Maud doesn’t parse the layout code at runtime, like ERB does in Ruby. Instead,
the <span class="code">@if</span> and <span class="code">@for</span> directives
above are macros. In fact, all of the HTML elements that appear above, like
<span class="code">div</span> and <span class="code">ul</span>, are macros
also. This means my Maud layout code is actually Rust code!  And that means the
Rust compiler will check it and make sure it’s valid before it ever runs.

Converting my old ERB templates into Rust macros was a bit tedious, but it was
a great way to review and clean up my HTML. In fact, I found a number of
mistakes and errors in my HTML code that had been there for 10 years or longer.
It was like showing my dirty laundry to the Rust compiler. By the time the
compiler was done and let me compile my layout, it was very clean!

## What It Worth It?

It took me several months on a spare time basis - an hour here an hour there -
to rewrite my blog in Rust. An experienced Rust developer working full time
could have done it in a day or two probably.

What did I get for all this effort? Now I have a script that compiles all 146
of my markdown posts very quickly. My old Ruby script took 7.7 seconds to do
this, while the new Rust script only takes 0.28 seconds! That’s over 27 times
faster! In fact, the Rust code is so fast at parsing and compiling the markdown
files that I don’t check which files need to be recompiled by comparing
timestamps, i.e. what a Makefile would do during a build process. And I don’t
process the posts in parallel. Why bother? By the time I pressed ENTER and
looked up Rust was almost done building all 146 files in sequence, one after
the other.

But what else did I get? The biggest improvement to my blog script, actually,
wasn’t the performance. It was the error handling I added. I didn’t mention
this above, but using the Rust standard library required me to use the
<span class="code">Result&lt;T></span> generic type. This, in turn, forced me to
think about what might go wrong and what to do when it did go wrong. I’ll cover
this in my next article.  I ended up with a script that was much more reliable
and resilient to silly mistakes in my source files, and that gave me helpful
error messages… all the while running 27 times faster.

However, the biggest benefit to rewriting my blog in Rust was that I clawed my
way up the Rust learning curve a bit. But that wouldn’t have been possible
without crates.io and Cargo. Using code from smarter, more seasoned Rust
developers gave me a chance to build a useful app, even as a beginner. Cargo
found, downloaded and compiled open source code from experts with just a few
simple commands.
