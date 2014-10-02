title: "20,000 Leagues Under ActiveRecord"
date: 2014/9/17

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/17/title.jpg"><br/>
  <i>First published in 1870, <a href="http://en.wikipedia.org/wiki/Twenty_Thousand_Leagues_Under_the_Sea">20,000 Leagues Under the Sea</a><br/>describes an underwater adventure that takes place<br/>onboard a submarine called the “Nautilus.”</i>
</div>

<b>
This is the first in a series of posts based on a presentation I just did
last Friday at the [Barcelona Ruby Conference](http://www.baruco.org). You can
also watch the [video recording of the
presentation](https://www.youtube.com/watch?v=rnLnRPZZ1Q4).

</b>

Let me start with a question: How many of you have actually read 20,000 Leagues
Under the Sea, either in the original French or in some translation? [ A few
people raised their hands, but most people in the audience did not. ] Yes, I’m
not surprised. 20,000 Leagues is one of those classic novels we have all heard
of, but few of us take the time to read.

While thinking about this presentation over the Summer, I decided to actually
read the novel - and I’m glad that I did! It blew my mind on a number of
different levels. Today I’m going to take the time to tell you a bit about the
novel itself as we go.

However, actually I’m here today to talk about ActiveRecord. I’d like to explore
how ActiveRecord works internally; how it finds the information we ask for and
returns it to us as a Ruby object.

<div style="clear: left"/></div>

<img src="http://patshaughnessy.net/assets/2014/9/17/example1.png"/>

But why talk about ActiveRecord? We all know how to use ActiveRecord; most of
you understand exactly what this line of code does. You didn’t need to come to
Barcelona to learn how to use ActiveRecord.

The reason why I want to discuss this is that ActiveRecord hooked me; it
first got me excited about using Rails. I came across Rails back in 2008, about
six years ago now. And I can still remember the moment when I first typed a
line of code similar to this one into the Rails console or into a Rails app
somewhere. It was amazing!  Before that I was using PHP or Java - I’ve since
blocked out all memory of that so I’m not quite sure which it was :) - and when
I saw how easy it was to use Rails to query a database I was very impressed.

<img src="http://patshaughnessy.net/assets/2014/9/17/irb.png"/>

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/17/underwater-walk.png"><br/>
  <i>Captain Nemo takes Professor Aronnax, Conseil and Ned Land<br/>on a hike through an underwater “forest.” Verne’s novel has<br/>many beautiful, detailed descriptions of underwater worlds<br/>he had never actually seen, and could only imagine.</i>
</div>

Somehow Rails could take a simple, beautiful and readable line of Ruby code and
convert it into a SQL statement like this. How could this possibly work? It
seemed like magic!

But it’s not magic. There are thousands of lines of code and years of computer
science research behind this simple line of code, the hard work behind the
scenes that makes ActiveRecord queries possible.

Just as Professor Aronnax went on an underwater adventure with Captain Nemo,
I’d like to take you on an adventure inside of ActiveRecord to find out how it
works, how ActiveRecord generates and executes SQL statements.

And why stop there? Later we’ll dive underneath ActiveRecord and inside an
actual database server to find out how it works too. How does it understand the SQL
statements we give it? How does it find the data we ask for and return it to
us? Let’s find out!

## Agenda

Here’s our plan: First we’ll start with a look at <span
class='code'>ActiveRecord::Relation</span>, the top, public API for
ActiveRecord many of you use everyday. Then, we’ll look deeper inside of
ActiveRecord to find out how it converts our Ruby queries into SQL statements.

<img src="http://patshaughnessy.net/assets/2014/9/17/agenda.png"/>

Later, in the second half of the presentation we’ll dive even deeper and
directly into an actual relational database server (RDBMS); today I’ll use
PostgreSQL as an example. How does Postgres understand the SQL statement that
ActiveRecord sent it? How does it find the data we want? Finally, if we have
time we’ll look at the B-Tree Algorithm, which is part of the real magic that
allows database servers to work.

## ActiveRecord::Relation

Let’s use my line of code from earlier as an example - today we’ll be searching
for Captain Nemo together. Therefore, we’ll start with a <span
class='code'>User</span> class, a subclass of <span
class='code'>ActiveRecord::Base</span>.

<img src="http://patshaughnessy.net/assets/2014/9/17/activerecord-base.png"/>

When I call <span class='code'>where</span> what happens? It turns out the
<span class='code'>where</span> method is defined in the <span
class='code'>ActiveRecord::Querying</span> module:

<img src="http://patshaughnessy.net/assets/2014/9/17/activerecord-querying.png"/>

But as you can see, ActiveRecord delegates the <span class='code'>where</span>
method over to another method called <span class='code'>all</span>, which
returns a new instance of a different class called <span
class='code'>ActiveRecord::Relation</span>. In fact, my call to <span
class='code'>User.where</span> is entirely equivalent to calling <span
class='code'>User.all.where</span>:

<img src="http://patshaughnessy.net/assets/2014/9/17/user-all.png"/>

ActiveRecord actually implements the <span class='code'>where</span> method
using the new instance of <span class='code'>ActiveRecord::Relation</span>.
Internally, <span class='code'>where</span> is implemented by the <span
class='code'>ActiveRecord::QueryMethods</span> module, included into <span
class='code'>ActiveRecord::Relation</span>.  Next, <span
class='code'>ActiveRecord::QueryMethods#where</span> returns, in turn, a second
new instance of <span class='code'>ActiveRecord::Relation</span>:

<img src="http://patshaughnessy.net/assets/2014/9/17/activerecord-relation1.png"/>

If you look at the right, you can see the second <span
class='code'>ActiveRecord::Relation</span> object contains information about
what record we’re looking for, that we want the records where the name is
“Captain Nemo.”

Of course, we don’t want all of the Captain Nemo users; we just
want the first one. Next, we call the <span class='code'>first</span> method on
the new <span class='code'>ActiveRecord::Relation</span>:

<img src="http://patshaughnessy.net/assets/2014/9/17/activerecord-relation2.png"/>

## The Internal Implementation of ActiveRecord::FinderMethods#first

Now you can see ActiveRecord creates a third instance of <span
class='code'>ActiveRecord::Relation</span>
- this time with even more information about the query we’d like to execute.
  But what are all those other values: <span class='code'>order</span>, <span
  class='code'>limit</span> and <span class='code'>offset</span>? Where do they
  come from? We can find out by taking a look at the implementation of <span
  class='code'>first</span>. The <span
  class='code'>ActiveRecord::FinderMethods</span> module implements this, which
  ActiveRecord includes into the <span class='code'>ActiveRecord::Relation</span> class.

<img src="http://patshaughnessy.net/assets/2014/9/17/first1.png"/>

Here you can see because we didn’t pass in a value for <span class='code'>limit</span>, ActiveRecord
calls <span class='code'>find\_nth</span> and passes in a value of 0,
indicating we want the first record from the query result set. The second
argument, <span class='code'>offset\_index</span>, turns out to have a value of
zero, meaning we want to count into a window of records at the beginning of the
result set, not a window located somewhere farther along the result set.

ActiveRecord implements a series of similar methods that will return the
second, third, fourth or even fifth record:

<img src="http://patshaughnessy.net/assets/2014/9/17/second-fifth.png"/>

You can see the pattern here; the first argument is a zero-based index
indicating which record we want. And just in case we want the forty second
record from the result set, ActiveRecord implements this useful method... :)

