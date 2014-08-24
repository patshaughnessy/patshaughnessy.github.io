title: "Use An Ask, Don’t Tell Policy With Ruby"
date: 2014/2/10

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/2/10/innisfree.jpg"><br/>
  <i>Innisfree is an actual island in Lough Gill, County Sligo, Ireland</i>
</div>

The next time you need to develop a new algorithm, ask Ruby for what you want,
don’t tell it what to do. Don’t think of your code as a series of instructions
for the computer to follow. Instead, ask Ruby for what you need: Your code
should state the solution to your problem, even if you’re not sure what that
solution is yet! Then dive into more and more detail, filling in your
solution’s gaps as you do. This can lead to a more expressive, functional
solution that you might not find otherwise.

Too often over the years I’ve written code that consists of instructions for
the computer to follow. Do this, do that, and then finish by doing this third
thing. As I write code I imagine I am the computer, in a way, asking myself:
What do I need to do first to solve this problem? When I decide, this becomes
the first line of code in my program. Then I continue, writing each line of
code as another instruction for the computer to follow.

But what does “Ask, Don’t Tell” mean exactly? And how could Ruby possibly know
the answer when I ask it something? An example will help you understand what I
mean.

## Parsing a Yeats Poem

Last week I needed to parse a text file to obtain the lines of text that
appeared after a certain word. My actual task was very boring (separating blog
articles from their metadata), so instead let’s work with something more
beautiful, _The Lake Isle Of Innisfree_:

<p></p>

<blockquote>
  I will arise and go now, and go to Innisfree,<br/>
  And a small cabin build there, of clay and wattles made:<br/>
  Nine bean-rows will I have there, a hive for the honeybee,<br/>
  And live alone in the bee-loud glade.<br/>
  <br/>
  And I shall have some peace there, for peace comes dropping slow,<br/>
  Dropping from the veils of the morning to where the cricket sings;<br/>
  There midnight's all a glimmer, and noon a purple glow,<br/>
  And evening full of the linnet's wings.<br/>
  <br/>
  I will arise and go now, for always night and day<br/>
  I hear lake water lapping with low sounds by the shore;<br/>
  While I stand on the roadway, or on the pavements grey,<br/>
  I hear it in the deep heart's core.<br/>
</blockquote>

My task is to write a Ruby script to return the line that contains a given
word, along with the following lines:

<img src="http://patshaughnessy.net/assets/2014/2/10/console.png"><br/>

## Telling Ruby What To Do

When I first wrote this script, I put myself in the computer’s shoes: What do I
need to do to find the target word? I started writing instructions for Ruby to
follow.

First I need to open the file and read in the poem:

<img src="http://patshaughnessy.net/assets/2014/2/10/one.png"><br/>

Here <span class="code">File#readlines</span> saves all the lines of text into an array, which the <span class="code">parse</span>
method will process, returning the result in another array. Later I join the
result lines together and print them out.

How do I implement <span class="code">parse</span>? Again, I imagine that I am the computer, that I am
Ruby. How do I find the lines that follow _glimmer_? Well, obviously I need to
loop through the array looking for the target word.

<img src="http://patshaughnessy.net/assets/2014/2/10/two.png"><br/>

Once I find the word, I’ll start saving the lines into a new array called
<span class="code">result</span>. Since I want to save all the following lines and not just the matching
line, I’ll also use a boolean flag to keep track of whether I’ve already seen
the target.

<img src="http://patshaughnessy.net/assets/2014/2/10/three.png"><br/>

What’s wrong with this code? Nothing really. It works just fine, and it’s
even somewhat idiomatic Ruby. In the past, I would have probably considered
this done and moved on.

However, I can do better than this. I can ask Ruby for what I want, instead of
telling Ruby what to do.

## Ask Ruby For What You Want

Don’t imagine you are the computer. Don’t think about how to solve a problem by
figuring out what Ruby should do and then writing down instructions for it to
follow. Instead, start by asking Ruby for the answer.

What should my method return? An array of the lines that appear after the
target word. To reflect this, I’ll rename my method from <span class="code">parse</span> (telling Ruby
what to do) to <span class="code">lines_after</span> (asking Ruby for what I want).


This might seem like an unimportant detail, but naming methods is one of the
most difficult and important things a programmer does. Picking a name for a
method gives the reader a hint about what the method does, about what your
intentions were when you wrote it. Think of writing code the same way you would
think of writing an essay or story. You want your readers to understand what
you are saying, and to be able to follow along. (You also want them to enjoy
reading enough that they consider the code to be their own someday.)

To get started I’ll write the new method to return an empty array.

<img src="http://patshaughnessy.net/assets/2014/2/10/four.png"><br/>

Notice on the left I changed the label from “Instructions:” to “What do I
want?” This reflects my new way of thinking about the problem.

Now, what does “appear after the target word” mean exactly? It means the lines
that appear in the array after (and including) the line containing the target.
Ah… in other words, the <span class="code">lines_after</span> method should return a subset or slice of the
array. Rewriting the problem in a different way lead me towards a solution I
hadn't thought of before.

Now I can rewrite the “What do I want?” text like this:

