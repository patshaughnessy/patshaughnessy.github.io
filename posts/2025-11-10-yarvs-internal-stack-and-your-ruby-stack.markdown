title: "YARV’s Internal Stack and Your Ruby Stack"
date: 2025/11/10
tag: Updating Ruby Under a Microscope

I've started working on a new edition of <a
href="https://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.

The content of Chapter 3, about the YARV virtual machine, hasn't changed much
since 2014.  However, I did update all of the diagrams to account for some new
values YARV now saves inside of each stack frame. And some of the common YARV
instructions were renamed as well. I also moved some content that was previously
part of Chapter 4 here into Chapter 3. Right now I'm rewriting Chapter 4 from
scratch, describing Ruby's new JIT compilers.

## Chapter 3: How Ruby Executes Your Code

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>YARV’s Internal Stack and Your Ruby Stack</td><td>4</td>
	</tr>
	<tr>
		<td>Stepping Through How Ruby Executes a Simple Script</td><td>5</td>
	</tr>
	<tr>
		<td>Executing a Call to a Block</td><td>8</td>
	</tr>
	<tr>
		<td>Taking a Close Look at a YARV Instruction</td><td>10</td>
	</tr>
	<tr>
		<td>Local and Dynamic Access of Ruby Variables</td><td>12</td>
	</tr>
	<tr>
		<td>Local Variable Access</td><td>12</td>
	</tr>
	<tr>
		<td>Method Arguments Are Treated Like Local Variables</td><td>15</td>
	</tr>
	<tr>
		<td>Dynamic Variable Access</td><td>16</td>
	</tr>
	<tr>
		<td>Climbing The Environment Pointer Ladder In C</td><td>22</td>
	</tr>
	<tr>
		<td>Experiment 3-1: Exploring Special Variables</td><td>23</td>
	</tr>
	<tr>
		<td>Controlling the Flow of Execution</td><td>27</td>
	</tr>
	<tr>
		<td>How Ruby Executes an if Statement</td><td>27</td>
	</tr>
	<tr>
		<td>Jumping from One Scope to Another</td><td>29</td>
	</tr>
	<tr>
		<td>Catch Tables</td><td>30</td>
	</tr>
	<tr>
		<td>Other Uses for Catch Tables</td><td>33</td>
	</tr>
	<tr>
		<td>Experiment 3-2: Testing How Ruby Implements
For Loops Internally</td><td>34</td>
	</tr>
	<tr>
		<td>Summary</td><td>35</td>
	</tr>
</table>
</div>

## YARV’s Internal Stack and Your Ruby Stack

As we’ll see in a moment, YARV uses a stack internally to track intermediate values, arguments, and return values. YARV is a stack-oriented virtual machine.

In addition to its own internal stack, YARV keeps track of your Ruby program’s
_call stack_, recording which methods call which other methods, functions, blocks,
lambdas, and so on. In fact, YARV is not just a stack machine—it’s a
double-stack machine! It has to track the arguments and return values not only
for its own internal instructions but also for your Ruby program.

Figure 3-1 shows YARV’s basic registers and internal stack.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-1.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-1: Some of YARV’s internal registers, including the program counter and stack pointer
</span>
</div>

YARV’s internal stack is on the left. The SP label is the stack pointer, or the
location of the top of the stack. On the right are the instructions that YARV is
executing. PC is the program counter, or the location of the current
instruction. 

You can see the YARV instructions that Ruby compiled from the <span
class="code">puts 2+2</span> example on the right side of Figure 3-1. YARV
stores both the SP and PC registers in a C structure called <span
class="code">rb_control_frame_t</span>, along with the current value of Ruby’s
<span class="code">self</span> variable and some other values not shown here. 

At the same time, YARV maintains another stack of these <span
class="code">rb_control_frame_t</span> structures, as shown in Figure 3-2.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="40%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-2.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-2: YARV keeps track of your Ruby call stack using a series of rb_control_frame_t structures.
</span>
</div>

This second stack of <span class="code">rb_control_frame_t</span> structures
represents the path that YARV has taken through your Ruby program, and YARV’s
current location. In other words, this is your Ruby call stack—what you would
see if you ran <span class="code">puts caller</span>. 

