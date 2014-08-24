title: "Ruby 2.0 Works Hard So You Can Be Lazy"
date: 2013/4/3
tag: Ruby 2.0 Internals

<div style="float: left; padding: 8px 30px 10px 0px;
line-height:16px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td align="center"><img src="http://patshaughnessy.net/assets/2013/4/3/work1.jpg"></td></tr>
    <tr><td align="center"><i>Lazy enumeration isn’t magic;<br/>it’s just a matter of hard work</i></td></tr>
  </table>
</div>

Ruby 2.0’s new lazy enumerator feature seems like magic. In case you haven’t
tried it yet, it allows you to iterate over an infinite series of values and
take just the values you want. It brings the functional programming concept of
lazy evaluation to Ruby - at least for enumerations.

For example, in Ruby 1.9 and earlier you would run into an endless loop if you
tried to iterate over an infinite range:

<img src="http://patshaughnessy.net/assets/2013/4/3/code1.png"/>

Here the call to <span class="code">collect</span> starts an endless loop; the
call to <span class="code">first</span> never happens. However, if you upgrade
to Ruby 2.0 and use the new <span class="code">Enumerable#lazy</span> method,
you can avoid the endless loop and get just the values you need:

<img src="http://patshaughnessy.net/assets/2013/4/3/code2.png"/>

But how does lazy evaluation actually work? How does Ruby know I only want ten
values, in this example? All I have to do is make the simple call to the <span class="code">lazy</span>
method and it just works.

It seems like magic, but actually it’s just a matter of hard work. A lot
happens inside of Ruby when you call <span class="code">lazy</span>. To give
you just the values you need, Ruby automatically creates and uses many
different types of internal Ruby objects. Like heavy equipment at a work site,
these objects work together to process the input values from my infinite range
in just the right way. What are these objects? What do they do? How do they
work together? Let’s find out!

## The Enumerable module: many different ways of calling “each”

When I call <span class="code">collect</span> on the range above I’m using
Ruby’s <span class="code">Enumerable</span> module.  As you probably know, this
module contains a series of methods, such as <span class="code">select</span>, 
<span class="code">detect</span>, <span class="code">any?</span> and many more,
that process lists of values in different ways. Internally, all of these
methods work by calling <span class="code">each</span> on the target object or
receiver:

<img src="http://patshaughnessy.net/assets/2013/4/3/collect1.png"/>

<div style="float: right; padding: 17px 0px 10px 30px;
line-height:16px">
  <img src="http://patshaughnessy.net/assets/2013/4/3/work2.jpg">
</div>

You can think of the <span class="code">Enumerable</span> methods as a series
of different types of machines that operate on data in different ways, all via
the <span class="code">each</span> method:

<img src="http://patshaughnessy.net/assets/2013/4/3/select-any.png"/>

## Enumerable is eager

Many of the <span class="code">Enumerable</span> methods, including <span
  class="code">collect</span>, return an array of values.  Since the <span
  class="code">Array</span> class also includes the <span
  class="code">Enumerable</span> module and responds to <span
  class="code">each</span>, you can chain different <span
  class="code">Enumerable</span> methods together easily:

<img src="http://patshaughnessy.net/assets/2013/4/3/collect-first.png"/>

In my code example above, the <span class="code">Enumerable#first</span> method
calls <span class="code">each</span> on the result of <span
  class="code">Enumerable#collect</span>, an array which was generated in turn
by another call to <span class="code">each</span> on the input range.

One important detail to notice here is that both <span
  class="code">Enumerable#collect</span> and <span
  class="code">Enumerable#first</span> are eager: this means that they process
all of the values returned by <span class="code">each</span> before returning
the new array value. So in my example, first <span class="code">collect</span>
processes all the values from the range and saves the results into the first
array. Then in a second step <span class="code">first</span> processes all the
values from the first array, placing the results into the second array:

<img src="http://patshaughnessy.net/assets/2013/4/3/two-steps.png"/>

This is what leads to the endless loop for an infinite range; since <span
  class="code">Range#each</span> will never stop returning values, <span
  class="code">Enumerable#collect</span> will never finish, and <span
  class="code">Enumerable#first</span> will never get a chance to stop the
iteration.

<img src="http://patshaughnessy.net/assets/2013/4/3/endless-loop.png"/>

<div style="float: left; padding: 47px 30px 10px 0px;
line-height:16px">
  <img src="http://patshaughnessy.net/assets/2013/4/3/work3.jpg">
</div>

## The Enumerator object: deferred enumeration

