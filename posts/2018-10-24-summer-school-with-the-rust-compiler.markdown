title: "Summer School With The Rust Compiler"
date: 2018/10/24
tag: Rust

<div style="float: left; padding: 8px 30px 40px 0px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2018/10/24/steves-tweet.png"><br/>
<i> <small>(source: <a href="https://twitter.com/steveklabnik/status/1022518806814617601">Steve Klabnik via Twitter</a>)</small></i>
</div>

A few months ago, I saw this tweet from Steve. I'm not even sure what
"derridean" means, but now the image of an insane coach pops into my head every
time I try to write Rust code.

Learning Rust is hard for everyone, but it’s even worse for me because I’ve
been working with Ruby during past ten years. Switching to Rust from Ruby is
leaving an anything-goes hippie commune to a summer school for delinquent
programmers run by a sadistic and unforgiving teacher.

Why would anyone use a compiler like this? The answer is simple: to learn how
to write better code. This past summer I had some free time and decided to
convert a simple Ruby script into Rust. As you’ll see, the Rust compiler beat
me up a few times; it wasn’t easy. But after some tough love I ended up
learning something, not only about Rust but about Ruby too.

<div style="clear: both"></div>

## Iterating Over an Array in Ruby

Here’s my example program. It’s so short and simple you can read and understand
it in just a few seconds:

<pre type="ruby">
array = [1, 2, 3]
for i in array
  puts i
end
</pre>

When I ran it, the output was:

<pre class="console">
$ ruby int-loop.rb
1
2
3
</pre>

<div style="float: right; padding: 70px 0px 30px 30px; text-align: center;">
  <img src="https://patshaughnessy.net/assets/2018/10/24/garden-earthly-delights.png"><br/>
  <i><small><a href="https://en.wikipedia.org/wiki/The_Garden_of_Earthly_Delights">The Garden of Earthly Delights</a> (detail), by Hieronymus Bosch</small></i>
</div>

Ruby’s syntax and feature set are designed to make my life easier as a
developer.  Writing Ruby for me is as natural as writing English; it’s like
having a pleasant conversation with my computer. I’m living in the Garden of
Earthly Delights. If I can imagine a code change, I can write it. Using Ruby,
all of my dreams can come true.

Next I decided to increment the values before printing them out. I added just
one line of code to my example, <span class="code">i = i+1</span>:

<pre type="ruby">
array = [1, 2, 3]
for i in array
  i = i+1
  puts i
end
</pre>

As I expected, Ruby printed out 2 through 4:

<pre class="console">
$ ruby int-loop.rb
2
3
4
</pre>

Of course, there are other ways to produce the same result. I could have used
<span class="code">puts i+1</span>, or mapped the original array to a new array
<span class="code">[2, 3, 4]</span>. But Ruby doesn’t care. Today I felt like
writing <span class="code">i = i+1</span>, and Ruby let me do it without
comment. Ruby is the parent of an unruly teenager that gets away with anything.

As I found out later, using <span class="code">i = i+1</span> might have broken
a Computer Science rule or two, but I was blissfully unaware. What you don’t
know can’t hurt you. Ruby didn’t tell me anything might be wrong… but as we’ll
see Rust certainly did!

## Rust: Similar to Ruby At First Glance