<img src="http://patshaughnessy.net/assets/2014/2/10/five.png"><br/>

I rewrote what I want from Ruby to be more specific: I want a “portion of the
array” and I want the portion “including and following the line containing the
target.” I haven’t written much code yet, but I’ve taken a big step forward in
how I think about the problem.

On the right, I’ve written code to return a subset of the array,
<span class="code">lines[target_index..-1]</span>. But my solution is still incomplete; what should
<span class="code">target_index</span> be?

Thinking about this a bit, it’s easy to see how to find the line containing the
target string: I can use <span class="code">detect</span> to find the line that includes the target word.

<img src="http://patshaughnessy.net/assets/2014/2/10/six.png"><br/>

But I’m still not done. I need the index of the line containing the target, not
the line itself. How can I find <span class="code">target_index</span>? Again, I shouldn’t tell Ruby what
to do (maybe create a local variable and loop through the lines checking each
one). Instead, I should ask Ruby for what I need. What do I need? I need the
index which corresponds to the line containing the target. In other words, I
need to find (to detect) the target index, not the target line.

Here’s how to do it:

<img src="http://patshaughnessy.net/assets/2014/2/10/seven.png"><br/>

Here I use Ruby’s <span class="code">detect</span> method to search a range of index values, not lines.
Inside the block I check whether the line corresponding to each index
(<span class="code">lines[i]</span>) contains the target. At the bottom I return the correct slice of the
array if I found the target, or an empty array if I didn’t.

## Learning From Functional Languages

In my opinion this code is better than what I showed earlier. Why? They both
work equally well. What’s the difference? Let's take a look at them side-by-side.

<img src="http://patshaughnessy.net/assets/2014/2/10/compare.png"><br/>

First of all, I have simpler, more terse code. Less code is better. The <span class="code">lines_after</span>
method contains just 4 lines of code while the <span class="code">parse</span> method
contains 9. Of course, I could find ways to rewrite <span class="code">parse</span> to use
fewer lines, but any way you look at it <span class="code">lines_after</span> is simpler than <span class="code">parse</span>.

The <span class="code">parse</span> method contains two local variables which are changed, or _mutated_, by
code inside the loop. This makes the method harder to understand. What is the
value of <span class="code">flag</span>? What about <span class="code">result</span>? To really understand how <span class="code">parse</span> works you
almost need to simulate the loop inside your head, thinking about how the flag
and result values change over time.

The <span class="code">lines_after</span> method also contains two local variables. However, they aren’t used
in the same way - they aren’t changed as the program runs. The block
parameter, <span class="code">i</span>, while different each time the block is called, doesn’t change
inside the block. It’s meaning is clear and unambiguous while that block is
running. Similarly, the <span class="code">target_index</span> variable is set once to an intermediate
value, not changed once each time around a loop.

Terse, simple code that doesn’t change values while it is running is the
hallmark of functional programming languages like Haskell or Clojure. While
these languages allow you to write concurrent code without using locks, their
chief benefit is that they encourage (Clojure) or even force you (Haskell) to write simple, terse
code. Code that asks the computer for what you need, not code that tells the
computer what to do.

But, as we’ve seen, you don’t need to abandon Ruby to write functional code.

<b>Update:</b> Simon Kröger and Josh Cheek both suggested using <span
  class="code">drop_while</span>, which gives us an even more readable,
functional solution:

<img src="http://patshaughnessy.net/assets/2014/2/10/eight.png"><br/>

I also decided to rename the <span class="code">after</span> method to <span
  class="code">lines_after</span>, based on the comments from TenderGlove and
John Kary. I agree with them <span class="code">after</span> would make more sense if I
called it as a method on an object containing the lines (e.g. <span class="code">lines.after</span>). But as a simple
function like in this example <span class="code">lines_after</span> is more
expressive.

Thanks guys!

## Learning From Sandi Metz 

In her famous book, [Practical Object-Oriented Design in Ruby](http://www.poodr.com), Sandi Metz
mentions the Ask, Don’t Tell policy also, but using slightly different words.
With her brilliant bicycle examples, Sandy shows us in Chapter 4 of
POODR why we should be _Asking for "What" Instead of Telling "How"_. When you
send a message to an object, you should ask it for what you want, not tell it
what to do or make assumptions about how it works internally. Sandi shows us
how this policy - along with other important design principles - helps us write
classes that are more independent and decoupled one from the other.

The Ask, Don’t Tell policy applies equally well to functional programming and
object oriented programming. At a lower level, it helps us write more terse,
functional Ruby methods. Stepping back, it can also help us design object
oriented applications that are easier to maintain and extend. 

<b>Update #2:</b> Apparently I’ve (unknowingly) conflated “Ask, Don’t Tell” with the
“Tell, Don’t Ask,” advice Dave Thomas has been giving us for years to make a
different but related point about object oriented design.  Dave explains here:
[Telling, Asking, and the Power of
Jargon](http://pragdave.me/blog/2014/02/11/telling-asking-and-the-power-of-jargon/).
He also disagrees with my opinion that the <span
  class="code">parse_lines</span> example was written in a functional style.
