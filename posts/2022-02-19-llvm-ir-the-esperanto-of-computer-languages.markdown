title: "LLVM IR: The Esperanto of Computer Languages"
date: 2022/02/19
tag: Crystal

<div style="float: left; padding: 8px 30px 0px 0px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2022/2/19/esperanto.png"><br/>
  <i> Esperanto grammar is logical and self<br/>
consistent, designed to be easy to learn. <br/>
  <small> <a title="Renatoeo, CC BY-SA 4.0 &lt;https://creativecommons.org/licenses/by-sa/4.0&gt;, via Wikimedia Commons" href="https://commons.wikimedia.org/wiki/File:CARD_GRAM%C3%81TICA_ESPERANTO.png">via Wikimedia Commons</a></small> </i>
</div>

I empathize for people who have to learn English as a foreign language. English
grammar is inconsistent, arbitrary and hard to master. English spelling is even
worse. I sometimes find myself apologizing for my language’s shortcomings. But
learning any foreign language as an adult is very difficult.

[Esperanto](https://en.wikipedia.org/wiki/Esperanto), an “artificial language,”
is different. Invented by Ludwik Zamenhof in 1873, Esperanto has a vocabulary
and grammar that are logical and consistent, designed to be easier to learn.
Zamenhof intended Esperanto to become the universal second language.

Computers have to learn foreign languages too. Every time you compile and run
a program, your compiler translates your code into a foreign language: the
native machine language that runs on your target platform. Compilers should
have been called translators. And compilers struggle with the same things we
do: inconsistent grammar and vocabulary, and other peculiarities of the target
platform.

Recently, however, more and more compilers translate your code to an
artificial machine language. They produce a simpler, more consistent, more
powerful machine language that doesn’t actually run on any machine. This
artificial machine language, LLVM IR, makes writing compilers simpler and
reading the code compilers produce simpler too.

LLVM IR is becoming the universal second language for compilers.

## One Line of LLVM IR

The [Low Level Virtual Machine](https://llvm.org) (LLVM) project had the novel
idea of inventing a virtual machine that was easy for compiler engineers to use
as a target platform. The LLVM team designed a special instruction set called
[intermediate representation](https://llvm.org/docs/LangRef.html) (IR). New,
modern languages such as Rust, Swift, Clang-based versions of C and many
others, first translate your code to LLVM IR. Then they use the LLVM framework
to convert the IR into actual machine language for any target platform LLVM
supports:

<img style="width: 500px; margin-bottom: 20px" src="https://patshaughnessy.net/assets/2022/2/19/platforms.svg">

LLVM is great for compilers. Compiler engineers don’t have to worry about the
detailed instruction set of each platform, and LLVM optimizes your code for
whatever platform you choose automatically. And LLVM is also great for people
like me who are interested in what machine language instructions look like and
how CPUs execute them. LLVM instructions are much easier to follow than real
machine instructions. Let’s take a look at one!

Here’s a line of LLVM IR I generated from a simple
[Crystal](https://crystal-lang.org) program:

<pre type="console">
%57 = call %"Array(Int32)"* @"*Array(Int32)@Array(T)::unsafe_build<Int32>:Array(Int32)"(i32 610, i32 2), !dbg !89
</pre>

Wait a minute! This isn’t simple or easy to follow at all! What am I talking
about here? At first glance, this does look confusing. But as we’ll see, most
of the confusing syntax is related to Crystal, not LLVM. Studying this line of
code will reveal more about Crystal than it will about LLVM.

The rest of this article will unpack and explain what this line of code means.
It looks complex, but is actually quite simple.

## The Call Instruction

The instruction above is a function call in LLVM IR. To produce this code, I
wrote a small Crystal program and then translated it using this command:

<pre type="console">
$ crystal build array_example.cr --emit llvm-ir
</pre>

The `--emit` option directed Crystal to generate a file called array_example.ll,
which contains the line above along with thousands of other lines. We’ll get to
the Crystal code in a minute. But for now, how do I get started understanding
what the LLVM code means?

The [LLVM Language Reference Manual](https://llvm.org/docs/LangRef.html) has
documentation for `call` and all of the other LLVM IR instructions. Here’s the
syntax for `call`:

<pre type="console">
&lt;result> = [tail | musttail | notail ] call [fast-math flags] [cconv] [ret attrs] [addrspace(&lt;num>)]
         &lt;ty>|&lt;fnty> &lt;fnptrval>(&lt;function args>) [fn attrs] [ operand bundles ]
</pre>

My example `call` instruction doesn’t use many of these options. Removing the
unused options, I can see the actual, basic syntax of `call`:

<pre type="console">
&lt;result> = call &lt;ty> &lt;fnptrval>(&lt;function args>)
</pre>

In order from left to right, these values are:

* `<result>` which register to save the result in

* `<ty>` the type of the return value

* `<fnptrval>` a pointer to the function to call

* `<function args>` the arguments to pass to that function

What does all of this mean, exactly? Let’s find out!

## A CPU With Infinite Registers

Starting on the left and moving right, let’s step through the `call` instruction:

<img src="https://patshaughnessy.net/assets/2022/2/19/result.svg">

The token `%57` to the left of the equals sign tells LLVM where to save the
return value of the function call that follows. This isn’t a normal variable;
`%57` is an LLVM “register.”

Registers are physical circuits located on microprocessor chips used to save
intermediate values. Saving a value in a CPU register is much faster than
saving a value in memory, since the register is located on the same chip as the
rest of the microprocessor. Saving a value in RAM memory, on the other hand,
requires transmitting that value from one chip to another and is much slower,
relatively speaking. Unfortunately, each CPU has a limited number of registers
available, and so compilers have to decide which values are used frequently
enough to warrant saving in nearby registers, and which other values can be
moved out to more distant memory.

Unlike the limited number of registers available on a real CPU, the imaginary
LLVM microprocessor has an infinite number of them. Because of this, compilers
that target LLVM can simply save values to a register whenever they would like.
There’s no need to find an available register, or to move an existing value out
of a register first before using it for something else. Busy work that normal
machine language code can’t avoid.

In this program, the Crystal compiler had already saved 56 other values in
“registers” and so for this line of LLVM IR, Crystal simply used the next
register, number 57.

## LLVM Structure Types

Moving left to right, LLVM `call` instructions next indicate the type of the
function call’s return value:

<img src="https://patshaughnessy.net/assets/2022/2/19/type.svg">

This name of this type, `Array(Int32)`, is generated by the Crystal compiler, not
by LLVM. That is, this is a type from my Crystal program. It could have been
anything, and indeed other compilers that target LLVM will generate completely
different type names.

The example Crystal program I used to generate this LLVM code was:

<pre type="ruby">
arr = [12345, 67890]
puts arr[1]
</pre>

When I compiled this program, Crystal generated the `call` instruction above,
which returns a pointer to the new array, `arr`. Since `arr` is an array
containing integers, Crystal uses a generic type `Array(Int32).`

Machine languages that target real machines only support hardware types that
machine supports.  For example, Intel x86 assembly language allows you to save
integers of different widths, 16, 32 or 64 bits for example, and an Intel x86
CPU has registers designed to hold values of each of these sizes.

LLVM IR is more powerful. It supports “structure types,” similar to a C
structure or an object in a language like Crystal or Swift. Here the `%"…"`
syntax indicates the name inside the quotes is the name of a structure type.
And the asterisk which follows, like in C, indicates the type of the return
value of my function call is a pointer to this structure.

My example LLVM program defines the type `Array(Int32)` like this:

<pre type="console">
%"Array(Int32)" = type { i32, i32, i32, i32, i32* }
</pre>

Structure types allow LLVM IR programs to create pointers to structures or
objects, and to access any of the values inside each object. That makes writing
a compiler much easier. In my example, the call instruction returns a pointer
to an object which contains 4 32-bit integer values, followed by a pointer to
other 32 integer values. But what are all of these integer values? Above I said
this function call was returning a new array - how can that be the case?

LLVM itself has no idea, and no opinion on the matter. To understand what these
values are, and what they have to do with the array in my program, we need to
learn more about the Crystal compiler that generated this LLVM IR code.

Reading the [Crystal standard
library](https://github.com/crystal-lang/crystal/blob/master/src/array.cr#L48),
we can see Crystal implements arrays like this:

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

# In 64 bits the Array is composed then by:
# - type_id            : Int32   # 4 bytes -|
# - size               : Int32   # 4 bytes  |- packed as 8 bytes
#
# - capacity           : Int32   # 4 bytes -|
# - offset_to_buffer   : Int32   # 4 bytes  |- packed as 8 bytes
#
# - buffer             : Pointer # 8 bytes  |- another 8 bytes
</pre>

The comments above are very illustrative and complete - the Crystal team took
the time to document their standard library and explain not only how to use
each class, like `Array(T)`, but how they are implemented internally.

In this case, we can see the four `i32` values inside the `Array(Int32)` LLVM
structure type hold the size and capacity off the array, among other things.
And the `i32*` value is a pointer to the actual contents of the array.

## Functions

The target of the call instruction appears next, after the return type:

<img src="https://patshaughnessy.net/assets/2022/2/19/function.svg">

This is quite a mouthful! What sort of function is this?

There are two steps to understanding this: First, the `@"…"` syntax. This is
simply a global identifier in this LLVM program. So my `call` instruction is just
calling a global function. In LLVM programs, all functions are global; there is
no concept of a class, module or similar groupings of code.

But what in the world does that crazy identifier mean?

LLVM ignores this complex name. For LLVM this is just a name like `foo` or `bar`.
But for Crystal, the name has much more significance. Crystal encoded a lot of
information into this one name. Crystal can do this because the LLVM code isn’t
intended for anyone to read directly. Crystal has created a “mangled name,”
meaning the original version of the function to call is there but it’s been
mangled or rewritten in a confusing manner.

Crystal rewrites function names to ensure they are unique. In Crystal, like in
many other statically typed languages, functions with different argument types
or return value types are actually different functions. So in Crystal if I
write:

<pre type="ruby">
def foo(a : Int32)
puts "Int: #{a}"
end

def foo(a : String)
puts "String: #{a}"
end

foo(123)
#=> Int: 123
foo("123")
#=> String: 123
</pre>

…I have two separate, different functions both called `foo`. The type of the
parameter `a` distinguishes one from the other.

Crystal generates unique function names by encoding the arguments, return value
and type of the receiver into the into the function name string, making it
quite complex. Let’s break it down:

<img src="https://patshaughnessy.net/assets/2022/2/19/mangled.svg">

* `Array(Int32)@Array(T)` - this is the type of the receiver. That means the
`unsafe_build` function is actually a method on the `Array(T)` generic class.
And in this case, the receiver is an array holding 32 bit integers, the
`Array(Int32)` class. Crystal includes both names in the mangled function name.

* `unsafe_build` - this is the function Crystal is calling.

* `Int32` - these are the function’s parameter types. In this case, Crystal is
passing in a single integer, so we just see one `Int32` type.

* `Array(Int32)` - this is the return value type, a new array containing integers.

As I discussed in [my last
post](https://patshaughnessy.net/2022/1/22/visiting-an-abstract-syntax-tree),
the Crystal compiler internally rewrites my array literal expression `[12345,
67890]` into code that creates and initializes a new array object:

<pre type="ruby">
__temp_621 = ::Array(typeof(12345, 67890)).unsafe_build(2)
__temp_622 = __temp_621.to_unsafe
__temp_622[0] = 12345
__temp_622[1] = 67890
__temp_621
</pre>

In this expanded code, Crystal calls `unsafe_build` and passes in `2`, the
required capacity of the new array. And to distinguish this use of
`unsafe_build` from other `unsafe_build` functions that might exist in my
program, the compiler generated the mangled name we see above. 

## Arguments

Finally, after the function name the LLVM IR instruction shows the arguments
for the function call:

<img src="https://patshaughnessy.net/assets/2022/2/19/args.svg">

LLVM IR uses parentheses, like most languages, to enclose the arguments to a
function call. And the types precede each value: `610` is a 32-bit integer and
`2` is also a 32-bit integer.

But wait a minute! We saw just above the expanded Crystal code for generating
the array literal passes a single value, `2`, into the call to `unsafe_build`.
And looking at the mangled function name above, we also see there is a single
`i32` parameter to the function call.

But reading the LLVM IR code we can see a second value is also passed in:
`610`. What in the world does `610` mean? I don’t have 610 elements in my new
array, and 610 is not one of the array elements. So what is going on here?

Crystal is an object oriented language, meaning that each function is
optionally associated with a class. In OOP parlance, we say that we are
“sending a message” to a “receiver.” In this case, `unsafe_build` is the message,
and `::Array(typeof(12345, 67890))` is the receiver. In fact, this function is
really a class method. We are calling `unsafe_build` on the `Array(Int32)` class,
not on an instance of one array.

Regardless, LLVM IR does’t support classes or instance methods or class
methods. In LLVM IR, we only have simple, global functions. And indeed, the
LLVM virtual machine doesn’t care what these arguments are or what they mean.
LLVM doesn’t encode the meaning or purpose of each argument; it just does what
the Crystal compiler tells it to do.

But Crystal, on the other hand, has to implement object oriented behavior
somehow. Specifically, the `unsafe_build` function needs to behave differently
depending on which class it was called for, depending on what the receiver is.
For example:

<pre type="ruby">
::Array(typeof(12345, 67890)).unsafe_build(2)
</pre>

… has to return an array of two integers. While:

<pre type="ruby">
::Array(typeof("abc", "def")).unsafe_build(2)
</pre>

…has to return an array of two strings. How does this work in the LLVM IR code?

To implement object oriented behavior, Crystal passes the receiver as a hidden,
special argument to the function call:

<img src="https://patshaughnessy.net/assets/2022/2/19/args2.svg">

This receiver argument is a reference or pointer to the receiver’s object, and
is normally known as `self`. Here `610` is a reference or tag corresponding to
the `Array(Int32)` class, the receiver. And `2` is the actual argument to the
`unsafe_build` method.

Reading the LLVM IR code, we’ve learned that Crystal secretly passes a hidden
`self` argument to every method call to an object. Then inside each method, the
code has access to `self`, to the object instance that code is running for. Some
languages, like Rust, require us to pass `self` explicitly in each method call;
in Crystal this behavior is automatic and hidden.

## Learning How Compilers Work

LLVM IR is a simple language designed for compiler engineers. I think of it
like a blank slate for them to write on. Most LLVM instructions are quite
simple and easy to understand; as we saw above, understanding the basic syntax
of the call instruction wasn’t hard at all.

The hard part was understanding how the Crystal compiler, which targets LLVM
IR, generates code. The LLVM syntax itself was easy to follow; it was the
Crystal language’s implementation that was harder to understand.

And this is the real reason to learn about LLVM IR syntax. If you take the time
to learn how LLVM instructions work, then you can start to read the code your
favorite language’s compiler generates. And once you can do that, you can learn
more about how your favorite compiler works, and what your programs actually do
when you run them.
