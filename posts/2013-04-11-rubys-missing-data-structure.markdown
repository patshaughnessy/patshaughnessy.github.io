title: "Ruby’s Missing Data Structure"
date: 2013/4/11
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
  <img src="http://patshaughnessy.net/assets/2013/4/11/linked-list2.png">
</div>

Have you ever noticed Ruby doesn’t include support for linked lists? Most
computer science textbooks are filled with algorithms, examples and exercises
based on linked lists: inserting or removing elements, sorting lists, reversing
lists, etc. Strangely, however, there is no linked list object in Ruby.

Recently after studying Haskell and Lisp for a couple of months, I returned to
Ruby and tried to use some of the functional programming ideas I had learned
about. But how do I create a list in Ruby? How do I add or remove an element
from a list in Ruby? Ruby contains fast, internal C implementations of the
Array and Hash classes, and in the standard library you can find Ruby code
implementing the Set, Matrix, and Vector classes among many other things. But
no linked lists – why?

The answer is simple: Matz included features you would normally associate with
linked lists in the Ruby Array class. For example, you can use <span
  class="code">Array#push</span> and <span class="code">Array#unshift</span> to
add an element to an array, or <span class="code">Array#pop</span> and <span
  class="code">Array#shift</span> to remove an element from an array.

[Today on RubySource.com](http://rubysource.com/rubys-missing-data-structure/)
I wrote about a few common operations you would normally use a linked list for,
and then took a look at how you would implement them using an array. I also
looked into how Ruby’s array object works internally.


