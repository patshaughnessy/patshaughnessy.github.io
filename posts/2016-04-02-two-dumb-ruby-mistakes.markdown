title: "Two Dumb Ruby Mistakes"
date: 2016/4/2

<div style="float: right; padding: 0px 0px 20px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2016/4/2/rope-climber.jpg"><br/>
  <i>Coding is like climbing: You need equipment<br/> that will catch you when you make a mistake.<br/>
	<small>(source: <a href="https://commons.wikimedia.org/wiki/File:Free_climbing_20060701.jpg">Elke Wetzig via Wikimedia Commons</a>)</small></i>
</div>

Most Ruby blog posts show you examples of code you should write: how to solve a
certain problem, how to use some library or gem, how Ruby itself works. But
today I decided to write about a few dumb mistakes I’ve made using Ruby
recently. Read on to see two things you _should not do_ with the Ruby language,
for a change.

The depressing thing about this is that I made these dumb mistakes just in the
past few weeks! I’ve been using Ruby professionally every day for eight years,
I’ve researched and written about Ruby in my spare time as a hobby, and _I still
make dumb mistakes with the language all the time!_

Coding is like climbing: Sooner or later we all make mistakes and fall. What
you need to do is plan on this happening and use the appropriate equipment to
avoid disaster. Climbers use carabiners, ropes and harnesses to catch them when
they fall. Developers should use a language that will catch them when they make
dumb mistakes.

## Searching For An Array Element

Let’s start with some test data. Here’s an array of <span
class="code">Person</span> objects, each with a first name, last name and an
insult count:

<pre type="ruby">
	Person = Struct.new(:first_name, :last_name, :insults)
	candidates = [ 
		Person.new('Ted', 'Cruz', 432),
		Person.new('Donald', 'Trump', 892),
		Person.new('Marco', 'Rubio', 321)
	]
</pre>

A couple of weeks ago (using different data of course) I wrote this line of
code to search for a specific element in the array:

<img src="http://patshaughnessy.net/assets/2016/4/2/mistake1.png"/>

<div style="clear: both"></div>

What I wanted was the first person in the array named “Marco.” Instead when I
ran the code what I got was the first element of the array, but with the first
name set to “Marco:”

<pre type="ruby">
p marco
=> #<struct Person first_name="Marco", last_name="Cruz", insults=432>
</pre>

Of course, I should have known better. The proper line of code is:

<img src="http://patshaughnessy.net/assets/2016/4/2/correct1.png"/>

I should have used <span class="code">==</span> instead of <span
class="code">=</span>. What a dumb mistake. I can’t believe I wrote this code;
how embarrassing! I’m sure you all saw the problem right away, and maybe a few
of you have made the same mistake before. But let’s walk through what happened
when I ran the incorrect code, just to be sure we thoroughly understand the
problem.

Ruby started with the <span class="code">candidates</span> array, and called
the <span class="code">find</span> method on it:

<img src="http://patshaughnessy.net/assets/2016/4/2/array1.svg"/>

The <span class="code">find</span> method is actually a member of the <span
class="code">Enumerable</span> module, which Ruby includes automatically into
the <span class="code">Array</span> class. When <span class="code">find</span>
ran, it iterated over the elements of the array and called the block I
provided, passing in each element. The first element was the “Ted Cruz” person
object:

<img src="http://patshaughnessy.net/assets/2016/4/2/call-block1.svg"/>

Now the block executed. And my dumb mistake came into play. What I intended was
for the block to return whether or not the first name of the given person was
equal to “Marco.” If the first name was “Marco” then <span
class="code">Person#first_name == &quot;Marco&quot;</span> would return true, the block
would return <span class="code">true</span> and <span
class="code">Enumerable#find</span> would return the target person. In this
case, "Ted" is not "Marco" so the block would return <span
class="code">false</span>.

But my block didn’t check whether the person is named “Marco;” instead, it
called the <span class="code">Person#first_name=</span> method, setting the
person’s name to “Marco!”

