title: "Never create Ruby strings longer than 23 characters"
tag: Ruby internals
date: 2012/1/4

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2012/1/4/microscope.jpg"></td></tr>
  <tr><td align="center"><small><i>Looking at things through a microscope<br/>sometimes leads to surprising discoveries</i></small></td></tr>
</table>
</div>

Obviously this is an utterly preposterous statement: it’s hard to think of a more ridiculous and esoteric coding requirement. I can just imagine all sorts of amusing conversations with designers and business sponsors: “No... the size of this &lt;input&gt; field should be 23... 24 is just too long!”  Or: “We need to explain to users that their subject lines should be less than 23 letters...” Or: “Twitter got it all wrong... the 140 limit should have been 23!”

Why in the world would I even imagine saying this? As silly as this requirement might be, there is actually a grain of truth behind it: creating shorter Ruby strings is actually much faster than creating longer ones.DELIM It turns out that this line of Ruby code:

<pre type="ruby">
str = "1234567890123456789012" + "x"
</pre>

... is executed about twice as fast by the MRI 1.9.3 Ruby interpreter than this line of Ruby code:

<pre type="ruby">
str = "12345678901234567890123" + "x"
</pre>

Huh? What’s the difference? These two lines look identical! Well, the difference is that the first line creates a new string containing 23 characters, while the second line creates one with 24. It turns out that the MRI Ruby 1.9 interpreter is optimized to handle strings containing 23 characters or less more quickly than longer strings. This isn’t true for Ruby 1.8.

Today I’m going to take a close look at the MRI Ruby 1.9 interpreter to see how it actually handles saving string values... and why this is actually true.

## Not all strings are created equal

