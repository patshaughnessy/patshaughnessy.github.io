title: "Write Barriers"
date: 2025/02/18
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
</i>

<p>
Ruby’s garbage collection implementation is complex and confusing - yet powerful
and beautiful at the same time. Chapter 13 summarizes and explains some of these
more advanced aspects of Ruby’s garbage collector. Writing it, I don’t shy away
from advanced topics. I both learning about the difficult bits myself, and
then understanding them at a deep enough level to be able to explain them to all
of you.

In RUM I show C code snippets in gray callouts. These sections are for readers
who do understand C syntax, or who would like to learn it. These can give you
sign posts in case you ever decide to read Ruby’s source code for yourself.
</p>


## Chapter 13: Incremental and Generational Garbage Collection

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>Incremental GC</td><td>3</td>
	</tr>
	<tr>
		<td>Marking in More Detail</td><td>4</td>
	</tr>
	<tr>
		<td>Stop The World Garbage Collection</td><td>8</td>
	</tr>
	<tr>
		<td>Incremental Garbage Collection</td><td>10</td>
	</tr>
	<tr>
		<td>How Ruby Implements Incremental GC</td><td>11</td>
	</tr>
	<tr>
		<td>Write Barriers</td><td>12</td>
	</tr>
	<tr>
		<td>A Write Barrier in Action</td><td>14</td>
	</tr>
	<tr>
		<td>Experiment 13-1: What happens if GC can’t free enough slots?</td><td>15</td>
	</tr>
	<tr>
		<td>Generational GC</td><td>22</td>
	</tr>
	<tr>
		<td>Keeping Track of an Object’s Age</td><td>23</td>
	</tr>
	<tr>
		<td>How Marking and Sweeping Work Together</td><td>24</td>
	</tr>
	<tr>
		<td>Seeing Generational GC In Action</td><td>26</td>
	</tr>
	<tr>
		<td>Write Barriers Again</td><td>29</td>
	</tr>
	<tr>
		<td>The Remember Set and Unprotected Values</td><td>30</td>
	</tr>
	<tr>
		<td>Garbage Collection Bitmaps</td><td>31</td>
	</tr>
	<tr>
		<td>Major and Minor GCs</td><td>34</td>
	</tr>
	<tr>
		<td>Experiment 13-2: Major and Minor GCs</td><td>35</td>
	</tr>
	<tr>
		<td>Summary</td><td>40</td>
	</tr>
</table>
</div>

## Write Barriers

Write barriers warn Ruby’s garbage collection algorithm whenever your program
creates a new object that might have to be marked. You can think of the
“barriers” as small boxes that surround arrays, hashes and other data structures
you might have in your program. For example, in Listing 13-1 Ruby places a write
barrier around arr, the array that contains all of the new objects:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="http://localhost/assets/2025/2/11/Figure-13-9.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 13-9: A Write Barrier
</span>
</div>

Figure 13-9 repeats Figure 13-6, which showed the array arr, just after Ruby
removed it from the mark stack, along with all of the elements of arr, still on
the stack. However, on the left the dotted rectangle represents a writer barrier
around this array. (In reality, there’s nothing special about this array; Ruby
uses write barriers for all arrays, hashes and other data structures.)

The write barrier, as the name implies, jumps into action whenever your program
writes to the array inside. In this example, the barrier will be triggered when
the program runs the line of code at (3) in Listing 13-1 inside the loop:

<pre type="ruby">
(3) arr.push(Object.new)
</pre>

If your program was running between incremental GC steps when Ruby added a new
element to the array, Ruby would move the array back on to the mark stack:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img src="http://localhost/assets/2025/2/11/Figure-13-10.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 13-10: Triggering a Write Barrier Pushes the Array Back on the Mark Stack
</span>
</div>

Figure 13-10 shows the array arr on the left, inside of a write barrier. The
line of code just above, <span class="code">arr.push</span> triggers the write
barrier because it writes to or modifies the array’s contents. This triggers the
write barrier, which moves the array arr back on to the mark stack, shown on the
right. Now during the next GC step, Ruby will process the array’s children
again, even if it had processed them earlier. This allows Ruby to find and mark
the new object just added to the array.

The mark stack is how Ruby remembers its place inside of the GC algorithm,
between one incremental GC and the next. Whenever an incremental GC step starts,
Ruby continues to mark the objects present on the mark stack that were pending
from last time *or that were modified in the meantime*.