title: "How Arel Converts Ruby Queries Into SQL Statements"
date: 2014/9/23

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/23/battle.png"><br/>
  <i>In one of the climactic scenes in</i> 20,000 Leagues Under the<br/>Sea, <i>Captain Nemo and his crew battle a giant octopus.</i>
</div>

<b>
This is the second in a series of posts based on a presentation I did at the
[Barcelona Ruby Conference](http://www.baruco.org) called “20,000 Leagues Under
ActiveRecord.”</b> I took an innocent and unsuspecting audience on an adventure
inside and underneath ActiveRecord to find out how Rails and PostgreSQL
actually execute a simple SQL query.

In the [first part of the
presentation](http://patshaughnessy.net/2014/9/17/20000-leagues-under-activerecord)
I showed what ActiveRecord does internally when you call methods such as <span
class="code">where</span> and <span class="code">limit</span>. We saw how each
scoping method returns a new instance of the <span
class="code">ActiveRecord::Relation</span> class, gradually building up a
description of your query. 

Today I’ll continue by looking at what ActiveRecord does next: How it uses the
Arel gem to convert the <span class="code">ActiveRecord::Relation</span> object
describing your query into a string containing a SQL statement. Later, in the
third post we’ll dive down inside the PostgreSQL database server itself to see
how it executes this SQL statement.

## The Arel Gem

Here again is the instance of the <span class="code">ActiveRecord::Relation</span> class that represents
our query to find the user named “Captain Nemo:”

<img src="http://patshaughnessy.net/assets/2014/9/23/ar-relation1.png"/>

Now that we’ve specified the query we want to execute, what does ActiveRecord
do next? How does it actually execute the query and return the results to us?
We can find a clue by looking more closely at the relation object and the
metadata values it stores:

<img src="http://patshaughnessy.net/assets/2014/9/23/AR-Relation2.png"/>

If you inspect an <span class="code">ActiveRecord::Relation</span> object in
the Rails console, you’ll find that its instance variables are not simple
values, but instead references to other Ruby objects. These objects have class
names such as <span class="code">Arel::Nodes::Equality</span> and <span
class="code">Arel::Nodes::Attribute</span>. What are these Ruby objects?  Where
are they created? What do these class names mean?

It turns out that ActiveRecord itself doesn’t convert your
<span class="code">ActiveRecord::Relation</span> query to SQL; instead, it uses
a separate gem called Arel. Googling for “Arel” we can easily find its Github
repo:

<img src="http://patshaughnessy.net/assets/2014/9/23/arel1.png"/><br/>
<img src="http://patshaughnessy.net/assets/2014/9/23/arel2.png"/>

The gem’s description is simply “A Relational Algebra.” What in the world does
this mean? And farther down in the Readme there’s another interesting line:
“Arel is a SQL AST manager for Ruby.” What does “AST” mean, and what does an
“AST manager” do?

AST stands for “Abstract Syntax Tree,” an important concept from computer
science. I’ll explain what that means in a minute. But first let’s look at some
computer science history to find out what Relational Algebra is.

## Relational Algebra

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/23/codd.jpg"><br/>
  <i>Edgar Codd</i>
</div>

Relational Algebra is a branch of computer science that forms the mathematical
foundation underpinning relational database servers and the SQL language. An
influential computer scientist named [Edgar
Codd](http://en.wikipedia.org/wiki/Edgar_F._Codd) first described Relational
Algebra in his groundbreaking academic paper [A Relational Model of Data for
Large Shared Data Banks](http://www.seas.upenn.edu/~zives/03f/cis550/codd.pdf),
published in 1970. Codd described the term “relation” as follows:

<blockquote>
1.3. A RELATIONAL VIEW OF DATA<br/>
The term relation is used here in its accepted mathematical sense. Given sets
S<sub>1</sub>, S<sub>2</sub>, ... , S<sub>n</sub>, (not necessarily distinct), R is a relation on
these n sets if it is a set of n-tuples each of which has its first element
from S<sub>1</sub>, its second element from S<sub>2</sub>, and so on.
</blockquote>

He later went on to define various mathematical operations on relations,
including _projection_, _restriction_, and _join_. He also used terms such as _normal
form_, _primary key_ and _foreign key_. Today, almost 45 years later, we still use
Codd’s terminology and the associated mathematical theories when discussing
database tables and queries.

In another interesting passage, Codd wrote about the need for a language we
could use to articulate and describe Relational Algebra concepts:

<blockquote>
1.5. SOME LINGUISTIC ASPECTS<br/>
The adoption of a relational model of data, as described above, permits the
development of a universal data sublanguage based on an applied predicate
calculus.
</blockquote>

This “universal sublanguage” is the Structured Query Language or SQL. I find
the term “sublanguage” to be very appropriate; SQL is a language used inside
larger applications written in some other programming language, such as Ruby.

Returning to our example, here’s the SQL statement that represents our search
for the user named Captain Nemo:

<img src="http://patshaughnessy.net/assets/2014/9/23/sql.png"/>

The SQL language existed before Codd wrote his paper on Relational Algebra in
1970, but it didn’t resemble the version of SQL we all know and love today. The
mathematical concepts Codd first described form the basis for the modern
version of SQL.

Now let’s return to the question of what an “AST manager” is.

## Abstract Syntax Trees

An abstract syntax tree is a hierarchical arrangement of objects or memory
structures that represent a series of words or some syntax from a text
language. In this case, the AST inside of Arel is a tree of Ruby objects that
represents a SQL statement.

Here’s the AST Arel creates internally for our example query:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree1.png"/>

You can see the top or root of the tree is a Ruby object called
<span class="code">SelectStatement</span>, and under that the various branches
of the tree represent the where, order by and limit clauses in our
select statement.

The Arel gem “is a Relational Algebra” in the sense that it provides a Ruby API
that contains methods such as <span class="code">project</span>, <span
class="code">where</span> and <span class="code">order</span> that represent
concepts from Relational Algebra. Internally, these methods create Ruby objects
and save them in the AST. Arel’s API is similar to ActiveRecord’s, but is
somewhat more granular and detailed. When we call ActiveRecord methods such as
<span class="code">where</span> and <span class="code">limit</span>, internally
ActiveRecord calls the corresponding methods in the Arel gem. Here’s our
example query written using both ActiveRecord (top) and Arel (bottom) method
calls:

<img src="http://patshaughnessy.net/assets/2014/9/23/arel-and-ar.png"/>

<div style="float: right; padding: 7px 0px 60px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/23/window.png"><br/>
  <i>Professor Aronnax, Conseil and Ned Land spent hours marveling<br/>at the underwater world through the windows of the Nautilus.</i>
</div>

Notice the Arel query is longer and more verbose. Expressing our query using Arel we call:

* <span class="code">project</span> to specify which columns or attributes we
  are looking for (_projection_ in Codd’s Relational Algebra)
* <span class="code">where</span> and <span class="code">eq</span> to specify
  how to filter the result set (_restriction_ in Relational Algebra)
* <span class="code">order</span> to specify the sort order, and
* <span class="code">limit</span> to specify how many records we want.

Each time you call an ActiveRecord scoping method, it calls down into one of
these Arel methods, inserting objects into the same AST.

## The Visitor Pattern

Creating an AST containing Ruby objects is one thing, but generating an actual
SQL statement is another. How does Arel do this? Why is building up an AST
useful in any way?

Using a very elegant algorithm, Arel iterates over the nodes in the AST and
concatenates different SQL fragments to form a complete SQL statement. This
algorithm is an example of the “visitor pattern.” The term visitor pattern
simply means that some object, function or other piece of code is executed once
for each node in some data structure, such as an array, linked list or tree.

To understand this a bit better, let’s take our example AST and follow Arel’s
visitor as it traverses the tree, starting at the <span
class="code">SelectStatement</span> root node:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree2.png"/>

The blue arrow at the top is the visitor, a Ruby object. Depending on which
database server you are using, Arel creates a different visitor object. This is
a fascinating detail about Arel’s internal design: Arel can generate different
variations of SQL equally well by using different visitor objects. If you
connect your Rails app to SQLite, Arel uses a SQLite visitor. If you are using
MySQL, Arel uses a MySQL visitor. Because we’re using PostgreSQL today, Arel
creates a Postgres visitor.

## Visiting All the Nodes in the AST

Now let’s follow Arel’s visitor as it iterates over the Ruby objects in the
AST, shown as a moving blue arrow. Above each diagram I’ll show the SQL string
the visitor cumulatively builds up as it goes.

<img src="http://patshaughnessy.net/assets/2014/9/23/tree2b.png"/>

Above you can see the visitor arrow next to the <span class="code">SelectStatement</span> node. Above the
diagram I’ve written the word “SELECT.” Arel’s visitor knows to write SELECT
when it encounters <span class="code">SelectStatement</span> root node.

Next Arel moves down to the left:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree3.png"/>

This time Arel doesn’t write anything new into the string; <span
class="code">SelectCode</span> is just a container for other branches of the
tree.

Next, Arel moves down and left again:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree4.png"/>

Now Arel’s visitor see the <span class="code">Attribute</span> node. This
represents the projection or list of attributes we want in the result set. Arel
appends <span class="code">&quot;users&quot;.*</span> to the SQL string.

Next, the visitor moves to the right:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree5.png"/>

Encountering the <span class="code">JoinSource</span> node, Arel writes <span
class="code">FROM &quot;users&quot;</span> onto the end of the SQL statement. <span
class="code">JoinSource</span> and its child nodes list the tables that our
query will read from. In this example, we don’t have any joins and just a
single table, so <span class="code">JoinSource</span> has only one <span
class="code">Table</span> child node.

Next, the visitor moves to the right again:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree6.png"/>

Now Arel writes the where clause for our SQL statement: <span class="code">WHERE &quot;users&quot;.&quot;name&quot; =
$1</span>. The <span class="code">And</span> node is the root of a subbranch of the AST that represents the
boolean expression we want the database server to use to filter our result set.
In our example we are only checking that the name column equals “Captain Nemo”
so the AST contains a single <span class="code">Equality</span> node under
<span class="code">And</span>. The <span class="code">And</span> node doesn’t
really do anything in this case.

Now the visitor continues to the right:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree7.png"/>

Here you can see Arel finds the <span class="code">Ascending</span> node and appends our order by clause.

Finally, the visitor moves to the right one last time:

<img src="http://patshaughnessy.net/assets/2014/9/23/tree8.png"/>

Finding the <span class="code">Limit</span> node, Arel’s visitor completes the
SQL statement by concatenating <span class="code">LIMIT 1</span> onto our select statement.

Using the visitor pattern in this way, Arel has converted our query from a
collection of Ruby objects into a single string containing a SQL select
statement. Arel has expressed our Ruby query using Codd’s Relational Algebra.

Every time you execute a simple database query using ActiveRecord in your Rails
app, you are relying on a series of elegant algorithms and computer science
theories developed many years ago. Rails is so simple and easy to use only
because we are standing on the shoulders of giants - computer scientists like
Edgar Codd - who have already done the difficult theoretical work that makes
building apps today possible.

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/9/23/south-pole.png"><br/>
  <i>A citizen of no country, Captain Nemo claimed the <br/>south pole as his own using a black flag with a large "N."</i>
</div>

## Why Stop Here?

We’ve learned a lot about ActiveRecord and the Arel gem. Now we know what
happens when we call scoping methods such as <span class="code">where</span>
and <span class="code">first</span>. We’ve seen how ActiveRecord calls Arel’s
lower level, more granular API, and now we know how Arel uses the visitor
pattern and an AST to convert these Ruby method calls into a SQL string.

But why stop here? Why not dive even deeper? … farther below the surface of
your Rails app into the PostgreSQL server itself! Next we’ll leave the world of
Ruby entirely and look at what the Postgres server does when it receives this
select statement. How does it understand the SQL we send it? How does it
actually find our data… the user record with the name “Captain Nemo?” In my next
post, I’ll continue our underwater adventure by looking at Postgres internals.