Over the holidays I decided to read through the the [Ruby Hacking Guide](http://rhg.rubyforge.org/). If you’ve never heard of it, it’s a great explanation of how the Ruby interpreter works internally. Unfortunately it’s written in Japanese, but a few of the chapters have been translated into English. [Chapter 2](http://rhg.rubyforge.org/chapter02.html), one of the translated chapters, was a great place to start since it explains all of the basic Ruby data types, including strings.

After reading through that, I decided to dive right into the MRI 1.9.3 C source code to learn more about how Ruby handles strings; since I use RVM for me the Ruby source code is located under ~/.rvm/src/ruby-1.9.3-preview1. I started by looking at include/ruby/ruby.h, which defines all of the basic Ruby data types, and string.c, which implements Ruby String objects.

Reading the C code I discovered that Ruby actually uses three different types of string values, which I call:
<ul>
  <li>Heap Strings,</li>
  <li>Shared Strings, and</li>
  <li>Embedded Strings</li>
</ul>

I found this fascinating! For years I’ve assumed every Ruby String object was like every other String object. But it turns out this is not true! Let’s take a closer look...

## Heap Strings

The standard and most common way for Ruby to save string data is in the “heap.” The heap is a core concept of the C language: it’s a large pool of memory that C programmers can allocate from and use via a call to the <span class="code">malloc</span> function. For example, this line of C code allocates a 100 byte chunk of memory from the heap and saves its memory address into a pointer:

<pre type="c">
char *ptr = malloc(100);
</pre>

Later, when the C programmer is done with this memory, she can release it and return it to the system using <span class="code">free</span>:

<pre type="c">
free(ptr);
</pre>

Avoiding the need to manage memory in this very manual and explicit way is one of the biggest benefits of using any high level programming language, such as Ruby, Java, C#, etc. When you create a string value in Ruby code like this, for example:

<pre type="ruby">
str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit"
</pre>

... the Ruby interpreter creates a structure called “RString” that conceptually looks like this:

![heap strings](http://patshaughnessy.net/assets/2012/1/4/heap-string.png)

You can see here that the RString structure contains two values: <span class="code">ptr</span> and <span class="code">len</span>, but not the actual string data itself. Ruby actually saves the string character values themselves in some memory allocated from the heap, and then sets <span class="code">ptr</span> to the location of that heap memory, and <span class="code">len</span> to the length of the string.

Here’s a simplified version of the C RString structure:

<pre type="c">
struct RString {
  long len;
  char *ptr;
};
</pre>

I’ve simplified this a lot; there are actually a number of other values saved in this C struct. I’ll discuss some of them next, and others I’ll skip over for today. If you’re not familiar with C, you can think of <span class="code">struct</span> (short for “structure”) as an object that contains a set of instance variables, except in C there’s no object at all - a struct is just a chunk of memory containing a few values.

I refer to this type of Ruby string as “Heap String,” since the actual string data is saved in the heap.

## Shared Strings

Another type of string value that the Ruby interpreter uses is called a “Shared String” in the Ruby C source code. You create a Shared String every time you write a line of Ruby code that copies one string to another, similar to this:

<pre type="ruby">
str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit"
str2 = str
</pre>

Here the Ruby interpreter has realized that you are assigning the same string value to two variables: str and str2. So in fact there’s no need to create two copies of the string data itself; instead Ruby creates two RString values that share the single copy of the string data. The way this works is that both RString structs contain the same ptr value to the shared data... meaning both strings contain the same value. There’s also a <span class="code">shared</span> value saved in the second RString struct that points to the first RString struct. There are some other details which I’m not showing here, such as some bit mask flags that Ruby uses to keep track of which RString’s are shared and which are not.

![shared strings](http://patshaughnessy.net/assets/2012/1/4/shared-string.png)

Aside from saving memory, this also speeds up execution of your Ruby programs dramatically by avoiding the need to allocate more memory from the heap using another call to <span class="code">malloc</span>. <span class="code">Malloc</span> is actually a fairly expensive operation: it takes time to track down available memory of the proper size in the heap, and also to keep track of it for freeing later.

Here’s a somewhat more accurate version of the C RString structure, including the <span class="code">shared</span> value:

<pre type="console">
struct RString {
    long len;
    char *ptr;
    VALUE shared;
};
</pre>

Strings that are copied from one variable to another like this I call “Shared Strings.”

## Embedded Strings

The third and last way that MRI Ruby 1.9 saves string data is by embedding the characters into the RString structure itself, like this:

<pre type="ruby">
str3 = "Lorem ipsum dolor"
</pre>

![embedded strings](http://patshaughnessy.net/assets/2012/1/4/embedded-string.png)

This RString structure contains a character array called <span class="code">ary</span> and not the <span class="code">ptr</span>, <span class="code">len</span> and <span class="code">shared</span> values we saw above. Here’s another simplified definition of the same RString structure, this time containing the <span class="code">ary</span> character array:

<pre type="c">
struct RString {
  char ary[RSTRING_EMBED_LEN_MAX + 1];
}
</pre>

If you’re not familiar with C code, the syntax <span class="code">char ary[100]</span> creates an array of 100 characters (bytes). Unlike Ruby, C arrays are not objects; instead they are really just a collection of bytes. In C you have to specify the length of the array you want to create ahead of time.

How do Embedded Strings work? Well, the key is the size of the <span class="code">ary</span> array, which is set to <span class="code">RSTRING_EMBED_LEN_MAX+1</span>. If you’re running a 64-bit version of Ruby <span class="code">RSTRING_EMBED_LEN_MAX</span> is set to 24. That means a short string like this will fit into the RString <span class="code">ary</span> array:

<pre type="ruby">
str = "Lorem ipsum dolor"
</pre>

... while a longer string like this will not:

<pre type="ruby">
str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit"
</pre>

## How Ruby creates new string values

Whenever you create a string value in your Ruby 1.9 code, the interpreter goes through an algorithm similar to this:

<ul>
  <li>Is this a new string value? Or a copy of an existing string? If it’s a copy, Ruby creates a Shared String. This is the fastest option, since Ruby only needs a new RString structure, and not another copy of the existing string data.</li>
  <li>Is this a long string? Or a short string? If the new string value is 23 characters or less, Ruby creates an Embedded String. While not as fast as a Shared String, it’s still fast because the 23 characters are simply copied right into the RString structure and there’s no need to call <span class="code">malloc</span>.</li>
  <li>Finally, for long string values, 24 characters or more, Ruby creates a Heap String - meaning it calls <span class="code">malloc</span> and gets some new memory from the heap, and then copies the string value there. This is the slowest option.</li>
</ul>

## The actual RString structure

For those of you familiar with the C language, here’s the actual Ruby 1.9 definition of RString:

<pre type="c">
struct RString {

  struct RBasic basic;

  union {
    struct {
      long len;
      char *ptr;
      union {
        long capa;
        VALUE shared;
      } aux;
    } heap;

    char ary[RSTRING_EMBED_LEN_MAX + 1];
  } as;
};
</pre>

I won’t try to explain all the code details here, but here are a couple important things to learn about Ruby strings from this definition:

<ul>
<li>The <span class="code">RBasic</span> structure keeps track of various important bits of information about this string, such as flags indicating whether it’s shared or embedded, and a pointer to the corresponding Ruby String object structure.</li>
<li>The <span class="code">capa</span> value keeps track of the “capacity” of each heap string... it turns out Ruby will often allocate more memory than is required for each heap string, again to avoid extra calls to <span class="code">malloc</span> if a string size changes.</li>
<li>The use of <span class="code">union</span> allows Ruby to EITHER save the len/ptr/capa/shared information OR the actual string data itself.</li>
<li>The value of <span class="code">RSTRING_EMBED_LEN_MAX</span> was chosen to match the size of the len/ptr/capa values. That’s where the 23 limit comes from.</li>
</ul>

Here’s the line of code from ruby.h that defines this value:

<pre type="c">
#define RSTRING_EMBED_LEN_MAX ((int)((sizeof(VALUE)*3)/sizeof(char)-1))
</pre>

On a 64 bit machine, sizeof(VALUE) is 8, leading to the limit of 23 characters. This will be smaller for a 32 bit machine.

## Benchmarking Ruby string allocation

Let’s try to measure how much faster short strings are vs. long strings in Ruby 1.9.3 - here’s a simple line of code that dynamically creates a new string by appending a single character onto the end:

<pre type="ruby">
new_string = str + 'x'
</pre>

The <span class="code">new_string</span> value will either be a Heap String or an Embedded String, depending on how long the <span class="code">str</span> variable’s value is. The reason I need to use a string concatenation operation, the <span class="code">+ 'x'</span> part, is to force Ruby to allocate a new string dynamically. Otherwise if I just used <span class="code">new_string = str</span>, I would get a Shared String.

Now I’ll call this method from a loop and benchmark it:

<pre type="ruby">
require 'benchmark'

ITERATIONS = 1000000

def run(str, bench)
  bench.report("#{str.length + 1} chars") do
    ITERATIONS.times do
      new_string = str + 'x'
    end
  end
end
</pre>

Here I’m using the benchmark library to measure how long it takes to call that method 1 million times. Now running this with a variety of different string lengths:

<pre type="ruby">
Benchmark.bm do |bench|
  run("12345678901234567890", bench)
  run("123456789012345678901", bench)
  run("1234567890123456789012", bench)
  run("12345678901234567890123", bench)
  run("123456789012345678901234", bench)
  run("1234567890123456789012345", bench)
  run("12345678901234567890123456", bench)
end
</pre>

We get an interesting result:

<pre type="console">
       user     system      total        real
21 chars  0.250000   0.000000   0.250000 (  0.247459)
22 chars  0.250000   0.000000   0.250000 (  0.246954)
23 chars  0.250000   0.000000   0.250000 (  0.248440)
24 chars  0.480000   0.000000   0.480000 (  0.478391)
25 chars  0.480000   0.000000   0.480000 (  0.479662)
26 chars  0.480000   0.000000   0.480000 (  0.481211)
27 chars  0.490000   0.000000   0.490000 (  0.490404)
</pre>

Note that when the string length is 23 or less, it takes about 250ms to create 1 million new strings. But when my string length is 24 or more, it takes around 480ms, almost twice as long!

Here’s a graph showing some more data; the bars show how long it takes to allocate 1 million strings of the given length:

![string allocation chart](http://patshaughnessy.net/assets/2012/1/4/string-allocations.png)

## Conclusion

Don’t worry! I don’t think you should refactor all your code to be sure you have strings of length 23 or less. That would obviously be ridiculous. The speed increase sounds impressive, but actually the time differences I measured were insignificant until I allocated 100,000s or millions of strings - how many Ruby applications will need to create this many string values? And even if you do need to create many string objects, the pain and confusion caused by using only short strings would overwhelm any performance benefit you might get.

For me I really think understanding something about how the Ruby interpreter works is just fun! I enjoyed taking a look through a microscope at these sorts of tiny details. I do also suspect having some understanding of how Matz and his colleagues actually implemented the language will eventually help me to use Ruby in a wiser and more knowledgeable way. We’ll have to see... stay tuned for some more posts about Ruby internals!

