title: "Trying to Represent a Tree Structure Using Postgres"
date: 2017/12/11

<div style="float: left; padding: 8px 30px 40px 0px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2017/12/11/tree1.jpg">
</div>

Suppose you had a hierarchical data structure in your application - how would
you save it in a database? How would you represent a complex tree structure
using flat rows and columns?

There are a few different, equally valid options. In this series of blog posts,
I’ll take a close look at one option that Postgres provides, the [LTREE
extension](https://www.postgresql.org/docs/current/static/ltree.html). If you
install and enable LTREE on your Postgres server, it will add powerful SQL
operators and functions that support tree operations.

But what are these new SQL operators, and how do you use them? And how does
LTREE actually work? What Computer Science does it use behind the scenes to
enable fast tree operations?

This week I’ll publish a series of blog posts on the Postgres LTREE extension.
I'll get started today by trying to insert a tree structure into a Postgres
table using standard SQL, and during the rest of the week I'll take a close
look at LTREE: how to install it, how to use it, and how it works.

<div style="clear: both"></div>

## An Example Tree

My actual data set was more complicated, of course, but for the sake of example
let’s suppose I needed to save this tree in a Postgres table:

<img src="http://patshaughnessy.net/assets/2017/12/11/example-tree.png">

There are many different tree-like data structures in Computer Science, but
this is probably the simplest: no cycles, no ordering of child nodes, and all
the child nodes are accessible moving down from a single root. Should be easy,
right?

At first, I thought it would be. I started by creating a table with a
<span class="code">parent\_id</span> foreign key column, like this:

<pre>
create table tree(
    id serial primary key,
    letter char,
    parent_id integer references tree (id)
);
</pre>

The idea was that each row in my table represented a single node or element of
the tree, and would identify its parent using the <span class="code">parent\_id</span> column. My single
root node, <span class="code">A</span>, had no parent so I saved it first with
a <span class="code">NULL</span> parent id:

<pre>
insert into tree (letter, parent_id) values ('A', null);

select * from tree;

id  | letter | parent_id 
----+--------+-----------
1   | A      |          
(1 row)
</pre>

And then I inserted each of its child nodes like this:

<pre>
insert into tree (letter, parent_id) values ('B', 1);
insert into tree (letter, parent_id) values ('C', 1);

select * from tree;

id  | letter | parent_id 
----+--------+-----------
1   | A      |          
2   | B      |         1
3   | C      |         1
(3 rows)
</pre>

Because <span class="code">A</span> has <span class="code">id</span>=1, I set
<span class="code">parent\_id</span>=1 for <span class="code">B</span> and
<span class="code">C</span>. This is a simple example of the _adjacency list_
algorithm: each row contains a list of its neighbors or adjacent rows. In this
case I was only recording each row’s parent. And the table is _self-referencing_
because it contains a foreign key (<span class="code">parent\_id</span>)
referencing another column in the same table.

I continued to fill out my tree structure with a few more insert statements:

<pre>
insert into tree (letter, parent_id) values ('D', 3);
insert into tree (letter, parent_id) values ('E', 3);
insert into tree (letter, parent_id) values ('F', 3);
insert into tree (letter, parent_id) values ('G', 2);

select * from tree;

id  | letter | parent_id 
----+--------+-----------
1   | A      |          
2   | B      |         1
3   | C      |         1
4   | D      |         3
5   | E      |         3
6   | F      |         3
7   | G      |         2
(7 rows)
</pre>

## Did My Postgres Tree Work?

At first glance, my data structure worked well. I could easily find the parent
of <span class="code">E</span>:

<pre>
select parent_id from tree where letter = 'E'

parent_id 
-----------
3
(1 row)

select letter from tree where id = 3

letter 
--------
C
(1 row)
</pre>

And the children of <span class="code">C</span> like this:

<pre>
select letter from tree where parent_id = 3

letter 
--------
D
E
F
(3 rows)
</pre>

## Recursive Tree Operations

And it was also very easy to count how many children each node had, for example
this SQL statement returns the number of children under <span class="code">A</span>:

<pre>
select count(*) from tree where parent_id = 1;

count 
-------
2
(1 row)
</pre>

But what if I needed to know how many children <span class="code">A</span> had,
recursively? That is, what if I needed to count its children, grandchildren,
great-grandchildren, etc.?

Well, first I would have to find the direct children of <span class="code">A</span>:

<pre>
select id from tree where parent_id = 1;

id 
----
2
3
(2 rows)
</pre>

Then to find the grandchildren I would need to query for the children of the
children, inserting the id values from the previous statement:

<pre>
select id from tree where parent_id in (2, 3);

id 
----
4
5
6
7
(4 rows)
</pre>

And then I would add the child count with the grandchild count: 2 + 4 = 6.

My example tree ends here, so I’m done. But this doesn’t scale; suppose my tree
had 10, 20 or 100 levels in it. I would have to execute repeated select
statements, stepping down each level of the tree structure under the parent
node:

<pre>
select count(*) from tree where parent_id in (GREAT-GRANDCHILD-IDS);
select count(*) from tree where parent_id in (GREAT-GREAT-GRANDCHILD-IDS);
select count(*) from tree where parent_id in (GREAT-GREAT-GREAT-GRANDCHILD-IDS);
</pre>

etc.

In other words, I need to execute _n_-1 SQL statements, where _n_ is the number of
levels in the tree under the parent node, each time inserting all of the ids
returned by the previous query. And to find the total count I would have to sum
the number of ids returned by each query along the way. Certainly not an
efficient algorithm!

## There must be a better way

My <span class="code">parent\_id</span> foreign key worked well for very simple
tree operations, but not for more complex tasks, such as recursively counting
nodes. If I setup my database schema differently, in a more thoughtful and
complex way, can I avoid the repeated SQL calls?

Yes! There are a variety of options. One common solution is to use a [nested set](https://en.wikipedia.org/wiki/Nested_set_model)
approach. In this design, each row contains a description of the set of other
nodes that appear under it in the tree by saving the maximum and minimum id
values of that subset of rows, the “nested set.” Using this scheme, querying
children and grandchildren recursively becomes very easy. The drawback is that
I would have to recalculate these values up and down the tree each time a new
row was added.

Another solution, _path enumeration_, involves using a column to save the path or
position of each node in the tree. This can be a powerful solution, but
recursive queries and other tree operations require special support to parse
and manipulate these paths.

A completely different approach would be to use a [graph-oriented
database](https://en.wikipedia.org/wiki/Graph_database), such as
[Neo4J](https://neo4j.com). These are database servers designed entirely around
this problem: saving hierarchical data, or more generally networks of related
data values.

But I didn’t want to leave Postgres behind: I already had a working, well
tested application. Why start over from scratch just because I added a single
tree structure to my app? Why add new infrastructure and complexity to my
overall system to support a single new data structure?

It turns out I didn’t have to: Postgres itself supports one of the two tree
algorithms I mentioned above: path enumeration. Bundled inside of the Postgres
source tree is an “extension,” an optional piece of C code you need compile,
install and enable, that supports tree SQL operations using path enumeration.
In my next post, I’ll show you how to install and use the [LTREE Postgres
extension](https://www.postgresql.org/docs/current/static/ltree.html).
