title: "Saving a Tree in Postgres Using LTREE"
date: 2017/12/13
tag: the Postgres LTREE Extension

<div style="float: right; padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2017/12/13/tree3.jpg">
</div>

In [my last
post](http://patshaughnessy.net/2017/12/12/installing-the-postgres-ltree-extension),
I showed you how to install and enable a Postgres extension called
[LTREE](https://www.postgresql.org/docs/current/static/ltree.html). LTREE
allows me to save, query on and manipulate trees or hierarchical data
structures using a relational database table. [As we’ll see](http://patshaughnessy.net/2017/12/14/manipulating-trees-using-sql-and-the-postgres-ltree-extension), using LTREE I can
count leaves, cut off branches, and climb up and down trees easily - all using
SQL right inside my application’s existing Postgres database!

But trees are natural, haphazard, branching structures with countless leaves,
while database tables are man-made rectangles full of numbers and text. How can
I possibly save a beautiful tree structure into an ugly, boring database table?

## Path Enumeration

Let’s return to the example tree from the [first
post](http://patshaughnessy.net/2017/12/11/trying-to-represent-a-tree-structure-using-postgres)
in this series:

<img src="http://patshaughnessy.net/assets/2017/12/11/example-tree.png">

The LTREE extension uses the _path enumeration_ algorithm, which calls for each
node in the tree to record the path from the root you would have to follow to
reach that node.

For example, to find <span class="code">G</span> starting from <span
class="code">A</span>, the root, I would move down to <span
class="code">B</span>, and then down again to <span class="code">G</span>:

<img src="http://patshaughnessy.net/assets/2017/12/13/tree-path1.png">

So the path to <span class="code">G</span> is:

<img src="http://patshaughnessy.net/assets/2017/12/13/path1.png">

Here’s another example:

<img src="http://patshaughnessy.net/assets/2017/12/13/tree-path2.png">

This time I’ve traced a path from <span class="code">A</span> to <span
class="code">D</span>, via <span class="code">C</span>. So the path of <span
class="code">D</span> is:

<img src="http://patshaughnessy.net/assets/2017/12/13/path2.png">

## Saving Tree Paths Using LTREE

To use LTREE, I need to create a column to hold these paths. For my example
tree, I’ll use the same table I did before, but instead of the <span
class="code">parent_id</span> column I’ll use a <span class="code">path</span>
column:

<pre>
create table tree(
    id serial primary key,
    letter char,
    path ltree
);
create index tree_path_idx on tree using gist (path);
</pre>

I chose the name <span class="code">path</span>; I could have picked any name
here. However, notice the <span class="code">path</span> column uses a Postgres
data type called <span class="code">ltree</span> - the LTREE extension provides
this special new type.  And also notice I created a special <span
class="code">gist</span> index on the <span class="code">path</span> column;
more on this later!

Next, I save the path of each tree node into the <span class="code">path</span>
column, encoded as a series of strings joined together by periods. For example
to save the path of <span class="code">G</span> into my table I use this insert
statement:

<img src="http://patshaughnessy.net/assets/2017/12/13/insert1.png">

And to save the path to node D I write:

<img src="http://patshaughnessy.net/assets/2017/12/13/insert2.png">

Following this pattern, I can save my entire tree using these insert
statements, one for each node in my tree:

<pre>
insert into tree (letter, path) values ('A', 'A');
insert into tree (letter, path) values ('B', 'A.B');
insert into tree (letter, path) values ('C', 'A.C');
insert into tree (letter, path) values ('D', 'A.C.D');
insert into tree (letter, path) values ('E', 'A.C.E');
insert into tree (letter, path) values ('F', 'A.C.F');
insert into tree (letter, path) values ('G', 'A.B.G');
</pre>

The root node, <span class="code">A</span>, contains the simplest path <span
class="code">A</span>. Its two child nodes, <span class="code">B</span> and
<span class="code">C</span>, use paths <span class="code">A.B</span> and <span
class="code">A.C</span>; the child nodes under <span class="code">C</span> use
paths <span class="code">A.C.D</span>, <span class="code">A.C.E</span>, etc.
You get the idea.

## The Ancestor Operator: @>

Now for the fun part: LTREE provides a series of new SQL operators that allow
me to query and manipulate tree data structures. The most powerful of these is
<span class="code">@></span>, the “ancestor” operator. It tests whether one path is an ancestor of
another.

Returning to my question from [the first post in this
series](http://patshaughnessy.net/2017/12/11/trying-to-represent-a-tree-structure-using-postgres),
what if I needed to know how many children <span class="code">A</span> had,
recursively? That is, what if I needed to count its children, grandchildren,
great-grandchildren, etc.? Earlier we saw that using a <span
class="code">parent_id</span> column this would require an ever increasing
number of SQL statements: 

<pre>
select count(*) from tree where parent_id = ID;
select count(*) from tree where parent_id in (CHILD IDs);
select count(*) from tree where parent_id in (GRANDCHILD IDs);
select count(*) from tree where parent_id in (GREAT-GRANDCHILD IDs);
select count(*) from tree where parent_id in (GREAT_GREAT-GRANDCHILD IDs);

etc.
</pre>

<span class="code">@></span> solves this problem for us. I can now recursively
count the total number of nodes under any given parent like this:

<pre>
select count(*) from tree where PARENT-PATH @> path;
</pre>

In my example, this SQL would return the number of nodes, recursively, under
the root node <span class="code">A</span>:

<pre>
select count(*) from tree where 'A' @> path;
count 
-------
7
(1 row)
</pre>

LTREE counts the parent node itself, so the total count is 7, not 6. That is,
<span class="code">A @> A</span> evaluates to true. Another example; this
returns the count of tree nodes under and including <span class="code">C</span>:

<pre>
select count(*) from tree where ‘A.C' @> path;
count 
-------
4
(1 row)
</pre>

Or I could have written these predicates in the opposite order using <span
class="code"><@</span>:

<pre>
select count(*) from tree where path <@ 'A';
select count(*) from tree where path <@ 'A.C';
</pre>

As you can see, the <span class="code"><@</span> and <span
class="code">@></span> operators treat the <span class="code">path</span>
column, the column I defined with the <span class="code">ltree</span> data
type, as simple strings. But there’s some magic going on here: The path values
are not simple strings. Although I typed them in as strings, <span
class="code"><@</span> and <span class="code">@></span> efficiently determine
whether or not one path is an ancestor of another.

And the <span class="code">@></span> ancestor operator is just one way of using
<span class="code">ltree</span> columns; the LTREE extension provides a long list of powerful
operators and functions!  For a complete list, see
[https://www.postgresql.org/docs/current/static/ltree.html](https://www.postgresql.org/docs/current/static/ltree.html).

In [my next post](http://patshaughnessy.net/2017/12/14/manipulating-trees-using-sql-and-the-postgres-ltree-extension), I’ll explore more of these functions and show you how to
perform some tree operations that I’ve found useful.

## Maybe You’re Not Impressed

However, thinking about the path strings for a moment, it’s fairly obvious
whether one path is an ancestor of another. For example, it’s clear that <span class="code">A</span> and
A.C are ancestors of A.C.D, while A.B is not. In fact, it looks like all the <span class="code">@></span>
operator does it check whether the string on the left (the ancestor) is a
prefix or leading substring inside the string on the right (the descendant).

In fact, you might not be very impressed by LTREE, so far. The <span class="code">@></span> operator
seems like a fancy way of performing a simple string operation. I could have
written SQL code to determine that A is an ancestor of A.C.D myself. I probably
would have used one of Postgres’s [many string
functions](https://www.postgresql.org/docs/current/static/functions-string.html)
to achieve this, maybe something like this:

<pre>
select count(*) from tree where strpos(path::varchar, 'A') = 1
</pre>

Postgres would calculate the answer for my 7-node example tree very quickly.
But to calculate this count, internally Postgres would have to iterate over all
the records in my table (this is called a _full table scan_ or _sequence scan_ in
DB jargon) and calculate the <span class="code">strpos</span> function on each
row. If my tree had thousands or millions of rows, then this SQL statement
would take a long time to finish.

## Enabling the Real Magic: Using a GiST Index with LTREE

The power of the <span class="code">@></span> operator is that it allows
Postgres to search _efficiently_ across an entire tree using an index. Saying
this in a more technical way: The <span class="code">@></span> operator
integrates with Postgres’s GiST index API to find and match descendant nodes.
To take advantage of this technology, be sure to create a GiST index on your
<span class="code">ltree</span> column, for example like this:

<pre>
create index tree_path_idx on tree using gist (path);
</pre>

What is a “GiST” index? How does it help LTREE find and count tree nodes
efficiently? Read the [last post in this
series](http://patshaughnessy.net/2017/12/15/looking-inside-postgres-at-a-gist-index)
to find out. There I describe the Generalized Search Index (GiST) project,
explore the Computer Science behind GiST and look at how LTREE uses GiST to
make fast tree operations inside of Postgres possible.

## What’s Next?

But before we dive into LTREE’s internal implementation, first we should see
what else LTREE can do. So far I’ve shown you how to count descendant tree
nodes. Tomorrow in my next post, [Manipulating Trees Using SQL and the Postgres
LTREE
Extension](http://patshaughnessy.net/2017/12/14/manipulating-trees-using-sql-and-the-postgres-ltree-extension),
I’ll show you how to use other LTREE’s operators and functions to work with
tree data.


