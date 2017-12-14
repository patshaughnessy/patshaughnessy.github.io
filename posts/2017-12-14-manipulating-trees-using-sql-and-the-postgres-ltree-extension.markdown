title: "Manipulating Trees Using SQL and the Postgres LTREE Extension"
date: 2017/12/14

<div style="float: right; padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2017/12/14/tree4.jpg">
</div>

[Yesterday](http://patshaughnessy.net/2017/12/13/saving-a-tree-in-postgres-using-ltree),
I used the [LTREE](https://www.postgresql.org/docs/current/static/ltree.html)
extension to save a tree data structure in a Postgres table. After saving the
tree, I used the <span class="code">@></span> or ancestor operator to count the
number of descendant nodes on a given branch.

But that’s not all LTREE can do. Today I’ll show you how to delete, move and
copy branches from one place to another in your tree, using <span
class="code">@></span> in combination with other LTREE functions. After that,
in my last post in this series, I’ll look at how LTREE works under the hood, at
the Computer Science that makes all of this possible.

## My Example Tree Again

Here’s the tree I’ve been working with during the last few blog posts:

<img src="http://patshaughnessy.net/assets/2017/12/11/example-tree.png">

[In my last
post](http://patshaughnessy.net/2017/12/13/saving-a-tree-in-postgres-using-ltree),
I saved this tree in my database using a series of insert statements:

<pre>
insert into tree (letter, path) values ('A', 'A');
insert into tree (letter, path) values ('B', 'A.B');
insert into tree (letter, path) values ('C', 'A.C');
insert into tree (letter, path) values ('D', 'A.C.D');
insert into tree (letter, path) values ('E', 'A.C.E');
insert into tree (letter, path) values ('F', 'A.C.F');
insert into tree (letter, path) values ('G', 'A.B.G');
</pre>

And we saw how easy it is to count the number of tree nodes in a given branch
using the <span class="code">@></span> operator:

<pre>
select count(*) from tree where 'A.C' @> path;
</pre>

## Cutting Off a Branch

But suppose I wanted to remove these nodes from the tree entirely; that is,
suppose I wanted to “cut off this branch” of the tree, so to speak:

<img src="http://patshaughnessy.net/assets/2017/12/14/cut-branch.png"/>

How can I do this? Simple! I just use a SQL delete statement:

<pre>
delete from tree where 'A.C' @> path;
</pre>

As you can see, I can use <span class="code">@></span> equally well in delete
statements as in select statements.

## Replanting a Branch as a New Tree

Now suppose I want to keep this branch, and save it as a separate tree in my
table. That is, I want two trees: the original <span class="code">A</span> tree
and a new tree consisting of the <span class="code">C</span> branch “replanted”
as a new root:

<img src="http://patshaughnessy.net/assets/2017/12/14/replanting.png"/>

Thinking about this for a moment, moving some nodes from one location to
another in my tree means I’ll need to update their path values somehow in my
table. That is, I’ll need to use an update statement and not a select or delete
statement.  But how? Writing an update statement is easy enough, but how do I
know what the new path of each tree node will be? Let’s take <span
class="code">C</span> as an example. Because <span class="code">C</span> will
become the root node of my new tree, I want to change its path from <span
class="code">A.C</span> to just <span class="code">C</span>:

<pre>
update tree set path = 'C' where path = 'A.C';
</pre>

And I will want to update <span class="code">D</span>, one of <span
class="code">C</span>’s children, in a similar way:

<pre>
update tree set path = 'C.D' where path = 'A.C.D';
</pre>

I could write a separate update statement for each node, just 4 SQL statements
for my example. But imagine I had 100s or 1000s of nodes in my tree. Updating
the records one SQL statement at a time would require repeated network
connections from my application to Postgres, slowing down the overall operation
tremendously.

Instead, I need to update the path of <span class="code">C</span> and each of
its descendants all in a single operation. But how can I do this? Two LTREE
functions, <span class="code">NLEVEL()</span> and <span
class="code">SUBPATH()</span>, can help.

## The NLEVEL Function

First, <span class="code">NLEVEL</span>. As you might guess, this returns the
number of levels in a given path string:

<pre>
select letter, path, nlevel(path) from tree;

letter | path  | nlevel 
-------+-------+--------
A      | A     |      1
B      | A.B   |      2
C      | A.C   |      2
D      | A.C.D |      3
E      | A.C.E |      3
F      | A.C.F |      3
G      | A.B.G |      3
(7 rows)
</pre>

Looking at this, it’s easy to understand what the function returns: For a root
node like <span class="code">A</span>, <span class="code">NLEVEL</span> returns
1. For <span class="code">A</span>’s child nodes, <span class="code">A.B</span>
and <span class="code">A.C</span>, <span class="code">NLEVEL</span> returns 2,
and for the grandchild nodes it returns 3. It simply counts the number of
levels in each path string; internally, it parses the path string for period
characters.

Before we continue, consider one subtle but important point. Notice that I was
able to calculate <span class="code">NLEVEL</span> on _all of the records_ in
the tree table with a single SQL statement! Postgres applied the function to
all of the matching paths for me. The power of LTREE’s functions is that they
seamlessly integrate with SQL, harnessing and extending the power of Postgres.

## The SUBPATH Function

LTREE provides another new SQL function that will also help us write a general
tree path formula: <span class="code">SUBPATH</span>. As you might guess, this
returns a selected substring from a given path. Let’s try running it on my
example tree:

<pre>
select letter, subpath(path, 1) from tree;
ERROR:  invalid positions
STATEMENT:  select letter, subpath(path, 1) from tree;
</pre>

Oops - I’ve done something wrong here. Calling <span class="code">SUBPATH(path,
1)</span> returns the portion of the path starting with offset 1. Not a
character offset, but a _path offset_. So <span class="code">SUBPATH(path,
1)</span> drops the first level of the path, <span class="code">A</span> in my
tree, and returns the remaining portion of each path starting from the second
path element. Internally, LTREE parses the periods for me, drops the requested
number of path levels and removes the extra leading period.

In the statement above, the error was caused by the root node in the tree:
<span class="code">A</span>.  This path has only one level, and so LTREE
returns an error in this case.

Let’s try using <span class="code">SUBPATH</span> only on the <span
class="code">C</span> branch, the branch we want to move:

<pre>
select letter, subpath(path, 1) from tree where path <@ 'A.C';
letter | subpath 
-------+---------
C      | C
D      | C.D
E      | C.E
F      | C.F
(4 rows)
</pre>

Now I get only four records in the result, one for <span class="code">C</span>
and one for each node that appears under <span class="code">C</span>. And you
can see the <span class="code">subpath</span> column contains the portion of
the path that appears after <span class="code">A</span>, for each of these 4
records.

And again, notice that I was able to execute the <span
class="code">SUBPATH</span> function on all 4 tree records I wanted to, in a
single operation. This time, the <span class="code">SUBPATH</span> function
worked in concert with the <span class="code"><@</span> operator. LTREE has
made the SQL language I already know how to use even more powerful.

## Moving Tree Nodes Using One UPDATE Statement

Now let’s return to the question of moving a branch into a new tree. As this
diagram shows, I want to delete <span class="code">C</span> and its children
from the <span class="code">A</span> tree, and move them to a new location:

<img src="http://patshaughnessy.net/assets/2017/12/14/replanting.png"/>

Earlier I considered moving the nodes using a single update statement for each:

<pre>
update tree set path = 'C' where path = 'A.C';
update tree set path = 'C.D' where path = 'A.C.D';
update tree set path = 'C.E' where path = 'A.C.E';
update tree set path = 'C.F' where path = 'A.C.F';
</pre>

Now that I know about <span class="code">SUBPATH</span>, it’s easy to write a
single SQL update statement that will move all 4 nodes in the <span
class="code">C</span> branch in one operation:

<pre>
update tree set path = subpath(path, 1) where path <@ 'A.C';
</pre>

I use <span class="code">where path <@ &apos;A.C'</span> to scope the update to the
<span class="code">C</span> branch, and I use <span class="code">subpath(path,
1)</span> to remove the <span class="code">A</span> root element from the path
of <span class="code">C</span> and each of its descendants.

I can generalize this a bit more using the <span class="code">NLEVEL</span>
function also:

<pre>
update tree set path = subpath(path, nlevel('A.C')-1) where path <@ 'A.C';
</pre>

This follows because <span class="code">nlevel(&apos;A.C') = 2</span>, and
therefore, <span class="code">nlevel(&apos;A.C')-1</span> returns the same formula
we had above. Replacing <span class="code">A.C</span> with “BRANCH_PATH” I
arrive at a general formula for “replanting” a branch as a new tree using a
single SQL statement:

<pre>
update tree set path = subpath(path, nlevel(BRANCH_PATH)-1) where path <@ BRANCH_PATH
</pre>

…assuming <span class="code">nlevel(BRANCH_PATH) > 1</span>, that is assuming
the branch we want to replant isn’t already a root.

## The || Concatenation Operator

This seems somewhat useful, but what if I want to move a branch from one
location in my tree to some other location, not necessary to the root? This is
a more general problem. For example, suppose I want to move the <span
class="code">C</span> branch under <span class="code">G</span>, like this:

<img src="http://patshaughnessy.net/assets/2017/12/14/moving.png"/>

To write a formula for this transformation using SQL, we need to use one more
important LTREE operator: the <span class="code">||</span> or concatenation
operator. Let’s try it out with an example first:

<pre>
select 'A.B.G' || path as concatenated from tree;
concatenated 
--------------
A.B.G.A
A.B.G.A.B
A.B.G.A.C
A.B.G.A.C.D
A.B.G.A.C.E
A.B.G.A.C.F
A.B.G.A.B.G
(7 rows)
</pre>

You can see LTREE has automatically added <span class="code">A.B.G</span> along
with a period separator to each path in my table. And it has done this for all
the paths in my table in a single operation.

## Moving a Branch

Now using <span class="code">||</span> I can write a single SQL statement to
move a tree branch from one location to another. First, of course, I need to
scope the SQL operation to the target branch using the ancestor operator:

<pre>
select 'A.B.G' || path as concatenated from tree where path <@ 'A.C';
concatenated 
---------------
A.B.G.A.C
A.B.G.A.C.D
A.B.G.A.C.E
A.B.G.A.C.F
(4 rows)
</pre>

I get the same results as above, but now only for the tree nodes I want to
move.

But my next problem is the new paths above start with <span
class="code">A.B.G.A.C</span>…. Instead, I want them to be <span
class="code">A.B.G.C</span>…. I need to remove that extra <span
class="code">A</span> character from the new paths, using the <span
class="code">SUBPATH</span> operator:

<pre>
select 'A.B.G' || subpath(path, 1) as concatenated from tree where path <@ 'A.C';
concatenated 
--------------
A.B.G.C
A.B.G.C.D
A.B.G.C.E
A.B.G.C.F
(4 rows)
</pre>

And finally, converting this into an update statement:

<pre>
update tree set path = 'A.B.G' || subpath(path, 1) where path <@ 'A.C'
</pre>

…I have the single SQL statement I need!

And generalizing this, we arrive at a SQL formula you could use in your own
Postgres database:

<pre>
update tree set path = DESTINATION_PATH || subpath(path, nlevel(SOURCE_PATH)-1)
where path <@ SOURCE_PATH;
</pre>

## Copying a Branch

One last puzzle: How can I copy a tree branch instead of moving it? I just use
an insert SQL statement instead of update. Simple, right?

But how, exactly? I need to insert multiple rows, one record for each node in
the branch I copy. Again, I could write a series of insert statements like
this:

<pre>
insert into tree (letter, path) values ('C', 'A.B.G.C');
insert into tree (letter, path) values ('D', 'A.B.G.C.D');
insert into tree (letter, path) values ('E', 'A.B.G.C.E');
insert into tree (letter, path) values ('F', 'A.B.G.C.F');
</pre>

But using LTREE functions and operators, I can achieve this using a single SQL
statement! I just have to write SQL that will insert the result of a select,
like this:

<pre>
insert into tree (letter, path) (
    select letter, 'A.B.G' || subpath(path, 1) from tree where 'A.C' @> path
)
</pre>

Executing this, Postgres will first find all the nodes inside the branch I want
to copy, and recalculate their paths. Then it will insert that result set into
the tree as a copy, leaving my original branch unchanged!

By writing this tree-related logic using LTREE operators in SQL, I ask Postgres
to do all of the hard work of manipulating and copying the path strings for me.
I don’t have to write application code to keep track of these strings, and no
data needs to be transmitted back and forth between my application server and
the database server.

## What’s Next? LTREE Internals

In my last post about LTREE, I’ll look closely at how it works internally. It’s
easy enough to imagine how simple functions like <span
class="code">NLEVEL</span>, || or <span class="code">SUBPATH</span> work.
That’s not the interesting part for me. These functions are shorthand for
fairly simple string operations.

The special sauce that makes LTREE such a powerful tool is that it integrates
with Postgres GiST indexes. By using an index, Postgres can execute any of the
SQL expressions I wrote above equally fast on 7000 records as it would on 7!
How? The only way to find out is by Looking Inside Postgres at a GiST Index.
