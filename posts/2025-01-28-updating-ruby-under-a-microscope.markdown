title: "Updating Ruby Under a Microscope"
date: 2025/01/28

<div style="float: left; padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img id="microscope" src="https://patshaughnessy.net/assets/2014/12/17/microscope.png"><br/>
<span style="font-style: italic; font-size: small">
  Ruby stores much of its own internal data in hash tables.
</span>
</div>

I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while to finish. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.

In the meantime, here’s an excerpt from a rewrite of Chapter 7 about hash
tables. Vladimir Makarov and the Ruby team redesigned Ruby’s hash table
implementation back in 2015 to use open addressing, shortly after I published
the first edition of RUM.  This seemed like a good place to start.

<div style="clear: left"></div>

## Chapter 7: The Hash Table: The Workhorse Of Ruby Internals

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>Hash Tables in Ruby</td><td>3</td>
	</tr>
	<tr>
		<td>Saving a Value in a Hash Table</td><td>3</td>
	</tr>
	<tr>
		<td>Retrieving a Value from a Hash Table</td><td>5</td>
	</tr>
	<tr>
		<td>Experiment 7-1: Retrieving a Value from Hashes of Varying Sizes</td><td>7</td>
	</tr>
	<tr>
		<td>How Hash Tables Expand to Accommodate More Values</td><td>9</td>
	</tr>
	<tr>
		<td>Hash Collisions and Open Addressing</td><td>9</td>
	</tr>
	<tr>
		<td>Searching For an Element Using Open Addressing</td><td>11</td>
	</tr>
	<tr>
		<td>Expanding a Hash Table</td><td>14</td>
	</tr>
	<tr>
		<td>How Does Ruby Decide How Many Entries And Bins To Use?</td><td>15</td>
	</tr>
	<tr>
		<td>Experiment 7-2: Inserting One New Element into Hashes of Varying Sizes</td><td>17</td>
	</tr>
	<tr>
		<td>Optimization for Small Hashes</td><td>20</td>
	</tr>
	<tr>
		<td>How Does Ruby Convert A Packed Hash Into A Hash Table?</td><td>22</td>
	</tr>
	<tr>
		<td>How Ruby Implements Hash Functions</td><td>23</td>
	</tr>
	<tr>
		<td>Experiment 7-3: Using Objects as Keys in a Hash</td><td>25</td>
	</tr>
	<tr>
		<td>Summary</td><td>30</td>
	</tr>
</table>
</div>

## Hash Tables in Ruby

*Hash tables* are a commonly used, well-known, age-old concept in computer
science. They organize values into groups, or *bins*, based on an integer value
calculated from each value — a *hash*. When you need to find a value, you can
figure out which bin it’s in by recalculating its hash value, thus speeding up
the search. 

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/1/28/bins.png"><br/>
<span style="font-style: italic; font-size: small">
  Every time you write a method, Ruby creates an entry in a hash table. 
</span>
</div>

<div style="clear: right"></div>

## Saving a Value in a Hash Table

Figure 7-1 shows a single hash object and its hash table.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/1/28/Figure-7-1.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 7-1: A Ruby hash object with an empty hash table 
</span>
</div>

On the left is the `RHash` (short for *Ruby hash*) structure. On the right, you see
the hash table used by this hash, represented by the `st_table` structure. This C
structure contains the basic information about the hash table, including the
number of entries saved in the table and pointers to the entries and bins. Each
`RHash` structure contains a pointer to a corresponding `st_table` structure. For
many hashes, Ruby initially creates 32 entries and 64 bins. (Hashes with 8 or
fewer entries work somewhat differently; see “Optimization for Small Hashes” on
page 20.) The best way to understand how a hash table works is by stepping
through an example. Suppose I add a new key/value to a hash called `my_hash`: 

```
my_hash[:key] = "value"
```

