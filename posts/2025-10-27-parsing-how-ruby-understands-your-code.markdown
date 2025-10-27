title: "Parsing: How Ruby Understands Your Code"
date: 2025/10/27
tag: Updating Ruby Under a Microscope

I've started working on a new edition of <a
href="http://patshaughnessy.net/ruby-under-a-microscope">Ruby Under a
Microscope</a> that covers Ruby 3.x. I'm working on this in my spare time, so it
will take a while. Leave a comment or <a
href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update">drop
me a line</a> and I'll email you when it's finished.

__Update__: I’ve made a lot of progress so far this year. I had time to
completely rewrite Chapters 1 and 2, which cover Ruby’s new Prism parser and the Ruby
compiler which now handles the Prism AST. I also updated Chapter 3 about YARV
and right now I’m working on rewriting Chapter 4 which will cover YJIT and
possibly other Ruby JIT compilers.

Here’s an excerpt from the new version of Chapter 1. Many thanks to Kevin
Newton, who reviewed the content about Prism and had a number of corrections and
great suggestions. Also thanks to Douglas Eichelberger who had some great
feedback as well.

I’ll post more excerpts from Chapters 2, 3 and 4 in the coming weeks. Thanks for
everyone’s interest in Ruby Under a Microscope! 

## Chapter 1: Tokenization And Parsing

<div style="font-size: small">
<table id="toc">
	<tr>
		<td>Tokens: The Words That Make Up the Ruby Language</td><td>3</td>
	</tr>
	<tr>
		<td>Which Words Are Reserved Words?</td><td>8</td>
	</tr>
	<tr>
		<td>Experiment 1-1: Using Prism to Tokenize Different Ruby Scripts</td><td>10</td>
	</tr>
	<tr>
		<td>Parsing: How Ruby Understands Your Code</td><td>13</td>
	</tr>
	<tr>
		<td>Identifying Tokens</td><td>14</td>
	</tr>
	<tr>
		<td>Parsing Subexpressions Recursively</td><td>16</td>
	</tr>
	<tr>
		<td>Comparing Tokens</td><td>17</td>
	</tr>
	<tr>
		<td>Operator Precedence</td><td>19</td>
	</tr>
	<tr>
		<td>Left and Right Associative Operators</td><td>24</td>
	</tr>
	<tr>
		<td>Binding Powers</td><td>28</td>
	</tr>
	<tr>
		<td>Experiment 1-2: Using Prism to Parse Different Ruby Scripts</td><td>30</td>
	</tr>
	<tr>
		<td>Summary</td><td>33</td>
	</tr>
</table>
</div>

## Parsing: How Ruby Understands Your Code

Once Ruby converts your code into a series of tokens, what does it do next? How
does it actually understand and run your program? Does Ruby simply step through
the tokens and execute each one in order?

No. Your code still has a long way to go before Ruby can run it. The next step
on its journey through Ruby is called _parsing_, where words or tokens are grouped
into sentences or phrases that make sense to Ruby. When parsing, Ruby takes into
account the order of operations, methods, blocks, and other larger code
structures.

Ruby’s parsing engine defines Ruby’s syntax rules. Reading in tokens, Ruby
matches the token types and the order the tokens appear with a large series of
patterns. These patterns, indeed, are the heart and soul of the Ruby language.
How we write a function call, how we define a method using the <span
class="code">def</span> keyword, how we write classes and modules - the patterns
Ruby looks for define the language.

Ruby’s parse algorithm has three high level steps:

* __Identify__: First, Ruby identifies what the next token represents. Ruby does
this by comparing the token’s type - and possibly the types of the following
tokens - with a large series of patterns. If one pattern matches, Ruby
understands what your code means. If not, Ruby emits a syntax error.
* __Recurse__: Secondly, Ruby calls itself. Each value in one of the syntax
patterns can itself be a subexpression - a smaller program that Ruby needs to
parse. To do this, Ruby calls itself recursively.
* __Compare__: Third, Ruby compares the current token with the next token to
determine which has a higher precedence. This comparison leads Ruby down a
specific path, processing the tokens in a certain order.

Let’s break down these ideas further, by following Ruby through the “Hello
World” program. Afterwards, we’ll look at a second, slightly more complicated
example.

<pre type="ruby">
puts "Hello World"
</pre>
<div style="font-style: italic; font-size: small; margin: -20px 0 20px 0">
  Listing 1-11: Hello World in Ruby
</div>

As we saw in the previous section, Ruby first converts the text in this code
file into tokens. For Hello World, Ruby’s tokenizer produces these five tokens:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="75%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-14.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-14: Hello World Tokenized
</span>
</div>

To make the following diagrams simpler, let’s redraw these tokens in a more
compact format:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="40%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-15.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-15: Hello World tokens in a more compact format
</span>
</div>

Using a single gray line of text, Figure 1-15 shows the five tokens from Figure
1-14 in a more compact format. First, <span
class="code">PM_TOKEN_IDENTIFIER</span> represents the word “puts” from the
beginning of the program. Next, three tokens make up the string literal value:
<span class="code">PM_TOKEN_STRING_BEGIN</span> for the first double quote,
followed by <span class="code">PM_TOKEN_STRING_VALUE</span> for the words Hello
and World, and <span class="code">PM_TOKEN_STRING_END</span> represents the
second quote. Finally, the program ends with <span
class="code">PM_TOKEN_EOF</span> to mark the end of the source code file.