One interesting trick you can use with the <span class="code">Enumerable</span>
module’s methods is to call them without providing a block. For example,
suppose I call <span class="code">collect</span> on my range, but I don’t
provide a block:

<img src="http://patshaughnessy.net/assets/2013/4/3/code3.png"/>

Here Ruby has prepared an object you can use later to actually enumerate over
the range, called an “Enumerator.” As you can see from the inspect string, Ruby
has saved a reference to the receiver (<span class="code">1..10</span>) along with the name of the
enumerable method I want to use (<span
  class="code">collect</span>) inside the enumerator object.

<img src="http://patshaughnessy.net/assets/2013/4/3/enumerator-collect.png"/>

Later when I want to actually iterate through the range and collect the values
in an array, I can just call <span class="code">each</span> on the enumerator:

<img src="http://patshaughnessy.net/assets/2013/4/3/code4.png"/>

There are a few other ways of using enumerators, such as calling <span class="code">next</span>
repeatedly, which I don’t have time to discuss today.

## Enumerator::Generator - generating new values for enumeration

In my previous examples I used a <span class="code">Range</span> object to produce a series of values.
However, the <span class="code">Enumerator</span> class provides another more
flexible way of generating a series of values using a block. Here’s an example:

<img src="http://patshaughnessy.net/assets/2013/4/3/enumerator-new.png"/>

Let’s take a look at what sort of enumerator this is:

<img src="http://patshaughnessy.net/assets/2013/4/3/inspect-enum.png"/>

As you can see, Ruby has created a new enumerator object that contains a
reference to an internal object called <span
  class="code">Enumerator::Generator</span>, and has setup to call the <span
  class="code">each</span> method on that generator. Internally, the generator
object converts the block I provided above into a <span
  class="code">Proc</span> object and saves it away:

<img src="http://patshaughnessy.net/assets/2013/4/3/enum-generator.png"/>

Now when I use the <span class="code">Enumerator</span> object, Ruby will call
the <span class="code">Proc</span> saved inside the generator to get the values
for the enumeration:

<img src="http://patshaughnessy.net/assets/2013/4/3/code5.png"/>

In other words, the <span class="code">Enumerator::Generator</span> object is a
source of data for an enumeration - it “generates” the values and passes them
along.

## Enumerator::Yielder - allowing one block to yield to another

If you take a close look at the code above, there’s something strange about it.
I first created the <span class="code">Enumerator</span> object using a block:

<img src="http://patshaughnessy.net/assets/2013/4/3/enumerator-new.png"/>

…which yields values to a second block I provide later when I call each:

<img src="http://patshaughnessy.net/assets/2013/4/3/code5.png"/>

In other words, the enumerator somehow allows you to yield values directly from
one block to another:

<img src="http://patshaughnessy.net/assets/2013/4/3/two-blocks.png"/>

But of course this isn’t how Ruby works. Blocks can’t pass values directly to
each other like this. The trick to making this work is another internal object
called the <span class="code">Enumerator::Yielder</span> object, passed into
the block with the <span class="code">y</span> block parameter:

<img src="http://patshaughnessy.net/assets/2013/4/3/enumerator-new.png"/>

The <span class="code">y</span> parameter is very easy to miss here. But if you
re-read the block’s code, you’ll notice I’m not actually yielding values at
all, I’m simply calling the <span class="code">yield</span> method on the <span
  class="code">y</span> object, which is an instance of the built in <span
  class="code">Enumerator::Yielder</span> class. You can see and use this
class for yourself in IRB as follows:

<img src="http://patshaughnessy.net/assets/2013/4/3/irb1.png"/>

The yielder catches values I want the enumerator to generate, using the <span
  class="code">yield</span> method, and then later actually yields them to the
target block. As a Ruby developer, aside from calling <span
  class="code">yield</span> I don’t normally ever need to interact with the
generator or the yielder; they are used internally by the enumerator. When I
call <span class="code">each</span> on the enumerator, it uses these two
objects to generate and yield the values I want:

<img src="http://patshaughnessy.net/assets/2013/4/3/enumerator-yields.png"/>

## Enumerators generate data; Enumerable methods consume it

Stepping back for a moment, the pattern we’ve seen so far with enumerations in Ruby is:

* Enumerator objects produce data.
* Enumerable methods consume data.

<img src="http://patshaughnessy.net/assets/2013/4/3/each-and-yield.png"/>

From right to left, the enumerable method calls <span class="code">each</span>
to request data; later from left to right the enumerator object provides the
data by yielding it to a block.