The CFP pointer indicates the current frame pointer. Each stack frame in your
Ruby program stack contains, in turn, a different value for the self, PC, and SP
registers, as shown in Figure 3-1. Ruby also keeps track of type of code running
at each level in your Ruby call stack, indicated by the “[BLOCK]”, “[METHOD]”
notation in Figure 3-2.

## Stepping Through How Ruby Executes a Simple Script

In order to help you understand this a bit better, here are a couple of
examples. I’ll begin with the simple 2+2 example from Chapters 1 and 2, shown
again in Listing 3-1.

<pre type="ruby">
puts 2+2
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
 Listing 3-1: A one-line Ruby program that we’ll execute as an example
</div>

This one-line Ruby script doesn’t have a Ruby call stack, so I’ll focus on the
internal YARV stack for now. Figure 3-3 shows how YARV will execute this script,
beginning with the first instruction, <span class="code">putself</span>.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-3.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-3: On the left is YARV’s internal stack, and on the right is the compiled version of my puts 2+2 program.
</span>
</div>

As you can see in Figure 3-3, YARV starts the program counter (PC) at the first
instruction, and initially the stack is empty. Now YARV executes the <span
class="code">putself</span> instruction, and pushes the current value of self
onto the stack, as shown in Figure 3-4.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-4.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-4: putself pushes the top self value onto the stack
</span>
</div>

Because this simple script contains no Ruby objects or classes, the self pointer
is set to the default top self object. This is an instance of the <span
class="code">Object</span> class that Ruby automatically creates when YARV
starts. It serves as the receiver for method calls and the container for
instance variables in the top-level scope.  The top self object contains a
single, predefined <span class="code">to_s</span> method, which returns the
string “main.” You can call this method by running the following command in the
console: 

<pre type="ruby">
$ ruby -e 'puts self'
=> main
</pre>

YARV will use this self value on the stack when it executes the
<span class="code">opt_send_without_block</span> instruction: self is the
receiver of the <span class="code">puts</span> method because I didn’t specify a
receiver for this method call. 

Next, YARV executes <span class="code">putobject 2</span>. It pushes the numeric
value 2 onto the stack and increments the PC again, as shown in Figure 3-5.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-5.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-5: Ruby pushes the value 2 onto the stack, the receiver of the + method.
</span>
</div>

This is the first step of the receiver (arguments) operation pattern described
in “How Ruby Compiles a Simple Script” on page 34. First, Ruby pushes the
receiver onto the internal YARV stack. In this example, the <span
class="code">Fixnum</span> object 2 is the receiver of the message/method <span
class="code">+</span>, which takes a single argument, also a 2.  Next, Ruby
pushes the argument 2, as shown in Figure 3-6.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-6.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-6: Ruby pushes another value 2 onto the stack, the argument of the + method.
</span>
</div>

Finally, Ruby executes the + operation. In this case, <span
class="code">opt_plus</span> is an optimized instruction that will add two
values: the receiver and the argument, as shown in Figure 3-7.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-7.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-7: Figure 3-7: The opt_plus instruction calculates 2 + 2 = 4.
</span>
</div>

As you can see in Figure 3-7, the <span class="code">opt_plus</span> instruction
leaves the result, 4, at the top of the stack. Now Ruby is perfectly positioned
to execute the <span class="code">puts</span> function call: The receiver <span
class="code">self</span> is first on the stack, and the single argument, 4, is
at the top of the stack.  (I’ll describe how method lookup works in Chapter 6.) 

Next, Figure 3-8 shows what happens when Ruby executes the <span
class="code">puts</span> method call. As you can see, the <span
class="code">opt_send_without_block</span> instruction leaves the return value,
<span class="code">nil</span>, at the top of the stack. Finally, Ruby executes
the last instruction, <span class="code">leave</span>, which finishes the
execution of our simple, one-line Ruby program. Of course, when Ruby executes
the <span class="code">puts</span> call, the C code implementing the <span
class="code">puts</span> function will actually display the value 4 in the
console output. 

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/10/Figure-3-8.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 3-8: Ruby calls the puts method on the top self object.
</span>
</div>

