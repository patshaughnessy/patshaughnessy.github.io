title: "Using Different Size Pools"
date: 2025/02/11
tag: Updating Ruby Under a Microscope

<i>
<p>
I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.
</p>

<p>
The Ruby team has done a tremendous amount of work over the past decade on
Ruby's garbage collection (GC) implementation. In fact, Ruby's new GC is one of
the key reasons Ruby 3 is so much faster than Ruby 2. To bring all of this work
to light, I decided to rewrite Chapter 12 from scratch, covering garbage
collection in Ruby more accurately and in more depth.  But then, after a few
months, I realized I had gotten carried away and wrote too much material for one
chapter. So the updated book will have two new chapters on garbage collection:
one on garbage collection basics and a second new chapter on incremental and
generational garbage collection. Here's a small excerpt.
</p>
</i>

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

<div style="float: left; padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img id="microscope" src="https://patshaughnessy.net/assets/2025/2/4/experiment.png"><br/>
</div>

## Experiment 12-1: Using Different Size Pools

Ruby 3.2 and later provides a way to see statistics about size pools, the
<span class="code">GC.stat_heap</span> class method. <span
class="code">GC.stat_heap</span> returns a hash as shown in Listing 12-5.

<div style="clear: left"></div>

<pre type="ruby">
require 'pp'
pp GC.stat_heap