<img src="http://patshaughnessy.net/assets/2014/9/17/forty-second.png"/>

It’s no joke! <span class='code'>forty_two</span> is actually in the
ActiveRecord source code; you can try it for yourself. Replacing <span
class='code'>first</span> with the equivalent call to <span
class='code'>find\_nth</span>, here’s our example again:

<img src="http://patshaughnessy.net/assets/2014/9/17/find-nth.png"/>

Following the code path through the <span
class='code'>ActiveRecord::FinderMethods</span> module, we can see <span
class='code'>find\_nth</span> calls, in turn, <span
class='code'>find\_nth\_with\_limit</span>:

<img src="http://patshaughnessy.net/assets/2014/9/17/find-nth-with-limit.png"/>

Now the arguments have been reversed; the first argument, 0, is now the offset
and the second, 1, is the number of records we want, or the limit value.

Substituting one more time, let’s replace <span
class='code'>find\_nth\_with\_limit</span> with more detailed calls it makes to
<span class='code'>order</span> and <span class='code'>limit</span>:

<img src="http://patshaughnessy.net/assets/2014/9/17/detailed-calls.png"/>

Now you can see where all of the values in the final <span
class='code'>ActiveRecord::Relation</span> object come from. Each call to a
scoping method saves a new piece of information about our query, and returns a
new instance of the <span class='code'>ActiveRecord::Relation</span> class.
(We’ll see what <span class='code'>arel\_table</span> means in a minute.)

