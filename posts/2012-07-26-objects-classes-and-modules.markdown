title: "Objects, Classes and Modules"
date: 2012/7/26
tag: Ruby

<b>This is an excerpt from the third chapter of an eBook I’m writing this Summer called “Ruby Under a Microscope.” My goal is to teach you how Ruby works internally without assuming you know anything about the C programming language.

If you’re interested in Ruby internals you can [sign up here](https://patshaughnessy.net/ruby-under-a-microscope) and I’ll send you an email when the eBook is finished.  I also posted [one entire chapter](https://patshaughnessy.net/2012/5/9/one-chapter-from-my-upcoming-ebook-ruby-under-a-microscope) in May, and two other excerpts from [Chapter One](https://patshaughnessy.net/2012/6/18/the-start-of-a-long-journey-how-ruby-parses-and-compiles-your-code) and [Chapter Two](https://patshaughnessy.net/2012/6/29/how-ruby-executes-your-code) last month.</b>

<p></p>

<div style="float: left; padding: 17px 30px 10px 0px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td><img src="https://patshaughnessy.net/assets/2012/7/26/three_peppers.jpg"></td></tr>
  </table>
</div>

We all learn very early on that Ruby is an object oriented language, descended from languages like Smalltalk and Simula. Everything is an object and all Ruby programs consist of a set of objects and the messages that are sent back and forth among them. Typically we learn about object oriented programming by looking at how to use objects and what they can do: how they can group together data values and behavior related to those values, how each class should have a single responsibility or purpose or how different objects can be related to each other through encapsulation or inheritance.

But what are Ruby objects, exactly? What information does an object contain? If I were to look at a Ruby object through a microscope, what would I see? Are there any moving parts inside? And what about Ruby classes - all of us know how to create and use Ruby classes, but what exactly is a class? Finally, what are modules in Ruby? How are modules and classes related? What happens when I include a module into a class? How does Ruby find which class or module implements a given method?

In this chapter I’m going to answer these questions by exploring how Ruby works internally. Looking at exactly how Ruby implements objects, classes and modules can give you some insight into how they were intended to be used, and into how to write object oriented programs using Ruby.

## What’s inside a Ruby object?

Ruby saves all of your custom objects inside a C memory structure called <span class="code">RObject</span>, which looks like this in Ruby 1.9 and 2.0:

<img src="https://patshaughnessy.net/assets/2012/7/26/ruby-object.png"/>

On the left is a pointer to the <span class="code">RObject</span> structure. Internally Ruby always refers to any value using these <span class="code">VALUE</span> pointers. On the right you can see the <span class="code">RObject</span> value is divided into two halves: <span class="code">RBasic</span> and <span class="code">RObject</span>. The <span class="code">RBasic</span> section contains information that all values use, not only objects: a set of boolean values called <span class="code">flags</span> which store a variety of internal, technical values and also a class pointer, called <span class="code">klass</span>. The class pointer indicates which class this object is an instance of. At the bottom in the <span class="code">RObject</span> specific portion Ruby saves an array of instance variables that this object instance contains using two values: <span class="code">numiv</span>, the instance variable count, and <span class="code">ivptr</span>, a pointer to an array of values.

<div style="float: right; padding: 0px 0px 0px 30px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td><img src="https://patshaughnessy.net/assets/2012/7/26/cut_pepper.jpg"></td></tr>
    <tr><td align="center"><small><i>If I could slice open a Ruby object, what would I see?</i></small></td></tr>
  </table>
</div>

Summarizing the contents of the <span class="code">RObject</span> structure, we can write a very technical definition of what a Ruby object is:

> Every Ruby object is the combination of a class pointer and an array of instance variables.

At first glance, this definition doesn’t seem that useful at all: it doesn’t help me understand the meaning or purpose behind objects, or how to use them in a Ruby program. Why does Ruby implement objects in this way? The answer is simple: Ruby saves this information in <span class="code">RObject</span> because it has to in order to support the basic features of the language.

For example, suppose I have a simple Ruby class:

<pre type="ruby">
class Mathematician
  attr_accessor :first_name
  attr_accessor :last_name
end
</pre>

Ruby needs to save the class pointer in <span class="code">RObject</span> because every object has to keep track of the class you used to create it:

<pre type="console">
> euler = Mathematician.new
 => #&lt;Mathematician:0x007fbd738608c0>
</pre>

Here by displaying the class name, “#&lt;Mathematician…,” Ruby is displaying the value of the class pointer for the <span class="code">euler</span> object when I inspect it. The hex string that follows is actually the <span class="code">VALUE</span> pointer for the object. This will be different for every instance of <span class="code">Mathematician</span> .

Ruby also has to keep track of any values you save in it - Ruby uses the instance variable array to do this:

<pre type="console">
> euler.first_name = 'Leonhard'
 => "Leonhard" 
> euler.last_name  = 'Euler'
 => "Euler" 
> euler
 => #&lt;Mathematician:0x007fbd738608c0 @first_name="Leonhard", @last_name="Euler"> 
</pre>

Here you can see Ruby now also displays the instance variable array for <span class="code">euler</span> when I inspect it again. Ruby needs to save this array of values in each object since every object instance can have different values for the same instance variables - for example:

<pre type="console">
> euclid = Mathematician.new
> euclid.first_name = 'Euclid'
> euclid
 => #&lt;Mathematician:0x007fabdb850690 @first_name="Euclid">
</pre>

Now let’s take a look at Ruby’s C memory structures in a bit more detail - when you run this simple script, Ruby will create one <span class="code">RClass</span> structure and two <span class="code">RObject</span> structures:

<img src="https://patshaughnessy.net/assets/2012/7/26/script-and-objects.png"/>

I’ll cover how Ruby implements classes with the <span class="code">RClass</span> structure in the next section, but here is how Ruby saves the the mathematician information in the two <span class="code">RObject</span> structures in more detail:

<img src="https://patshaughnessy.net/assets/2012/7/26/more-detail.png"/>

You can see each of the <span class="code">klass</span> values point to the <span class="code">Mathematician</span> <span class="code">RClass</span> structure, and each <span class="code">RObject</span> structure has a separate array of instance variables. Both arrays contain <span class="code">VALUE</span> pointers, the same pointer that Ruby uses to refer to the <span class="code">RObject</span> structure. One of the objects contains two instance variables, while the other contains only one.

This is how Ruby saves custom classes, like my <span class="code">Mathematician</span> class, in <span class="code">RObject</span> structures. But we all know that every Ruby value, including basic data types such as integers, strings or symbols, are also objects. The Ruby source code internally refers to these built in types as “generic” types. How does Ruby store these generic objects? Do they also use the <span class="code">RObject</span> structure? The answer is no: internally Ruby uses a different C memory structure to save values for each of its generic data types, and not <span class="code">RObject</span>. For example, Ruby saves string values in <span class="code">RString</span> structures, arrays in <span class="code">RArray</span> structures and regular expressions in <span class="code">RRegexp</span> structures, etc. Ruby only uses <span class="code">RObject</span> to save instances of custom object classes that you create, and for a few custom object classes Ruby creates internally as well.

However, all of these different structures share the same <span class="code">RBasic</span> information that we saw in <span class="code">RObject</span>:

<img src="https://patshaughnessy.net/assets/2012/7/26/three-structs.png"/>

Since the <span class="code">RBasic</span> structure contains the class pointer, each of these generic data types is also an object - they are all instances of some Ruby class, indicated by the class pointer saved inside of <span class="code">RBasic</span>.

As a performance optimization, Ruby saves small integers, symbols and a few other simple values without any structure at all. Ruby saves these values right inside the <span class="code">VALUE</span> pointer:

<img src="https://patshaughnessy.net/assets/2012/7/26/value-pointer.png"/>

That is, these <span class="code">VALUE</span>s are not pointers at all; instead they are the values themselves. For these simple data types, there is no class pointer. Instead Ruby remembers the class using a series of bit flags saved in the first few bits of the <span class="code">VALUE</span>. For example, all integers have the <span class="code">FIXNUM_FLAG</span> bit set, like this:

<img src="https://patshaughnessy.net/assets/2012/7/26/value-pointer-with-flag.png"/>

Whenever the <span class="code">FIXNUM_FLAG</span> is set, Ruby knows this <span class="code">VALUE</span> is really a small integer, an instance of the <span class="code">Fixnum</span> class, and not a pointer to a value structure. There is also a similar bit flag to indicate if the <span class="code">VALUE</span> is a symbol, and values such as <span class="code">nil</span>, <span class="code">true</span> and <span class="code">false</span> also have special values.

It’s easy to see that integers, strings and other generic values are all objects using IRB:

<pre type="console">
$ irb
> "string".class
 => String 
> 1.class
 => Fixnum
> :symbol.class
 => Symbol 
</pre>

Here we can see Ruby saves a class pointer or the equivalent bit flag for all of these values by calling the <span class="code">class</span> method on each of them. The <span class="code">class</span> method returns the class pointer… or at least the name of the class the <span class="code">klass</span> pointer refers to.

Now let’s reread our definition of a Ruby object from above:

> Every Ruby object is the combination of a class pointer and an array of instance variables.

What about instance variables for generic objects? Do integers, strings and other generic data values have instance variables? That would seem a bit odd. But if integers and strings are objects, then this must be true! And if this is true, where does Ruby save these values, if it doesn’t use the <span class="code">RObject</span> structure?

Using the <span class="code">instance_variables</span> method you can see that each of these basic values can also contain an array of instance variables, as strange as that might seem at first:

<pre type="console">
$ irb
> str = "some string value"
 => "some string value" 
> str.instance_variables
 => [] 
> str.instance_variable_set("@val1", "value one")
 => "value one" 
> str.instance_variables
 => [:@val1] 
> str.instance_variable_set("@val2", "value two")
 => "value two" 
> str.instance_variables
 => [:@val1, :@val2] 
</pre>

You can repeat the same exercise using symbols, arrays, or any Ruby value whatsoever. Every Ruby value is an object, and every object contains a class pointer and an array of instance variables.

Internally, Ruby uses a bit of a hack to save instance variables for generic objects - that is, for objects that don’t use an <span class="code">RObject</span> structure. When you save an instance variable in a generic object, Ruby saves it in a special hash called the <span class="code">generic_iv_table</span>. This hash maintains a map between generic objects and pointers to other hashes that contain each object’s instance variables. For my <span class="code">str</span> string example above, this would look like this:

<img src="https://patshaughnessy.net/assets/2012/7/26/generic-iv-table.png"/>

## Experiment 3-1: How long does it take to save a new instance variable?

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## Deducing what’s inside the RClass structure

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## Experiment 3-2: Where does Ruby save class methods?

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## How Ruby implements modules and method lookup

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## Experiment 3-3: Modifying a module after including it

This experiment was suggested by [Xavier Noria](https://twitter.com/fxn/)… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## InvokeDynamic and method lookup in JRuby 1.7

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).

## Modules and methods in Rubinius

… read it in the [finished eBook](https://patshaughnessy.net/ruby-under-a-microscope).
