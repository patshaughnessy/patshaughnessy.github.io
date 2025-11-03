title: "Compiling a Call to a Block"
date: 2025/11/03
tag: Updating Ruby Under a Microscope

I've started working on a new edition of <a
href="https://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.

This week's excerpt is from Chapter 2, about Ruby's compiler. Whenever I think
about it, I'm always suprised that Ruby has a compiler like C, Java or any other
programming language. The only difference is that we don't normally interact
with Ruby's compiler directly.

The developers who contributed Ruby's new parser, Prism, also had to rewrite
the Ruby compiler because Prism now produces a completely different, redesigned
abstract syntax tree (AST). Chapter 2's outline is more or less the same as it
was in 2014, but I redrew all of the diagrams and updated much of the text to
match the new AST nodes and other changes for Prism.

## Chapter 2: Compilation

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>The Ruby Compiler</td><td>4</td>
	</tr>
	<tr>
		<td>Ruby 3.2 Introduces a Just-In-Time (JIT) Compiler </td><td>5</td>
	</tr>
	<tr>
		<td>How Ruby Compiles a Simple Script</td><td>6</td>
	</tr>
	<tr>
		<td>Scope AST Nodes</td><td>7</td>
	</tr>
	<tr>
		<td>Compiling a Simple AST</td><td>8</td>
	</tr>
	<tr>
		<td>Compiling a Call to a Block</td><td>12</td>
	</tr>
	<tr>
		<td>How Ruby Iterates Through the AST</td><td>16</td>
	</tr>
	<tr>
		<td>Experiment 2-1: Displaying YARV Instructions</td><td>19</td>
	</tr>
	<tr>
		<td>The Local Table</td><td>21</td>
	</tr>
	<tr>
		<td>Compiling Optional Arguments</td><td>23</td>
	</tr>
	<tr>
		<td>Compiling Keyword Arguments</td><td>24</td>
	</tr>
	<tr>
		<td>Unnamed Local Variables</td><td>25</td>
	</tr>
	<tr>
		<td>Experiment 2-2: Displaying the Local Table</td><td>28</td>
	</tr>
	<tr>
		<td>Summary</td><td>30</td>
	</tr>
</table>
</div>

## Compiling a Call to a Block

Next, let’s compile my <span class="code">10.times</span> do example from
Listing 1-1 in Chapter 1 (see Listing 2-2).

<pre type="ruby">
10.times do |n|
  puts n
end
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
 Listing 2-2: A simple script that calls a block (repeated from Listing 1-1)
</div>

Notice that this example contains a block parameter to the <span
class="code">times</span> method. This is interesting because it will give us a
chance to see how the Ruby compiler handles blocks. Figure 2-13 shows the AST
for the <span class="code">10.times do</span> example again.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-13.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-13: A simplified view of the AST for the call to 10.times, passing a block
</span>
</div>

The left side of Figure 2-13 shows the AST for the <span
class="code">10.times</span> function call: the call node and the receiver 10,
represented by integer node. On the right, Figure 2-13 shows the beginning of
the AST for the block: <span class="code">do |n| puts n end</span>, represented
by the block node. You can see Ruby has added a scope node on both sides, since
there are two lexical scopes in Listing 2-2: the top level and the block.  Let’s
break down how Ruby compiles the main portion of the script shown on the left of
Figure 2-13. As before, Ruby starts with the first <span
class="code">PM_NODE_SCOPE</span> and creates a new snippet of YARV instructions,
as shown in Figure 2-14.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="75%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-14.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-14: Each PM_SCOPE_NODE is compiled into a new snippet of YARV instructions.
</span>
</div>

Next, Ruby steps down the AST nodes to <span class="code">PM_CALL_NODE,</span>
as shown in Figure 2-15.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="75%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-15.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-15: Ruby stepping through an AST
</span>
</div>

At this point, there is still no code generated, but notice in Figure 2-13 that
two arrows lead from <span class="code">PM_CALL_NODE</span>: one to <span
class="code">PM_INTEGER_NODE</span>, which represents the 10 in the <span
class="code">10.times</span> call, and another to the inner block. Ruby will
first continue down the AST to the integer node and compile the <span
class="code">10.times</span> method call.  The resulting YARV code, following
the same receiver-arguments-message pattern we saw in Figures 2-7 through 2-11,
is shown in Figure 2-16.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="75%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-16.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-16: Ruby compiles the 10.times method call.
</span>
</div>

Notice that the new YARV instructions shown in Figure 2-16 push the receiver
(the integer object 10) onto the stack first, after which Ruby generates an
instruction to execute the <span class="code">times</span> method call. But
notice, too, the <span class="code">block in &lt;main&gt;</span> argument in the
<span class="code">send</span> instruction. This indicates that the method call
also contains a block argument: <span class="code">do |n| puts n end</span>. In
this example, the arrow from <span class="code">PM_CALL_NODE</span> to the
second <span class="code">PM_SCOPE_NODE</span> has caused the Ruby compiler to
include this block argument.  Ruby continues by compiling the inner block,
beginning with the second <span class="code">PM_CALL_NODE</span> shown at right
in Figure 2-13.  Figure 2-17 shows what the AST for that inner block looks like.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="100%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-17.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-17: The branch of the AST for the contents of the block
</span>
</div>

Notice Ruby inserted a scope node at the top of this branch of the AST also.
Figure 2-17 shows the scope node contains two values: <span
class="code">argc=1</span> and <span class="code">locals: [n]</span>.  These
values were empty in the parent scope node, but Ruby set them here to indicate
the presence of the block parameter <span class="code">n</span>.  From a
relatively high level, Figure 2-18 shows how Ruby compiles the inner block.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="75%" src="https://patshaughnessy.net/assets/2025/11/3/Figure-2-18.svg"><br/>
<span style="font-style: italic; font-size: small">
Figure 2-18: How Ruby compiles a call to a block
</span>
</div>

You can see the parent <span class="code">PM_NODE_SCOPE</span> at the top, along
with the YARV code from Figure 2-16. And below that Figure 2-18 shows the the
inner scope node for the block, along with the YARV instructions for the block’s
call to <span class="code">puts n</span>. Later in this chapter we’ll learn how
Ruby handles parameters and local variables, like <span class="code">n</span> in
this example; why Ruby generates these instructions for <span class="code">puts
n</span>.  The key point for now is that Ruby compiles each distinct scope in
your Ruby program—methods, blocks, classes, or modules, for example—into a
separate snippet of YARV instructions.