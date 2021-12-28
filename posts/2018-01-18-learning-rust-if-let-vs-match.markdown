title: "Learning Rust: If Let vs. Match"
date: 2018/1/18
tag: Rust

<div style="float: right; padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2018/1/18/dictionary.jpg"><br/>
  <i>Human languages have similar words with different<br/> shades of meaning. Some computer languages do too.<br/>
  <small>(from: <a href="https://commons.wikimedia.org/wiki/File:ĸra_in_a_dictionary_(ubt).JPG">Wikimedia Commons</a>)</small></i> 
</div>

This year I’ve decided to try to learn [Rust](https://www.rust-lang.org). I’m
fascinated by its ownership model for memory management; I’m curious what the
claims about safety are all about; and, I love how it incorporates ideas from
the functional programming world. But I haven’t gotten to all of that yet - I’m
just getting started learning the basic syntax.

Learning a computer language is just like learning a human language. You have
to try to read and write it everyday, even if just for a few minutes. You need
to get to know some native speakers. And there’s no way around it: You need to
learn the basic vocabulary of the language, word by word. To make things worse,
our human languages usually have several  words that mean the same thing. Which
one should I use? Sometime only a native speaker will really know.

This week I was reading about <span class="code">if let</span> and <span
class="code">match</span> in [The Rust Programming
Book](https://doc.rust-lang.org/book/second-edition/ch06-03-if-let.html)
(TRPL). I read that <span class="code">if let</span> is really syntactic sugar
for <span class="code">match</span>:

<div style="padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
<img width="500" src="https://patshaughnessy.net/assets/2018/1/18/trpl-quote.png"/>
</div>

This intrigued me. The phrase “syntactic sugar” implies the two code snippets
don’t only produce the same results, it means the compiler generates exactly
the same code in each case.

Does the Rust compiler really generate exactly the same code for <span
class="code">if let</span> as it does for <span class="code">match</span>? Read
on to find out. Today I’ll start with a quick review of the syntax and meaning
of <span class="code">if let</span> and <span class="code">match</span>. Then
I’ll take a look at how Rust compiles <span class="code">if let</span> and
<span class="code">match</span>, at what code it produces.

## If Let Compares a Pattern with a Value

The idea behind <span class="code">if let</span> is that it compares a pattern
with a value:

<img src="https://patshaughnessy.net/assets/2018/1/18/pattern-value.png"/>

In this example <span class="code">if let</span> compares the pattern <span
class="code">Some(3)</span> with the value <span
class="code">some_u8_value</span>. If there’s a match, <span class="code">if
let</span> executes the <span class="code">println!</span> code inside the
block.

## If Let Also Assigns Values

<span class="code">if let</span> assigns a value at the same time, when the
pattern matches the value. This is the idea behind including the <span class="code">let</span> keyword
after <span class="code">if</span>. This is more apparent if I rewrite the example using a variable <span class="code">i</span>
instead of 3. I'll also add a main function so I can execute the code:

<pre>
fn main() {
  let some_u8_value = Some(3u8);
  if let Some(i) = some_u8_value {
     println!("assigned {} to i", i);
  }
}
</pre>

When I saved this in a file called if-let.rs and ran it, I got:

<pre>
$ rustc if-let.rs
$ ./main
Assigned 3 to i
</pre>

<span class="code">if let</span> “unwrapped” the option structure, and assigned
the value 3 to the identifier <span class="code">i</span>.

## Match: If Let’s Big Brother

As TRPL explains, I could also have written this using the <span
class="code">match</span> keyword, as follows:

<pre>
fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(i) => println!("Matched: {}", i),
        None => (),
    }
}
</pre>

To write this all I had to do was move things around a bit in my <span
class="code">if let</span> code snippet from above:

<img src="https://patshaughnessy.net/assets/2018/1/18/if-let-match.png"/>

Because there was no else clause for the <span class="code">if let</span>
statement, I used <span class="code">None => ()</span> in match.

Saving this code in match.rs and running it I got the same result:

