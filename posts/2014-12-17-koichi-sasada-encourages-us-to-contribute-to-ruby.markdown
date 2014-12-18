title: "Koichi Sasada Encourages Us To Contribute To Ruby"
date: 2014/12/17

<div class="epigraph">
  <img src="http://patshaughnessy.net/assets/2014/12/17/microscope.png">
  <div class="epigraph-text">Ruby処理系開発コミュニティは、<br/>あなたの挑戦を歓迎する。</div>
  <div class="epigraph-caption">
  The Ruby core community welcomes your challenge.
  </div>
</div>

Today I’m happy to post an English translation of a new appendix to [Ruby Under a
Microscope](http://patshaughnessy.net/ruby-under-a-microscope) written by
Koichi Sasada ([@_ko1](https://twitter.com/_ko1)), included in the Japanese
edition last month. As you probably know, Koichi is the developer behind the
virtual machine used by Ruby since Ruby 1.9 was released, known as “YARV.” He
writes about the early days of YARV (apparently he wrote the first version
during a single vacation week!) as well as some interesting aspects of its
technical design.

But what struck me the most about this essay - what I found encouraging and
inspiring - was the story about how Koichi first became involved in Ruby
core development. He writes about how certain Ruby meetups and book
clubs created an environment that enabled him to learn and innovate. This
environment lead directly to the development of YARV, which enables all our
Ruby programs to run faster today.

He concludes with a message for all of us: We should follow in his footsteps
and not be afraid to contribute to Ruby, to create “yet another Ruby
implementation.” Thinking about his story, to do this maybe we need to seek out
- or to create - the right learning environment, like the one Koichi found in
Japan 10 years ago. Thanks for writing this, Koichi! I’m happy and proud that
RUM now concludes with this message.

Japanese-language edition copyright &copy; 2014 by Ohmsha, Ltd. Reproduced with
permission of the copyright owner.

<div style="clear: left"></div>

<div class="appendix">A</div>
<div class="appendix-title">
Yet More Ruby Virtual Machines
</div>

I (Koichi Sasada) am thankful to have a chance to write about YARV in this
appendix as a developer of YARV: Yet Another Ruby VM. Many pages of this book
describe the internals of YARV. Furthermore, the original edition is written in
English and has been read by people all over the world. I am greatly honored by
that as a software developer, although it humbles me to have found several
inefficiencies in YARV’s implementation while reading the book. In this appendix,
I will give some supplemental information and background on the design and
implementation of YARV.

## YARV: Yet Another RubyVM

I started the development of YARV during the New Year's holiday in 2004. I had
already been interested in a virtual machine for Ruby at that time. I built a
simple prototype within about a week. I must have had plenty of time to kill
back then. According to the first announcement ([ruby-dev:22494]), it was
capable of running a program to calculate Fibonacci numbers.

In its early stages of development, I implemented YARV as an extension library
for Ruby 1.8. Instead of replacing the whole runtime engine, I designed it to
be used by Ruby 1.8 as a VM to run specific programs. In other words, it was
another Ruby implementation on top of the Ruby 1.8 implementation. This
architecture allowed us to test YARV with relative ease using Ruby 1.8, which
was sufficiently stable. We could continue using the base mechanisms of Ruby
1.8, such as GC, C APIs and so on. After finishing a substantial part of the
development, the Ruby 1.8 core was removed and replaced with YARV all at once,
in order to support features such as threads. However, we kept using
infrastructure code, such as GC, after that. The Ruby interpreter (MRI/CRuby)
is known to have an affinity to the C programming language. YARV inherits that
characteristic as well. Then a new version of Ruby containing YARV at its core
was released as Ruby 1.9. As of this writing in 2014, YARV has been used as the
Ruby VM since then.

People often point out that YARV is not "Yet Another" anymore, because it is
the official VM now. Though we still use the name because it somehow sounds
familiar, we make it a rule not to use "YARV" in file names or class names. When
I started working on YARV, there had already been several proposals for the
development of new virtual machines for Ruby. RiteVM by Matz, the creator of
Ruby, and ByteCodeRuby were the most well known projects then, as far as I can
remember. That lead me to prefix the name of our VM with “Yet Another.” Of
course, I named it so hoping it would become popular. There are many examples
of software programs that have “Yet Another” in their names and nevertheless became
popular: for example yacc. By the way, RiteVM is now the name of the mruby VM
which Matz is actively developing.

## Design Principles of YARV

We chose to implement YARV for the Ruby 1.9 specification, instead of 1.8. At
the time, Ruby 1.9 was the next version, and we were discussing its
specification. We targeted YARV at that specification. We also had the option
of implementing it for Ruby 1.8, thereby supporting a large number of users
instantly. But some of the Ruby 1.8 features seemed difficult to implement with
the stack machine which YARV is based on. So I decided to implement my VM for
the newer spec, while negotiating with Ruby developers to change the parts of
the specification that were hard to implement. This strategy worked well, and
YARV became one of the interpreters to run Ruby 1.9. I think that was one of
the reasons that it was finally merged in as an official VM.

This book correctly explains YARV’s design details. I would, however, like to add
that it was not so straightforward to get to the current design. One of the
things I remember is the stack structure of the virtual machine. The book
describes YARV as a "double stack machine," but it was using only one stack at
first.  Actual microprocessors allocate a calculation area and a function call
frame one after another on a single stack. YARV used a similar architecture at
first, but it became too complicated. Later I concluded that it should have two
stacks, even if I had to give up some efficiency. YARV’s operation became too
complex especially when implementing the extraction of a block as a closure.
Because this book cleverly avoids such hairy details, readers fortunately do
not have to confront this sort of complexity. I hope, though, I have chance to
explain that, because it was one of the most difficult parts to implement. By
the way, having two stacks means that the cost of checking for stack overflows
also doubles. So I implemented them both in the same memory block: one going
from bottom to top and the other going from top to bottom.  This trick somewhat
reduced the cost of checking for stack overflows, because we only have to check
the positions of two stack pointers once.

## YARV Development Prehistory 

I'd like to describe the history of how I came to develop YARV. Because earlier
I had been interested in programming language processors, I had the experience
of implementing two Java virtual machines. That gave me some knowledge of what
was required to implement virtual machines intended for object-oriented
programming languages. At the time, Mr.  Nobuo Yamashita was periodically
holding meetups to read the book <i>The Structure and Interpretation of
Computer Programs</i> (or SICP). Attending them, I acquired knowledge and
insight about implementations of Scheme. This insight was important since
Ruby's block design was based on Lisp functions, as Chapter 8 of this book
points out.

It was December 2002 when <i>Ruby Source Code Kanzen Kaisetsu</i> (the <i>Ruby
Hacking Guide</i>, or RHG), by Mr. Minero Aoki was published, which is a unique
book that explains the entire Ruby source code. Mr. Masayoshi Takahashi held
meetings to read RHG about once a month. We took turns in a reading group, but
because the author Aoki-san himself was one of the members the other members
could talk with him in person when they had questions. In this way, we learned
the implementation details of Ruby very well. Let me add that both of these
meetups are held in the meeting rooms at Time Intermedia, Ltd., where Mr.
Yamashita was working then. I attended the meetups frequently: several times a
month. I wish to express my deep gratitude to the people who provided such an
environment for learning.

After reading RHG and learning more about the structure of Ruby’s
implementation, it became clear to me that the evaluation module Ruby used to
run programs - the heart of the Ruby interpreter - was not efficient enough. I
kept on studying and thinking about the ideal design of a virtual machine to run
Ruby programs precisely and efficiently, which I finally implemented all at
once during that New Year's holiday. I didn’t foresee that it would be released
as a part of Ruby 1.9. My first motivation was performance improvement - my
source code surely reflected that. In hindsight, it was such an early
optimization.

## Yet More Ruby Virtual Machines

This book explains the current architecture of YARV, which you might conclude
is the correct way of implementing Ruby. But, as I have explained in this
appendix so far, all of Ruby’s implementations, including YARV, are not much
different from any other software application: they are all developed through
trial and error by humans. While this book covers Ruby 2.0, we have already
made various improvements for Ruby 2.1. And we are working on even more
improvements that will make the forthcoming Ruby 2.2 even better.

For example, keyword arguments will be more efficient. Chapter 4 explains the
implementation of keyword arguments. Quickly summarizing: Ruby first passes a
hash object containing keyword name and value pairs as a normal argument. Then,
at compile time, the receiver implicitly expands code that reads the values
from the argument hash. Users don’t seem to be complaining about its
performance for now. I assume keyword arguments are not widely used, because it
is a new feature introduced in Ruby 2.0. But this implementation is not
efficient. Hash objects are created every time, incurring object creation and
GC costs. Also, reading from Hash objects using the implicitly expanded code is
slow, because it involves multiple method calls.

In order to address this problem, we are reimplementing how Ruby 2.2 handles
keyword arguments to avoid creating Hash objects as much as possible.
Meanwhile, we are implementing a new design that will collect the names of
keyword arguments at compile time, so that caller need only pass the values at
runtime. The callee will then recombine the values with the names collected
by the compiler. This design change will allow Ruby to process keyword
arguments 10 times faster. I would like to keep on improving the quality of
Ruby's implementation, including runtime efficiency.

If you become interested in YARV and Ruby implementations after reading this
book, if you have ideas for improving them, I encourage you to develop your own
“Yet Another Ruby Implementation.” The Ruby core community will welcome your
challenge.

November 2014<br/>
Koichi Sasada<br/>
Heroku, Inc.
