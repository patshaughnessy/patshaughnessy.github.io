title: "Using Different Size Pools"
date: 2025/02/11
tag: Updating Ruby Under a Microscope

<div style="float: left; padding: 8px 50px 20px 0px; text-align: center; line-height:18px">
<img id="microscope" src="https://patshaughnessy.net/assets/2014/12/17/microscope.png"><br/>
<span style="font-style: italic; font-size: small">
  The garbage collector is where Ruby<br/>objects are born and where they die.
</span>
</div>

I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished. In the meantime, here’s an excerpt from the updated book.

TODO

## Chapter 12: Garbage Collection Basics 

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>Where Do Ruby Values Live?</td><td>4</td>
	</tr>
	<tr>
		<td>The RVALUE Structure</td><td>7</td>
	</tr>
	<tr>
		<td>RVALUE Written in C</td><td>8</td>
	</tr>
	<tr>
		<td>The Free List</td><td>9</td>
	</tr>
	<tr>
		<td>Embedded Values</td><td>10</td>
	</tr>
	<tr>
		<td>Size Pools</td><td>12</td>
	</tr>
	<tr>
		<td>Experiment 12-1: Using Different Size Pools</td><td>14</td>
	</tr>
	<tr>
		<td>Cleaning Up Unused Values</td><td>19</td>
	</tr>
	<tr>
		<td>Filling Up a Page</td><td>19</td>
	</tr>
	<tr>
		<td>Marking</td><td>22</td>
	</tr>
	<tr>
		<td>Sweeping</td><td>24</td>
	</tr>
	<tr>
		<td>How Ruby Frees An Object</td><td>28</td>
	</tr>
	<tr>
		<td>Experiment 12-2: When Does Ruby Collect Your Garbage?</td><td>29</td>
	</tr>
	<tr>
		<td>Summary</td><td>25</td>
	</tr>
</table>
</div>

## Experiment 12-1: Using Different Size Pools

TODO

Ruby saves most of the values you create in your programs in memory segments
called pages. During startup, Ruby’s memory management system initially creates
six pages. As your program runs, Ruby will allocate more and more pages to hold
your program’s values. Ruby also saves values it creates internally in these
pages. In fact, because Ruby’s parser and compiler also use pages, Ruby will
often allocate 30 or more pages and fill them with values before your code even
starts running.

The most common type of page can save 1638 40-byte values. Figure 12-1 shows a
simplified view of a single page.

<div style="clear: left"></div>

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="50%" src="http://localhost/assets/2025/2/11/Figure-12-1.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-1: A Simplified View of a Page Containing 40-Byte Slots
</span>
</div>

On the right, Figure 12-1 shows a <span class="code">heap_page_body</span>
structure divided up into slots. Each slot is a location in memory that can hold
a single Ruby value. Most pages use 40-byte slots: multiplying the number of
slots (1638 slots) by the size of each slot (40 bytes/slot) gives us the total
size of a single page: 65520 or about 64k of memory.

On the left, Figure 12-1 shows a <span class="code">heap_page</span> control structure, which among other
things holds the total number of slots available in the body (<span class="code">total_slots</span>), and
a count of how many of these slots are free or available to hold new values
(<span class="code">free_slots</span>). This is a simplified view of the
<span class="code">heap_page</span> structure; Ruby also saves other values here which it uses during the
garage collection process, as we’ll see later in this chapter.

How does Ruby save values in pages? Let’s take an example. Suppose I run the
very simple one line program shown in Listing 12-1.

<pre type="ruby">
obj = Object.new
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 12-1: Creating A Single Ruby Object
</div>

When Ruby calls the <span class="code">Object.new</span> method, it creates a new instance of the <span class="code">Object</span>
class. As we saw in Chapter 5, Ruby represents objects using the <span class="code">RObject</span>
structure. Since this structure uses only 16 bytes for a simple object like
this, Ruby can save the object in one of the 40 byte slots from Figure 12-1.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="50%" src="http://localhost/assets/2025/2/11/Figure-12-2.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-2: One Ruby Value in a 40-Byte Slot
</span>
</div>

Figure 12-2 repeats Figure 12-1, but also shows where Ruby saves my new object,
at the end of the <span class="code">heap_page_body</span> structure.

Now suppose I add a new line to my program and create a second value, an array.

<pre type="ruby">
obj = Object.new 
arr = Array.new
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 12-2: Creating an Object And an Array
</div>

Listing 12-2 shows my new program which creates two values. When I run this
longer program Ruby saves both values, the object and the array, into the same
40-byte page.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="50%" src="http://localhost/assets/2025/2/11/Figure-12-3.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-3: Two Values in Page
</span>
</div>

Figure 12-3 shows my two values, the object and the array, inside the <span
class="code">heap_page_body</span> structure. Internally Ruby represents my new
array using an <span class="code">RArray</span> structure, which by default for
an empty array occupies 16 bytes. This means that, like my object value, Ruby is
able to save the empty array value into a 40 byte slot.

Note that as your program creates new values, Ruby saves them starting at the
end of the <span class="code">heap_page_body</span> structure, and then works
it’s way backwards saving each new value. We’ll learn why Ruby saves values
backwards, and how it finds the next available slot in “The Free List” section
on page 9.

## The RVALUE Structure

To manage slots and values, Ruby uses an C structure called <span
class="code">RVALUE</span>. <span class="code">RVALUE</span>s can hold one of
many different types of values. Figure 12-4 shows <span
class="code">RVALUE</span>s for three of these types.


<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="http://localhost/assets/2025/2/11/Figure-12-4.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-4: Three Example RVALUEs
</span>
</div>

On the left, Figure 12-4 shows an <span class="code">RVALUE</span> for a string
value, in the center an <span class="code">RVALUE</span> for an array, and on
the right an <span class="code">RVALUE</span> for an object. All of them occupy
40 bytes or less, and so can fit into the 40 byte slots in pages.

One important detail to note about these <span class="code">RVALUE</span>s: each
of them starts with a small header segment called <span
class="code">RBasic,</span> containing two values: <span
class="code">flags</span> and <span class="code">klass</span>. Ruby keeps track
of each value’s class in <span class="code">klass</span>, and saves some
important attributes about the value in <span class="code">flags</span>. As
we’ll see later in this chapter, some of these flags are related to garbage
collection and memory management.

Below the <span class="code">RBasic</span> values (<span
class="code">flags</span> and <span class="code">klass</span>), each of the
three <span class="code">RVALUE</span> structures contain information specific
to each value’s type: <span class="code">RString</span>, <span
class="code">RArray</span>, or <span class="code">RObject</span>.  For example
the <span class="code">len</span> value holds the length of the string or array,
and <span class="code">ivptr</span> contains a pointer to an object’s instance
variables.

Think of the <span class="code">RVALUE</span> structure as an “OR” statement. It
can contain one type of value OR another type OR another type, etc. This
flexibility allows Ruby to refer to and manipulate many different types of Ruby
values in a generic way, using the single <span class="code">RVALUE</span>
structure type.
