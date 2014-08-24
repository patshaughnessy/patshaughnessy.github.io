title: "The Joke Is On Us: How Ruby 1.9 Supports the Goto Statement"
tag: Ruby internals
date: 2012/2/29

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2012/2/29/fortran.png"></td></tr>
  <tr><td align="center"><small><i>Part of a Fortran IV program (source: <a href="http://en.wikibooks.org/wiki/Fortran/Fortran_examples">WikiBooks</a>)</i></small></td></tr>
</table>
</div>

The <span class="code">goto</span> statement is one of the most infamous and troublesome features of old, archaic languages like Fortran. It allowed programmers to quickly create spaghetti code that was confusing and impossible to understand. Thankfully when structured programming came into use in the late 1960s and 1970s with Algol, Pascal and later C, <span class="code">goto</span> was no longer necessary.

This is all ancient history: why am I bringing it up? Well, I just discovered this week that Ruby, one of the most beautiful and expressive languages in use today, includes support for the <span class="code">goto</span> statement!DELIM

<div style="clear: left"/></div>

Here’s how it works:

<pre type="ruby">
__label__(:loop)

puts "The Ruby core team is playing a joke on us!"

__goto__(:loop)

puts "This line of code is never reached."
</pre>

<div style="float: right; padding: 15px 0px 10px 30px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2012/2/29/legacy-data.jpg"></td></tr>
</table>
</div>

The <span class="code">\_\_goto\_\_</span> statement will cause the MRI Ruby 1.9 interpreter to jump up to the <span class="code">\_\_label\_\_</span> statement on the first line, since they both refer to the same symbol <span class="code">:loop</span>. The last line of code will never be executed at all. Brilliant, isn’t it? We’ve managed to degrade and reduce Ruby to the level of Fortran.

