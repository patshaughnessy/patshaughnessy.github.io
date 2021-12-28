title: "Using Result Combinator Functions in Rust"
date: 2019/11/19
tag: Rust

<div style="float: right; padding: 8px 0px 20px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2019/11/19/train-yard.jpeg"><br/>
  <i>Rust’s Result type can help you control your program’s<br/>
  flow by checking for errors in a succinct, elegant way</i>
</div>

Using Rust for the first time, error handling was my biggest stumbling block.
Was this value a <span class="code">Result<T, E></span> or just a T?  And the
right T? The right E? I couldn’t just write the code I wanted to write. It
felt confusing and overly elaborate.

But after a while, I started to get a feel for the basics of using <span
class="code">Result</span>. I discovered that the combinator methods Result
provides, like <span class="code">map</span>, <span class="code">or_else</span>
and <span class="code">ok</span>, made error handling fun. Well, maybe
that's a bit of an overstatement. They made using <span
class="code">Result</span> a bit easier, at least.

So far my favorite <span class="code">Result</span> combinator method is
<a
href="https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then"><span
class="code">and_then</span></a>. Using <span class="code">and_then</span> _is_
actually fun! For example, I wrote [this Rust
code](https://github.com/patshaughnessy/patshaughnessy.github.io/blob/master/src/lib.rs#L43)
to generate the static HTML pages for this blog site:

<pre type="rust">
let count = all_posts.len();
all_posts.sort_by_key(|p| Reverse(p.date));
let params = CompileParams {all_posts: all_posts, output_path: output_path, draft: draft};
Ok(params).and_then(compile_posts)
          .and_then(compile_home_page)
          .and_then(compile_rss_feed)
          .map(|_output| count)
</pre>

Ignoring the details about sorting and counting, my code:
* First creates a struct holding input parameters, and wraps it using <span class="code">Ok(params)</span>
* _And then_ tries to compile all the posts in my blog, passing in the input parameters
* _And then_ if this was successful, it tries to compile the home page
(index.html)
* _And then_ if this was successful, it tries to compile the RSS feed (index.xml)

If there was an error at any time in this process, it short circuits and stops.
Here’s a flowchart that illustrates this control flow:

<div style="margin-left: auto; margin-right: auto; width:235px">
<br/>
<img src="https://patshaughnessy.net/assets/2019/11/19/flowchart.png">
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

Now the Rust compiler knows how to map a <span class="code">std::io::Error</span> into an <span class="code">InvalidPostError</span>.

## Error Handling the Old Fashioned Way

Here’s the code I didn’t have to write: (This is Ruby; substitute your favorite
PL that doesn't support [monadic error
handling](https://medium.com/@huund/monadic-error-handling-1e2ce66e3810).)

<pre type="ruby">
if compile_posts(params)
  if compile_home_page(params)
    if compile_rss_feed(params)
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
def compile_posts(params)
  raise InvalidPostError.new("Failed compiling the posts")
end

def compile_home_page(params)
  raise InvalidPostError.new("Failed compiling the home page")
end

def compile_rss_feed(params)
  raise InvalidPostError.new("Failed compiling the RSS feed")
end

begin
  compile_posts(params)
  compile_home_page(params)
  compile_rss_feed(params)
  puts "Success"
rescue InvalidPostError => e
  puts e.message
end
</pre>

Once again this is fragile: I might raise the wrong exception type or not raise
one at all. Or I might rescue the wrong type. Worse, there’s no indication at
the call site what might happen.

To be honest, I probably won’t bother handling errors at all for a simple Ruby
script like this. If an exception happens someday while building my blog site,
then I’ll deal with it then. I’d probably just write this code:

<pre type="ruby">
compile_posts(params)
compile_home_page(params)
compile_rss_feed(params)
puts "Success"
</pre>

## Rust Error Handling: Easy To Read, Hard To Write

Combining results together using <span class="code">and_then</span> and other
<span class="code">Result</span> functions enables me to write error checking
code in a natural, succinct way:

<pre type="rust">
Ok(params).and_then(compile_posts)
          .and_then(compile_home_page)
          .and_then(compile_rss_feed)
</pre>

This is just as simple to read as the Ruby version above that doesn’t check for
any errors. While it’s harder to write, having the Rust compiler check my
thought process as I piece together different code paths is a huge help.
Learning to use and get along with the Rust compiler is worth it: You end up
with code that is both readable and correct.
