title: "Find Your Language’s Primitives"
date: 2021/11/29
tag: Crystal

<div style="float: right; padding: 8px 0px 30px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2021/11/29/dig1.jpg"><br/>
  <i>If you dig into your programming language's syntax, you might <br/>discover that it is capable of much more than you thought it was.
  </i>
</div>

Wikipedia defines “Language Primitive” [this
way](https://en.wikipedia.org/wiki/Language_primitive):

<blockquote>
In computing, language primitives are the simplest elements available in a
programming language. A primitive is the smallest 'unit of processing'
available to a programmer of a given machine, or can be an atomic element of an
expression in a language.
</blockquote>

By looking at a language’s primitives, we can learn what kind of code will be
easy to write or impossible to express, and what types of problems the language
was intended to solve.  Whether you’ve been using a language for years, or just
now learning a new language for fun, take the time to find and learn about your
language’s primitives. You might discover something you never knew, and will
come away with a deeper understanding of how your programs work.

As an example today, I’m going to look at how arrays work in three languages:
Ruby, Crystal and x86 Assembly Language.

## Retrieving an Array Element In Ruby

In Ruby I can create an array and later access an element like this:

<pre type="ruby">
arr = [12345, 67890]
puts arr[1]
</pre>

This code would be the same or almost the same in many other programming
languages. It just means: “find the second element of the array and print it to
stdout.”

But how does this actually work? In Ruby, the <span class="code">Array</span>
class and all of its methods are language primitives. This means array methods
like <span class="code">[]</span> or <span class="code">[]=</span> cannot be
broken down into smaller pieces of Ruby code. As Wikipedia says, these methods
are the smallest unit of processing available to Ruby programmers working with
arrays.

<img src="http://patshaughnessy.net/assets/2021/11/29/primitive1.png"><br/>

Ruby hides the details of how arrays actually work from us. To learn how Ruby
actually saves and retrieves values from an array, we would need to switch
languages and drop down a level of abstraction, and read the C implementation
in the Ruby source code:
[array.c](https://github.com/ruby/ruby/blob/master/array.c). There’s nothing
wrong with this, of course. Ruby developers use arrays every day without any
trouble. But switching from Ruby to C makes understanding internal details much
more difficult.

## Retrieving an Array Element In Crystal

This Fall I decided to learn more about [Crystal](https://crystal-lang.org), a
statically typed language with syntax that resembles Ruby. I expected to find a
similar <span class="code">Array#[]</span> primitive.  But surprisingly, I was
wrong!

The same code from above also works in Crystal:

<pre type="ruby">
arr = [12345, 67890]
puts arr[1]
</pre>

In Crystal, arrays are not language primitives because the Crystal standard
library implements arrays using Crystal itself. The <span
class="code">Array#[]</span> method is not the smallest unit of processing
available to Crystal programmers. Let’s dig into the details and divide up the
<span class="code">[]</span> method into smaller and smaller pieces to see how
the Crystal team implemented it.

Reading
[src/indexable.cr](https://github.com/crystal-lang/crystal/blob/master/src/indexable.cr#L56)
in the Crystal standard library, here’s the implementation of <span class="code">Indexable#[]</span>
which the array class uses when I call <span class="code">arr[1]</span> above:

<pre type="ruby">
# Returns the element at the given *index*.
#
# Negative indices can be used to start counting from the end of the array.
# Raises `IndexError` if trying to access an element outside the array's range.
#
# ```
# ary = ['a', 'b', 'c']
# ary[0]  # => 'a'
# ary[2]  # => 'c'
# ary[-1] # => 'c'
# ary[-2] # => 'b'
#
# ary[3]  # raises IndexError
# ary[-4] # raises IndexError
# ```
@[AlwaysInline]
def [](index : Int)
  fetch(index) { raise IndexError.new }
end
</pre>

The Crystal team implemented <span class="code">[]</span> using another method
called <span class="code">fetch</span>:

<pre type="ruby">
# Returns the element at the given *index*, if in bounds,
# otherwise executes the given block with the index and returns its value.
#
# ```
# a = [:foo, :bar]
# a.fetch(0) { :default_value }    # => :foo
# a.fetch(2) { :default_value }    # => :default_value
# a.fetch(2) { |index| index * 3 } # => 6
# ```
def fetch(index : Int)
  index = check_index_out_of_bounds(index) do
    return yield index
  end
  unsafe_fetch(index)
end
</pre>

Neither the <span class="code">[]</span> operator nor the <span
class="code">fetch</span> method are language primitives. To find a language
primitive, I need to keep dividing the code up into smaller and smaller pieces,
until it can’t be divided any further. The same process a chemist would use to
break up some material into smaller and smaller molecules until they are left
with a set of atoms.

Let’s continue by reading <span class="code">unsafe_fetch</span>:

<pre type="ruby">
# Returns the element at the given *index*, without doing any bounds check.
#
# `Indexable` makes sure to invoke this method with *index* in `0...size`,
# so converting negative indices to positive ones is not needed here.
#
# Clients never invoke this method directly. Instead, they access
# elements with `#[](index)` and `#[]?(index)`.
#
# This method should only be directly invoked if you are absolutely
# sure the index is in bounds, to avoid a bounds check for a small boost
# of performance.
abstract def unsafe_fetch(index : Int)
</pre>

Since <span class="code">Indexable#unsafe_fetch</span> is an abstract method, I
need to read how the <span class="code">Array</span> class implements it back
in
[src/array.cr](https://github.com/crystal-lang/crystal/blob/master/src/array.cr#L663):

<pre type="ruby">
@[AlwaysInline]
def unsafe_fetch(index : Int) : T
  @buffer[index]
end
</pre>

<div style="float: right; padding: 8px 0px 30px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2021/11/29/dig2.jpg"><br/>
	<i><small>(source: <a href="https://commons.wikimedia.org/wiki/File:Digging_in_permafrost.jpg">Nick Bonzey via Wikimedia Commons</a>)</small></i>
</div>

So far, I’ve drilled down through 3 levels of Crystal implementation:

<img src="http://patshaughnessy.net/assets/2021/11/29/primitive2.png"><br/>

But I haven’t found a primitive function yet. Let’s keep digging!

## The Crystal Array Class

To learn more, I need to scroll up and read the beginning of the Crystal <span
class="code">Array</span> class definition:

<pre type="ruby">
# An `Array` is an ordered, integer-indexed collection of objects of type T.
#
# Array indexing starts at 0. A negative index is assumed to be
# relative to the end of the array: -1 indicates the last element,
# -2 is the next to last element, and so on.
</pre>
etc...
<pre type="ruby">
# An `Array` is implemented using an internal buffer of some capacity
# and is reallocated when elements are pushed to it when more capacity
# is needed. This is normally known as a [dynamic array](http://en.wikipedia.org/wiki/Dynamic_array).
#
class Array(T)
</pre>
etc...
<pre type="ruby">
  # The buffer where elements start.
  @buffer : Pointer(T)
</pre>

I’ve deleted some of the comments and code for clarity. You can read the full,
original version in [src/array.cr.](https://github.com/crystal-lang/crystal/blob/master/src/array.cr)

Now I get a sense of how the <span class="code">unsafe_fetch</span> method
above works. Let’s repeat that again:

<pre type="ruby">
def unsafe_fetch(index : Int) : T
  @buffer[index]
end
</pre>

Crystal saves all of the elements in each array into a memory buffer called
<span class="code">@buffer</span>. And when I access an element of the array
like this:

<pre type="ruby">
puts arr[1]
</pre>

Crystal first checks that the array index (1 in this example) is valid, and
then loads the array element I want from the buffer using: <span
class="code">@buffer[1]</span>.

## The Crystal Pointer Class

But how does <span class="code">@buffer[index]</span> actually work? I haven’t learned anything yet! I’m
just going around in circles. So far all I’ve been able to find is that Crystal
implements <span class="code">Array#[]</span> with a different <span class="code">[]</span> operator, on a different class. What
type of object is <span class="code">@buffer</span>? What does it do?

Reading the array class declaration again more carefully, I can see that
<span class="code">@buffer</span> is an instance of the Pointer class:

<pre type="ruby">
# The buffer where elements start.
@buffer : Pointer(T)
</pre>

Let’s read how Crystal implements <span class="code">Pointer#[]</span> in
[src/pointer.cr:](https://github.com/crystal-lang/crystal/blob/master/src/pointer.cr#L107)

<pre type="ruby">
# Gets the value pointed at this pointer's address plus `offset * sizeof(T)`.
#
# ```
# ptr = Pointer.malloc(4) { |i| i + 10 }
# ptr[0] # => 10
# ptr[1] # => 11
# ptr[2] # => 12
# ptr[3] # => 13
# ```
def [](offset)
  (self + offset).value
end
</pre>

Reading this, I discovered the Crystal team has written a class that represents
pointers! Just like using a pointer in C, Crystal code can refer to and access
any memory location directly. Because the Pointer class is part of the
language, Crystal allows us to implement our own data structures and algorithms
in a very detailed manner, allocating and accessing memory just like the
Crystal team has while implementing arrays, hashes and other classes.

Now I’ve dug down through 4 levels of Crystal function calls, but I still
haven’t found a language primitive yet.

<img src="http://patshaughnessy.net/assets/2021/11/29/primitive3.png"><br/>

## A Crystal Language Primitive

We still haven’t discovered how arrays actually work - how pointers actually
work. That is, reading this line of code above:

<pre type="ruby">
(self + offset).value
</pre>

…I understand the meaning and intent of pointer arithmetic, and how it’s used
by Crystal arrays, but I still don’t see how or where Crystal actually obtains
the array element referenced by a given pointer.

Digging deeper, let’s read the <span class="code">Pointer#value</span> method
to find out - this method’s implementation should tell me exactly how Crystal
obtains the value:

<pre type="ruby">
# Gets the value pointed by this pointer.
#
# ```
# ptr = Pointer(Int32).malloc(4)
# ptr.value = 42
# ptr.value # => 42
# ```
@[Primitive(:pointer_get)]
def value : T
end
</pre>

As you can see, this is a language primitive - it says “primitive” right there
above the method definition!

<img src="http://patshaughnessy.net/assets/2021/11/29/primitive4.png"><br/>

Returning to Wikipedia’s definition of a language primitive, this is an atomic
element of an expression. The Crystal compiler knows not to try to compile this
code but to assume this behavior is part of the language. In fact, there is no
implementation for <span class="code">Pointer#value</span> here at all: the
method is empty!

<pre type="ruby">
def value : T
end
</pre>

The empty <span class="code">value</span> method above doesn’t tell us where
the value actually comes from, or how Crystal obtains it. To learn that, we
need to step down one level of abstraction - we need to use a lower level
language, not Crystal.

## Retrieving an Array Element In x86 Assembly Language

<div style="float: right; padding: 8px 0px 30px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2021/11/29/cave.png"><br/>
</div>

What lower level language should we use? Since the Crystal team used the [Low
Level Virtual Machine (LLVM)](https://llvm.org) project to implement their
compiler, I could look at LLVM’s low level instruction language. But since I’m
not familiar with that, or with how the Crystal compiler works, I decided to
jump down to the lowest level of abstraction available to me on my Intel Mac:
x86 Assembly Language.

Here’s my Crystal program again:

<pre type="console">
$ cat array_example.cr
arr = [12345, 67890]
puts arr[1]
</pre>

If I compile the program without running it, and then use the <span
class="code">llvm-objdump</span> command, LLVM will give me a version of my
code converted into Intel x86 Assembly Language:

<pre type="console">
$ crystal build array_example.cr
$ llvm-objdump -D array_example > array_example.a
</pre>

Now by reading the assembly produced by the Crystal compiler and LLVM, I can
see how the <span class="code">Pointer#[]</span> and <span
class="code">Pointer#value</span> methods actually work:

<pre type="console">
0000000100089bb0 <_*Pointer(Int32)@Pointer(T)#[]<Int32>:Int32>:
100089bb0: 50                           pushq %rax
100089bb1: e8 0a 00 00 00               callq 0x100089bc0 <_*Pointer(Int32)@Pointer(T)#+<Int32>:Pointer(Int32)>
100089bb6: 8b 00                        movl  (%rax), %eax
100089bb8: 59                           popq  %rcx
100089bb9: c3                           retq

0000000100089bc0 <_*Pointer(Int32)@Pointer(T)#+<Int32>:Pointer(Int32)>:
100089bc0: 48 63 c6                     movslq  %esi, %rax
100089bc3: 48 c1 e0 02                  shlq  $2, %rax
100089bc7: 48 01 c7                     addq  %rax, %rdi
100089bca: 48 89 f8                     movq  %rdi, %rax
100089bcd: c3                           retq
</pre>

Assembly language is just another programming language like any other, but with
a different set of primitives. The primitives in this language are hardware
instructions that my laptop’s CPU can understand and execute directly:

<img src="http://patshaughnessy.net/assets/2021/11/29/primitive5.png"><br/>

I won’t pretend to understand all the details here, but if you’re curious what
this code does - how my compiled Crystal program actually retrieves a value
from an array - here are a few highlights you can look for:

<img src="http://patshaughnessy.net/assets/2021/11/29/assembly-table.svg"><br/>

If you’re curious about x86 assembly language, I wrote an article a few years
ago explaining some of the basics: [Learning to Read x86 Assembly
Language](https://patshaughnessy.net/2016/11/26/learning-to-read-x86-assembly-language).

## Understand the Primitives of the Language You Are Using

Why did I bother with this exercise? To make sure I deeply understand the
programming languages I’m using. Ruby hides much of its implementation in C, so
I didn’t learn much looking at the Ruby array primitives in this example. And
the primitive functions of assembly language are by definition the instructions
my CPU can execute directly. It’s always fun trying to identify and understand
machine level instructions!

But Crystal surprised me - I expected to see a set of primitive array functions
like we have in Ruby but I was wrong. Instead, I learned that Crystal supports
pointers, just like C or other low level languages do. I discovered that
Crystal, unlike Ruby, might be an appropriate choice for low level systems
programming tasks. And I was able to learn all of this, along with the array
implementation details, because the Crystal team implemented its standard
library in the same, target language: Crystal. All I had to do was make an
effort to read some code.

Dive into details and find out what the language primitives are in your
favorite programming language. You might be surprised and discover that your
language is capable of much more than you thought it was.
