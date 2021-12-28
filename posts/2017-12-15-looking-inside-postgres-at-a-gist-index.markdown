title: "Looking Inside Postgres at a GiST Index"
date: 2017/12/15
tag: the Postgres LTREE Extension

<div style="float: left; padding: 8px 30px 40px 0px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2017/12/15/tree5.jpg"><br/>
  <i> What do Postgres GiST indexes look like? How are<br/>
  they similar or different from standard Postgres indexes?</i>
</div>

In the last few posts in this series 
([one](https://patshaughnessy.net/2017/12/11/trying-to-represent-a-tree-structure-using-postgres),
[two](https://patshaughnessy.net/2017/12/12/installing-the-postgres-ltree-extension),
[three](https://patshaughnessy.net/2017/12/13/saving-a-tree-in-postgres-using-ltree)
and
[four](https://patshaughnessy.net/2017/12/14/manipulating-trees-using-sql-and-the-postgres-ltree-extension))
I showed you how to save hierarchical data in a flat database table using the
Postgres [LTREE
extension](https://www.postgresql.org/docs/current/static/ltree.html). I
explained that you represent tree nodes using path strings and how to search
your tree data using special SQL operators LTREE provides.

But the real value of LTREE isn’t the operators and functions it gives you -
internally these boil down to fairly simple string operations. Instead, what
makes LTREE useful is that it integrates these new operators with Postgres’s
indexing code, which allows you to search for and find matching tree paths
_quickly_. To achieve this, LTREE takes advantage of the [Generalized Search Tree
(GiST) project](http://gist.cs.berkeley.edu), an API that allows C developers
to extend Postgres’s indexing system.

But what does the GiST API do? And what does it mean to extend Postgres’s
indexing system, exactly? Read on to find out!

## Searching Without an Index

Here again is the tree table I used as an example in the earlier posts in this
series:

<pre>
create table tree(
    id serial primary key,
    letter char,
    path ltree
);
</pre>

Note the path column uses the custom <span class="code">ltree</span> data type
the LTREE extension provides. If you missed the previous posts, <span
class="code">ltree</span> columns represent hierarchical data by joining
strings together with periods, e.g.  “A.B.C.D” or “Europe.Estonia.Tallinn.”

Earlier, I used a simple tree with only 7 nodes as an example. SQL operations
on a small table like this will always be fast. Today I’d like to imagine a
much larger tree to explore the benefits indexes can provide; suppose instead I
have a tree containing hundreds or thousands of records in the <span
class="code">path</span> column:

<img src="https://patshaughnessy.net/assets/2017/12/15/table.png"/>

Now suppose I search for a single tree node using a select statement:

<pre>
select letter from tree where path = 'A.B.T.V'
</pre>

Without an index on this table, Postgres has to resort to a _sequence scan_,
which is a technical way of saying that Postgres has to iterate over all of the
records in the table:

<img src="https://patshaughnessy.net/assets/2017/12/15/table2.png"/>

For each and every record in the table, Postgres executes a comparison <span
class="code">p == q</span> where <span class="code">p</span> is the value of
the path column for each record in the table, and <span class="code">q</span>
is the query, or the value I’m searching for, <span class="code">A.B.V.T</span>
in this example. This loop can be very slow if there are many records. Postgres
has to check all of them, because they can appear in any order and there’s no
way to know how many matches there might be in the data ahead of time.

## Searching With a B-Tree Index

Of course, there’s a simple solution to this problem; I just need to create an
index on the path column:

<pre>
create index tree_path_idx on tree (path);
</pre>

As you probably know, executing a search using an index is much faster. If you
see a performance problem with a SQL statement in your application, the first
thing you should check for is a missing index. But why? Why does creating an
index speed up searches, exactly? The reason is that an index is a sorted copy
of the target column’s data:

<img src="https://patshaughnessy.net/assets/2017/12/15/index1.png"/>

By sorting the values ahead of time, Postgres can search through them much more
quickly. It uses a binary search algorithm:

<img src="https://patshaughnessy.net/assets/2017/12/15/index2.png"/>

Postgres starts by checking the value in the middle of the index. If the stored
value (<span class="code">p</span>) is too large and is greater than the query
(<span class="code">q</span>), if <span class="code">p > q</span>, it moves up
and checks the value at the 25% position. If the value is too small, if <span
class="code">p < q</span>, it moves down and checks the value at the 75%
position. Repeatedly dividing the index into smaller and smaller pieces,
Postgres only needs to search a few times before it finds the matching record
or records.

However, for large tables with thousands or millions of rows Postgres can’t
save all of the sorted data values in a single memory segment. Instead,
Postgres indexes (and indexes inside of any relational database system) save
values in a _binary or balanced tree_ (B-Tree):

<img src="https://patshaughnessy.net/assets/2017/12/15/index3.png"/>

Now my values are saved in a series of different memory segments arranged in a
tree structure. Dividing the index up into pieces allows Postgres to manage
memory properly while saving possibly millions of values in the index. Note
this isn’t the tree from my LTREE dataset; B-Trees are internal Postgres data
structures I don’t have access to. To learn more about the Computer Science
behind this read my 2014 article [Discovering the Computer Science Behind
Postgres
Indexes](https://patshaughnessy.net/2014/11/11/discovering-the-computer-science-behind-postgres-indexes).

Now Postgres uses repeated binary searches, one for each memory segment in the
B-Tree, to find a value:

<img src="https://patshaughnessy.net/assets/2017/12/15/index4.png"/>

Each value in the parent or root segment is really a pointer to a child
segment. Postgres first searches the root segment using a binary search to find
the right pointer, and then jumps down to the child segment to find the actual
matching records using another binary search. The algorithm is recursive: The
B-Tree could contain many levels in which case the child segments would contain
pointers to grandchild segments, etc.

## What’s the Problem with Standard Postgres Indexes?

But there’s a serious problem here I’ve overlooked so far. Postgres’s index
search code only supports certain operators. Above I was searching using this
select statement:

<pre>
select letter from tree where path = 'A.B.T.V'
</pre>

I was looking for records that were equal to my query: <span class="code">p ==
q</span>. Using a B-Tree index I could also have searched for records greater
than or less than my query: <span class="code">p < q</span> or <span
class="code">p > q</span>.

But what if I want to use the custom LTREE <span class="code"><@</span>
(ancestor) operator? What if I want to execute this select statement?

<pre>
select letter from tree where path <@ 'A.B.V'
</pre>

As we saw in the previous posts in this series, this search will return all of
the LTREE records that appear somewhere on the branch under <span
class="code">A.B.V</span>, that are descendants of the <span
class="code">A.B.V</span> tree node.

A standard Postgres index doesn’t work here. To execute this search efficiently
using an index, Postgres needs to execute this comparison as it walks the
B-Tree: <span class="code">p <@ q</span>. But the standard Postgres index
search code doesn’t support <span class="code">p <@ q</span>. Instead, if I
execute this search Postgres resorts to a slow sequence scan again, even if I
create an index on the <span class="code">ltree</span> column.

To search tree data efficiently, we need a Postgres index that will perform
<span class="code">p <@ q</span> comparisons equally well as <span
class="code">p == q</span> and <span class="code">p < q</span> comparisons. We
need a GiST index!

## The Generalized Search Tree (GiST) project

<div style="float: right; padding: 8px 0px 0px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2017/12/15/berkeley.jpg"><br/>
  <i>The Generalized Search Tree (GiST) project, like<br/>
Postgres itself, started at UC Berkeley.</i>
</div>

Almost 20 years ago, an open source project at UC Berkeley solved this precise
problem. The [Generalized Search Tree (GiST)
project](http://gist.cs.berkeley.edu) added an API to Postgres allowing C
developers to extend the set of data types that can be used in a Postgres
index.

Quoting from the project’s web page:

<blockquote>
In the beginning there was the B-tree. All database search trees since the
B-tree have been variations on its theme. Recognizing this, we have developed a
new kind of index called a Generalized Search Tree (GiST), which provides the
functionality of all these trees in a single package. The GiST is an extensible
data structure, which allows users to develop indices over any kind of data,
supporting any lookup over that data.
</blockquote>

GiST achieves this by adding an API to Postgres’s index system anyone can implement for their specific data type. GiST implements the general indexing and searching code, but calls out to custom code at four key moments in the indexing process. Quoting from the project’s web page again, here’s a quick explanation of the 4 methods in the GiST API:

<div style="padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2017/12/15/gist.png"/>
</div>

GiST indexes use a tree structure similar to the B-Tree we saw above. But
Postgres doesn’t create the GiST index tree structure by itself; Postgres works
with implementations of the GiST <span class="code">Union</span>, <span
class="code">Penalty</span> and <span class="code">PickSplit</span> API
functions described above. And when you execute a SQL statement that searches
for a value in a GiST index, Postgres uses the <span
class="code">Consistent</span> function to find the target values.

The key here is the implementor of the GiST API can decide what type of data to
index and how to arrange those data values in the GiST tree. Postgres doesn’t
care what the data values are or how the tree looks. Postgres simply calls
<span class="code">Consistent</span> any time it needs to search for a value
and lets the GiST API implementor find the value.

An example would help understand this, and we have an example GiST API
implementation: The LTREE extension!

## Implementing the GiST API for Tree Paths

<div style="float: right; padding: 8px 0px 20px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2017/12/15/moscow-state.jpg"><br/>
  <i>The LTREE Postgres extension was developed at Moscow State<br/>
University by Oleg Bartunov and Teodor Sigaev.</i>
</div>

Starting in around 2001, two students at Moscow State University found the API
from the GiST project and decided to use it to build indexing support for tree
data. Oleg Bartunov and Teodor Sigaev, in effect, wrote a “Tree Paths
Consistent” function, a “Tree Path Union” function, etc. The C code that
implements this API is the LTREE extension. You can find these functions,
<span class="code">ltree_consistent</span> and <span
class="code">ltree_union</span>, among other functions, in a file called
ltree_gist.c, located in the contrib/ltree directory in the Postgres source
code. They also implemented the <span class="code">Penalty</span>, <span
class="code">PickSplit</span> and various other functions related to the GiST
algorithm.

I can use these custom functions on my own data simply by creating a GiST
index. Returning to my LTREE example, I’ll drop my B-Tree index and create a
GiST index instead:

<pre>
drop index tree_path_idx;
create index tree_path_idx on tree using gist (path);
</pre>

Notice the <span class="code">using gist</span> keywords in the <span
class="code">create index</span> command. That’s all it takes; Postgres
automatically finds, loads and uses the <span class="code">ltree_union</span>,
<span class="code">ltree_picksplit</span> etc., functions whenever I insert a
new value into the table. (It will also insert all existing records into the
index immediately.) Of course, earlier I [installed the LTREE
extension](https://patshaughnessy.net/2017/12/12/installing-the-postgres-ltree-extension)
also.

Let’s see how this works - suppose I add a few random tree records to my empty
tree table after creating the index:

<pre>
insert into tree (letter, path) values ('A', 'A.B.G.A');
insert into tree (letter, path) values ('E', 'A.B.T.E');
insert into tree (letter, path) values ('M', 'A.B.R.M');
insert into tree (letter, path) values ('F', 'A.B.E.F');
insert into tree (letter, path) values ('P', 'A.B.R.P');
</pre>

To get things started, Postgres will allocate a new memory segment for the GiST
index and insert my five records:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist1.png"/>

If I search now using the ancestor operator:

<pre>
select count(*) from tree where path <@ 'A.B.T'
</pre>

…Postgres will simply iterate over the records in the same order I inserted
then, and call the <span class="code">ltree_consistent</span> function for each
one. Here again is what the GiST API calls for the Consistent function to do:

<img src="https://patshaughnessy.net/assets/2017/12/15/consistent.png"/>

In this case Postgres will compare <span class="code">p <@ A.B.T</span> for
each of these five records:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist2.png"/>

Because the values of <span class="code">p</span>, the tree page keys, are
simple path strings, <span class="code">ltree_consistent</span> directly
compares them with </span>A.B.T</span> and determines immediately whether each
value is a descendent tree node of <span class="code">A.B.T</span> or not.
Right now the GiST index hasn’t provided much value; Postgres has to iterate
over all the values, just like a sequence scan.

Now suppose I start to add more and more records to my table. Postgres can fit
up to 136 LTREE records into the root GiST memory segment, and index scans
function the same way as a sequence scan by checking all the values.

<img src="https://patshaughnessy.net/assets/2017/12/15/gist3.png"/>

But if I insert one more record, the 137th record doesn’t fit. At this point
Postgres has to do something different:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist4.png"/>

Now Postgres “splits” the memory segment to make room for more values. It
creates two new child memory segments and pointers to them from the parent or
root segment.

<img src="https://patshaughnessy.net/assets/2017/12/15/gist5.png"/>

What does Postgres do next? What does it place into each child segment?
Postgres leaves this decision to the GiST API, to the LTREE extension, by
calling the the <span class="code">ltree_picksplit</span> function. Here again
is the API spec for <span class="code">PickSplit</span>:

<div style="padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2017/12/15/pick-split.png"/>
</div>

The <span class="code">ltree_picksplit</span> function - the LTREE
implementation of the GiST API - sorts the tree paths alphabetically and copies
each half into one of the two new child segments. Note that GiST indexes don’t
normally sort their contents; however, GiST indexes created specifically by the
LTREE extension do because of the way <span class="code">ltree_picksplit</span>
works. We’ll see why it sorts the data in a moment.

<img src="https://patshaughnessy.net/assets/2017/12/15/gist6.png"/>

Now Postgres has to decide what to leave in the root segment. To do this, it
calls the Union GiST API:

<div style="padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
<img src="https://patshaughnessy.net/assets/2017/12/15/union.png"/>
</div>

In this example, each of the child segments is a set S. And the <span
class="code">ltree_union</span> function has to return a “union” value for each
child segment that describes somehow what values are present in that segment:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist7.png"/>

Oleg and Teodor decided this union value should be a pair of left/right values
indicating the minimum and maximum tree branches inside of which all of the
values fit alphabetically. This is why the <span
class="code">ltree_picksplit</span> function sorted the values. For example,
because the first child segment contains the sorted values from <span
class="code">A.B.C.B</span> through <span class="code">A.B.M.Z</span>, the
left/right union becomes <span class="code">A</span> and <span
class="code">A.B.M</span>:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist8.png"/>

Note <span class="code">A.B.M</span> is sufficient here to form a union value
excluding <span class="code">A.B.N.X</span> and all the following values; LTREE
doesn’t need to save <span class="code">A.B.M.Z</span>.

Similarly, the left/right union for the second child segment becomes <span
class="code">A.B.N/A.B.X</span>:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist9.png"/>

This is what a GiST index looks like. Or, what an LTREE GiST index looks like,
specifically. The power of the GiST API is that anyone can use it to create a
Postgres index for any type of data. Postgres will always use the same pattern:
The parent index page contains a set of union values, each of which somehow
describe the contents of each child index page.

For LTREE GiST indexes, Postgres saves left/right value pairs to describe the
union of values that appear in each child index segment. For other types of
GiST indexes, the union values could be anything. For example, a GiST index
could store geographical information like latitude/longitude coordinates, or
colors, or any sort of data at all. What’s important is that each union value
describe the set of possible values that can appear under a certain branch of
the index. And like B-Trees, this union value/child page pattern is recursive:
A GiST index could hold millions of values in a tree with many pages saved in a
large multi-level tree.

## Searching a GiST Index

After creating this GiST index tree, searching for a value is straightforward.
Postgres uses the <span class="code">ltree_consistent</span> function. As an
example, let’s repeat the same SQL query from above:

<pre>
select count(*) from tree where path <@ 'A.B.T'
</pre>

To execute this using the GiST index, Postgres iterates over the union values
in the root memory segment and calls the <span
class="code">ltree_consistent</span> function for each one:

<img src="https://patshaughnessy.net/assets/2017/12/15/consistent.png"/>

Now Postgres passes each union value to <span
class="code">ltree_consistent</span> to calculate the <span class="code">p <@
q</span> formula. The code inside of <span class="code">ltree_consistent</span>
then returns "MAYBE" if <span class="code">q >
left</span>, and <span class="code">q < right</span>. Otherwise it returns
"NO."

<img src="https://patshaughnessy.net/assets/2017/12/15/gist10.png"/>

In this example you can see <span class="code">ltree_consistent</span> finds
that the query <span class="code">A.B.T</span>, or <span class="code">q</span>,
_maybe_ is located inside the second child memory segment, but not the first one.

For the first child union structure, <span class="code">ltree_consistent</span>
finds <span class="code">q > A</span> true but <span class="code">q <
A.B.M</span> false. Therefore <span
class="code">ltree_consistent</span> knows there can be no matches in the top
child segment, so it skips down to the second union structure.

For the second child union structure, <span class="code">ltree_consistent</span> finds both <span class="code">q > A.B.N</span>
true and <span class="code">q < A.B.X</span> true. Therefore it returns <span
class="code">MAYBE</span>, meaning the search continues in the lower child
segment:

<img src="https://patshaughnessy.net/assets/2017/12/15/gist11.png"/>

Note Postgres never had to search the first child segment: The tree structure
limits the comparisons necessary to just the values that might match <span
class="code">p <@ A.B.T</span>.

Imagine my table contained a million rows: Searches using the GiST index will
still be fast because the GiST tree limits the scope of the search. Instead of
executing <span class="code">p <@ q</span> on every one of the million rows,
Postgres only needs to run <span class="code">p <@ q</span> a handful of times,
on a few union records and on the child segments of the tree that contain
values that might match.

<div style="float: right; padding: 8px 0px 20px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2017/12/15/sternberg.jpg"><br/>
  <i>The Sternberg Astronomical Institute
at Moscow State University</i>
</div>

## Send Them a Postcard

Oleg Bartunov and Teodor Sigaev, the authors of the LTREE extension, explain
its usage and the algorithms I detailed above here on their [web
page](http://www.sai.msu.su/~megera/postgres/gist/ltree/). They included more
examples of SQL searches on tree data, including some which use the <span
class="code">LTREE[]</span> data type I didn’t have time to cover in these blog
posts.

But most importantly, they included this note at the bottom:

<img src="https://patshaughnessy.net/assets/2017/12/15/postcard.png"/>

Do you save tree data in Postgres? Does your app take advantage of the LTREE
extension? If so, you should send Oleg and Teodor a postcard! I just did.

