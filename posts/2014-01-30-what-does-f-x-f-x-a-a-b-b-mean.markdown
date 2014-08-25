title: "What does (((λ f . (λ x . (f x))) (λ a . a)) (λ b . b)) mean?"
date: 2014/1/30
url: /2014/1/30/what-does-f-x-f-x-a-a-b-b-mean

This week I’ve been spending some free time between projects working on my
function programming skills. I’m a Ruby developer but have always been
fascinated by functional languages such as Haskell, Clojure and of course Lisp.
I was excited this morning when I stumbled across a fascinating article called
[7 lines of code, 3 minutes: Implement a programming language from scratch](http://matt.might.net/articles/implementing-a-programming-language/). The
author, [Matthew Might](https://twitter.com/mattmight), touches on Lambda Calculus notation before implementing
a simple interpreter for Lambda Calculus using just seven lines of Scheme code.
He later proceeds to write a Scheme interpreter in about 100 lines of code,
using a dialect of Scheme called [Racket](http://racket-lang.org).

Matthew starts by introducing the basics of Lambda Calculus notation like this:

<div style="margin: 20px 0 0 100px">
  <img src="http://patshaughnessy.net/assets/2014/1/30/basics.png"><br/>
</div>

Matthew explains that all of Lambda Calculus is based on these two simple
concepts, along with variable references. He then shows a couple of simple examples:

<div style="margin: 20px 0 0 100px">
  <img src="http://patshaughnessy.net/assets/2014/1/30/examples.png"><br/>
</div>

...and then asks the reader this simple question:

<div style="margin: 20px 0 0 100px">
  <img src="http://patshaughnessy.net/assets/2014/1/30/question.png"><br/>
</div>

In this post I’ll try to answer this question using a series of diagrams. I’m a
visual learner, and teacher. For me drawing something makes it much easier to
understand and to explain to someone else. Lambda Calculus experts (“Lambda
Mathematicians?”) probably have accepted styles for drawing these concepts
which I’m not aware of. I’ll just draw what comes naturally to me.

## Getting Rid of All Those Parentheses

If I had a whiteboard, I’d rewrite the complex Lambda Calculus expression above
with circles instead of parentheses. This makes it much easier for me to see
the nesting and order of operations. An even better visualization might be a tree structure, but I'll keep things inline with cirlces today.

<img src="http://patshaughnessy.net/assets/2014/1/30/one.png"><br/>

For me, the biggest challenge to understanding Lisp or Scheme has always been
all the nested parentheses. This picture is still cryptic and meaningless, but
now at least I can see the order of operations and nesting more easily.

## Start From the Inside and Work Your Way Out

To understand what’s going on here, I started with the innermost lambda
expression:

<img src="http://patshaughnessy.net/assets/2014/1/30/two.png"><br/>

Referring to Matthew’s explanation above, <span class="code">(f x)</span> means evaluate the function <span class="code">f</span>
on the argument <span class="code">x</span>. Therefore, this expression represents an anonymous function
that applies some function <span class="code">f</span> to its argument, <span class="code">x</span>. The oval on the left is the
function and I’ve written this explanation on the right: what the function
does.

I usually visualize functions with inputs and outputs, like this:

<img src="http://patshaughnessy.net/assets/2014/1/30/three.png"><br/>

Here you can see the input <span class="code">x</span> comes from the left, and I’ve shown the anonymous
function in the center using a half-oval. It applies a function <span class="code">f</span>, which is
almost like a second input value, to <span class="code">x</span> and returns an output value on the
right.

## Higher Order Functions

Now let’s substitute this simple function back into the complex expression:

<img src="http://patshaughnessy.net/assets/2014/1/30/four.png"><br/>

You can see I’ve pasted the text “Apply f to my argument” into the center of
the expression where the previous lambda was located. Working my way out, let’s
take a look at what the next lambda expression means:

<img src="http://patshaughnessy.net/assets/2014/1/30/five.png"><br/>

Here you can see a definition for another anonymous function, another lambda
expression, which takes <span class="code">f</span> as an argument. The body of the function is our
previous expression.

What this boils down to, therefore, is a function that takes a function as an
argument and returns a second function as a result. Here’s what this might look
like using my input/output diagram style:

<img src="http://patshaughnessy.net/assets/2014/1/30/six.png"><br/>

On the left you can see the function takes a function, <span class="code">f</span>, as input. On the
right the dashed rectangle represents a single output value: another function.
This is an example of a higher order function, a function that can use other
functions as inputs or outputs. 

In this example, the output is a new function which applies <span class="code">f</span>, the input of the
higher order function, to its argument <span class="code">x</span>. We don’t have a value for <span class="code">x</span> yet, but
we have a new function which later can take an input <span class="code">x</span>. It will apply the
present input, <span class="code">f</span>, to whatever argument it receives later, <span class="code">x</span>.

The other subtle detail to think about here is that above, when we looked at
the innermost lambda expression, the value <span class="code">f</span> was undefined. The inner lambda
applied <span class="code">f</span> to its input <span class="code">x</span>, not knowing what <span class="code">f</span> was. Now <span class="code">f</span> has been provided by
the surrounding expression, the outer lambda. <span class="code">f</span> is the input to the outer
lambda. I’m not sure about this, but I suspect lambda mathematicians would say
the outer lambda is a closure around the free variable, <span class="code">f</span>.

## Identity Functions

Substituting again, here’s the entire expression simplified:

<img src="http://patshaughnessy.net/assets/2014/1/30/seven.png"><br/>

The two lambdas on the right are easier to follow. These are examples of the
identity function, a function that returns its argument. Matthew showed this
syntax as one of his examples.

<img src="http://patshaughnessy.net/assets/2014/1/30/eight.png"><br/>

You can see here the identity function simply passes its input along as an
output:

<img src="http://patshaughnessy.net/assets/2014/1/30/nine.png"><br/>

## Evaluating a Function

Substituting again, here’s what we have now:

<img src="http://patshaughnessy.net/assets/2014/1/30/ten.png"><br/>

Earlier we saw the higher order function on the left returns a function that
will later operate on an argument, which I showed as <span class="code">x</span>. Now its time has come.
Recall that <span class="code">(f x)</span> in Lambda Calculus means “evaluate the function <span class="code">f</span> on the
argument <span class="code">x</span>.”

In other words, this expression:

<img src="http://patshaughnessy.net/assets/2014/1/30/eleven.png"><br/>

… means: “evaluate the higher order function on the left, providing the
function on the right as an argument.”

The higher order function will take one function (the identity function) and
return another:

<img src="http://patshaughnessy.net/assets/2014/1/30/twelve.png"><br/>

The output is a new function which will apply the identity function (the
argument) to its own argument <span class="code">x</span>. Of course, this is equivalent to the identity
function itself!

<img src="http://patshaughnessy.net/assets/2014/1/30/thirteen.png"><br/>

There must be some Lambda Calculus axiom (the “lambda identity axiom?”) behind
this deduction, but I’m not sure. I’ll consider it obvious and just move on.

## The Result

Substituting this result back into the original expression, we’re left with
this:

<img src="http://patshaughnessy.net/assets/2014/1/30/fourteen.png"><br/>

Reapplying the “lambda identity axiom” again it’s
obvious this is equivalent to the identity function:

<img src="http://patshaughnessy.net/assets/2014/1/30/fifteen.png"><br/>

That is, the identity function is also a higher order function. It returns the
same functions you pass to it, unchanged.

Stepping back and taking stock, we’ve deduced that:

<span class="code">(((λ f . (λ x . (f x))) (λ a . a)) (λ b . b))</span>


Is equivalent to:

<span class="code">(λ x . x)</span>

Or the identity function.

## What Did I Get Wrong?

As I mentioned, I’m just learning about functional programming and lambda
calculus. If you have some computer science training and see something wrong
here, let me know. Or if you know the real name for the “lambda identity axiom”
or have examples of better diagrams for representing lambda expressions, please
pass along a link.
