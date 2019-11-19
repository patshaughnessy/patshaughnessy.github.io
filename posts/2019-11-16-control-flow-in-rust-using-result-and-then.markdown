title: "Control Flow in Rust Using Result#and_then"
date: 2019/11/16
tag: Rust

<div style="float: right; padding: 8px 0px 20px 30px; text-align: center; line-height:18px">
  <img src="http://localhost/assets/2019/11/16/train-yard.jpeg"><br/>
  <i>Rust’s Result type can help you control your program’s<br/>
  flow by checking for errors in a succinct, elegant way</i>
</div>

Using Rust for the first time, error handling was my biggest stumbling block. I
couldn’t just call a function and get the value I needed; I had to unwrap the
value, checking for errors. I couldn’t just write the code I wanted to write; I
had to think about generic types. Was this value a <span class="code">Result<T,
E></span> or just a T?  And the right T? The right E? It seemed like a
confusing, overly elaborate way of writing a simple check for an error code.

But after a while, I started to get a feel for the basics of using <span
class="code">Result</span>. I learned how to unwrap the encapsulated values; how to check
for errors. Then I discovered some of the combinator methods it provides, like
<span class="code">map</span>, <span class="code">or_else</span> and <span
class="code">ok</span>. These methods made error handling fun - well, ok maybe
that's a bit of an overstatement. They made using <span
class="code">Result</span> a bit easier, at least.

So far my favorite <span class="code">Result</span> combinator method is
<span class="code">and_then</span>. Using <span class="code">and_then</span>
_is_ actually fun! For example, I wrote [this Rust
code](https://github.com/patshaughnessy/patshaughnessy.github.io/blob/master/src/lib.rs#L43)
to generate the static HTML pages for this blog site:

<pre type="rust">
let params = CompileParams {all_posts: all_posts, output_path: output_path, draft: draft};
Ok(params).and_then(compile_posts)
          .and_then(compile_home_page)
          .and_then(compile_rss_feed)
          .map(|_output| count)
</pre>

After first creating a struct containing the inputs for my blog compile functions, my Rust code:
* First tries to compile all the posts in my blog
* _And then_ if this was successful, it tries to compile the home page
(index.html)
* _And then_ if this was successful, it tries to compile the RSS feed (index.xml)

If there was an error at any time in this process, it short circuits and stops.
Here’s a flowchart that illustrates shows this control flow:

<div style="margin-top: 20px; margin-left: auto; margin-right: auto; width: 489px;">
<img src="http://localhost/assets/2019/11/16/flowchart.png"><br/>
</div>

The happy path is from top to bottom, along the left side. If any of the
compile methods fail, <span class="code">and_then</span> short circuits the
happy path and jumps to the end.

## Matching Result Types

To chain <span class="code">and_then</span> methods together like this, I used
the same input and output types for each of the compile methods:

<pre type="rust">
fn compile_posts(params: CompileParams) -> Result<CompileParams, InvalidPostError>
</pre>

<pre type="rust">
fn compile_home_page(params: CompileParams) -> Result<CompileParams, InvalidPostError>
</pre>

<pre type="rust">
fn compile_rss_feed(params: CompileParams) -> Result<CompileParams, InvalidPostError>
</pre>

Each method expects a <span class="code">CompileParams</span> struct, and
returns one wrapped in <span class="code">Result</span>. Rust unwraps the <span
class="code">CompileParams</span> from one call and passes it to the next.

I use <span class="code">InvalidPostError</span> throughout my code to provide
a consistent way to return errors. This was a bit of a challenge at first,
until I realized it was easy to cast other types of errors into
<span class="code">InvalidPostError</span> like this:

<pre type="rust">
impl From<std::io::Error> for InvalidPostError {
  fn from(e: std::io::Error) -> InvalidPostError {
      let message = format!("{}", e);
      InvalidPostError::new(&message)
  }
}
</pre>

Now the Rust compiler knows how to map a <span
class="code">std::io::Error</span> into an <span
class="code">InvalidPostError</span>.

## Error Handling the Old Fashioned Way

Here’s the code I didn’t have to write. (This example uses Ruby; substitute
your favorite language here which doesn't support [monadic error
handling](https://medium.com/@huund/monadic-error-handling-1e2ce66e3810).)

<pre type="ruby">
if compile_posts
  if compile_home_page
    if compile_rss_feed
      puts "Success!"
    else
      puts "Error compiling RSS Feed"
    end
  else
    puts "Error compiling home page"
  end
else
  puts "Error compiling a blog post"
end
</pre>

I didn’t have to write a series of if/else blocks. This would have been tedious
to write and tedious to read. And I probably would have forgotten (or have been
too lazy) to check one of the return values.

And I didn’t have to write this code either:

<pre type="ruby">
def compile_posts
  raise InvalidPostError.new("Failed compiling the posts")
end

def compile_home_page
  raise InvalidPostError.new("Failed compiling the home page")
end

def compile_rss_feed
  raise InvalidPostError.new("Failed compiling the RSS feed")
end

begin
  compile_posts
  compile_home_page
  compile_rss_feed
  puts "Success"
rescue InvalidPostError => e
  puts e.message
end
</pre>

Once again this is fragile: I might raise the wrong exception type or not raise
one at all. Or I might rescue the wrong type. Worse, there’s no indication at
the call site for the compile methods what might happen.

To be honest, I probably won’t bother handling errors at all for a simple Ruby
script like this. If an exception happens someday while building my blog site,
then I’ll deal with it then. Probably the exception’s backtrace and message
will make it obvious what the problem might be.

## Result: Both Thorough and Succinct

The best thing about using <span class="code">and_then</span> and the other
powerful functions Result provides, is that they lead to very terse, readable
code. Compare the Ruby version with no error checking:

<pre type="rust">
compile_posts
compile_home_page
compile_rss_feed
</pre>

With the Rust version that has thorough, exhaustive error checking enforced at compile time:

<pre type="ruby">
Ok(params).and_then(compile_posts)
          .and_then(compile_home_page)
          .and_then(compile_rss_feed)
</pre>

The Rust version looks just as simple as the Ruby version, but don’t be fooled.
The Rust <span class="code">and_then</span> function provides me a simple way
to structure my control flow around error handling
