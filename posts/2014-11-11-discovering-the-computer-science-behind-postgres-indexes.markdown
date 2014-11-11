title: "Discovering the Computer Science Behind Postgres Indexes"
date: 2014/11/11

<div style="float: left; padding: 7px 30px 20px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/11/11/engineering-plans.png"><br/>
  <i> Captain Nemo and Professor Aronnax discussing the<br/>engineering and science behind
  the Nautilus.</i> </div>

<b> This is the last post in a series based on a
  presentation I did at the [Barcelona Ruby Conference](http://www.baruco.org)
  called “20,000 Leagues Under ActiveRecord.” (other posts:
  [one](http://patshaughnessy.net/2014/9/17/20000-leagues-under-activerecord)
  [two](http://patshaughnessy.net/2014/9/23/how-arel-converts-ruby-queries-into-sql-statements)
  [three](http://patshaughnessy.net/2014/10/13/following-a-select-statement-through-postgres-internals)
  and [video](https://www.youtube.com/watch?v=rnLnRPZZ1Q4)).  </b>

We all know indexes are one of the most powerful and important features of
relational database servers. How do you search for a value quickly? Create an
index. What do you have to remember to do when joining two tables together?
Create an index. How do you speed up a SQL statement that’s beginning to run
slowly? Create an index.

But what are indexes, exactly? And _how_ do they speed up our database searches?
To find out, I decided to read the C source code inside the PostgreSQL database
server, to follow along as it searched an index for a simple string value. I
expected to find sophisticated algorithms and efficient data structures. And I
did. Today I’ll show you what indexes look like inside Postgres and explain how
they work.

What I didn’t expect to find - what I discovered for the first time reading the
Postgres C source code - was the Computer Science theory behind what it was
doing. Reading the Postgres source was like going back to school and taking
that class I never had time for when I was younger. The C comments inside
Postgres not only explain what Postgres does, but _why_.

## Sequence Scans: A Mindless Search

When we left the crew of the Nautilus, they were exhausted and beginning to
faint: the Postgres sequence scan algorithm was mindlessly looping over all of
the records in the users table! Recall in [my last
post](http://patshaughnessy.net/2014/10/13/following-a-select-statement-through-postgres-internals)
we had executed this simple SQL statement to find Captain Nemo:

<img src="http://patshaughnessy.net/assets/2014/11/11/select-users.png"/>

Postgres first parsed, analyzed and planned the query. Then
[ExecSeqScan](http://doxygen.postgresql.org/nodeSeqscan_8c.html#af80d84501ff7621d2ef6249b148e7f44),
the C function inside of Postgres that implements the sequence scan (SEQSCAN)
plan node, quickly found Captain Nemo:

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan4.png"/>

But then inexplicably Postgres continued to loop through the entire user table,
comparing each name to “Captain Nemo,” even though we had already found what we
were looking for!

<img class="centered" src="http://patshaughnessy.net/assets/2014/10/13/seqscan5.png"/>

Imagine if our users table had millions of records; this could take a very long
time. Of course, we could have avoided this by removing the sort and rewriting
our query to accept the first name, but the deeper problem here is the
inefficient way Postgres searches for our target string. Using a sequence scan
to compare every single value in the users table with “Captain Nemo” is slow,
inefficient and depends on the random order the names appear in the table. What
are we doing wrong? There must be a better way!

The answer is simple: We forgot to create an Index. Let’s do that now.

## Creating an Index

Creating an index is straightforward - we just need to run this command:

<img src="http://patshaughnessy.net/assets/2014/11/11/create-index.png"/>

As Ruby developers, of course, we would use the <span class="code">add_index</span> ActiveRecord
migration instead; this would run the same <span class="code">CREATE INDEX</span> command behind the
scenes. When we rerun our select statement, Postgres will create a plan tree as usual -
but this time the plan tree will be slightly different:

<img class="centered" src="http://patshaughnessy.net/assets/2014/11/11/indexscan-plan.png"/>

Notice at the bottom Postgres now uses INDEXSCAN instead of SEQSCAN. Unlike
SEQSCAN, INDEXSCAN won’t iterate over the entire users table. Instead, it will
use the index we just created to find and return the Captain Nemo records
quickly and efficiently.

Creating an index has solved our performance problem, but it’s also left us with many interesting, unanswered questions:

* _What_ is a Postgres index, exactly?
* If I could go inside of a Postgres database and take a close look at an index, _what would it look like_?
* And _how_ does an index speed up searches?

Let’s try to answer these questions by reading the Postgres C source code.

## What Is a Postgres Index, Exactly?

We can get started with a look at the
[documentation](http://www.postgresql.org/docs/9.3/static/sql-createindex.html)
for the CREATE INDEX command.

<img src="http://patshaughnessy.net/assets/2014/11/11/documentation.png"/>

Here you can see all of the options we can use to create an index, such as
<span class="code">UNIQUE</span> and <span class="code">CONCURRENTLY</span>. Notice there’s an option called <span class="code">USING method</span>. This
tells Postgres what kind of index we want. Farther down the same page is some
information about <span class="code">method</span>, the argument to the <span class="code">USING</span> keyword:

<img src="http://patshaughnessy.net/assets/2014/11/11/method.png"/>

It turns out Postgres implements four different types of indexes. You can use
them for different types of data and in different situations. Because we didn’t
specify <span class="code">USING</span> at all, our <span
class="code">index\_users\_on\_name</span> index is a “btree” (or B-Tree) index,
the default type.

This is our first clue: a Postgres index is a B-Tree. But what is a B-Tree?
Where can we find one? Inside of Postgres, of course! Let’s search the Postgres
C source code for files containing “btree:”

<img src="http://patshaughnessy.net/assets/2014/11/11/find.png"/>

The key result is in bold: “./backend/access/nbtree.” Inside this directory is
a README file; let’s read it:

<img src="http://patshaughnessy.net/assets/2014/11/11/readme.png"/>

<div style="float: right; padding: 7px 0px 70px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/11/11/atlantis.png"><br/>
  <i>Nemo found the lost continent of Atlantis<br/>next to an underwater
  volcano.</i>
</div>

Amazingly, this README file turns out to be an extensive 12 page document! The
Postgres source code not only contains helpful and interesting C comments, it
also contains documentation about the theory and implementation of the database
server. Reading and understanding the code in open source projects can often be
intimidating and difficult, but not for Postgres. The developers behind
Postgres have gone to great lengths to help the rest of us understand their
work.

The title of the README document, “Btree Indexing,” confirms this directory
contains the C code that implements Postgres B-Tree indexes. But the first
sentence is even more interesting: it’s a reference to an academic paper that
explains what a B-Tree is, and how Postgres indexes work: [Efficient Locking for
Concurrent Operations on B-Trees](http://www.csd.uoc.gr/~hy460/pdf/p650-lehman.pdf), by Lehman and Yao.

We’ll find our first look at a B-Tree inside this academic paper.

## What Does a B-Tree Index Look Like?

Lehman and Yao’s paper explains an innovation they made to the B-Tree algorithm
in 1981. I’ll discuss this a bit later. But they start with a simple
introduction to the B-Tree data structure, which was actually invented 9 years
earlier in 1972. One of their diagrams shows an example of a simple B-Tree:

<img class="centered" src="http://patshaughnessy.net/assets/2014/11/11/figure2.png"/>

The term B-Tree actually stands for “balanced tree.” B-Trees make searching
easy and fast. For example, if we wanted to search for the value 53 in this
example, we first start at the root node which contains the value 40:

<img src="http://patshaughnessy.net/assets/2014/11/11/node1.png"/>

We compare our target value of 53 with the value we find in the tree node. Is
53 greater than or less than 40? Because 53 is greater than 40 we follow the
pointer down to the right. If we were searching for 29, we would go down to the
left. Pointers on the right lead to larger values; pointers on the left to
smaller ones.

Following the pointer down the tree to the next child tree node, we encounter a
node that contains 2 values:

<img src="http://patshaughnessy.net/assets/2014/11/11/node2.png"/>

This time we compare 53 with both 47 and 62, and find that 47 < 53 < 62. Note
the values in the tree node are sorted, so this will be easy to do. This time
we follow the center pointer down.

Now we get to another tree node, this one with 3 values in it:

<img src="http://patshaughnessy.net/assets/2014/11/11/node3.png"/>

Looking through the sorted list of numbers, we find 51 < 53 < 56, and follow
the second of four pointers down.

Finally, we come to a leaf node in the tree:

<img src="http://patshaughnessy.net/assets/2014/11/11/node4.png"/>

And we’ve found the value 53!

B-Trees speed up searches because:

* They sort the values (known as _keys_) inside of each node.
* They are _balanced_: B-Trees evenly distribute the keys among the nodes,
  minimizing the number of times we have to follow a pointer from one node to
  another. Each pointer leads to a child node that contains more or less the
  same number of keys each other child node does.

## What Does a Postgres Index Look Like?

Lehman and Yao drew this diagram over 30 years ago - what does it have to do
with how Postgres works today? Astonishingly, the <span
class="code">index\_users\_on\_name</span> index we created earlier looks very
similar to Figure 2: We created an index in 2014 that looks just like a diagram
from 1981!

When we executed the <span class="code">CREATE INDEX</span> command, Postgres
saved all of the names from our users table into a B-Tree. These became the
keys of the tree. Here’s what a node inside a Postgres B-Tree index looks like:

<img src="http://patshaughnessy.net/assets/2014/11/11/index-tuple-data1.svg"/>

Each entry in the index consists of a C structure called <span class="code">IndexTupleData</span>, and is
followed by a bitmap and a value. Postgres uses the bitmap to record whether
any of the index attributes in a key are NULL, to save space. The actual values
in the index appear after the bitmap.

Let’s take a closer look at the <span class="code">IndexTupleData</span>
structures:

<img src="http://patshaughnessy.net/assets/2014/11/11/index-tuple-data2.svg"/>

Above you can see each IndexTupleData structure contains:

* <span class="code">t\_tid</span>: This is a pointer to either another index
  tuple, or to a database record. Note this isn't a C pointer to physical
  memory; instead, it contains numbers Postgres can use to find the referenced
  value among its memory pages.
* <span class="code">t\_info</span>: This contains information about the index
  tuple, such as how many values it contains, and whether or not there are null
  values.

To understand this better, let’s show a few entries from our <span
class="code">index\_users\_on\_name</span> index:

<img src="http://patshaughnessy.net/assets/2014/11/11/index-tuple-data3.svg"/>

Now I’ve replaced “value” with some names from my users table. The upper tree
node includes the keys “Dr. Edna Kunde” and “Julius Powlowski,” while the lower
tree node contains “Julius Powlowski” and “Juston Quitzon.” Notice that, unlike
Lehman and Yao’s diagram, Postgres repeats the parent keys in each child node.
Here “Julius Powlowski” is a key in the upper node and in the child node. The
<span class="code">t\_tid</span> pointer from Julius in the upper node
references the same Julius name in the lower node.

To learn more about exactly how Postgres stores key values into a B-Tree node,
refer to the itup.h C header file:

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">IndexTupleData</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/itup_8h_source.html">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/11/11/itup.png"/>
  </div>
</div>
</p>

## Finding the B-Tree Node Containing Captain Nemo

Now let’s return to our original SELECT statement again:

<img src="http://patshaughnessy.net/assets/2014/11/11/select-users.png"/>

How exactly does Postgres search our <span
class="code">index\_users\_on\_name</span> index for “Captain Nemo?” Why is
using the index faster than the sequence scan we saw in my last post? To find
out, let’s zoom out a bit and take a look at some of the user names in our
index:

<img src="http://patshaughnessy.net/assets/2014/11/11/root-names1.svg"/>

This is the root node of the <span class="code">index\_users\_on\_name</span>
B-Tree. I’ve turned the tree on its side so the names would fit. You can see 4
names and a NULL value. Postgres created this root node when I created
<span class="code">index\_users\_on\_name</span>. Note that, aside from the
first NULL value which represents the beginning of the index, the other 4 names
are more or less evenly distributed in alphabetical order.

Remember that a B-Tree is a balanced tree. In this example, the B-Tree has 5 child nodes:

* the names that appear before Dr. Edna Kunde alphabetically
* names that appear between Dr. Edna Kunde and Julius Powlowski
* names that appear between Julius Powlowski and Monte Nicolas
* etc…

Because we’re searching for Captain Nemo, Postgres follows the first, top arrow
to the right. This is because Captain Nemo comes before Dr. Edna Kunde
alphabetically:

<img src="http://patshaughnessy.net/assets/2014/11/11/root-names2.svg"/>

You can see on the right that Postgres has found the B-Tree node that contains
Captain Nemo. For my test I added 1000 names to the users table; this child
node in the B-Tree contained about 200 names (240 actually). The B-Tree has
narrowed down Postgres’s search considerably.

To learn more about the precise algorithm Postgres uses to search for the
target B-Tree node among all of the nodes in the tree, read the <span
class="code">_bt_search</span> function.

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">_bt_search</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/nbtsearch_8c.html#a9053c37f2c25187580f3f690ad41bf01">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/11/11/bt_search.png"/>
  </div>
</div>
</p>

## Finding Captain Nemo Inside a Single B-Tree Node

Now that Postgres has narrowed down the search to a B-Tree node containing
about 200 names, it still has to find Captain Nemo… how does it do this? Does
it perform a sequence scan on this shorter list?

<img src="http://patshaughnessy.net/assets/2014/11/11/binary-search1.svg"/>

No. To search for a key value inside of a tree node, Postgres switches to use a
binary search algorithm. It starts by comparing the key that appears at the 50%
position in the tree node with “Captain Nemo:”

<img src="http://patshaughnessy.net/assets/2014/11/11/binary-search2.svg"/>

Because Captain Nemo comes after Breana Witting alphabetically, Postgres
jumps down to the 75% position and performs another comparison:

<img src="http://patshaughnessy.net/assets/2014/11/11/binary-search3.svg"/>

This time Captain Nemo comes before Curtis Wolf, so Postgres jumps back a bit.
Skipping a few more steps (it actually took Postgres 8 comparisons to find
Captain Nemo in my example), Postgres eventually finds what we are looking for:

<img src="http://patshaughnessy.net/assets/2014/11/11/binary-search4.svg"/>

To learn more about exactly how Postgres searches for a value in a single
B-Tree node, read the <span class="code">_bt_binsrch</span> function:

<p>
<div class="sign">
  <div class="sign-icon"></div>
  <div class="function-info">
    <div class="function-name">_bt_binsrch</div>
    <div class="function-link"><a href="http://doxygen.postgresql.org/nbtsearch_8c.html#acd3770ac6d3bc26d6f319d3255721280">view on postgresql.org</a></div>
  </div>
  <div class="function-code">
    <img src="http://patshaughnessy.net/assets/2014/11/11/bt_binsrch.png"/>
  </div>
</div>
</p>

## So Much More to Learn

I don’t have space in this blog post to cover many other fascinating details
about B-Trees, database indexes or Postgres internals… maybe I should write
_Postgres Under a Microscope_ :) But for now, here are just a few interesting
bits of theory you can read about in [Efficient Locking for Concurrent
Operations on B-Trees](http://www.csd.uoc.gr/~hy460/pdf/p650-lehman.pdf) or in
the other academic papers it references.

* Inserting into B-Trees: The most beautiful part of the B-Tree algorithm has
  to do with inserting new keys into a tree. Key are inserted in sorted order
  into the proper tree node - but what happens when there’s no more room for a
  new key? In this situation, Postgres splits the node into two, inserts the
  new key into one of them, and also adds the key from the split point into the
  parent node, along with a pointer to the new child node. Of course, the
  parent node might also have to be split to fit its new key, resulting in a
  complex, recursive operation.

* Deleting from B-Trees: The converse operation is also interesting. When
  deleting a key from a node, Postgres will combine sibling nodes together when
  possible, removing a key from their parent. This can also be a recursive
  operation.

* B-Link-Trees: Lehman and Yao’s paper actually discusses an innovation they
  researched related to concurrency and locking when multiple threads are using
  the same B-Tree. Remember, Postgres’s code and algorithms need to be
  multithreaded because many clients could be searching or modifying the same
  index at the same time. By adding another pointer from each B-Tree node to
  the next sibling node - the so-called “right arrow” - one thread can search a
  tree even while a second thread is splitting a node without locking the
  entire index:

<img src="http://patshaughnessy.net/assets/2014/11/11/right-arrow.png"/>

## Don’t Be Afraid To Explore Beneath The Surface

<div style="float: right; padding: 22px 0px 70px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/11/11/at-the-helm.png"><br/>
  <i>Captain Nemo at the helm</i>
</div>

Professor Aronnax risked his life and career to find the elusive Nautilus and
to join Captain Nemo on a long series of amazing underwater adventures. We
should do the same: Don’t be afraid to dive underwater - inside and underneath
the tools, languages and technologies that you use every day. You may know all
about how to use Postgres, but do you really know how Postgres itself works
internally? Take a look inside; before you know it, you’ll be on an underwater
adventure of your own.

Studying the Computer Science at work behind the scenes of our applications
isn’t just a matter of having fun, it’s part of being a good developer. As
software development tools improve year after year, and as building web sites
and mobile apps becomes easier and easier, we shouldn’t loose sight of the
Computer Science we depend on. We’re all standing on the shoulders of giants -
people like Lehman and Yao, and the open source developers who used their
theories to build Postgres. Don’t take the tools you use everyday for granted -
take a look inside them! You’ll become a wiser developer and you’ll find
insights and knowledge you could never have imagined before.
