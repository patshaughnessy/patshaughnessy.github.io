title: "Following a Select Statement Through Postgres Internals"
date: 2014/10/13

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/10/13/engine-room.png"><br/>
  <i>Captain Nemo takes Professor Aronnax on a tour<br/>of the engine room, a fascinating description<br/>of future technology from an 1870 perspective.</i>
</div>
<b>
This is the third of a series of posts based on a presentation I did at the
[Barcelona Ruby Conference](http://www.baruco.org) called “20,000 Leagues Under
ActiveRecord.” (posts:
[one](http://patshaughnessy.net/2014/9/17/20000-leagues-under-activerecord)
[two](http://patshaughnessy.net/2014/9/23/how-arel-converts-ruby-queries-into-sql-statements)
and [video](https://www.youtube.com/watch?v=rnLnRPZZ1Q4)).  </b>

Preparing for this presentation over the Summer, I decided to read through
parts of the PostgreSQL C source code. I executed a very simple select
statement and watched what Postgres did with it using LLDB, a C debugger. How
did Postgres understand my query? How did it actually find the data I was
looking for?

This post is an informal journal of my trip through the guts of Postgres. I’ll
describe the path I took and what I saw along the way. I’ll use a series of
simple, conceptual diagrams to explain how Postgres executed my query.
In case you understand C, I’ll also leave you a few landmarks and signposts you
can look for if you ever decide to hack on Postgres internals.

In the end, the Postgres source code delighted me. It was clean, well
documented and easy to follow. Find out for yourself how Postgres works
internally by following me on a journey deep inside a tool you use everyday.

<div style="clear: left"></div>

## Finding Captain Nemo

Here’s the example query from [the first half of my
presentation](http://patshaughnessy.net/2014/9/17/20000-leagues-under-activerecord);
we’ll follow Postgres as it searches for Captain Nemo:

<img src="http://patshaughnessy.net/assets/2014/9/23/sql.png"/>

<div style="float: right; padding: 7px 0px 0px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/10/13/maps.png"><br/>
  <i>Professor Aronnax and Captain Nemo<br/>plot the course of the Nautilus.</i>
</div>

Finding a single name in string column like this should be straightforward,
shouldn’t it? We’ll hold tightly onto this select statement while we explore
Postgres internals, like a rope deep sea divers use to find their way back to
the surface.

## The Big Picture

What does Postgres do with this SQL string? How does it understand what we
meant? How does it know what data we are looking for?

Postgres processes each SQL command we send it using a four step process.

<img src="http://patshaughnessy.net/assets/2014/10/13/4-steps.png"/>

In the first step, Postgres _parses_ our SQL statement and converts it into a
series of C memory structures, a _parse tree_. Next Postgres _analyzes and
rewrites_ our query, optimizing and simplifying it using a series of complex
algorithms. After that, Postgres generates a _plan_ for finding our data. Like
an obsessive compulsive person who won’t leave home without every suitcase
packed perfectly, Postgres doesn’t run our query until it has a plan. Finally,
Postgres actually _executes_ our query. In this presentation I’ll briefly touch
on the first three topics, and then focus more on the last step: _Execute_.

The C function inside of Postgres that implements this 4 step process is called
<span class="code">exec\_simple\_query</span>. You can find a link to it below, along with an LLDB
backtrace which gives some context about exactly when and how Postgres calls
<span class="code">exec\_simple\_query</span>.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">exec_simple_query</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/postgres_8c.html#a7908e75bd9f9494fdb8c4b47f01a9de9">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_simple_query.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_simple_query_bt.png"/>
  </div>
</div>
</p>

## Parse

How does Postgres understand the SQL string we sent it? How does it make sense
of the SQL keywords and expressions in our select statement? Through a process
called _parsing_, Postgres converts our SQL string into an internal data
structure it understands, the parse tree.

It turns out that Postgres uses the same parsing technology that Ruby does, a
_parser generator_ called [Bison](http://www.gnu.org/software/bison/). Bison
runs during the Postgres C build process and generates parser code based on a
series of grammar rules. The generated parser code is what runs inside of
Postgres when we send it SQL commands. Each grammar rule is triggered when the
generated parser finds a corresponding pattern or syntax in the SQL string,
and inserts a new C memory structure into the parse tree data structure.

I won’t take the time today to explain how parsing algorithms work in detail.
If you’re interested in that sort of thing, I’d suggest taking a look at my
book [Ruby Under a
Microscope](http://patshaughnessy.net/ruby-under-a-microscope). In Chapter One
I go through a detailed example of the LALR parse algorithm used by Bison and
Ruby. Postgres parses SQL statements in exactly the same way.

Using LLDB and enabling some C logging code, I observed the Postgres parser
produce this parse tree for our Captain Nemo query:

<img src="http://patshaughnessy.net/assets/2014/10/13/parse-tree.png"/>

At the top is a node representing the entire SQL statement, and below that are
child nodes or branches that represent the different portions of the SQL
statement syntax: the target list (a list of columns), the from clause (a list
of tables), the where clause, the sort order and a limit count.

If you want to learn more about how Postgres parses SQL statements, follow the
flow of control down from <span class="code">exec\_simple\_query</span> through
another C function called <span class="code">pg\_parse\_query</span>.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">pg_parse_query</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/postgres_8c.html#a0449a974d1a66a2fcdee8896a0690521">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_parse_query.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_parse_query_bt.png"/>
  </div>
</div>
</p>

As you can see there are many helpful and detailed comments in the Postgres
source code that not only explain what is happening but also point out important
design decisions.

## All That Hard Work For Nothing

The parse tree above should look familiar - it’s almost precisely the same as
the abstract syntax tree (AST) we saw ActiveRecord create earlier. Recall from
the [first half of the
presentation](http://patshaughnessy.net/2014/9/17/20000-leagues-under-activerecord)
ActiveRecord generated our Captain Nemo select statement when we executed this
Ruby query:

<img src="http://patshaughnessy.net/assets/2014/9/17/example1.png"/>

We saw that ActiveRecord internally created an AST when we called methods such
as <span class="code">where</span> and <span class="code">first</span>. Later
(see the [second
post](http://patshaughnessy.net/2014/9/23/how-arel-converts-ruby-queries-into-sql-statements)),
we watched as the Arel gem converted the AST into our example select statement
using an algorithm based on the visitor pattern.

Thinking about this, it’s ironic that the first thing Postgres does with your
SQL statement is convert it from a string back into an AST. Postgres’s parse
process reverses everything ActiveRecord did earlier; all of that hard work the
Arel gem did was for nothing! The only reason for creating the SQL string at
all was to communicate with Postgres over a network connection. Once Postgres
has the string, it converts it back into an AST, which is a much more
convenient and useful way of representing queries.

Learning this you might ask: Is there a better way? Is there some way of
conceptually specifying the data we want to Postgres without writing a SQL
statement? Without learning the complex SQL language or paying the performance
overhead of using ActiveRecord and Arel? It seems like a waste of time to go to
such lengths to generate a SQL string from an AST, just to convert it back to
an AST again. Maybe we should be using a NoSQL database solution instead?

Of course, the AST Postgres uses is much different from the AST used by
ActiveRecord. ActiveRecord’s AST was comprised of Ruby objects, while
Postgres’s AST is formed of a series of C memory structures. Same idea but very
different implementations.

## Analyze and Rewrite

Once Postgres has generated a parse tree, it then converts it into a another
tree structure using a different set of nodes. This is known as the _query
tree_. Returning to the <span class="code">exec\_simple\_query</span> C
function, you can see it next calls another C function <span
class="code">pg\_analyze\_and\_rewrite</span>.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">pg_analyze_and_rewrite</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/postgres_8c.html#a66930c41c305d22f3371cad134fd3dee">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_analyze_and_rewrite.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_analyze_and_rewrite_bt.png"/>
  </div>
</div>
</p>

Waving my hands a bit and glossing over many important details, the analyze and
rewrite process applies a series of sophisticated algorithms and heuristics to
try to optimize and simplify your SQL statement. If you had executed a complex
select statement with sub-selects and multiple inner and outer joins, then
there is a lot of room for optimization. It’s quite possible that Postgres
could reduce the number of sub-select clauses or joins to produce a simpler
query that runs faster.

For our simple select statement, here’s the query tree that <span
class="code">pg\_analyze\_and\_rewrite</span> produces:

<img src="http://patshaughnessy.net/assets/2014/10/13/query-tree.png"/>

I don’t pretend to understand the detailed algorithms behind <span
class="code">pg\_analyze\_and\_rewrite</span>. I simply observed that for our
example the query tree largely resembled the parse tree. This means the
select statement was so straightforward Postgres wasn’t able to simplify it further.

## Plan

The last step Postgres takes before starting to execute our query is to create
a plan. This involves generating a third tree of nodes that form a list of
instructions for Postgres to follow. Here’s the plan tree for our select
statement.

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/plan-tree.png"/>

Imagine that each node in the plan tree is a machine or worker of some
kind. The plan tree resembles a pipeline of data or a conveyor belt in a
factory. In my simple example there is only one branch in the tree. Each node
in the plan tree takes some the output data from the node below, processes it,
and returns results as input to the node above. We’ll follow Postgres as it
executes the plan in the next section.

The C function that starts the query planning process is called <span
class="code">pg\_plan\_queries</span>.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">pg_plan_queries</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/postgres_8c.html#a34e18d3874224b3b670ec0a3ae9c970c">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_plan_queries.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/pg_plan_queries_bt.png"/>
  </div>
</div>
</p>

Note the <span class="code">startup\_cost</span> and <span
class="code">total\_cost</span> values in each plan node. Postgres uses these
values to estimate how long the plan will take to complete. You don’t have
to use a C debugger to see the execution plan for your query. Just prepend the
SQL <span class="code">EXPLAIN</span> command to your query, like this:

<img src="http://patshaughnessy.net/assets/2014/10/13/explain.png"/>

This is a powerful way to understand what Postgres is doing internally with one
of your queries, and why it might be slow or inefficient - despite the
sophisticated planning algorithms in <span class="code">pg\_plan\_queries</span>.

## Executing a Limit Plan Node

By now, Postgres has parsed your SQL statement and converted it back into an
AST. Then it optimized and rewrote your query, possibly in a simpler way.
Third, Postgres wrote a plan which it will follow to find and return the data
you are looking for. Finally it’s time for Postgres to actually execute your
query. How does it do this? It follows the plan, of course!

Let’s start at the top of the plan tree and move down. Skipping the root node,
the first worker that Postgres uses for our Captain Nemo query is called Limit.
The Limit node, as you might guess, implements the <span
class="code">LIMIT</span> SQL command, which limits the result set to the
specified number of records. The same plan node also implements the <span
class="code">OFFSET</span> command, which starts the result set window at the
specified row.

<img src="http://patshaughnessy.net/assets/2014/10/13/limit1.png"/>

The first time Postgres calls the Limit node, it calculates what the limit and
offset values should be, because they might be set to the result of some
dynamic calculation. In our example, offset is 0 and limit is 1.

Next, the Limit plan node repeatedly calls the subplan, in our case Sort,
counting until it reaches the offset value:

<img src="http://patshaughnessy.net/assets/2014/10/13/limit2.png"/>

In our example the offset value is zero, so this loop will load the first data
value and stop iterating. Then Postgres returns the last data value loaded from
the subplan to the calling or upper plan. For us, this will be that first value
from the subplan.

Finally when Postgres continues to call the Limit node, it will pass the data
values through from the subplan one at a time:

<img src="http://patshaughnessy.net/assets/2014/10/13/limit3.png"/>

In our example, because the limit value is 1 Limit will immediately return NULL
indicating to the upper plan there is no more data available.

Postgres implements the Limit node using code in a file called nodeLimit.c

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">ExecLimit</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/nodeLimit_8c.html#a9fe32874f36f4a955f5b4b762d814631">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_limit.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_limit_bt.png"/>
  </div>
</div>
</p>

You can see the Postgres source code uses words such as _tuple_ (a set a
values, one from each column) and _subplan_. The subplan in this example is the
Sort node, which appears below Limit in the plan.

## Executing a Sort Plan Node

Where do the data values Limit filters come from? From the Sort plan node
that appears under Limit in the plan tree. Sort loads data values from its
subplan and returns them to its calling plan, Limit. Here’s what Sort does when
the Limit node calls it for the first time, to get the first data value:

<img src="http://patshaughnessy.net/assets/2014/10/13/sort1.png"/>

You can see that Sort functions very differently from Limit. It immediately
loads all of the available data from the subplan into a buffer, before
returning anything. Then it sorts the buffer using the
[Quicksort](http://en.wikipedia.org/wiki/Quicksort) algorithm, and finally
returns the first sorted value.

For the second and subsequent calls, Sort simply returns additional values from
the sorted buffer, and never needs to call the subplan again:

<img src="http://patshaughnessy.net/assets/2014/10/13/sort2.png"/>

The Sort plan node is implemented by a C function called ExecSort:

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">ExecSort</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/nodeSort_8c.html#afe145ec8ff9b3d3a654022f73eab2810">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_sort.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_sort_bt.png"/>
  </div>
</div>
</p>

## Executing a SeqScan Plan Node

Where does ExecSort get its values? From its subplan, or the SeqScan node that
appears at the bottom of the plan tree. SeqScan stands for _sequence scan_, which
means to look through the values in a table, returning values that match a
given filter. To understand how the scan works with our filter, let’s step
through an imaginary users table filled with fake names, looking for Captain
Nemo.

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan1.png"/>

Postgres starts at the first record in a table (known as a _relation_ in the
Postgres source code) and executes the boolean expression from the plan tree.
In simple terms, Postgres asks the question: “Is this Captain Nemo?” Because
Laurianne Goodwin is not Captain Nemo, Postgres steps down to the next record.

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan2.png"/>

No, Candace is also not Captain Nemo. Postgres continues:

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan3.png"/>

… and eventually finds Captain Nemo!

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan4.png"/>

Postgres implements the SeqScan node using a C function called ExecSeqScan.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">ExecSeqScan</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/nodeSeqscan_8c.html#af80d84501ff7621d2ef6249b148e7f44">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_seq_scan.png"/>
  </div>
  <div class="function-bt">
    <img src="http://patshaughnessy.net/assets/2014/10/13/exec_seq_scan_bt.png"/>
  </div>
</div>
</p>

## What Are We Doing Wrong?

Now we’re done! We’ve followed a simple select statement all the way through
the guts of Postgres, and have seen how it was parsed, rewritten, planned and finally
executed. After executing many thousands of lines of C code, Postgres has found the
data we are looking for! Now all Postgres has to do is return the Captain Nemo
string back to our Rails application and ActiveRecord can create a Ruby object.
We can finally return to the surface of our application.

But Postgres doesn’t stop! Instead of simply returning, Postgres continues to
scan through the users table, even though we’ve already found Captain Nemo:

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan5.png"/>

<div style="float: right; padding: 117px 0px 70px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/10/13/suffocating.png"><br/>
  <i>While returning from the South Pole, the air<br/> supply inside the Nautilus began to run out.</i>
</div>

What’s going on here? Why is Postgres wasting its time, continuing to search
even though it’s already found the data we’re looking for?

The answer lies farther up the plan tree in the Sort node. Recall that in order
to sort all of the users, ExecSort first loads all of the values into a buffer,
by calling the subplan repeatedly until there are no values left. That means
that _ExecSeqScan will continue to scan to the end of the table_, until it has
all of the matching users. If our users table contained thousands or even
millions of records (imagine we work at Facebook or Twitter), ExecSeqScan will
have to loop over every single user record and execute the string comparison
for each one. This is obviously inefficient and slow, and will get slower as
more and more user records are added.

If we have only one Captain Nemo record, then ExecSort will “sort” just that
single matching record, and ExecLimit will pass that single record through its
offset/limit filter… but only after ExecSeqScan has iterated over all of the
names.

## Next Time

How do we fix this problem? What should we do if our SQL queries on the users
table take more and more time to execute? The answer is simple: we  create an
index.

In the next and final post in this series we’ll learn how to create a Postgres
index and to avoid the use of ExecSeqScan. More importantly, I’ll show you what
a Postgres index looks like: _how_ it works and _why_ it speeds up queries like
this one.
