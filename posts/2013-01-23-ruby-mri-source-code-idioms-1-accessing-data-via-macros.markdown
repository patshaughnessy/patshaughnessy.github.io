title: "Ruby MRI Source Code Idioms #1: Accessing Data Via Macros"
date: 2013/1/23
tag: MRI Idioms

<div style="float: left; margin: 8px 25px 5px 0px; line-height:16px;">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td align="center" style="background-color: rgb(248, 248, 255);padding: 5px;"><img
    src="https://patshaughnessy.net/assets/2013/1/23/definition.png"></td></tr>
    <tr><td align="center"><i>From: <a href="http://en.wiktionary.org/wiki/idiom">wiktionary.org</a></i></td></tr>
  </table>
</div>

Don’t be afraid of reading Ruby’s C source code. If you’re a Ruby developer, it
can be a lot of fun to see how things work “under the hood,” and studying Ruby
internals can give you a deeper understanding of what Ruby really is and how to
use it.  A good way to get started looking at Ruby’s source code, to get a “lay
of the land,” would be to watch Peter Cooper and I walk through some code in a
[screencast we recorded last
month](http://www.rubyinside.com/ruby-mri-code-walk-tour-6020.html). However,
you might be reluctant to read Ruby’s source code on your own since it’s
written in C, a verbose, confusing low-level language that most of us don’t
have time to learn.

But is Ruby really written in C? I find Ruby’s C code to be very _idiomatic_; at
times it almost resembles another dialect or language. To see what I mean, take
a look at this snippet from MRI’s array.c file, which implements the
<span class="code">Array#compact!</span> method:

<img src="https://patshaughnessy.net/assets/2013/1/23/code1.png"/>

Most of the code in this function is composed of C macros; these appear in
capital letters. Macros are text formulas that C developers can use to make
their code more concise and easier to read. The Ruby core team very often uses
macros to access data, which is what most of the code in the function above is
doing. This is an example of what I call an “MRI idiom.”

Today I’m going to take a close look at this method, <span
  class="code">Array#compact!</span>, and explain how it works at a C
programming level. I’ll do this by explaining what these different macros do.
Beyond understanding this one method, learning this MRI idiom of accessing data
via macros will help you understand many, many different functions in the Ruby
C source code. In a series of upcoming blog posts, I’ll look at some different
MRI idioms as well.

## Array#compact!

Before we get to the C code, let’s review what the <span
  class="code">compact!</span> method does in Ruby. Here’s the example used in
the Ruby docs, the C comment that appears just above this code in array.c:

<img src="https://patshaughnessy.net/assets/2013/1/23/code-comment1.png"/>

The <span class="code">rb_ary_compact_bang</span> C function I showed above
actually implements this behavior. Whenever you use the <span
  class="code">compact!</span> method in your code, Ruby internally calls this
function and passes in the target array. Somehow it has to identify and remove
the <span class="code">nil</span> values. Also, it has to update the target
array, or the receiver of the <span class="code">compact!</span> message - the
normal <span class="code">compact</span> method would return a new array
instead and leave the original unchanged. Finally, it should return nil if
there were no changes made to the array.

## Array data in MRI

To understand how MRI accesses data via macros, let’s first look at how it
stores data, at least for array objects. Ruby stores all arrays and their
contents using the <span class="code">RArray</span> C struct, like this:

<img src="https://patshaughnessy.net/assets/2013/1/23/rarray1.png"/>

I won’t cover all the details here today; in fact, you can see some other MRI
idioms at work here, such as the <span
  class="code">ary[RARRY_EMBED_LEN_MAX]</span> struct member which is used for
space optimization, or the <span class="code">shared</span> value, which is
used for copy-on-write optimization. I wrote about these how these idioms work
in the String class last year; see: [Never create Ruby strings longer than 23
characters](https://patshaughnessy.net/2012/1/4/never-create-ruby-strings-longer-than-23-characters),
and [Seeing double: how Ruby shares string
values](https://patshaughnessy.net/2012/1/18/seeing-double-how-ruby-shares-string-values).

The details to learn for today are that: Ruby stores the Array data values in a
C memory array, tracked by the <span class="code">VALUE *ptr</span> pointer.  Ruby (usually) tracks the
length of the array in <span class="code">len</span>, and Ruby keeps a capacity
value in <span class="code">capa</span>. The capacity records the size of the
allocated C memory array (as a count of VALUEs, not as bytes). Ruby frequently
allocates more memory for an array than what is actually needed.

By taking some time to first study the <span class="code">RArray</span> C structure, you can quite easily
understand large parts of the Ruby C source code in the array.c file… and
understand how Ruby arrays actually work!

## Accessing array data via macros

However, as I explained above if you read array.c you’ll immediately notice
that Ruby doesn’t use the <span class="code">RArray</span> struct directly. Instead, it accesses values
such as <span class="code">ptr</span>, <span class="code">len</span> and <span
  class="code">capa</span> using macros. If you’re not a C programmer, macros
are formulas that the C “pre-processor” evaluates and substitutes into the C
code just before the C compiler runs.

So let’s just take a look at these macros and see what they do - should be simple,
right?

<img src="https://patshaughnessy.net/assets/2013/1/23/gibberish.png"/>

Oops! C programming isn’t that simple! One of the most challenging parts of
reading and understanding Ruby’s code is figuring out what these macros mean
and do. But this is essential, since they are used so frequently across the
code base. Most of the complexity in these particular formulas has to do with
Ruby’s embedded data idiom, which I’ll cover in one of my upcoming blog
posts.

But don’t despair, normally these macros just boil down to something very
simple:

<ol>

<li><span class="code">RARRAY_PTR(ary)</span> - this returns a pointer to the array’s actual data,
normally the same as the <span class="code">ptr</span> value. In <span
  class="code">rb_ary_compact_bang</span>, Ruby initializes the <span
  class="code">p</span> and <span class="code">t</span> pointers using <span
  class="code">RARRAY_PTR</span>:

<p><img src="https://patshaughnessy.net/assets/2013/1/23/rarray2.png"/></p></li>

<li><span class="code">RARRAY_LEN(ary)</span> - this returns the length of the array, normally just the
<span class="code">len</span> value. <span class="code">rb_ary_compact_bang</span> initializes the <span class="code">end</span> pointer using
<span class="code">RARRAY_LEN,</span> by adding the length to <span class="code">p</span>:

<p><img src="https://patshaughnessy.net/assets/2013/1/23/rarray3.png"/></p>

<p>In this diagram, I assume the length of the array is 3, and the
capacity of the array is 5.</p><br/></li>

<li><span class="code">ARY_CAPA(ary)</span> - the returns the capacity of the array, or the amount of memory Ruby
actually allocated for the array’s elements. Ruby allocates more memory than
necessary to avoid repeated memory allocations when an array changes size. This
normally just returns <span class="code">capa</span> (except when Ruby is using certain optimizations):

<p/>

<img src="https://patshaughnessy.net/assets/2013/1/23/rarray4.png"/></li>

<li><span class="code">ARY_SET_LEN(ary, n)</span> - this updates the array length, which is normally the <span class="code">len</span>
value:

<p/>

<img src="https://patshaughnessy.net/assets/2013/1/23/rarray5.png"/></li>

</ol>

## Putting it all together

Now that we understand how MRI accesses data values via macros, it’s not hard
to follow most of the code in <span class="code">rb_ary_compact_bang</span>:

<img src="https://patshaughnessy.net/assets/2013/1/23/rb_ary_compact_bang.png"/>

Let’s walk through it, starting with this loop:

<img src="https://patshaughnessy.net/assets/2013/1/23/compact-loop.png"/>

This C pointer arithmetic loop actually does the compact operation - if
you’re not familiar with C, here’s a 5 second lesson on how pointers work:

* If <span class="code">p</span> is a pointer, then <span
  class="code">*p</span> means to return the value that <span
  class="code">p</span> points to, and:

* <span class="code">*p++</span> means return the value <span
  class="code">p</span> points to, but also increment <span
  class="code">p</span> by one after obtaining this value, so it points to the
next value.

Let’s walk through this loop visually, to get a sense of how it works. I’ll use
the example from the Ruby docs:

<img src="https://patshaughnessy.net/assets/2013/1/23/ruby-example.png"/>

First, Ruby checks whether the first value in the array is nil, using another
macro: <span class="code">NIL_P(*t)</span>:

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting1.png"/>

Since it is not nil, Ruby copies the “a” onto itself:

<img src="https://patshaughnessy.net/assets/2013/1/23/else.png"/>

This has no effect, but both <span class="code">p</span> and <span class="code">t</span> move forward to the next element:

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting2.png"/>

Now <span class="code">NIL_P(*t)</span> is true, so Ruby just increments <span class="code">t</span> and not <span class="code">p</span>:

<img src="https://patshaughnessy.net/assets/2013/1/23/if.png"/>

Now <span class="code">t</span> points to the “b”, while <span class="code">p</span> remains the same:

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting3.png"/>

This time, <span class="code">NIL_P(*t)</span> is false, so Ruby copies the value “b” back, and
increments both pointers:

<img src="https://patshaughnessy.net/assets/2013/1/23/else.png"/>

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting4.png"/>

Continuing through the loop again, <span class="code">NIL_P(*t)</span> will be true this time:

<img src="https://patshaughnessy.net/assets/2013/1/23/if.png"/>

And Ruby will again only increment <span class="code">t</span>:

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting5.png"/>

Iterating again, <span class="code">t</span> points to the “c”, and so Ruby will copy it back:

<img src="https://patshaughnessy.net/assets/2013/1/23/else.png"/>

And again now both pointers will be incremented:

<img src="https://patshaughnessy.net/assets/2013/1/23/compacting6.png"/>

Finally, Ruby increments <span class="code">t</span> past the last nil value,
and exits the loop when <span class="code">t == end</span>. This leaves us with
the compacted array, which ends at the current location of <span
  class="code">p</span>.

## Wrapping up

Now the compacting operation is done, and Ruby just needs to wrap things up and
leave the array’s internal values in a self consistent state.

First, Ruby calculates the new length, using the <span class="code">p</span>
pointer and <span class="code">RARRAY_PTR</span> which returns the start of the
array again:

<img src="https://patshaughnessy.net/assets/2013/1/23/calc-length.png"/>

You can see if the new length is the same as the original length was Ruby will
return <span class="code">nil</span> and exit immediately. Otherwise, Ruby uses
<span class="code">ARY_SET_LEN</span> to save the new length back in the <span
  class="code">RArray</span> struct. In the example above, the new length would
be 3.

The last bit of confusing code in <span class="code">rb_ary_compact_bang</span>
updates the array’s capacity, using the <span class="code">ARY_CAPA</span>
macro:

<img src="https://patshaughnessy.net/assets/2013/1/23/reset-capacity.png"/>

This code is still very confusing, but at least we know now that <span
  class="code">ARY_CAPA(ary)</span> returns the current capacity of the array.
Remember the capacity is the actual size of the memory allocated to hold the
array data, measured as an element count. Here Ruby calls the <span
  class="code">ary_resize_capa</span> method if the new size of the smaller,
compacted  array is less than half of the current capacity, which will free up
some memory. The condition about <span class="code">ARY_DEFAULT_SIZE</span>
enforces a minimum capacity - this constant is set to 16 at the top of array.c:

<img src="https://patshaughnessy.net/assets/2013/1/23/default-size.png"/>

Note: this doesn’t mean that all new, empty arrays allocate enough memory to have a
capacity of at least 16; things aren’t so simple. I’ll explain how new arrays
look in my next post.

## Loose ends

I glossed over a few details here. First of all, as I said above sometimes
<span class="code">RARRAY_PTR</span> and <span class="code">RARRY_LEN</span>
sometimes work differently. I’ll cover this in my next blog post, on Ruby’s
“embedded data” idiom. Second, I didn’t explain the call to <span
  class="code">rb_ary_modify</span>, which is used for Ruby’s copy-on-write
optimization, another MRI idiom. While these are optimizations Ruby uses
internally to speed up your programs, I consider them to be idioms also since
they have a broad, widespread impact on the way MRI’s C code was
written. 