Now let’s follow Ruby as it processes the Hello World example using the three
steps: identify, recurse and compare.

## Identifying Tokens

First, _identify_. How does Ruby understand what the first token,
<span class="code">PM_TOKEN_IDENTIFIER</span>, means?

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="40%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-16.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-16: Parsing the first token
</span>
</div>

Figure 1-16 represents the state of Ruby’s parser when it starts to parse this
code. At this moment, Ruby is just getting started by inspecting the <span
class="code">puts</span> identifier. One of the patterns Ruby looks for matches
the identifier; but what does this identifier mean? Ruby knows <span class="code">puts</span> could be a
local variable, or it could be the name of a function to call. Since there are
no local variables defined in this program, Ruby determines that the <span class="code">puts</span>
identifier represents a function the program is calling. (It’s also possible
that the program is about to create a new local variable like this: <span class="code">puts =
"Hello World"</span>. If that were the case, Ruby would see the assignment operator
next and parse things differently.)

What happens next? After matching the token to the function call pattern, Ruby
records this match in a data structure called an abstract syntax tree (AST).
Ruby and most other programming languages use ASTs to record the results of
parsing tokens like this. As we’ll see, the AST’s tree structure is well suited
for holding the nested, recursive structure of computer programs.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="25%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-17.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-17: The first AST node
</span>
</div>

Figure 1-17 shows the first node Ruby saves in the AST tree. In a moment, Ruby
will begin to add more nodes to the AST.

Before proceeding to the next token, let’s imagine the syntax pattern for a
function call:

<pre type="ruby">
function-name ( argument1, argument2, argument3, etc. )
</pre>

Although in Ruby the parentheses are optional, so this pattern also applies:

<pre type="ruby">
function-name argument1, argument2, argument3, etc.
</pre>

__NOTE__<br/>
<i>
The original version of the Ruby parser used patterns or grammar rules like this
directly with a tool called a parser generator. However, starting with Ruby 3.3,
Ruby uses a new parser called Prism, which detects these patterns directly using
hand written C code.
</i>

After parsing the first token, Ruby inspects the second token. According to the
function call pattern, Ruby knows the second token might represent the first
argument to the function call. But, how many arguments are there? And what is
each argument? The program in Listing 1-11 is very simple, but it could have
instead printed a very complex expression - the arguments to <span
class="code">puts</span> could have run on for many lines and used hundreds of
tokens.

## Parsing Subexpressions

Second, _recurse_. To parse each of the arguments to puts, Ruby has to call itself.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="40%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-18.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-18: Parsing the second token
</span>
</div>

Figure 1-18 shows two levels of the Ruby parser’s call stack; the top line shows
Ruby parsing the puts identifier token, and matching the function call pattern.
The second line shows how Ruby called itself to parse the second token,
<span class="code">PM_TOKEN_STRING_BEGIN</span>, the leading quote of the string
literal. Think of these lines as the backtrace of the Ruby parser.

Figure 1-18 also shows a value 14 on the right side. While calling itself
recursively, Ruby passes in a numeric value called the _binding power_. We’ll
return to this later.

Now that Ruby has called itself, Ruby starts the 3-step process all over again:
identify, recurse and compare. This time, Ruby has to identify what the <span
class="code">PM_TOKEN_STRING_BEGIN</span> token means. This token always
indicates the start of a string value. In this example <span
class="code">PM_TOKEN_STRING_BEGIN</span> represents the double quote that
appears after <span class="code">puts</span>. But the same token might represent
a single quote or one of the other ways you can write a string in Ruby, for
example using <span class="code">%Q</span> or <span class="code">%q</span>.

Ruby’s new parser, Prism, next parses the string contents directly by processing
the following two tokens:

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="40%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-19.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-19: Parsing the third and fourth tokens
</span>
</div>

In this example, Ruby’s parser is done after finding the <span
class="code">PM_TOKEN_STRING_END</span> token and can continue to the next step.
More complex strings - strings that contain interpolated values using <span
class="code">#{}</span> for example - might have required Ruby to call itself
yet again to process more nested expressions. But for the simple <span
class="code">"Hello World"</span> string Ruby is done.

To record the string value, Ruby creates a new AST node called <span
class="code">PM_STRING_NODE</span>.

<div style="padding: 8px 30px 30px 0px; text-align: center; line-height:18px">
<img width="60%" src="https://patshaughnessy.net/assets/2025/10/27/Figure-1-20.svg"><br/>
<span style="font-style: italic; font-size: small">
  Figure 1-20: Two AST nodes
</span>
</div>

Figure 1-20 shows two AST nodes Ruby has created so far: the call node created
earlier, and now a new string node.

Ruby’s parser is a _recursive descent parser_. This Computer Science term
describes parsers that resemble the grammar or syntax rules of the programs they
parse, and call themselves recursively in a top-down manner as they process
nested structures. Many modern programming languages today use this general
approach.