I was curious: What would the Rust compiler think of this example? I was able
to rewrite it in only a few minutes:

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter() {</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

I had to type semicolons after each line and use a <span
class="code">main</span> function. A bit more typing, but really this is
exactly the same program. Running this, of course, produced the same result:

<pre class="console">
$ rustc int-loop.rs && ./int-loop
1
2
3
</pre>

Then I decided to try using the same <span class="code">i = i+1</span> line from above:

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter() {</span>
<span style="color:#000000;">        i </span><span style="color:#4f5b66;">=</span><span style="color:#000000;"> i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">;</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

## Lesson One: Passing By Reference vs. Passing By Value

Compiling this, the Rust compiler hit me over the head with Computer Science!

<pre class="console">
$ rustc int-loop.rs && ./int-loop
error[E0271]: type mismatch resolving `<&i32 as std::ops::Add<i32>>::Output == &i32`
  --> int-loop.rs:4:14
   |
 4 |         i = i+1;
   |              ^ expected i32, found &i32
   |
   = note: expected type `i32`
              found type `&i32`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
</pre>

What in the world does this mean? I wrote a very simple line of code, and got a
message straight out of type theory! The error <span class="code">type mismatch
resolving \`<&i32 as std::ops::Add>::Output == &i32\`</span> makes no sense to me at
all.

I decided to take the compiler’s suggestion and run the explain command:

<pre class="console">
$ rustc --explain E0271
This is because of a type mismatch between the associated type of some
trait (e.g. `T::Bar`, where `T` implements `trait Quux { type Bar; }`)
and another type `U` that is required to be equal to `T::Bar`, but is not.
Examples follow.
</pre>

The explain output continued for about two more pages, with examples that
didn’t resemble my code at all. What is a trait? What is an associated type? I
didn’t use any of these more advanced Rust concepts in my simple script. Maybe
I needed a PhD. in Computer Science even to try to use Rust?

Eventually, I figured it out. The key lines from the error message were:

<pre class="console">
 4 |         i = i+1;
   |              ^ expected i32, found &i32
</pre>

Rust is telling me that <span class="code">iter()</span> yielded references to
integers, but my code expected an actual integer, not a reference to an
integer. But what are references, exactly?

Running my code above, Ruby passed each integer from the array to my code as a
simple value:

<img width="185" src="https://patshaughnessy.net/assets/2018/10/24/ruby-passes-by-value.png"/>

But Rust passed each integer from the array as a reference, or in other words
as a pointer to the value itself:

<img width="287" src="https://patshaughnessy.net/assets/2018/10/24/rust-passes-by-reference.png"/>

In Ruby, of course, I didn’t have to worry about references, pointers or even
types, so none of this came up. Or at least that’s what I thought at the time.

## Lesson Two: Borrowed Values

Ah - according to the Rust compiler’s error message I just had to dereference
the reference before using it. I changed <span class="code">i = i+1</span> to
<span class="code">\*i = \*i+1</span>:

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter() {</span>
<span style="color:#000000;">        </span><span style="color:#4f5b66;">*</span><span style="color:#000000;">i </span><span style="color:#4f5b66;">= *</span><span style="color:#000000;">i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">;</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

Then Rust slapped me in the face again with more Computer Science:

<pre class="console">
$ rustc int-loop.rs && ./int-loop
error[E0594]: cannot assign to immutable borrowed content `*i`
  --> int-loop.rs:26:9
   |
26 |         *i = *i+1;
   |         ^^^^^^^^^ cannot borrow as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
</pre>

Ugh. I guess that was a bad idea. What in the world happened here? I thought I
had the dereferencing syntax correct, <span class="code">\*i</span>, the same
syntax I’m used to from C.  Actually Rust didn’t complain about types any more
or about using a reference vs. a value. But what does “borrow as mutable” mean?
And why doesn’t Rust let me do that?

Again, the problem here is that I don’t know enough Rust even to understand the
compiler’s error messages. I need to take a few months off from my day job and
read a book, or take a class. I need to understand Rust’s ownership model.

In Rust, every value is “owned” by the spot in my code where I allocate that
value. In this example, the integers and the array that contains them are owned
by the <span class="code">main</span> function. When the <span
class="code">main</span> function goes out of scope, Rust frees the memory for
that array automatically. In this diagram, the red arrow shows where Rust
allocates the array (at the top), and where Rust frees it (at the bottom):

<img width="339" src="https://patshaughnessy.net/assets/2018/10/24/rust-lifetime.png"/>

You can think of the red arrow as the “lifetime” of the array. When I pass a
value from one spot to another, when I call a function or a closure, I can
either “move” that value to the new function, or the function can ”borrow” it.
In this example, the call to <span class="code">iter()</span> borrowed the
elements inside the array, passing a reference to each element into the
closure. The blue array in this diagram indicates each element of the array,
<span class="code">i</span>, is a borrowed value inside the closure:

<img width="334" src="https://patshaughnessy.net/assets/2018/10/24/rust-borrow.png"/>

## Lesson Three: Immutable vs. Mutable Values

But using borrowed values isn’t the problem here. The problem is that my code
tries to change them, or mutate them:

<pre>
<span style="color:#4f5b66;">*</span><span style="color:#000000;">i </span><span style="color:#4f5b66;">= *</span><span style="color:#000000;">i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">;</span>
</pre>

Because the value of <span class="code">i</span> each time around the loop was
an element of the array, and because <span class="code">iter()</span> borrowed
each element from the original array, the elements are marked as immutable,
just as the array was. Or at least I that’s how I understood the previous error
message.

Back in the <span class="code">main</span> function when I typed:

<pre>
<span style="color:#a71d5d;">let</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
</pre>

…Rust created an immutable array of three integers. All variables in Rust are
immutable by default. Because it was immutable, my code can’t change it.

Ah… so the fix is to mark my array as mutable:

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let mut</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter() {</span>
<span style="color:#000000;">        </span><span style="color:#4f5b66;">*</span><span style="color:#000000;">i </span><span style="color:#4f5b66;">= *</span><span style="color:#000000;">i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">;</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

## Lesson Four: Declaring Side Effects

Running the Rust compiler again, I got the same error along with a new warning:

<pre class="console">
$ rustc int-loop.rs && ./int-loop
error[E0594]: cannot assign to immutable borrowed content `*i`
  --> int-loop.rs:14:9
   |
14 |         *i = *i+1;
   |         ^^^^^^^^^ cannot borrow as mutable

warning: variable does not need to be mutable
  --> int-loop.rs:12:9
   |
12 |     let mut array = [1, 2, 3];
   |         ----^^^^^
   |         |
   |         help: remove this `mut`
   |
</pre>

Wait - so now Rust was telling me I shouldn’t add the <span
class="code">mut</span> keyword? That my last change was dead wrong? Why was it
wrong? Probably I didn’t understand what “cannot borrow as mutable” really
meant.

It took me a while to figure this out but eventually I ran into this [great
article](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
which explained what I was doing wrong and how to fix it. I needed to use
<span class="code">iter_mut</span> instead of <span class="code">iter</span>. <span
class="code">iter_mut</span> yields mutable references to the closure, while
<span class="code">iter</span> yields normal, immutable references.

That is, by calling <span class="code">iter_mut</span> I’m declaring that the
code inside of the closure might mutate the elements of the array. This is
knowns as a _side effect_. As a side effect of the iteration, the code inside
might also change the values of the collection it is iterating over. Rust
forced me to declare that my code might change the array.

Finally, running my program with <span class="code">iter_mut</span> finally worked!

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let mut</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter_mut() {</span>
<span style="color:#000000;">        </span><span style="color:#4f5b66;">*</span><span style="color:#000000;">i </span><span style="color:#4f5b66;">= *</span><span style="color:#000000;">i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">;</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

<pre class="console">
$ rustc int-loop.rs && ./int-loop
2
3
4
</pre>

## What Rust Taught Me

My example today started out as a trivial, 4 line Ruby script. It was so
simple, there really wasn’t anything that could possibly go wrong when I ran
it. Then I added one simple line of code: i = i+1. When I added this to my Ruby
script, it worked just fine.

As we saw, this line of code got the Rust compiler very angry. It slapped me in
the face with four Computer Science lessons. I learned:

* about passing values vs. passing references.
* about mutable vs. immutable values.
* about value ownership, lifetimes and borrowing values.
* about side effects, and declaring them.

As you can see, the Rust compiler is an amazing tool you can use to learn more
about Computer Science. The problem is that it’s hard to get along with.
Compiling a Rust program will fail over and over again until you your code is
100% correct. You need to have tremendous patience to use Rust, especially as a
beginner.

Worse than that, the Rust compiler’s error messages are hard to understand, and
easy to misinterpret. They can seem to be self-contradictory as we saw above. The
Rust compiler assumes you already know what it is trying to teach you. Not only
is Rust a violent teacher, it’s a bad one. If I knew that <span
class="code">iter()</span> borrowed immutable values, if I knew what
“borrowing” and “immutable” even meant, then I likely wouldn’t have run into
that compiler error in the first place.

And Rust’s confusing error message lead me in the wrong direction. In this
example, I didn’t really want to mutate the array, I just wanted to print out
the incremented values. I could have just incremented an intermediate value and
left the original array alone. Instead, the complex error messages confused and
mislead me, and I never discovered this simpler code:

<pre>
<span style="color:#a71d5d;">fn </span><span style="color:#795da3;">main</span><span style="color:#000000;">() {</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">let</span><span style="color:#000000;"> array </span><span style="color:#4f5b66;">= </span><span style="color:#000000;">[</span><span style="color:#d08770;">1</span><span style="color:#000000;">, </span><span style="color:#d08770;">2</span><span style="color:#000000;">, </span><span style="color:#d08770;">3</span><span style="color:#000000;">];</span>
<span style="color:#000000;">    </span><span style="color:#a71d5d;">for</span><span style="color:#000000;"> i </span><span style="color:#4f5b66;">in</span><span style="color:#000000;"> array.iter() {</span>
<span style="color:#000000;">        println!(</span><span style="color:#4f5b66;">&quot;</span><span style="color:#d08770;">{}</span><span style="color:#4f5b66;">&quot;</span><span style="color:#000000;">, i</span><span style="color:#4f5b66;">+</span><span style="color:#d08770;">1</span><span style="color:#000000;">);</span>
<span style="color:#000000;">    }</span>
<span style="color:#000000;">}</span>
</pre>

The Rust compiler is an amazing tool for learning; the problem is you need to
have a deep understanding of the Rust language before you can use it
effectively. Rust needs a \-\-beginner option. Using this option on the
command line would intstruct the compiler to produce error messages designed
for Rust learners, rather than Rust experts.

## What Ruby Didn’t Tell Me

I had the opposite experience using Ruby. No confusing compiler errors; in
fact, no compiler at all. No types, no need to worry about immutability or
whether I’m passing references or values. Everything just worked.

Or did it? Because Ruby passed integers by value, the array in my original
example wasn’t modified:

<pre>
array = [1, 2, 3]
for i in array
  i = i+1
  puts i
end
puts "----"
p array
</pre>

<pre class="console">
$ ruby int-loop.rb
2
3
4
----
[1, 2, 3]
</pre>

This is probably a good thing. Side effects like mutating a collection while
iterating over it can easily lead to bugs. Maybe code later in my program
needed the original, unchanged values in that array? Maybe another thread was
trying to use that collection at the same time?

The problem with using Ruby is that you don’t know what Ruby isn’t telling you.
Because Ruby didn’t display any warnings or error messages when I added <span
class="code">i = i+1</span> to my loop, I didn’t even think about any of these
issues. Fortunately, Ruby didn't modify the array so it wasn't a problem.

But suppose my array contained strings and not integers:

<pre>
array = ["one", "two", "three"]
for str in array
    str = str << "-mutated"
    puts str
end
puts "----"
p array
</pre>

<pre class="console">
$ ruby string-loop.rb
one-mutated
two-mutated
three-mutated
----
["one-mutated", "two-mutated", "three-mutated"]
</pre>

Now the array was mutated! It turns out Ruby passed integers to the closure by
value, but strings by reference. Updating each string inside the loop also
updated that string inside the array. Now my program will have bugs, unless the
point of running that loop was to mutate the array, and not just to print it
out.
