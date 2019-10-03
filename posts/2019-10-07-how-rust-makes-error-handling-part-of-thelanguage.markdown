title: "How Rust Makes Error Handling Part of the Language"
date: 2019/10/07
tag: Rust

<div style="float: left; padding: 8px 30px 20px 0px; text-align: center; line-height:18px">
  <img src="http://localhost/assets/2019/10/7/fingers-toes.png"><br/>
<i>In Spanish these are all “dedos,” while in English<br/>we can distinguish between fingers and toes. </i>
</div>

Learning a foreign language can be an incredible experience, not only because
you can talk to new people, visit new countries, read new books, etc. When you
learn the words someone from a different culture uses, you start to see things
from their perspective. You understand the way they think a bit more.

The same is true for programming languages. Learning the syntax, keywords and
patterns of a new programming language enables you to think about problems from
a different perspective. You learn to solve problems in a different way.

I’ve been studying [Rust](https://www.rust-lang.org) recently, a new
programming language for me. As a Ruby developer, I was curious to learn how
Rust developers approach solving problems. What do Rust programs look like?
What new words would I learn?

## Why Rust Was Difficult For Me

I knew it would be a challenge to learn such a difficult language. I had heard
horror stories about how difficult the Rust compiler can be to use, or about
how confusing the ownership memory model and the borrow checker can be. And I
was right: Rust is a very difficult language to learn. But not because of move
semantics or memory management.

For me, the most challenging syntax in Rust had to do with simple error
handling. Let’s take an example: opening and reading a text file. In Ruby, this
is a one-liner and error handling is completely optional:

<pre type="ruby">
string = File.read("/path/to/file")
</pre>

In Ruby, <span class="code">File.read</span> returns a simple string. Will this
ever return an error? Who knows. Maybe Ruby will raise an exception, maybe not.
I don’t have to worry about that at the call site when I’m writing the code. I
can focus on the happy path, but I end up with a program that can’t handle
errors.

Golang, at least, returns an error value explicitly when I try to read a file:

<pre>
b, err := ioutil.ReadFile(“/path/to/file“)
if err != nil {
    fmt.Print(err)
} else {
    str := string(b)
}
</pre>

Here the Golang ioutil.ReadFile function returns two values: the string I want
and also an error value. The Go compiler forces me to think about errors that
might occur, at least for a moment. But error handling is still optional. I can
simply choose to ignore the <span class="code">err</span> value entirely. Most
C programs work in a similar fashion, returning an error code in some manner.
And if I do choose to handle the error, I end up with verbose, messy code that
checks for error codes over and over again.

In Rust error handling in mandatory. Let’s try to rewrite the same example
using Rust:

<pre type="rust">
let mut file = File::open("foo.txt");
let mut contents = String::new();
file.read_to_string(&mut contents);
</pre>

Right away I run into trouble when I try to compile this:

<pre class="console">
error[E0599]: no method named `read_to_string` found for type
`std::result::Result<std::fs::File, std::io::Error>` in the current scope
</pre>

What? What is the Rust compiler talking about? I can see there’s a <span
class="code">read_to_string</span> method on the <span class="code">File</span>
struct [right in the
documentation](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)!
(Actually the method is on the <span class="code">Read</span> trait which <span
class="code">File</span> implements.)

The problem is the <span class="code">File::open</span> function doesn’t return
a file at all. It returns a value of type <span
class="code">io::Result<File></span>. Reading the documentation, I see:

<pre type="rust">
pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File>
</pre>

How do I use this? What does <span class="code">io::Result<File></span> even
mean? When I try to write Rust code the way I write Ruby or Go code, I get
cryptic errors and it doesn’t work.

The problem is I’m trying to speak Rust the same way I speak in Ruby. Rust is a
foreign language; I need to learn some vocabulary before I can try to talk to
someone. This is why Rust is difficult to learn: It’s a foreign language that
uses many words completely unfamiliar to most developers.

## Types Are the Vocabulary of Programming Languages.

My wife is Spanish, and lucky for me she’s had the patience and the endurance
to teach me and our kids Spanish over the years. As a native English speaker,
it always seemed curious and amusing to me that Spanish has only one word for
fingers and toes, _dedos_. Don’t people in Spain or Latin America ever need to
talk about only fingers and not toes? Or vice-versa? And in Spain I invariably
end up saying silly things like _dedos altos_ (“upper fingers”), or _dedos
bajos_ (“lower fingers”). I always worry about which digits I’m talking about.
Somehow, though, the Spanish never have any trouble with this; where the
_dedos_ are located always seems obvious to them from the context.

But I wonder: Do Spanish speakers have trouble learning English when it comes
to fingers vs. toes? Do they ever say finger when they mean toe? The problem is
not just learning a new word. You have to learn the meaning behind the word.
The problem is English has a concept, a distinction, that Spanish doesn’t.

Back to computer programming, the “words” we use in programming languages
aren’t only syntax tokens like if, else, let, etc. They are the values that we
pass around in our programs. And those values have types, even for loosely,
dynamically types languages like Ruby.

Aside from whatever formal definition Computer Science has for types, I simply
think of a value’s type as it’s meaning or purpose. To understand what role a
value plays in your program, you need to understand the concept behind its
type. Just like the words finger and toe represent certain anatomical concepts
in English, types like <span class="code">Result<T, E></span> or <span
class="code">Option<T></span> represent programming concepts in Rust - concepts
that foreigners need to learn for the first time.

<p></p>

<blockquote>
Language shapes the way we think, and determines what we can think about.<br/>
-- Benjamin Lee Whorf
</blockquote>

In fact, some linguists take this to the extreme: That a language’s words
determine what people in that community are able to think and talk about, what
concepts they can understand. (However, most modern linguists, [according to
Wikipedia](https://en.wikipedia.org/wiki/Linguistic_relativity), don’t believe
this is actually true.)

Because Rust includes the <span class="code">Result<T, E></span> type, Rust
programmers are empowered to talk about error handling in a very natural way.
It’s part of their daily vocabulary. Rust programmers, in fact, often think
about error handling precisely because they have the word <span
class="code">Result<T, E></span>.

Of course, native Spanish speakers, I’m guessing, have no trouble understanding
the distinction between fingers and toes. But I certainly have trouble
understanding the concept behind <span class="code">Result<T, E></span> in
Rust, a foreign language I’m trying to learn.

## If Rust is Spanish, then Haskell is Latin

So what does <span class="code">Result<T, E></span> mean? What is a value of
type <span class="code">Result<T, E></span>?

Just as human language borrow words from other languages - many Spanish words
are taken from Latin or Arabic; English borrowed many words from French and
German - programming languages borrow words and concepts from other, older
programming languages.

Rust borrowed the concept behind the <span class="code">Result<T, E></span>
type from Haskell, a strongly typed functional programming language. Haskell
includes a type called <span class="code">Either</span>:

<pre>
data Either a b = Left a | Right b
</pre>

This syntax seems bizarre at first glance but in fact it’s simple. Haskell
makes it easy to create new types by combining other types together. This line
of code means the <span class="code">Either</span> type is a combination of two
other types: <span class="code">a</span> and <span class="code">b</span>.
Drawing that type equation, this is how I visualize Haskell <span
class="code">Either</span> values:

<img src="http://localhost/assets/2019/10/7/left-or-right.png"><br/>

But a single <span class="code">Either</span> value can only encapsulate either
a value of type <span class="code">a</span> or a value of type <span
class="code">b</span>:

- If the <span class="code">Either</span> value is <span
  class="code">Left</span>, then it contains an inner value of type <span
  class="code">a</span>. This is written: <span class="code">Left a</span>

- If the <span class="code">Either</span> value is <span
  class="code">Right</span>, then it contains an inner value of type <span
  class="code">b</span>. This is written: <span class="code">Right b</span>

The <span class="code">Either</span> type is also “monad,” because Haskell also
provides certain functions that create and operate on <span
class="code">Either</span> values. I won’t cover this concept here, because
there are a number of great articles out there already about what monads are
and how to use them. In Haskell, the <span class="code">Either</span> type is
completely general, and you can use it to represent any programming concept you
would like.

Rust uses the same concept behind the <span class="code">Either</span> type in
Haskell for a specific purpose: to implement error handling. If Haskell is
Latin, then Rust is Spanish
- a younger language that borrows some of the older languages’s vocabulary and
  grammar.

## Result<T, E> in Rust

In Rust, the <span class="code">Result</span> type encapsulates two other types
like <span class="code">Either.</span> And a single <span
class="code">Result</span> value has either one of those types or the other:

<img src="http://localhost/assets/2019/10/7/ok-or-err.png">

Instead of <span class="code">Left a</span> and <span class="code">Right
b</span> like in Haskell, Rust uses the words <span class="code">Ok(T)</span>
and <span class="code">Err(E)</span>:

- If the <span class="code">Result</span> value is <span
  class="code">Ok</span>, then it contains an inner value of type <span
  class="code">T</span>. This is written: <span class="code">Ok(T)</span>

- If the <span class="code">Either</span> value is <span
  class="code">Err,</span> then it contains an inner value of type <span
  class="code">E</span>. This is written: <span class="code">Err(E)</span>

And in Rust these values have specific meanings related to error handling: A
<span class="code">Result</span> value of <span class="code">Ok(T)</span> means
some operation was successful, and the result of the operation is a value of
type <span class="code">T</span>. Similarly, a <span class="code">Result</span>
value of type <span class="code">Err(E)</span> means the operation was a
failure, and the result of the operation is an error of type <span
class="code">E</span>.

Back to my open file example, the proper way to open a file and read it using
Rust is to check the type of the <span class="code">Result</span> values
returned by the Rust standard library functions:

<pre type="rust">
fn main() {
    let file = File::open("foo.txt");
    match file {
        Ok(file) => println!("I have a file: {:?}", file),
        Err(e) => println!("There was an error: {}", e)
    }
}
</pre>

And If I want to actually read in the contents of that file, I would check that
return value also:

<pre type="rust">
fn main() {
    let file = File::open("foo.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => println!("The file's contents are: {}", contents),
                Err(e) => println!("There was an error: {}", e)
            }
        }
        Err(e) => println!("There was an error: {}", e)
    }
}
</pre>