{0 =>
  {slot_size: 40,
   heap_eden_pages: 10,
   heap_eden_slots: 16374,
   total_allocated_pages: 10,
   force_major_gc_count: 0,
   force_incremental_marking_finish_count: 0,
   total_allocated_objects: 26378,
   total_freed_objects: 10231},
 1 =>
  {slot_size: 80,

Etc…
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 12-5: Calling GC.stat_heap
</div>

Listing 12-5 shows the output of <span class="code">GC.stat_heap</span> for Size
Pool 0, which includes the slot size (<span class="code">:slot_size=>40</span>),
the number of pages Ruby has allocated so far for this size (<span
class="code">:heap_eden_pages=>10</span>) and the total number of slots
allocated across all of these pages (<span
class="code">:heap_eden_slots=>16374</span>). The output of <span
class="code">GC.stat_heap</span> continues on to show the same statistics for
the other size pools.

We can use <span class="code">GC.stat_heap</span> to investigate how Ruby uses
size pools as we allocate more and more objects. Listing 12-6 shows a Ruby
program that allocates arrays of varying sizes, and then records the output from
<span class="code">GC.stat_heap.</span>

<pre type="ruby">
(1) CAPACITY_ITER = 100
    ALLOCATE_ITER = 10000

(2) all = []

(3) CAPACITY_ITER.times do |capa|

(4)   ALLOCATE_ITER.times do |i|
        all << Array.new(capa)
      end

(5)   total_slots_by_size_pool = []
      GC.stat_heap.each do |size_pool, stats|
        total_slots_by_size_pool[size_pool] = stats[:heap_eden_slots]
      end

      print "#{capa}, "
      print "#{total_slots_by_size_pool[0]}, "
      print "#{total_slots_by_size_pool[1]}, "
      print "#{total_slots_by_size_pool[2]}, "
      print "#{total_slots_by_size_pool[3]}, "
      puts  "#{total_slots_by_size_pool[4]}"

    end
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 12-6: Detecting Which Size Pool Ruby Uses for Arrays of Varying Sizes
</div>

This program contains an inner loop and an outer loop. The outer loop at (3) in
Listing 12-6 iterates over arrays of different capacities, from 1 up to 100
(<span class="code">CAPACITY_ITER</span>). For each array capacity, the program
creates 10,000 (<span class="code">ALLOCATE_ITER</span>) array objects of that
size using the inner loop (4). Note the program saves all of the new arrays into
a single array called <span class="code">all</span>, created at (2). This
insures that Ruby doesn’t free all of our new arrays by running a garbage
collection.

After creating 10,000 arrays of the given capacity, the program saves the <span
class="code">heap_eden_slots</span> value from the return value of <span
class="code">GC.stat_heap</span> for all of the size pools at (5), and then
prints out the results at (6).

Running the code in Listing 12-6 produces this output:

<pre type="ruby">
0, 22923, 6548, 2861, 611, 306
1, 34385, 6548, 2861, 611, 306
2, 44209, 6548, 2861, 611, 306
3, 54033, 6548, 2861, 611, 306
4, 54033, 13914, 2861, 611, 306
5, 54033, 23735, 2861, 611, 306
6, 54033, 34374, 2861, 611, 306
7, 54033, 44195, 2861, 611, 306
8, 54033, 54016, 2861, 611, 306
9, 54033, 54016, 11446, 611, 306
10, 54033, 54016, 21257, 611, 306
11, 54033, 54016, 31477, 611, 306

Etc…
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 12-7: The Output From Running the Program in Listing 12-6
</div>

The output in Listing 12-7 shows how many slots Ruby has allocated in total
after each iteration of the outer, array capacity loop. For example:

<pre>
0, 22923, 6548, 2861, 611, 306
</pre>

… shows that after allocating 10,000 empty arrays, Ruby has now uses a total of
22923 slots for Size Pool 0, 6548 for Size Pool One, 2861 for Size Pool Two,
etc. If you try running this, you will see slightly different values.

Plotting these values, we can see which pool Ruby uses for the new arrays of
various capacities:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/2/11/Figure-12-10.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-10: Allocating Slots for Arrays of Various Sizes - Size Pools 0, 1 and 2.
</span>
</div>

Each bar in Figure 12-10 represents values from a line of output from Listing
12-7. For example, the first bar on the far left plots this output:

<pre>
0, 22923, 6548, 2861, 611, 306
</pre>

The first value, 0, is the position of each bar on the x-axis, while the bar’s
color segments display the following three values: The dark grey bar at the
bottom left corner represents Size Pool 0 (22923), the lighter bar above it
shows Size Pool 1 (6548), and the lightest, top bar shows Size Pool 2 (2861).

Moving to the right, each successive bar shows the values for different array
capacities. Looking over the entire graph, we can see the following pattern in
Figure 12-10:

* The dark grey bars for Size Pool 0 at the bottom of Figure 12-10 increase in
size from capacity 0 through capacity 3, and then remain the same height after
that for capacities 4 and greater.

* The medium grey bars for Size Pool 1 have the same height from capacity 0
through 3, but then increase from capacity 4 through 8. From capacity 9 and
onward, the medium grey bars have the same height again.

* The light grey bars at the top for Size Pool 2 remain small for capacities 0
through 8, and then increase in size steadily for capacities 9 through 18. Then
the remain unchanged after that.

Figure 12-10 shows Ruby saves new arrays using the following size pools:

<pre>
Capacity Range	Size Pool
0-3			0
4-8			1
9-18			2
</pre>

Plotting the entire output from Listing 12-7 up to capacity=100, we see the
following:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/2/11/Figure-12-11.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-11: Complete Output from Listing 12-7
</span>
</div>

Figure 12-11 shows an interesting pattern. Ruby uses size pools 0 through 4 to
save arrays with capacities from 0 up to 78 in a similar way:

<pre>
Capacity Range	Size Pool
0-3			0
4-8			1
9-18			2
19-38			3
39-78			4
</pre>

For each capacity range, the length of the corresponding bars grows steadily,
and then stops growing.

However, once we started to save large arrays with capacities of 79 or more
elements, Ruby saved them in the original Size Pool Zero again. This indicates
that Ruby stopped embedding the array elements in the size pool entirely, and
instead allocated a new, separate memory segment to save the elements. For these
large arrays, small 40 byte <span class="code">RVALUE</span> slots in Size Pool
Zero were sufficient, because they each contained a pointer to the array data,
and not the embedded array data itself.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2025/2/11/Figure-12-12.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 12-12: A Large Array Saving Its Elements In A Separate Memory Segment
</span>
</div>

Figure 12-12 shows how large arrays, arrays which contain 79 or more elements,
do not save their elements inside of the <span class="code">Array</span>
structure, but instead save a pointer (ptr) which contains the location of a
separate memory segment that holds the array elements.

One key detail of this experiment was in Listing 12-6 at (2): the <span
class="code">all</span> array. The inner loop just below in Listing 12-6 at (4)
saved each new array into the <span class="code">all</span> array. This meant
all the new arrays were in fact still being used and Ruby’s garbage collector
could not reclaim their memory. Without this line of code, we would not have
seen the total number of slots continually increase, preventing us from
discovering which slots Ruby saved the arrays into.

But how did Ruby’s garbage collector know this, exactly? How does Ruby identify
which values are used and unused by our programs? And how does it reclaim memory
from the unused values? Let’s take a look at Ruby’s garbage collection process
next.