However, there’s a catch. Ruby 1.9 doesn’t support <span class="code">goto</span> by default. You need to enable it by setting an optional virtual machine setting called <span class="code">SUPPORT_JOKE</span> and then recompiling Ruby from source. That’s right: the Ruby core team has included optional support for the <span class="code">goto</span> statement in Ruby 1.9 as a joke! But it’s no joke: the <span class="code">\_\_goto\_\_</span> and <span class="code">\_\_label\_\_</span> methods actually do work if you enable <span class="code">SUPPORT_JOKE</span>. Today I’ll show you exactly how to do this, and also explain how the Ruby 1.9 byte code compiler and virtual machine (called [YARV](http://www.atdot.net/yarv/)) implement <span class="code">goto</span>.

In case you get bored by all of the technical details, be sure to skip down to the bottom of the article, where I reveal a couple of other jokes included in Ruby 1.9 by the Ruby core team!

## Enabling goto support in your Ruby 1.9 installation

First, let’s try running the code above directly using a normal Ruby 1.9.3 MRI interpreter. I’ll paste the code above into a file called “joke.rb” and then run it like this:

<pre type="console">
$ rvm 1.9.3
$ ruby joke.rb 
joke.rb:1:in `<main>': undefined method `__label__' for main:Object (NoMethodError)
</pre>

No surprise: since I haven’t yet enabled <span class="code">goto</span> statement support, I get an error from Ruby stating that the <span class="code">\_\_label\_\_</span> method is not defined on line 1. Until you enable the <span class="code">SUPPORT_JOKE</span> setting, the <span class="code">\_\_goto\_\_</span> and <span class="code">\_\_label\_\_</span> methods aren’t defined.

Here’s how to enable the <span class="code">goto</span> statement support. First, find the source code for your Ruby 1.9 installation, and open the “vm_opts.h” header file in an editor. Here’s how I would do that on my Mac laptop using RVM:

<pre type="console">
$ cd ~/.rvm/src/ruby-1.9.3-preview1
$ mvim vm_opts.h
</pre>

Depending on exactly which version of Ruby 1.9 you have installed and how you obtained it, you might have to look in a different path for the Ruby C source code. Anyway, once you find the vm_opts.h file, you’ll see a series of settings for the Ruby 1.9 virtual machine:

<pre type="c">
/**********************************************************************

  vm_opts.h - VM optimize option

  $Author: akr $

  Copyright (C) 2004-2007 Koichi Sasada

**********************************************************************/


#ifndef RUBY_VM_OPTS_H
#define RUBY_VM_OPTS_H

/* Compile options.
 * You can change these options at runtime by VM::CompileOption.
 * Following definitions are default values.
 */

#define OPT_TRACE_INSTRUCTION        1
#define OPT_TAILCALL_OPTIMIZATION    0
#define OPT_PEEPHOLE_OPTIMIZATION    1
#define OPT_SPECIALISED_INSTRUCTION  1

... etc ...
</pre>

This is actually interesting; from the comments it looks like using the <span class="code">VM::CompileOption</span> Ruby object you can enable or disable different YARV optimizations at runtime. Someday I might take a look into what these mean and write up a blog post on YARV internals. But for today, let’s continue to scroll down and take a look at the bottom of the vm_opts.h file:

<pre type="c">
/* Build Options.
 * You can't change these options at runtime.
 */

/* C compiler depend */
#define OPT_DIRECT_THREADED_CODE     1
#define OPT_TOKEN_THREADED_CODE      0
#define OPT_CALL_THREADED_CODE       0

... etc ...

/* misc */
#define SUPPORT_JOKE                 0

#endif /* RUBY_VM_OPTS_H */
</pre>

And there it is: at the very bottom of the file we see a value called <span class="code">SUPPORT_JOKE</span> which is set to 0. Strangely, there’s no documentation or explanation as to what the joke might be, or what might happen if you changed the value to 1. As I explain below in the rest of this article, what happens is that changing this value to 1 will enable Ruby’s <span class="code">goto</span> statement support... along with two other jokes!

Let’s try it out - I’ll change the value to 1 like this:

<pre type="c">
 /* misc */
#define SUPPORT_JOKE                 1
</pre>

... and save the modified vm_opts.h file. Now I need to recompile Ruby, using the “make” command like this:

<pre type="console">
$ make
	CC = /usr/bin/gcc-4.2
	LD = ld
	LDSHARED = /usr/bin/gcc-4.2 -dynamiclib

... etc ...

compiling miniprelude.c
compiling class.c
compiling enum.c
compiling error.c
compiling eval.c

... etc ...

installing default zlib libraries
linking ruby
</pre>

You can see most of the C source code files depend on vm_opts.h and will be recompiled with the new setting. Now I just need to install the new Ruby executable file like this:

<pre type="console">
$ make install
./miniruby -I./lib -I. -I.ext/common  ./tool/rbinstall.rb --make="make" ...
installing binary commands:   /Users/pat/.rvm/rubies/ruby-1.9.3-preview1/bin
installing base libraries:    /Users/pat/.rvm/rubies

... etc ...
</pre>

And I’m done - my special, customized Ruby 1.9 executable is now ready. Let’s give it a try and see what happens:

<pre type="console">
$ ruby joke.rb
The Ruby core team is playing a joke on us!
The Ruby core team is playing a joke on us!
The Ruby core team is playing a joke on us!
The Ruby core team is playing a joke on us!
The Ruby core team is playing a joke on us!
The Ruby core team is playing a joke on us!

...etc...
</pre>

## Understanding how Ruby supports goto

While this might be worth a few laughs, I view it as a learning opportunity as well. The rest of this article will show you what happens inside Ruby when you set <span class="code">SUPPORT_JOKE=1</span>, and how the <span class="code">goto</span> statement actually works. Before we get to the C language details of the Ruby byte code compiler and the <span class="code">SUPPORT_JOKE</span> setting, let’s start with a brief overview of the Ruby 1.9 parse and compile process. Here’s a high level picture of what happens when you run a Ruby 1.9 program:

![high level view](http://patshaughnessy.net/assets/2012/2/29/high-level.png)

Going from left to right, whenever you run a Ruby program it:
<ul>
  <li>First tokenizes your Ruby source code into a series of tokens</li>
  <li>Parses the stream of tokens using an open source parsing engine called <a href="http://www.gnu.org/software/bison/">Bison</a>, creating a tree structure called an <a href="http://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Tree</a>, or AST.</li>
  <li>Compiles that AST into byte code</li>
  <li>Finally, the Ruby virtual machine (YARV) interprets these byte code instructions and executes your program.</li>
</ul>

## Parsing

Now let’s take a look at how the “joke.rb” program above is parsed by Ruby. The best way to get a sense of how Ruby does this is by using the [Ripper](http://www.ruby-doc.org/stdlib-1.9.3/libdoc/ripper/rdoc/Ripper.html) library, like this:

<pre type="ruby">
require 'ripper'
require 'pp'

joke_code = <<SRC
__label__(:loop)
puts "The Ruby core team is playing a joke on us!"
__goto__(:loop)
puts "This line of code is never reached!"
SRC

pp Ripper.sexp(joke_code)
</pre>

I first learned about Ripper watching Peter Cooper’s excellent [RubyTrickShots](http://rubyreloaded.com/trickshots/) screen cast. Also, I've skipped over the tokenize/lex step here today, but adding a call to <span class="code">Ripper.lex</span> would display the output of the Ruby tokenizer. Running this, <span class="code">Ripper.sexp</span> will display the AST produced by the Bison parser inside of Ruby:

<pre type="console">
[:program,
 [[:method_add_arg,
   [:fcall, [:@ident, "__label__", [1, 0]]],
   [:arg_paren,
    [:args_add_block,
     [[:symbol_literal, [:symbol, [:@ident, "loop", [1, 11]]]]],
     false]]],
  [:command,
   [:@ident, "puts", [2, 0]],
   [:args_add_block,
    [[:string_literal,
      [:string_content,
       [:@tstring_content,
        "The Ruby core team is playing a joke on us!",
        [2, 6]]]]],

... etc ...
</pre>

To make this easier to understand, here’s a graphical representation of the syntax tree for part of the joke.rb script:

![Abstract Syntax Tree](http://patshaughnessy.net/assets/2012/2/29/ast.png)

This is just part of the syntax tree for my joke.rb file. Notice that in the node on the left, representing the call to <span class="code">\_\_label\_\_(:loop)</span>, I’ve included the constant <span class="code">NODE_FCALL</span> used internally by the Ruby C source code to identify function calls in the syntax tree. We’ll see this used in a moment.
## Compiling

After tokenizing and parsing your script, the next thing Ruby does is to compile the AST node tree into byte code that the YARV engine can later execute. As [Rohit Arondekar](http://www.rohitarondekar.com/) pointed out in a comment on [my last post](http://patshaughnessy.net/2012/2/15/is-ruby-interpreted-or-compiled) the easiest way to see the output of the Ruby 1.9 compiler is to use the <span class="code">RubyVM</span> object like this:

<pre type="ruby">
code = <<SRC
__label__(:loop)
puts "The Ruby core team is playing a joke on us!"
__goto__(:loop)
puts "This line of code is never reached!"
SRC

bytecode = RubyVM::InstructionSequence.compile(code)
puts bytecode.disasm
</pre>

Running this, I get:

<pre type="console">
$ ruby byte_code.rb 
== disasm: <RubyVM::InstructionSequence:<compiled>@<compiled>>==========
0000 trace            1                                               (   1)
0002 trace            1                                               (   2)
0004 putnil           
0005 putstring        "The Ruby core team is playing a joke on us!"
0007 send             :puts, 1, nil, 8, <ic:0>
0013 pop              
0014 trace            1                                               (   3)
0016 jump             2
0018 trace            1                                               (   4)
0020 putnil           
0021 putstring        "This line of code is never reached!"
0023 send             :puts, 1, nil, 8, <ic:1>
0029 leave            
</pre>

This is actually easy to understand, since the byte code instructions are quite high level and intuitive. You can see how the <span class="code">\_\_goto\_\_</span> and <span class="code">\_\_label\_\_</span> statements work: on line 16 there’s a <span class="code">jump</span> instruction which sends the YARV engine back up to line 2 in an endless loop.

## Finding where SUPPORT_JOKE is used in the Ruby C source code

Now that we have a high level understanding of how my simple joke.rb Ruby script works, let’s return to the <span class="code">SUPPORT_JOKE</span> setting and take a look at exactly what happens when you enable it. <span class="code">SUPPORT_JOKE</span> appears in a few places in the Ruby 1.9 source code, but primarily it’s used in the compile.c file, which implements the Ruby AST to byte code compiler. If you open compile.c and search for <span class="code">SUPPORT_JOKE</span>, you’ll find this C code:

<pre type="c">
#if SUPPORT_JOKE

... etc ...

/* only joke */
{
   ID goto_id;
   ID label_id;

   CONST_ID(goto_id, "__goto__");
   CONST_ID(label_id, "__label__");

   if (nd_type(node) == NODE_FCALL &&
       (mid == goto_id || mid == label_id)) {

... etc ...

     if (mid == goto_id) {
       ADD_INSNL(ret, nd_line(node), jump, label);
     }
     else {
       ADD_LABEL(ret, label);
     }

... etc ...
</pre>

For the sake of simplicity, I’ve omitted some of the C code you would see in compile.c. But I have shown enough code to get the basic idea across. Let’s take it one step at a time....

First, you should notice the two calls to <span class="code">CONST_ID</span> - these define two identifiers called <span class="code">\_\_goto\_\_</span>and <span class="code">\_\_label\_\_</span>. This is actually a very unusual, odd way of defining Ruby keywords! In the Ruby parser and compiler, most reserved words and other hard coded constants of the language are defined in a variety of standard ways. But what we see here is that, only in the case when <span class="code">SUPPORT_JOKE=1</span>, the Ruby compiler explicitly defines these two special identifiers inline right in the ruby byte code compiler C code. Very strange!

Next, if you read the following <span class="code">if</span> statement, you see that Ruby is checking whether the current <span class="code">node</span> object is of type <span class="code">NODE_FCALL</span>. Here <span class="code">node</span> refers to one of the nodes in the AST tree we saw above. <span class="code">NODE_FCALL</span> corresponds to the <span class="code">:fcall</span> symbol we saw in the Ripper output. What’s happening in this part of the compile.c file is that Ruby is stepping around the AST tree above, and converting each tree element into byte code. Translating this <span class="code">if</span> statement into English we get: “If the current node is a function call and the function being called is either <span class="code">\_\_goto\_\_</span>or <span class="code">\_\_label\_\_</span>....”

Finally, if the statement is true and after executing some other C code I’ve omitted here, Ruby will call either <span class="code">ADD_INSNL</span> or <span class="code">ADD_LABEL</span> to output new byte code. In this case we can see <span class="code">ADD_INSNL</span> adds a <span class="code">jump</span> byte code command to the specified label:

<pre type="c">
ADD_INSNL(ret, nd_line(node), jump, label);
</pre>

All of this code is only included in Ruby if <span class="code">SUPPORT_JOKE</span> is true; if you haven’t enabled <span class="code">SUPPORT_JOKE</span> then none of this C code will even be compiled, because of the <span class="code">#if SUPPORT_JOKE</span> statement at the top.

## Other jokes

It turns out there are two other jokes the Ruby core team has included in the compiler:

<pre type="c">
// (from id.c)
#if SUPPORT_JOKE
    REGISTER_SYMID(idBitblt, "bitblt");
    REGISTER_SYMID(idAnswer, "the_answer_to_life_the_universe_and_everything");
#endif

// (from compile.c near the snippet I showed above)
#if SUPPORT_JOKE
if (nd_type(node) == NODE_VCALL) {
  if (mid == idBitblt) {
    ADD_INSN(ret, nd_line(node), bitblt);
    break;
  }
  else if (mid == idAnswer) {
    ADD_INSN(ret, nd_line(node), answer);
    break;
  }
}

... etc ...
</pre>

Once you’ve enabled <span class="code">SUPPORT_JOKE</span> and recompiled, you’ll be able to run these two other special, hard coded methods:

<pre type="console">
$ irb
ruby-1.9.3-preview1 :001 > bitblt
 => "a bit of bacon, lettuce and tomato" 
ruby-1.9.3-preview1 :002 > the_answer_to_life_the_universe_and_everything
 => 42 
</pre>

