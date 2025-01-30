title: "Inserting One New Element into Hashes of Varying Sizes"
date: 2025/02/04

I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished. In the meantime, here’s an
excerpt from the updated book.  Experiment 7-2 is a fun way to see exactly when
Ruby increases the number of bins in a hash table.  It was one of the first
experiments I wrote back in 2013 that digs into the details of how Ruby works
internally.

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

<div style="float: left; padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img id="microscope" src="http://localhost/assets/2025/2/4/experiment.png"><br/>
</div>

## Experiment 7-2: Inserting One New Element into Hashes of Varying Sizes

One way to test whether this rehashing, or redistribution, of entries really
occurs when Ruby expands a hash is to measure the amount of time Ruby takes to
save one new element into existing hashes of different sizes. As we add more
elements to the same hash, we should eventually see evidence that Ruby is taking
extra time to rehash the elements. 

<div style="clear: left"></div>

The code for this experiment is shown in Listing 7-3.

<pre type="ruby">
  require 'benchmark'

(1) 100.times do |size|

    hashes = []
(2) 10000.times do
      hash = {}
      (1..size).each do
        hash[rand] = rand
      end 
      hashes << hash
    end 

    GC.disable

    Benchmark.bm do |bench|
      bench.report("adding element number #{size+1}") do
        10000.times do |n| 
(3)       hashes[n][size] = rand
        end 
      end 
    end 

    GC.enable

  end
</pre>
<div style="font-style: italic; margin: 0 0 20px 0">
  Listing 7-3: Adding one more element to hashes of different sizes  
</div>

At 1 the outer loop iterates over hash sizes from 0 to 100, and at 2 the inner
loop creates 10,000 hashes of the given size. After disabling garbage
collection, this experiment uses the benchmark library to measure how long it
takes Ruby to insert a single new value at 3 into all 10,000 hashes of the given
size. The results are surprising! Figure 7-13 shows the results for Ruby 3.2. 

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="http://localhost/assets/2025/2/4/Figure-7-3.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 7-13: Time to add 10,000 key/value pairs vs. hash size (Ruby 3.2) 
</span>
</div>

Interpreting these data values from left to right, we see the following:

* It takes about 2.2 ms to insert the first element into an empty hash (10,000
times). 

* As the hash size increases from 2 to 8, the amount of time required to insert
a new element is less: between 1.2ms and 1.5ms.

* Inserting the 9th key/value pair takes much longer, almost 8ms for 10,000
hashes! 

* Next, as the hash size increases from 10 up to 33, once again Ruby can insert
new elements into the quickly, between about 1.4ms and 2ms (10,000 times).

* A huge spike! It takes almost 19ms to insert the 33rd element.

* And then once again starting with the 34th element, the time to insert each
element reduced to around 1.5ms-2.ms.

* A 3rd spike appears when Ruby inserts the 65th element.

* The graph once again flattens and returns to around 2ms per element (10,000
times).

What’s going on here? Well, Ruby spends the extra time required to insert that
33rd key/value pair expanding the hash table: reallocating an entries array from
32 to 64 entries, and the bin array from 64 to 128 bins, and then reassigning
the `st_table_entry` structures to the new bin array. Ruby expands the entries and
bins arrays a second time again after inserting the 65 entry, this from from 64
to 128 entries and 128 to 256 bins. (Recall the `st_features` table, shown on page
15, used powers of 2 to determine these array sizes.)

The two smaller spikes on the 1st and 9th insert in this figure are curious.
While not as pronounced as the spike at the 33rd element, these smaller spikes
are nonetheless noticeable. As it turns out, Ruby contains another optimization
that speeds up hash access even more for small hashes that contain less than 9
elements.