<pre>
$ rustc match.rs
$ ./main
Matched: 3
</pre>

## Mid-Level IR (MIR)

I was curious though: If these two code snippets are entirely equivalent, then
the Rust compiler should generate _exactly the same executable program_ when I
compile them. In theory, therefore, I should be able to compare the two
executable binaries to test whether TRPL’s statement about syntactic sugar is
accurate. But comparing binary executables might not work. Likely there are
timestamps or other ephemeral values encoded in the executable that would break
the comparison. I decided to look for an easier way to test the compiler’s
output.

Then I came across mid-level intermediate representation (MIR), described [here
on the Rust blog](https://blog.rust-lang.org/2016/04/19/MIR.html). MIR is an
internal text language the rust compiler produces when you include the
<span class="code">—emit-mir</span> flag, like this:

<pre>
$ rustc --emit mir if-let.rs
</pre>

With this option specified, rust generates a file called if-let.mir. Opening up
this file, I see:

<pre>
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: std::option::Option<u8>; // "some_u8_value" in scope 1 at src/if-let.rs:16:9: 16:22

etc…
</pre>

“Knock yourself out;” now I’m really intrigued!

## A First Look at MIR

I decided to compare the MIR text file the Rust compiler produced for the <span class="code">if
let</span> snippet vs. the <span class="code">match</span> snippet. If Rust
considers <span class="code">if let</span> to be syntactic sugar for <span
class="code">match</span>, then the MIR representation of the two snippets
should be the same.

But when I started reading the MIR code, I found the call to the <span
class="code">println!</span> macro generated a lot of verbose text:

<pre>
let mut _3: isize;
let mut _4: ();
let mut _5: std::fmt::Arguments;
let mut _6: &[&str];
let mut _7: &[&str; 2];
let mut _8: &[&str; 2];
let mut _9: &[std::fmt::ArgumentV1];
let mut _10: &[std::fmt::ArgumentV1; 1];
let mut _11: &[std::fmt::ArgumentV1; 1];
let mut _12: [std::fmt::ArgumentV1; 1];
let mut _13: (&u8,);
let mut _14: &u8;
let mut _16: std::fmt::ArgumentV1;
let mut _17: &u8;
let mut _18: fn(&u8, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>;
</pre>

All of this MIR pseudocode might confuse my comparison unnecessarily, so I
decided to simplify my <span class="code">if let</span> example by removing the
<span class="code">println!</span> call entirely. I rewrote the <span
class="code">if let</span> snippet like this (if-let.rs):

<pre>
fn main() {
    let some_u8_value = Some(3u8);
    if let Some(i) = some_u8_value {
        let _ = i;
    }
}
</pre>

And the <span class="code">match</span> snippet like this (match.rs):

<pre>
fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(i) => { let _ = i; }
        None => ()
    }
}
</pre>

I also noticed the MIR file contained many comments with line numbers:

<pre>
_2 = ((_1 as Some).0: u8);       // scope 3 at if-let.rs:3:17: 3:18
StorageLive(_5);                 // scope 3 at <print macros>:2:27: 2:58
StorageLive(_6);                 // scope 3 at <println macros>:3:18: 3:43
</pre>

I realized the line numbers would likely cause problems comparing one MIR file
to another, so I removed all of the comments using sed:

<pre>
$ rustc if-let.rs --emit mir
$ cat if-let.mir | sed -e 's/\/\/.*$//' > if-let.mir.nocomments
</pre>

This generates a new text file called if-let.mir.nocomments, which contains the
same content as if-let.mir, but with no comments. And this command processes
the match.rs file in the same way:

<pre>
$ rustc match.rs --emit mir
$ cat match.mir | sed -e 's/\/\/.*$//' > match.mir.nocomments
</pre>

## Comparing MIR Files

Now I ran a simple diff command on the simplified MIR text files. If the
compiler considers <span class="code">if let</span> to be exactly the same as
<span class="code">match</span> then there should be no difference, then the
output of diff should be empty.

But running diff I saw:

