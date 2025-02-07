title: "Write Barriers"
date: 2025/02/18
tag: Updating Ruby Under a Microscope

I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.

Ruby’s garbage collection implementation is complex and confusing - yet powerful
and beautiful at the same time. Chapter 13 summarizes and explains advanced
aspects of Ruby’s garbage collector. I love learning about the difficult bits at
a deep enough level to be able to explain them to someone else. In RUM I often
show snippets from Ruby's C source code in gray callouts. These sections are for
readers who already understand C syntax, or who would like to learn it.  These
can give you sign posts in case you ever decide to read Ruby’s source code for
yourself.

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
barriers as small boxes that surround arrays, hashes and other data structures
you might have in your program. For example, in Listing 13-1 Ruby places a write
barrier around <span class="code">arr</span>, the array that contains all of the
new objects:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="50%" src="http://localhost/assets/2025/2/18/Figure-13-9.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 13-9: A Write Barrier
</span>
</div>

Figure 13-9 repeats Figure 13-6, which showed the array <span
class="code">arr</span>, just after Ruby removed it from the mark stack, along
with all of the elements of <span class="code">arr</span>, still on the stack.
However, on the left the dotted rectangle represents a writer barrier around
this array. (In reality, there’s nothing special about this array; Ruby uses
write barriers for all arrays, hashes and other data structures.)

The write barrier, as the name implies, jumps into action whenever your program
writes to the array inside. In this example, the barrier will be triggered when
the program runs the line of code at (3) in Listing 13-1 inside the loop:

<pre type="ruby">
(3) arr.push(Object.new)
</pre>

If your program was running between incremental GC steps when Ruby added a new
element to the array, Ruby would move the array back on to the mark stack:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="50%" src="http://localhost/assets/2025/2/18/Figure-13-10.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 13-10: Triggering a Write Barrier Pushes the Array Back on the Mark Stack
</span>
</div>

Figure 13-10 shows the array <span class="code">arr</span> on the left, inside
of a write barrier. The line of code just above, <span
class="code">arr.push</span>, triggers the write barrier because it writes to
the array’s contents. This triggers the write barrier, which moves the array
<span class="code">arr</span> back on to the mark stack, shown on the right. Now
during the next GC step, Ruby will process the array’s children again, even if
it had processed them earlier. This allows Ruby to find and mark the new object
just added to the array.

The mark stack is how Ruby remembers its place inside of the GC algorithm,
between one incremental GC and the next. Whenever an incremental GC step starts,
Ruby continues to mark the objects present on the mark stack that were pending
from last time *or that were modified in the meantime*.

<div class="c-code">

<h2>A Write Barrier in Action</h2>

Whenever your program modifies an array, hash or other value protected by write
barriers, Ruby calls this C code internally. `rb_gc_impl_writebarrier_remember` at
(1) in Listing 13-2 pushes a modified object back on to the mark stack, as shown
above in Figure 13-10:

<pre>
(1) void rb_gc_impl_writebarrier_remember(void *objspace_ptr, VALUE obj)
    {
        rb_objspace_t *objspace = objspace_ptr;
    
        gc_report(1, objspace, "rb_gc_writebarrier_remember: %s\n", rb_obj_info(obj));
    
(2)     if (is_incremental_marking(objspace)) {
(3)         if (RVALUE_BLACK_P(objspace, obj)) {
(4)             gc_grey(objspace, obj);
            }
        }
        else {
(5)         if (RVALUE_OLD_P(objspace, obj)) {
                rgengc_remember(objspace, obj);
            }
        }
    }
</pre>
<div style="font-style: italic; margin: 0 0 20px 0">
  Listing 13-2: The rb_gc_writebarrier_remember function
</div>

Ruby first checks at (2) whether it is currently in the midst of an incremental
garbage collection. If it is, Ruby continues to the line at (3), and checks
whether the given object was already completely processed: whether Ruby marked
it and all of its children. (Recall the non-inclusive term “black” from the
tricolor GC algorithm.) For example, in Figure 13-9 the `arr` value was
completed processed, since Ruby marked it and also moved all of its children on
to the mark stack, removing `arr` from the mark stack. 

If Ruby did already mark the value, and if Ruby already pushed the value’s
children on to the mark stack, then Ruby knows it needs to process the value
again - since your program modified it. In this case, Ruby continues on to the
line at (4) and calls `gc_grey`, which pushes the value back on to the mark
stack.  Later Ruby will iterate over its children again, pushing each of them
back on to the mark stack.

Looking ahead to the next section, Generational GC on page 22, write barriers
use the code in the else clause at (5) to handle “old” values. We’ll cover
Generational GC next. But first, Experiment 13-1 will take a look at incremental
GC in action.

</div>