## The ? Macro In Rust

That last code snippet is quite a mouthful - error checking with Rust is even
more tedious and verbose than it is using Go!

Fortunately, Rust includes an operator (actually a macro, strictly speaking)
that allows Rust programmers to abbreviate all of this logic. By appending the
<span class="code">?</span> character to the call site of a function that
returns a <span class="code">Result<T, E></span> value, Rust automatically
generates code that checks the <span class="code">Result<T, E></span> value,
and returns underlying <span class="code">T</span> value if the result is <span
class="code">Ok(T)</span>:

<pre type="rust">
fn main() {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
}
</pre>

Here, the use of <span class="code">?</span> after <span
class="code">File::open(“foo.txt")?</span> tells the Rust compiler to check the
return value of File::open for me automatically:

<img src="http://localhost/assets/2019/10/7/success-or-failure.png"><br/>

If the return value of <span class="code">File::open</span> is <span
class="code">Ok(T)</span>, then Rust assigns the inner <span
class="code">T</span> value to file. If <span class="code">File::open</span>
returns <span class="code">Err(E)</span>, then Rust jumps to the end of the
main function immediately, and returns the inner <span class="code">E</span>
value.

The program above is much more concise and easy to understand. The only problem
is that it doesn’t work! When I try to compile this, I get:

<pre class="console">
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:5:20
  |
5 |     let mut file = File::open("foo.txt")?;
  |                    ^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
</pre>

## Rust Programs Revolve Around Error Handling

As the error message says, the problem here is that the <span
class="code">?</span> macro generates code that will jump to the end of the
main function and return the inner <span class="code">E</span> value, which is
this example is of type <span class="code">std::io::Error</span>. The problem
is that I haven’t declared a return value for main. Therefore the Rust compiler
gives me an error:

<pre class="console">
the `?` operator can only be used in a function that returns `Result` or
`Option` (or another type that implements `std::ops::Try`)
</pre>

The function containing the use of the <span class="code">?</span> operator has
to return a value of type <span class="code">Result<T, E></span> with a
matching <span class="code">E</span> type in order for this to make sense. I
have to extract my <span class="code">File</span> calls into a separate
function, like this:

<pre type="rust">
fn read() -> Result<String, std::io::Error> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read() {
        Ok(str) => println!("{}", str),
        Err(e) => println!("{:?}", e)
    }
}
</pre>