<pre>
$ diff if-let.mir.nocomments match.mir.nocomments
19c19
<         switchInt(_3) -> [1isize: bb2, otherwise: bb1];
---
>         switchInt(_3) -> [0isize: bb1, otherwise: bb2];
</pre>

My two MIR files are _almost_ identical; the MIR text Rust generates for <span
class="code">if let</span> is exactly the same as the MIR text Rust generates
for <span class="code">match</span>, except for line 19. I’ve _almost_ proven
the hypothesis that <span class="code">if let</span> is syntactic sugar for
<span class="code">match</span>, but not quite.

Let’s take a close look at the MIR code around line 19 and try to understand
what it means. Here’s a portion of if-let.mir.nocomments, produced by the Rust
compiler from my <span class="code">if let</span> code above:

<pre>
bb0: {
    StorageLive(_1);
    _1 = std::option::Option<u8>::Some(const 3u8,);
    _3 = discriminant(_1);
    switchInt(_3) -> [1isize: bb2, otherwise: bb1];
}

bb1: {
    _0 = ();
    goto -> bb3;
}

bb2: {
    StorageLive(_2);
    _2 = ((_1 as Some).0: u8);
    _0 = ();
    goto -> bb3;
}
</pre>

I don’t understand MIR syntax, but it’s not hard to guess what’s going on. Each
of these “bb” blocks of code <span class="code">{ … }</span> probably
represents a logical piece of my program.

The first block, <span class="code">bb0</span>, seems to assign the value <span class="code">Some(3)</span> to \_1, and then calls
<span class="code">discriminant(_1)</span> and saves the “discriminant,” whatever that is, in \_3.
Finally, it tests whether the discriminant is 1. If the discriminant is 1 it
jumps to <span class="code">bb2</span>, or otherwise to <span
class="code">bb1</span>. So <span class="code">bb0</span> likely represents the
<span class="code">if</span> portion of my <span class="code">if let</span>
snippet, testing a condition:

<pre>
if let Some(i) = some_u8_value
</pre>

The <span class="code">bb1</span> block saves <span class="code">()</span> in <span
class="code">_0</span> and jumps to <span class="code">bb3</span>. This likely represents the
missing/default else clause of my <span class="code">if let</span> statement.

And the <span class="code">bb2</span> block saves 3, the unwrapped value inside of <span class="code">Some(3)</span>, in <span class="code">\_2</span> and
jumps to <span class="code">bb3</span>. Probably <span class="code">\_2</span> is the variable <span class="code">i</span>, and this block of MIR text
represents the <span class="code">let</span> portion of my <span
class="code">if let</span> snippet:

<pre>
let Some(i) = some_u8_value
let _ = i;
</pre>

Now let’s take a look at the <span class="code">match</span> version, the contents of
match.mir.nocomments. It’s entirely the same, except for the <span class="code">switchInt</span> line:

<pre>
bb0: {
    StorageLive(_1);
    _1 = std::option::Option<u8>::Some(const 3u8,);
    _3 = discriminant(_1);
    <b>switchInt(_3) -> [0isize: bb1, otherwise: bb2];</b>
}
</pre>

Reading this carefully, I saw that actually it does mean the same thing: If the
discriminant is 0, Rust calls the <span class="code">bb1</span> block, or
otherwise the <span class="code">bb2</span> block.

So, summarizing, the <span class="code">if let</span> snippet ran this
pseudo-code:

<blockquote>
If the discriminant is 1, call bb1, else bb2.
</blockquote>

…and the <span class="code">match</span> snippet ran this pseudo-code:

<blockquote>
If the discriminant is 0, call bb2, else bb1.
</blockquote>

So, in fact, the two versions use the same logic, assuming the value of
discriminant is either 0 or 1. If discriminant = 0, Rust assumes the comparison
was true and executes the match clause; if discriminant = 1, Rust executes the
else clause.

Clearly the discriminant function is crucial - when I have time next, I’ll
explore what discriminant means, where it’s implemented and how it works. Or if
anyone from the Rust teams happens to read this, let us know.