<img src="http://patshaughnessy.net/assets/2016/4/2/set-first-name.svg"/>

And now, to make matters worse, the block returned the value returned by
<span class="code">Person#first_name=</span>, which was the string “Marco,” the new value of the first
name attribute. Because Ruby considered “Marco” to be _truthy_, <span class="code">Enumerable#find</span>
returned the first person, even though that person was originally named Ted
Cruz. My surrounding code now thinks it found Marco Rubio, but instead has Ted
Cruz, renamed to Marco Cruz. What a mess.

## Why Didn’t Ruby Tell Me Something Was Wrong?

<div style="float: right; padding: 0px 0px 20px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2016/4/2/bouldering.jpg"><br/>
  <i> As a developer, you’re always just one<br/> keystroke away from falling off a cliff.  <br/>
	<small>(source: <a href="https://commons.wikimedia.org/wiki/File:Bouldering.jpg">DecafGrub47393 via Wikimedia Commons</a>)</small></i>
</div>

Think about this for a moment: I used the <span class="code">find</span>
method, which called a block and expected that block to return <span
class="code">true</span> or <span class="code">false</span>.  But my block
returned neither <span class="code">true</span> nor <span
class="code">false</span>. It returned “Marco.”

Why didn’t Ruby issue some sort of error or warning in this case? Yes, I
understand that Ruby considers all values, except for <span
class="code">false</span> and <span class="code">nil</span>, to be equivalent
to <span class="code">true</span>. In fact, Ruby developers quite often take
advantage of this fact to write more concise readable code: We can write: <span
class="code">if value</span> instead of <span class="code">if value !=
false</span> or <span class="code">if value != nil</span> or whatever.

But in this case, Ruby’s silence allowed my simple coding mistake to become a
serious problem. If Ruby had given me some sort of warning or error the first
time I ran this code, I would have found the problem and fixed it in 5 seconds.
Instead, this code ran for weeks and failed every single time, and I had no
idea.

When I fell, Ruby didn’t catch me, it allowed me to fall off the cliff!