Note the new <span class="code">read()</span> function above returns a value of
type <span class="code">Result<String, std::io::Error></span>. This allows the
use of the <span class="code">?</span> operator to compile properly. For the
happy path, if my code is able to find the “foo.txt” file and read it, then
<span class="code">read()</span> returns <span
class="code">Ok(contents)</span>. However, if there’s an error, <span
class="code">read()</span> will return <span class="code">Err(e)</span>, where
<span class="code">e</span> is a value of type <span
class="code">std::io::Error</span>. Note the <span class="code">E</span>
portion of the <span class="code">Result<T, E></span> return type of the inner
function, <span class="code">File::open</span>, matches the <span
class="code">E</span> portion of the <span class="code">Result<T, E></span>
return type of the surrounding function. <span class="code">open</span> returns
the same error type that read does:

<img src="http://localhost/assets/2019/10/7/error-types.png">

This is where Rust shines. It allows for concise and readable error handling,
that is also thorough and correct. The Rust compiler checks for error handling
completeness at compile time.

Now that I’ve learned some vocabulary words, now that I can understand how
native Rust speakers use the word Result<T, E>, I can have a Rust conversation
about error handling. I can begin to think like Rust developers think. I can
start to see things from their perspective.

And I begin to realize that the Rust culture revolves around error handling.
Because of words like <span class="code">Result<T, E></span> and <span
class="code">?</span>, Rust makes it easy for developers to talk errors and how
to handle them correctly.

However, Rust programs tend to be designed with error handling in mind. Notice
above how I had to extract a separate function that returned a value of type
<span class="code">Result<T, E></span>, just because of the <span
class="code">?</span> operator. The overall structure of my program is
determined by error handling just as much as it’s determined by the nature of
the task I’m trying to accomplish. Rust programmers think about errors and what
might go wrong from the very beginning, from when they start writing code.
Developers using other languages tend to think about errors and what might go
wrong as an afterthought.
