title: "How Rust Implements Tagged Unions"
date: 2018/3/15
tag: Rust

<div style="float: left; padding: 8px 30px 40px 0px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2018/3/15/covers.png"><br/>
<i> <small>(<a href="https://www.amazon.com/Programming-Language-Brian-W-Kernighan/dp/0131101633/ref=pd_lpo_sbs_14_img_1?_encoding=UTF8&psc=1&refRID=J7H21QEX2A2NN3Y6EG00">The C Programming Language</a> and <a href="https://nostarch.com/Rust">The Rust Programming Language</a>)</small></i> 
</div>

Rust [describes itself](https://www.rust-lang.org/en-US/) as:

<blockquote>
…a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
</blockquote>

Of course, this is in contrast to C, a different systems programming language
that encourages segfaults and makes no guarantees at all about thread safety.
Rust improves on C in many ways, most famously with its innovative ownnership
model for managing memory.

Another less obvious improvement Rust makes to C has to do with the <span
class="code">union</span> keyword. The Rust compiler implements _tagged
unions_, which prevent you from crashing your program by initializing a union
with one variant and accessing it with another. <del>But the Rust doesn’t include
the <span class="code">union</span> keyword at all</del>; instead, Rust
uses <span class="code">enum</span> to improve on both C enums and C unions at
the same time.

<div style="clear: both"></div>

(Update: I heard [on
Twitter](https://twitter.com/josh_triplett/status/974312496717742080) and in
the comments that Rust does include untagged unions for use in FFI to
interoperate with C, or for unsafe code building custom unions.)

Not sure what a tagged union is? Or why it’s an improvement over an old
fashioned C union? Today I’ll explain. First I’ll start with a quick review of
C unions, how they work and why they are dangerous. Then I’ll show you how Rust
enums improve on them.

## C Unions

Unions are one of the most dangerous features of C. Here’s an example:

<img src="http://patshaughnessy.net/assets/2018/3/15/union1.png"/>

Here the union <span class="code">num\_or\_str</span> saves either a number or
a character pointer but not both. (A union can contain any number of members;
for simplicity my example union has only two.) On the right I show how the C
compiler would allocate memory for an instance of <span
class="code">num_or_str</span>. It allocates enough memory to hold the longest
value in the union, but not both values at the same time. The integer is a
short, meaning it occupies 16 bits or two bytes, and the string is a char
pointer which takes 64 bits or 8 bytes using a modern 64-bit CPU. The two
options for what might be stored in the union, <span class="code">num</span>
and <span class="code">str</span> in this example, are known as _variants_.

## Why C Unions Are Dangerous

Unions are dangerous because you, the C programmer, need to remember which
variant you set in the union. If you save one type of value but then access the
other, your program will crash.

For example this code works fine:

<img src="http://patshaughnessy.net/assets/2018/3/15/c-code1.png"/>

But if you forget <span class="code">a_number</span> contains a number, and use
<span class="code">a_number</span> as a string instead, your program will
crash:

<img src="http://patshaughnessy.net/assets/2018/3/15/c-code2.png"/>

Notice the C compiler didn’t help me here at all. It didn’t display any sort of
warning or error when I wrote <span class="code">a_number.str</span>. It
silently allowed me to write dangerous code; in fact, union syntax encouraged
me to introduce a segmentation fault.

Writing C code with unions is like driving very fast down a highway full of
potholes. You might be the best driver in the world, but eventually you’re
going to hit one of the holes and crash.

## Tagged Unions

C programmers have been writing code with unions for years - for decades in
fact. How have they avoided this problem? There must be a safe way of writing C
code with unions.

The most common and robust solution is to keep track of which union variant is
valid using an integer value saved right next to the union in memory. This
integer is known as a tag, and the combination of the tag and the union is a
_tagged union_.

Here’s an example:

<img src="http://patshaughnessy.net/assets/2018/3/15/tagged_union.png"/>

On the right side I’ve allocated some memory right before the union for the tag
using a struct. C structs, unlike unions, allocate enough memory to store all
of their members at once. Note: Using two bytes to save a small integer value
is unnecessary. C programs often use only one byte, or even represent the
integer value using a bit mask inside the union’s values. But the principle
remains the same.

Now when I save an integer in an instance of the union I can also set the tag
to the value 1, for example, which I decide will mean that <span
class="code">a_number</span> contains a number:

<img src="http://patshaughnessy.net/assets/2018/3/15/c-code3.png"/>

And if I want to save a string instead, I set the tag to 2, for example:

<img src="http://patshaughnessy.net/assets/2018/3/15/c-code4.png"/>

Later when I access the tagged union, I first check the tag before deciding
which variant I can access:

<img src="http://patshaughnessy.net/assets/2018/3/15/c-code5.png"/>

Of course, tagged unions are not foolproof. I invented the tag values 1 and 2
and wrote the code that checks for them. There’s nothing to prevent me from
forgetting to save the tag value, saving the wrong tag value or misinterpreting
the tag value when I later read it. And, if I ever add new variants to the
union, I have to add a new branch to every <span class="code">if</span>
statement in my app that checks the tags, handling the new value. Needless to
say, the C compiler won’t help me find those <span class="code">if</span>
statements or check whether I’ve covered all the possible tag values.

I’m a forgetful and  easily distracted person. I need a programming language
that will keep me out of trouble. Even with tagged unions I’m sure I would
write dangerous, crashing C code before long.

## Tagged Unions in Rust

Rust implements tagged unions using the <span class="code">enum</span> keyword.
For example, to declare a Rust enum type equivalent to the C tagged union above
I write:

<img src="http://patshaughnessy.net/assets/2018/3/15/rust-enum1.png"/>

The questions for today are: Why are enums equivalent to tagged unions in C?
And: What should I draw on the right side? What would I see if I could find and
examine an enum in the memory space of a running Rust process?

## Saving a Rust Enum

To find out, let’s create an instance of <span class="code">NumOrStr</span>:

<img src="http://patshaughnessy.net/assets/2018/3/15/rust-enum2.png"/>

Notice that instead of 4, I’ve saved a more recognizable value, 1234. Now, if I
compile it with the <span class="code">-—emit asm</span> flag:

<img src="http://patshaughnessy.net/assets/2018/3/15/rust-emit-asm.png"/>

…Rust generates a file called union.s which contains the assembly language
version of my program. If I open union.s and search for 1234, the integer value
I saved above, I see:

<img src="http://patshaughnessy.net/assets/2018/3/15/asm1.png"/>

I’ve found it; here are the x86 assembly language instructions that initialize
<span class="code">a_number</span>. These show me exactly how Rust represents
enums in memory, how Rust implements tagged unions.

The only problem is… I have no idea what this means!

## The movw x86 Instruction

What does <span class="code">movw</span> mean? And what about <span
class="code">-32(%rbp)</span>?

It turns out x86 assembly language isn’t that hard to follow, once you learn
the basic syntax. For a quick introduction, see my article from 2016: [Learning
to Read x86 Assembly
Language](http://patshaughnessy.net/2016/11/26/learning-to-read-x86-assembly-language).
Intel, the company that built the microprocessor inside my Mac, defines the
<span class="code">mov</span> instruction to mean “move.” (Note: the
instructions I show here that <span class="code">rustc —emit asm</span>
generates aren’t written using Intel x86 syntax, but with GAS x86 syntax
instead.)

Here’s a diagram showing what the first <span class="code">movw</span>
instruction moves:

<img src="http://patshaughnessy.net/assets/2018/3/15/asm2.png"/>

It turns out that <span class="code">movw</span> stands for “move a word.” A
word is defined as 16 bits, or 2 bytes. There are a few different variations on
move, <span class="code">movb</span>, <span class="code">movw</span>, <span
class="code">movl</span>, <span class="code">movq</span>, which move 1
byte, 2 bytes, 4 bytes or 8 bytes respectively.

Next, the <span class="code">$</span> notation indicates a literal value -  in
this case zero: <span class="code">$0</span>. Now we can see the first
instruction above is moving 2 bytes containing the value zero. Similarly, the
second instruction is moving 2 bytes containing the value 1234:

<img src="http://patshaughnessy.net/assets/2018/3/15/asm3.png"/>

## The rbp Register

But where are these <span class="code">movw</span> instructions moving these
values to? To understand that we need to understand the odd <span
class="code">-32(%rbp)</span> syntax on the right side of the instructions. The
<span class="code">%</span> sign indicates a register inside my Mac’s
microprocessor, in this case the “base pointer” register. So "bp" means "base
pointer." And the “r” prefix in "rbp" means the move instruction is using all 8
bytes (64 bits) of this register’s value.

The <span class="code">-32(%rbp)</span> notation calculates a memory address
for the instruction using the contents of the <span class="code">%rbp</span>
register - in this case the address of where to move the data to. The
expression <span class="code">-32(%rbp)</span> in English means: “Take the 64
bit memory address value from the base pointer register, and subtract 32 from
it.”

Compiled Rust programs - all programs really - that run on the x86 platform
store values for local variables on the stack, using the base pointer register
in this fashion. The base pointer, as it’s name indicates, stores the base
address of my program's current stack frame. Each local variable in my code,
for example <span class="code">a_number</span>, is saved somewhere on the
stack. If you’re not familiar with the concept of a stack, think of it as a
convenient place for quickly saving and retrieving values while your program is
running.

## How Rust Saves an Integer Enum Variant

Taking a step back for a moment, here’s what we’ve learned so far. When I save
an enum value containing an integer, Rust saves _two_ values, 0 and 1234:

<img src="http://patshaughnessy.net/assets/2018/3/15/save1.png"/>

What does the <span class="code">0</span> mean? Rust records a zero to indicate
that <span class="code">a_number</span> uses the <span
class="code">NumOrStr::Num</span> variant. In other words, <span
class="code">a_number</span> is a tagged union, and the zero value is the tag.
We know the tag occupies 2 bytes because of the <span class="code">movw</span>
instruction above. The integer value itself, <span class="code">1234</span>,
also takes 2 bytes because I declared it using <span
class="code">Num(i16)</span>, and we saw Rust used a movw to save that also.

## How Rust Saves an String Enum Variant

But what about the other variant, the string? When I save a string in <span
class="code">NumOrStr</span>, what does Rust do? To find out, I’ll replace my
main function from above with this line of code:

<img src="http://patshaughnessy.net/assets/2018/3/15/save2.png"/>

The I’ll compile it again using the <span class="code">--emit asm</span>
option. Now I find this assembly language code in the union.s file:

<img src="http://patshaughnessy.net/assets/2018/3/15/asm4.png"/>

Unfortunately this code snippet is much more complex: It first calls <span
class="code">String::from</span> passing a string literal, and then saves the
string into the enum via a method called <span
class="code">drop_in_place</span>. This is much harder to understand.

Rather than trying to figure this out, I decided to debug my Rust sample
program using LLDB, and inspect the memory <span class="code">a_string</span>
occupies. I found that Rust used 26 bytes to represent the string variant,
starting with a 16 bit word containing 1:

<img src="http://patshaughnessy.net/assets/2018/3/15/save3.png"/>

This is again the tag; in this case <span class="code">1</span> means <span
class="code">a_string</span> uses the <span class="code">NumOrStr::Str</span>
variant. Following this I found a pointer to the string itself:

<img src="http://patshaughnessy.net/assets/2018/3/15/save4.png"/>

Pointers on a 64-bit microprocessor occupy 8 bytes and contain the memory
address of something, in this case my string "This is a test.” After the
pointer I found two 64 bit values, each containing 15:

<img src="http://patshaughnessy.net/assets/2018/3/15/save5.png"/>

These are two attributes of the string: its capacity and length. By inspecting
my process's memory I’ve started to learn a bit about how Rust manages memory
for strings.

But what’s important for me today is the first word, the value 1. Again, we see
the same pattern. Rust saves an integer value, the tag, indicating which
variant this instance of the enum uses. Then Rust saves the enum variant’s
payload in the memory that follows:

<img src="http://patshaughnessy.net/assets/2018/3/15/save6.png"/>

## Tagged Unions in Rust and C

Let’s review by declaring a tagged union in C and Rust:

<img src="http://patshaughnessy.net/assets/2018/3/15/review1.png"/>

On the left using C, I have to include the tag explicitly in a surrounding
struct. Rust handles this for me automatically, saving the tag value inside the
enum alongside the enum’s value. The code looks very different, but as we saw above
the implementations are_ identical_.

Using a tagged union looks somewhat similar in C and Rust:

<img src="http://patshaughnessy.net/assets/2018/3/15/review2.png"/>

But there are very important differences here! Using C, I need to remember to
check the tag and to use the proper variant inside the union. The Rust
compiler, on the other hand, checks the tag for me automatically and won’t
allow me to access the wrong variant. The code inside of <span class="code">if
let</span> will never be executed unless the internal tag value matches the
<span class="code">NumOrStr::Num</span> variant.

Under the hood, the two languages implement tagged unions the same way. But
writing code in C and Rust is very different. C encourages me to write
dangerous, crashing code, while Rust prevents me from writing dangerous code in
the first place.