## The Beauty of ActiveRecord::Relation

Taking a step back, we can see that our simple line of code, <span
class='code'>User.where(name: &quot;Captain Nemo&quot;).first</span>, is creating a
series of <span class='code'>ActiveRecord::Relation</span> objects like this:

<img src="http://patshaughnessy.net/assets/2014/9/17/method-chain.png"/>

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/17/south-pacific.png"><br/>
  <i>Captain Nemo allowed Canadian harpoonist Ned Land<br/>to leave the submarine for a short time and explore a<br/>tropical island off the coast of Papua New Guinea.</i>
</div>

There are two interesting and beautiful things about this, I think. First,
notice the pattern that ActiveRecord uses: each call to a method returns a new
instance of the class that implemented that method. This is what allows us to
easily chain together different method calls. We can add on as many different
scopes as we wish; because each new object is also an <span
class='code'>ActiveRecord::Relation</span>, it implements all of the same
methods. You can use the same pattern in your own code. All you need to do is
create a new instance of the class that implements each method, and return
that. One reason to study internal code like this is to learn about and find
new ideas that you can use in your own code.

The second beautiful thing about <span
class='code'>ActiveRecord::Relation</span> is that it’s lazy.  Using this chain
of method calls we are building up metadata or information about our query,
without actually executing the query itself. It’s almost as if we were using a
functional programming language like Haskell or Lisp. Using this trick,
ActiveRecord allows us to specify exactly the query we want, without having to
worry about executing it until we’re ready.

It’s not until we call the <span class='code'>to\_a</span> method, in other to convert the
<span class='code'>ActiveRecord::Relation</span> object into an array and access the result set, that
ActiveRecord executes the query:

<img src="http://patshaughnessy.net/assets/2014/9/17/to-a.png"/>

You can see here that <span class='code'>to\_a</span> calls <span class='code'>load</span> internally,
which later calls the <span class='code'>DatabaseStatements#select\_all</span>
method. Note the <span class="code">find\_nth\_with\_limit</span> method calls
<span class="code">to\_a</span>, so <span class="code">first</span>,
<span class="code">second</span> and <span class="code">forty\_two</span> are
not lazy and will all execute the query immediately. Because of this, these are
known as “terminating methods.” To prevent the query from executing immediately -
to keep it lazy - just use <span class="code">order</span> and <span
class="code">limit</span> instead.

## Next time

In the next few days I’ll post [the second part of my presentation from
Barcelona](http://patshaughnessy.net/2014/9/23/how-arel-converts-ruby-queries-into-sql-statements).
We’ll look at what “Relational Algebra” means and how the Arel gem converts our
<span class='code'>ActiveRecord::Relation</span> object into a string
containing a SQL statement.