While executing this line of code, Ruby saves the key and value into the first
entry, as shown in Figure 7-2.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/1/28/Figure-7-2.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 7-2: A Ruby hash object containing a single value
</span>
</div>

Here you can see the new key/value pair inside the first entry slot, called an
`st_table_entry` in Ruby’s C source code. Ruby saves the keys and values in the
entries array in the order you add them. This makes it easy for Ruby to return
them back to you in the same order. Also see that Ruby saved an entry index of 0
in the third bin, number 2. Ruby did this by taking the given key — in this
example, the symbol `:key` — and passing it to an internal hash function that
returns a pseudorandom integer: 

```
some_value = internal_hash_function(:key)
```

Next, Ruby takes the hash value — in this example, `some_value` — and calculates the modulus by the number of bins, which is the remainder after dividing by the number of bins.

```
some_value % 64 = 2
```

<div style="margin-bottom: 2.5rem; font-style: italic">
NOTE: In Figure 7-2, I assume that the actual hash value for <span
class="code">:key</span> divided by 64 leaves a remainder of 2. Later in this
chapter, I’ll explore in more detail the hash functions that Ruby actually uses.
We’ll see how using 64 bins (a power of 2) speeds up this calculation.  </div>

Now let’s add a second element to the hash:

```
my_hash[:key2] = "value2"
```

This time let’s imagine that the hash value of `:key2` divided by 64 yields a
remainder of 5. 

```
internal_hash_function(:key2) % 64 = 5
```

Figure 7-3 shows that Ruby fills in a second `st_table_entry` structure in the
entries array, and the entry index 1 in bin number 5, the sixth bin.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/1/28/Figure-7-3.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 7-3: A Ruby hash object containing two values
</span>
</div>

## Retrieving a Value from a Hash Table 

The benefit of using a hash table becomes clear when you ask Ruby to retrieve
the value for a given key. For example: 

```
p my_hash[:key]
 => "value"
```

If Ruby had saved all of the keys and values in an array or linked list, it
would have to iterate over all the elements in that array or list, looking for
:key. This might take a very long time, depending on the number of elements. But
using a hash table, Ruby can jump straight to the key it needs to find by
recalculating the hash value for that key.  To recalculate the hash value for a
particular key, Ruby simply calls the hash function again: `some_value =
internal_hash_function(:key)`.

Then, it redivides the hash value by the number of bins to get the remainder, or
the modulus. `some_value % 64 = 2` At this point, Ruby knows to look in bin
number 2 and finds the entry index 0, and in turn finds the entry with the key
of `:key` in entry number 0 or the first entry slot.  Ruby can later find the
value for `:key2` by repeating the same hash calculation
`internal_hash_function(:key2) % 64 = 5`.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/1/28/Figure-7-4.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 7-4: Finding Values in a Hash Table
</span>
</div>

Figure 7-4 explains how Ruby would find these two values in the hash table: On
the left side, the first arrow starts from the third bin (bin index 2 =
`internal_hash_function(:key) % 64`) and points to the first key/value pair, at
index 0. To the right, the second arrow starts from the sixth bin (bin index 5 =
`internal_hash_function(:key2) % 64`) and points to the second key/value pair, at
index 1.

<span style="font-style: italic">
NOTE: The C library used by Ruby to implement hash tables was written in the
1980s by Peter Moore from the University of California, Berkeley. Later in 2015
Vladimir Makarov rewrote the hash table code, using a data structure which saves
the entry and bin arrays in contiguous memory segments. By saving all the
entries and bins nearby each other in memory, modern CPUs are able to cache all
of the data in the hash table more efficiently, speeding up the overall process.
You can find Makarov’s hash table code in the C code files st.c and
include/ruby/st.h. All of the function and structure names in that code use the
naming convention st_. The definition of the RHash structure that represents
every Ruby Hash object is in the include/ruby/ruby.h file. Along with RHash,
this file contains all of the other primary object structures used in the Ruby
source code: RString, RArray, RValue, and so on. 
</span>
