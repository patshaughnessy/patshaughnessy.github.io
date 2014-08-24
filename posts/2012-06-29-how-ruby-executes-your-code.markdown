title: "How Ruby Executes Your Code"
date: 2012/6/29

<b>This is an excerpt from the second chapter of an eBook I’m writing this Summer called “Ruby Under a Microscope.” My goal is to teach you how Ruby works internally without assuming you know anything about the C programming language.

If you’re interested in Ruby internals you can [sign up here](http://patshaughnessy.net/ruby-under-a-microscope) and I’ll send you an email when the eBook is finished.  I also posted [one entire chapter](http://patshaughnessy.net/2012/5/9/one-chapter-from-my-upcoming-ebook-ruby-under-a-microscope) in May, and [an excerpt from the first chapter](http://patshaughnessy.net/2012/6/18/the-start-of-a-long-journey-how-ruby-parses-and-compiles-your-code) last week.</b>

<br/>

<div style="float: left; padding: 17px 30px 10px 0px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td><img src="http://patshaughnessy.net/assets/2012/6/29/gears.jpg"></td></tr>
  </table>
</div>

Now that Ruby has tokenized, parsed and compiled your code, Ruby is finally ready to execute it. But exactly how does it do this? We’ve seen how the Ruby compiler creates YARV (“Yet Another Ruby Virtual Machine”) instructions, but how does YARV actually run them? How does it keep track of variables, return values and arguments? How does it implement <span class="code">if</span> statements and other control structures?

Just like your computer’s actual microprocessor hardware, Koichi Sasada and the Ruby core team designed YARV to use a stack pointer and a program counter. In this chapter, I’ll start by looking at the basics of YARV instructions: how they pop arguments off the stack and push return values onto the stack. I’ll continue by explaining how Ruby accesses variables in two different ways: locally and dynamically. Then I’ll show you how YARV implements Ruby control structures - including a look at how Ruby implements the <span class="code">break</span> keyword internally by throwing an exception! Finally, I’ll compare the instruction sets used by the JRuby and Rubinius virtual machines to YARV’s instruction set.

## YARV’s internal stack and your Ruby stack

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## Experiment 2-1: Benchmarking Ruby 1.9 vs. Ruby 1.8

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## Local and dynamic access of Ruby variables

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## Experiment 2-2: Exploring scope and variables using the binding object

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## How YARV controls your program’s execution flow

<div style="float: left; padding: 17px 30px 10px 0px">
  <img src="http://patshaughnessy.net/assets/2012/6/29/railroad.jpg">
</div>

We’ve seen how YARV uses a stack while executing its instruction set and how it can access variables locally or dynamically, but what about control structures? Controlling the flow of execution is a fundamental requirement for any programming language, and Ruby has a rich set of control structures. How does YARV implement them?

Just like Ruby itself, YARV has it own control structures, albeit at a much lower level. Instead of <span class="code">if</span> or <span class="code">unless</span> statements, YARV uses two low level instructions called <span class="code">branchif</span> and <span class="code">branchunless</span>. And instead of using control structures such as <span class="code">while...end</span> or <span class="code">until...end</span> loops, YARV has a single low level function called <span class="code">jump</span> that allows it to change the program counter and move from one place to another in your compiled program. By combining the <span class="code">branchif</span> or <span class="code">branchunless</span> instruction with the <span class="code">jump</span> instruction YARV is able to execute most of Ruby’s simple control structures.

A good way to understand how YARV controls execution flow is to take a look at how the if/else statement works. Here’s a simple Ruby script that uses both <span class="code">if</span> and <span class="code">else</span>:

<img src="http://patshaughnessy.net/assets/2012/6/29/if-statement.png"/>

On the right you can see the corresponding snippet of compiled YARV instructions. I’ve simplified the YARV instructions a bit by removing the <span class="code">trace</span> commands and a couple of other things. Reading the YARV instructions, you can see Ruby follows this pattern for implementing if/else statements:

<ul>
  <li>evaluate condition</li>
  <li>jump to false code if condition is false</li>
  <li>true code; jump to end</li>
  <li>false code</li>
</ul>

This is a bit easier to follow if I paste the instructions into a flowchart:

<img src="http://patshaughnessy.net/assets/2012/6/29/if-flowchart.png"/>

You can see how the <span class="code">branchunless</span> instruction is the key to how Ruby implements <span class="code">if</span> statements; here’s how it works:
<ul>
  <li>First at the top Ruby evaluates the condition of my <span class="code">if</span> statement, <span class="code">i &lt; 10,</span> using the <span class="code">opt_lt</span> (optimized less-than) instruction. This will leave either a true or false value on the stack.</li>
  <li>Then <span class="code">branchunless</span> will jump down to the false/else condition if the condition is false. That is, it “branches unless” the condition is true. Ruby uses <span class="code">branchunless</span> and not <span class="code">branchif</span> for if/else conditions since the positive case, the code that immediately follows the if statement, is compiled to appear right after the condition code. Therefore YARV needs to jump if the condition is false.</li>
  <li>Or if the condition is true Ruby will not branch and will just continue to execute the positive case code. After finishing the positive code Ruby will then jump down to the instructions following the if/else statement using the <span class="code">jump</span> instruction.</li>
  <li>Finally either way Ruby will continue to execute the subsequent code.</li>
</ul>

YARV implements the <span class="code">unless</span> statement in a similar way using the same <span class="code">branchunless</span> instruction, except the positive and negative code snippets are in reverse order. For looping control structures like <span class="code">while...end</span> and <span class="code">until...end</span> YARV uses the <span class="code">branchif</span> instruction instead. But the idea is the same: calculate the loop condition, then execute <span class="code">branchif</span> to jump as necessary, and finally use <span class="code">jump</span> statements to implement the loop.

One of the challenges YARV has implementing some control structures is that, similar to dynamic variable access, Ruby sometimes can jump from one scope to another. The simplest example of this is the <span class="code">break</span> statement. <span class="code">break</span> can be used both to exit a simple loop like this:

<p></p>

<pre type="ruby">
i = 0
while i<10
  puts i
  break
  i += 1
end
puts "continue from here"
</pre>

<p></p>

… or from a block iteration like this:

<p></p>

<pre type="ruby">
10.times do |n|
  puts n
  break
end
puts "continue from here"
</pre>

<p></p>

In the first case, YARV can exit the while loop using simple <span class="code">jump</span> instructions like we saw above in the if/else example. However, exiting a block is not so simple: in this case YARV needs to jump to the parent scope and continue execution after the call to <span class="code">10.times</span>. How does it do this? How does it know where to jump to? And how does it adjust both it’s internal stack and your Ruby call stack to be able to continue execution properly in the parent scope?

To implement jumping from one place to another in the Ruby call stack - that is, outside of the current scope - Ruby uses the <span class="code">throw</span> YARV instruction. That’s right: internally Ruby implements <span class="code">break</span> from within a block by throwing an exception!

Let’s take a look at how that works; here’s the compiled code for the block above containing the <span class="code">break</span> statement:

<img src="http://patshaughnessy.net/assets/2012/6/29/break-code.png"/>

You can see a <span class="code">throw 2</span> instruction appears in the compiled code for the block. <span class="code">Throw</span> implements throwing an exception at the YARV instruction level by using something called a “catch table.” A catch table is a table of pointers optionally attached to any YARV code snippet. Conceptually, a catch table might look like this:

<img src="http://patshaughnessy.net/assets/2012/6/29/catch-table.png"/>

Here, the catch table from my example contains just a single pointer to the <span class="code">pop</span> statement, which is where execution would continue after an exception. Whenever you use a break statement in a block, Ruby not only compiles the <span class="code">throw</span> instruction into the block’s code, but it also adds the BREAK entry into the catch table of the parent scope. For a <span class="code">break</span> within a series of nested blocks, Ruby would add the BREAK entry to a catch table even farther down the rb_control_frame stack.

Later, when YARV executes the <span class="code">throw</span> instruction it checks to see whether there’s a catch table containing a BREAK pointer for the current YARV instruction sequence:

<img src="http://patshaughnessy.net/assets/2012/6/29/catch1.png"/>

If there isn’t, Ruby will start to iterate down through the stack of rb_control_frame structures looking for a catch table containing a BREAK pointer...

<img src="http://patshaughnessy.net/assets/2012/6/29/catch2.png"/>

… and continue to iterate until it finds one:

<img src="http://patshaughnessy.net/assets/2012/6/29/catch3.png"/>

In my simple example, there is only one level of block nesting, so Ruby will find the catch table and BREAK pointer after just one iteration:

<img src="http://patshaughnessy.net/assets/2012/6/29/caught.png"/>

Once Ruby finds the catch table pointer, it resets both the Ruby call stack (the CFP pointer) and the internal YARV stack to reflect the new program execution point. Then YARV continues to execute your code from there. That is, YARV resets the internal PC and SP pointers as needed.

What is interesting to me about this is how Ruby uses exceptions internally to implement a very commonly used control structure: the <span class="code">break</span> keyword. In other words, what in more verbose languages is an exceptional occurrence becomes in Ruby a common, everyday action. Ruby has wrapped up a confusing, unusual syntax - throwing/catching of exceptions - into a simple keyword, <span class="code">break</span>, and made it very easy to understand and use. Of course, Ruby needs to use exceptions because of the way blocks work: they are on one hand like separate functions or subroutines, but on the other hand just part of the surrounding code. For this reason Ruby needs a keyword like <span class="code">break</span> that seems simple at first glance but internally is quite complex.

Another commonplace, ordinary Ruby control structure that also uses catch tables is the <span class="code">return</span> keyword. Whenever you call <span class="code">return</span> from inside a block Ruby internally throws an exception and catches it with a catch table pointer like this. In fact, <span class="code">break</span> and <span class="code">return</span> are implemented with exactly the same YARV instructions; the only difference is that for <span class="code">return</span> Ruby passes a 1 to the <span class="code">throw</span> instruction (e.g. <span class="code">throw 1</span>), while for <span class="code">break</span> it passes a 2 as we saw above. The <span class="code">return</span> and <span class="code">break</span> keywords are really two sides of the same coin.

Finally, besides BREAK there are other types of pointers that Ruby can use in the catch table. The others are used to implement different control structures: <span class="code">rescue</span>, <span class="code">ensure</span>, <span class="code">retry</span>, <span class="code">redo</span> and <span class="code">next</span>. For example, when you explicitly raise an exception in your Ruby code using the <span class="code">raise</span> keyword, Ruby implements the <span class="code">rescue</span> block in a similar way by using the catch table, but this time with a RESCUE pointer. The catch table is simply a list of event types that can be caught and handled by that sequence of YARV instructions, similar to how you would use a rescue block in your Ruby code.

## Experiment 2-3: How Ruby compiles different control structures into YARV instructions

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## High level vs. low level VM instructions - comparing how JRuby and Ruby 1.9 compile your code

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).

## Comparing Rubinius high level instructions with LLVM's low level instructions

… read it in the [finished eBook](http://patshaughnessy.net/ruby-under-a-microscope).