**Update:** Erik Michaels-Ober [pointed out today on
Twitter](https://twitter.com/sferik/status/716289922005475328) that it you
always put the variable on the right and the constant on the left, for example
like this:

<pre type="ruby">
marco = candidates.find { |person| 'Marco' = person.first_name }
</pre>

…then Ruby will immediately report a syntax error and tell you where the
problem was if you ever confuse <span class="code">=</span> with <span
class="code">==</span>. Joshua Ballanco told us that this style of putting the
constant before the variable is known as a [Yoda
condition](https://en.wikipedia.org/wiki/Yoda_conditions).


## Finding The Maximum Value in an Array

We all have a bad day from time to time. After making that mistake I just
continued to work on my project, trying harder not to make any more dumb
mistakes. It was my fault, I thought. I just needed to be a better programmer.

But of course, it happened again! I made another dumb Ruby mistake just a few
days later. This time I wanted to sort the same array. Specifically, I wanted
to find the array element with the maximum value for some attribute. I was
using different data, of course, but we can translate the problem to our
candidate data set easily.

Suppose I wanted to find the candidate with the maximum number of insults.
Easy, right? Here’s the line of code I wrote:

<img src="http://patshaughnessy.net/assets/2016/4/2/mistake2.png"/>

Can you spot the problem here? When I run that code I don’t get Donald Trump, who has the most insults. Instead, I get:

<pre type="ruby">
p most_insulting
=> #<struct Person first_name="Marco", last_name="Rubio", insults=321>
</pre>

Again a simple, dumb mistake. I should have called <span
class="code">max_by</span>, instead of <span class="code">max</span>. Here’s
the correct code:

<img src="http://patshaughnessy.net/assets/2016/4/2/correct2.png"/>

<span class="code">Enumerable#max_by</span> does what I thought <span
class="code">Enumerable#max</span> would do: It sorts the values returned by
the block, and then returns the object corresponding to the maximum value.
This is only slightly less embarrassing than my first dumb mistake. Almost all
modern programming languages use <span class="code">==</span> and <span
class="code">=</span> for equality vs. assignment. There’s no excuse for making
that mistake: It was just dumb.

The difference between <span class="code">max</span> and <span
class="code">max_by</span> is not quite as obvious. But again, I’ve been using
Ruby for 8 years now. I should know better! I’m just a bad Ruby developer. But
before we blame this mistake entirely on me, let’s take a closer look at what
actually happened when I ran my bad code. Let’s step through what <span
class="code">Enumerable#max</span> did, just as we did before with <span
class="code">Enumerable#find</span>.

Again Ruby started by calling <span class="code">Enumerable#max</span> on the
candidates array:

<img src="http://patshaughnessy.net/assets/2016/4/2/array2.svg"/>

And again, just like </span>find</span>, <span class="code">max</span> iterates
over the array elements. However, instead of passing each person to the block
one at a time, it actually passes the array elements in pairs:

<img src="http://patshaughnessy.net/assets/2016/4/2/call-block2.svg"/>

Why did Ruby pass two <span class="code">Person</span> objects to my block?
<span class="code">Enumerable#max</span> searches for the array element - not
the return value of a block - which has the maximum value. It assumes that the
values in the array can be compared, that they have a natural sort order.
<span class="code">Enumerable#max</span> is perfect for an array of integers or
an array of strings. Ruby can sort them automatically and find the maximum
value by returning the last element.

Additionally, Ruby allows you to use <span class="code">max</span> when the
array elements can’t be sorted automatically, when you have an array of
objects, like my <span class="code">Person</span> structures. Because Ruby
doesn’t know whether one person is greater or less than another, it allows you
to pass a block to <span class="code">max</span> that answers that question.
The block should accept two arguments return one of three numeric values: -1, 0
or 1:

* -1 if the first value is less than the second (they are in ascending order)

* 0 indicates they are the same, at least in terms of their sort order, and

* 1 if the first value is greater than the second (they are in descending order)

So what happened here was that by using <span
class="code">Enumerable#max</span> and providing a block, Ruby assumed my block
was there to determine the sort order of the Person objects, not to return an
attribute for each one.

As you probably know, Ruby makes our lives easier by providing the “space ship”
operator, <span class="code"><=></span>, that compares two values and returns
this sort order number: -1, 0 or 1. The correct way to find the most insulting
candidate using <span class="code">max</span> would be to compare the two
values of <span class="code">Person#insults</span> using <span
class="code"><=>:

<pre type="ruby">
most_insulting = candidates.max{|person1, person2| person1.insults <=> person2.insults}
p most_insulting
=> #<struct Person first_name="Donald", last_name="Trump", insults=892>
</pre>

## Why Didn’t Ruby Tell Me Something Was Wrong?

I knew all about the space ship operator and sort order blocks, but for
whatever reason in the moment I typed in my bad code I just forgot. Maybe I was
in a rush, maybe I was just tired. Maybe I really thought I typed <span
class="code">max_by</span> but somehow the “\_by” part just didn’t leave my
brain and make it to the keyboard.

But Ruby knew I should have used <span class="code">max_by</span>, or least
that I should have accepted two parameters in my block. Why didn’t it tell me?

That is, my block expected only one argument, not two. I wrote:

<pre type="ruby">
{|person| etc…}
</pre>

and not:

<pre type="ruby">
{|person1, person2| etc… }
</pre>

Why didn’t Ruby complain when it tried to pass two objects, but my block
only accepted one? It turns out when you pass extra arguments to a block Ruby
silently ignores them. Note: Ruby does check the number of arguments when you
explicitly use <span class="code">lambda{}</span> or <span
class="code">->()</span> and then call it using the <span
class="code">Proc.call</span> method.  But 99% of the time Ruby developers use
blocks in the standard, default manner and don’t create <span
class="code">Proc</span> objects explicitly.

Ruby could have told me something was wrong by displaying a warning or an error
message, maybe: “wrong number of arguments (2 for 1) (ArgumentError).” But
instead, it remained silent. It assumed that I just didn’t need that second
block argument, that I wanted to keep my code simpler and easier to read, and
conveniently allowed me to leave it out of the block’s argument list. Ruby
assumed I was a smart, experienced developer who doesn’t make dumb mistakes
like this. Ruby was so wrong!

What happened next? Ruby continued to run my block, and things got really ugly.
Take another look at the block’s code:

<pre type="ruby">
{|person| person.insults}
</pre>

It returns the insult count for the given person - a number! Next Ruby
interpreted the numerical value my block returned, 432, 892 or 321, as the sort
order indicator. That’s right: Ruby will accept any positive value from the
sort order block, not just 1, and consider that to mean the two objects are in
descending order. Similarly, it will take any negative value to mean the values
are in ascending order.

Again, Ruby could have told me: “wrong type for block return value (Integer for
SortOrder) (TypeError).” But, of course, Ruby isn’t a statically typed
language. It doesn't check the types of method and block arguments, or their
return values.

<div style="float: right; padding: 70px 0px 30px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2016/4/2/carabiner.jpg"><br/>
  <i>Your coding equipment should catch you<br/>when you make a mistake and fall.</i><br/>
	<small>(source: <a
href="https://commons.wikimedia.org/wiki/File:Carabiner.jpg">Marcin Jahr via
Wikimedia Commons</a>)</small></i> </div>

Once again, Ruby erred on the side of convenience, and assumed I knew what I
was doing. It conveniently allowed me to return 321 instead of 1, just in case
I really wanted to return 321 without having to convert it to 1.  As we’ve
seen, I make dumb mistakes all the time. Ruby is very wrong to believe I know
what I am doing.

## Our Programming Language Should Catch Our Dumb Mistakes

We actually make dumb mistakes all the time, not just once or twice a week, but
probably hundreds of times every day. Every time we misspell a keyword, forget
a method argument, or use an API the wrong way we have made a mistake. But we
don’t think of these mistakes as mistakes - they are just how we work as
humans. When we type, we usually press the backspace key quite often. When we
use an API or run shell commands we have to check the documentation or
StackOverflow to remind ourselves what arguments or options to use.

And usually our programming language, whether it’s Ruby or something else,
finds our mistakes immediately and tells us about them with a syntax error
message. We correct the mistake within seconds and continue coding, climbing
higher and higher up the cliff. But in my two examples the mistakes,
unfortunately, weren’t apparent immediately.  This incorrect code ran for weeks
before I discovered the problem. You always want to fail fast: The worst
mistakes are the ones you never notice until it’s too late.

But why didn’t I discover these mistakes sooner by running tests? Don’t I use
TDD? Don’t I at least write tests to check my code after I’ve written it? Yes.
But in my actual project, these mistakes were part of my test code. They
allowed my tests to pass, but caused them to return a false positive result. My
tests were green, but actually weren’t functioning at all. Tests aren't
perfect. They are only as good as the code you write to implement them.

Maybe these two dumb Ruby mistakes were exactly that: mistakes Ruby made and
not me. I’m only human; it’s normal for me to type in nonsense and garbage all
day long into the computer. But Ruby is a programming language. It’s job - it’s
most important job - is to tell me when my code is incorrect as soon as
possible. In these two examples, it was the Ruby language itself that made the
dumb mistake. The bugs weren’t in my code, they were in the language itself.

Of course, I could just switch to a statically typed language, like Java or Go.
These languages automatically check the types of arguments and return values
for me. If I used Swift I could take advantage of static types and use
blocks/closures. I could even use a language like Haskell where the type system
is so powerful that merely by allowing my code to run with no errors, the
compiler has mathematically proven my code is correct. (If this could only be
true!)

But I love Ruby. It’s a joy to use. Ruby code has a very human elegance to it
that I haven’t seen in other programming languages. I just wish Ruby would
catch me every time I fall.