<div style="float: right; padding: 47px 0px 10px 30px;
line-height:16px">
  <img src="http://patshaughnessy.net/assets/2013/4/3/work4.jpg">
</div>

## Enumerator::Lazy - putting it all together

Ruby 2.0 implements lazy evaluation using an object called <span
  class="code">Enumerator::Lazy</span>.  What makes this special is that it
plays both roles! It is an enumerator, and also contains a series of <span
  class="code">Enumerable</span> methods. It calls <span
  class="code">each</span> to obtain data from an enumeration source, and it
yields data to the rest of an enumeration:

<img src="http://patshaughnessy.net/assets/2013/4/3/left-and-right.png"/>

Since <span class="code">Enumerator::Lazy</span> plays both roles, you can
chain them up together to produce a single enumeration. This is what happens in
my infinite range example:

<img src="http://patshaughnessy.net/assets/2013/4/3/code2.png"/>

The call to <span class="code">lazy</span> produces one <span
  class="code">Enumerator::Lazy</span> object. Then when I call <span
  class="code">collect</span> on this first object, the <span
  class="code">Enumerator::Lazy#collect</span> method returns a second one:

<img src="http://patshaughnessy.net/assets/2013/4/3/lazy-chain.png"/>

You can see here that the second <span class="code">Enumerator::Lazy,</span> created by the call to
<span class="code">Enumerator::Lazy#collect,</span> also calls my block, the <span class="code">x\*x</span> code.

How does all of this work? How does <span class="code">Enumerator::Lazy</span>
do all of this? To serve both as a data producer and consumer, <span
  class="code">Enumerator::Lazy</span> uses generator and yielder objects in a
special way. The generator first calls <span class="code">each</span> to obtain
data, and then it passes each value it obtains immediately into a special
block:

<img src="http://patshaughnessy.net/assets/2013/4/3/lazy-details.png"/>

Let’s take a closer look at the block from the diagram - this block implements
the <span class="code">Enumerator::Lazy#collect</span> method. (The other lazy
enumeration methods use slightly different blocks.) Ruby implements it
internally using C code, but this is the equivalent Ruby code:

<img src="http://patshaughnessy.net/assets/2013/4/3/lazy-map.png"/>

Reading the code, we can see the block takes a yielder and a value. Then it
yields the value to another block - this is actually the block I provide to
<span class="code">Enumerator::Lazy#collect</span> or <span
  class="code">x*x</span> in my example. Then the <span
  class="code">Enumerator::Lazy#collect</span> block calls the yielder, passing
the result of my block onto the rest of the enumeration.  

This is the key to lazy evaluation in Ruby. Each value from the data source is
yielded to my block, and then the result is immediately passed along down the
enumeration chain. This enumeration is not eager - the <span class="code">Enumerator::Lazy#collect</span>
method does not collect the values into an array. Instead, each value is passed
one at a time along the chain of <span class="code">Enumerator::Lazy</span> objects, via repeated yields.
If I had chained together a series of calls to <span class="code">collect</span> or other
<span class="code">Enumerator::Lazy</span> methods, each value would be passed
along the chain from one of my blocks to the next, one at a time:

<img src="http://patshaughnessy.net/assets/2013/4/3/lazy-chain2.png"/>

## Lazy evaluation: executing code backwards

Why is this chain lazy evaluation? Why does this allow Ruby to avoid an endless loop
and provide me with just the values I need? The answer is that the code at the
end of the enumeration chain, in my example the <span
  class="code">first(10)</span> method call, controls how long the enumeration
runs:

<img src="http://patshaughnessy.net/assets/2013/4/3/code2.png"/>

At the end of the enumeration chain the values are yielded to the Enumerable#first
method:

<img src="http://patshaughnessy.net/assets/2013/4/3/lazy-chain-end.png"/>

After the <span class="code">Enumerable#first</span> method receives enough
values, 10 in my example, it stops the iteration by raising an exception.

In other words, the code at the right side of my enumeration chain, the code at
the end, actually controls the execution flow. The <span
  class="code">Enumerable#first</span> both starts the iteration by calling
<span class="code">each</span> on the lazy enumerators, and ends the iteration
by raising an exception when it has enough values.

At the end of the day, this is the key idea behind lazy evaluation: the
function or method at the end of a calculation chain starts the execution
process, and the program’s flow works backwards through the chain of function
calls until it obtains just the data inputs it needs. Ruby achieves this using
a chain of <span class="code">Enumerator::Lazy</span> objects, as we’ve seen
above. However, functional languages such as Haskell implement this in a
deeper, more fundamental way, that encompasses all execution and not just
enumeration.
