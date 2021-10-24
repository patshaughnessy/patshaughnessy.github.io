title: "To Learn a New Language, Read Its Standard Library"
date: 2021/10/23
tag: Crystal

<div style="float: left; padding: 8px 30px 20px 0px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2021/10/23/chicken-little.png"><br/>
  <i>If I was learning to read English as a foreign language,<br/> I would need something simple to get started.<br/>
  <small>(from The Remarkable Story of Chicken Little, 1840)</small></i> 
</div>

The best way to learn a new programming language, just like a human language,
is from example. To learn how to write code you first need to read someone
else’s code. But who is the best person to learn from? Which code should we
read? Where should we look to find it?

This year in my spare time I was learning about
[Crystal](https://crystal-lang.org). I had played around with some simple
scripts, but I wanted to learn more. Then I stumbled on to Crystal’s [standard
library](https://github.com/crystal-lang/crystal/tree/master/src). I was
relieved to see that Crystal’s core classes are implemented using Crystal
itself!

Crystal’s standard library is clear, simple, concise and well documented.
Reading Crystal’s internal implementation of Array or Hash is like reading
a fairy tale in a children’s book. Anyone can understand it, even people
without a Ph.D. in Computer Science or systems programming experience.

## At First Glance, Crystal Is Ruby

At first glance, when I read Crystal’s [Array
implementation](https://github.com/crystal-lang/crystal/blob/master/src/array.cr),
I thought I was reading a Ruby program:

<pre type="ruby">
class Array(T)
  include Indexable::Mutable(T)
  include Comparable(Array)

  # Size of an Array that we consider small to do linear scans or other optimizations.
  private SMALL_ARRAY_SIZE = 16

  # The size of this array.
  @size : Int32

  # The capacity of `@buffer`.
  # Note that, because `@buffer` moves on shift, the actual
  # capacity (the allocated memory) starts at `@buffer - @offset_to_buffer`.
  # The actual capacity is also given by the `remaining_capacity` internal method.
  @capacity : Int32

  # Offset to the buffer that was originally allocated, and which needs to
  # be reallocated on resize. On shift this value gets increased, together with
  # `@buffer`. To reach the root buffer you have to do `@buffer - @offset_to_buffer`,
  # and this is also provided by the `root_buffer` internal method.
  @offset_to_buffer : Int32 = 0

  # The buffer where elements start.
  @buffer : Pointer(T)
</pre>

There are lots of familiar keywords, like `class`, `include` and `private`. I also
see Ruby’s `@` character indicating an instance variable. This code is about 100x
easier to read vs. [Ruby’s own C implementation of
Array](https://github.com/ruby/ruby/blob/master/array.c).

Along with the familiar Ruby-like syntax, notice the helpful comments. Even
though I’ve just started reading I can already make an educated guess at how
Crystal arrays function internally. I can see there’s a pointer to memory which
holds the array elements, and that the code keeps track of the capacity of this
memory along with the actual size of the array. Finally, reading the comment for
`offset_to_buffer` I can imagine there are some optimizations related to adding
and removing elements. The comment is both helpful and intriguing.

But I’m not reading Ruby code. There are important differences here: generic
type syntax and most importantly each of the instance variables is declared
with a static type known at compile time. How do I use static types in Crystal?
What types are available? What about the generic type parameter `T`? Should I
use that in my own Crystal code? What other syntax differences vs. Ruby are
there?

The best way to learn how to write Crystal code is simply to scroll down and
read one of the Array methods.

## Array#uniq

Here’s how Crystal finds the unique elements of an array:

<pre type="ruby">
def uniq
  if size <= 1
    return dup
  end

  # Heuristic: for a small array it's faster to do a linear scan
  # than creating a Hash to find out duplicates.
  if size <= SMALL_ARRAY_SIZE
    ary = Array(T).new
    each do |elem|
      ary << elem unless ary.includes?(elem)
    end
    return ary
  end

  # Convert the Array into a Hash and then ask for its values
  to_lookup_hash.values
end
</pre>

The first three lines handle the trivial case of when an array is empty or
contains only one element:
<pre type="ruby">
if size <= 1
  return dup
end
</pre>

Obviously in this case, there are no duplicate elements and `Array#uniq` should
simply return the original array. One important detail: Crystal uses `dup` to
return a copy of the array. This reminds me that in Ruby `uniq` returns a copy
of the receiver, while `uniq!` mutates the receiver. My guess is that Crystal
implements Array methods in the same way…

The second passage is an optimization:

<pre type="ruby">
# Heuristic: for a small array it's faster to do a linear scan
# than creating a Hash to find out duplicates.
if size <= SMALL_ARRAY_SIZE
  ary = Array(T).new
  each do |elem|
    ary << elem unless ary.includes?(elem)
  end
  return ary
end
</pre>

For small arrays (16 or fewer elements) Crystal iterates over them and removes
duplicates using a simple algorithm. I’ll take a look at how that works in a
moment.

The final line of code handles arrays with 17 or more elements:

<pre type="ruby">
# Convert the Array into a Hash and then ask for its values
to_lookup_hash.values
</pre>

As you might guess, Crystal removes duplicate values from larger arrays using a
hash. I'll dive into the details about how this works in my next post.

## Arrays With 16 Or Fewer Elements

But first, let’s take a closer look at case #2 from above, when the array
contains 16 or fewer elements. First, Crystal creates a new, empty array called
ary:

<pre type="ruby">
ary = Array(T).new
</pre>

Note the generic type syntax `Array(T).new`. This tells the Crystal compiler
that the new array, what will become the return value from `Array#uniq`, will
only contain elements of the same type as the original array.

Ruby developers will find the rest of this code easy to follow…

<pre type="ruby">
each do |elem|
  ary << elem unless ary.includes?(elem)
end
</pre>

Crystal calls `each` to iterate over all the elements in the receiver, the
array we are calling `uniq` on. Then using `<<`, Crystal appends each of the
original array’s elements to the new array, unless the new array already
contains a given element.

Like Ruby, Crystal implements the `includes?` method inside the `Enumerable`
module. Crystal arrays are enumerable because of the `include
Indexable::Mutable(T)` statement we read above. (`Indexable::Mutable` includes
`Indexable` which includes `Enumerable`). You can find Crystal’s implementation
of `includes?` (not `include?` as in Ruby) in
[enumerable.cr](https://github.com/crystal-lang/crystal/blob/master/src/enumerable.cr):

<pre type="ruby">
def includes?(obj) : Bool
  any? { |e| e == obj }
end
</pre>

Here the `any?` method calls the given block once for each element in the
array, and returns true if the block returns true for any of the elements. In
other words, this code searches the array in a linear fashion, one element at a
time. Crystal’s development team has decided that it’s faster to filter out
repeated elements from small arrays by repeatedly searching the array using
linear scans. Since there are never more than 16 elements, those scans won’t
take too much time.

## Simple and Concise

You might be thinking: This is an incredibly simple algorithm; anyone could have
written this code! Why bother writing a blog post about this?

That’s exactly my point: This is simple and concise code. I could have written
it - you could have also. There’s nothing superfluous, not an extra word here.
Just enough code to get the job done. And there’s no noise… no macros, no odd C
memory tricks, no weird bitwise mask operations. This is the kind of code I
need to read now when I’m learning how to use Crystal. As a side benefit, I
also get to learn how Crystal works internally.

But what happens for longer arrays, with 100s or 1000s of elements? How does
Crystal remove duplicates from longer arrays efficiently? I'll take a look at
how that works in my next post.
