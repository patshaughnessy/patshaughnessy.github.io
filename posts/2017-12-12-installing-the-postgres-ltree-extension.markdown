title: "Installing the Postgres LTREE Extension"
date: 2017/12/12
tag: the Postgres LTREE Extension

<div style="float: right; padding: 8px 0px 40px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2017/12/12/tree2.jpg">
</div>

Hidden inside of your Postgres server is code that provides special SQL
operators and functions designed to support tree operations. It’s called the
[LTREE extension](https://www.postgresql.org/docs/current/static/ltree.html).
I’m guessing this stands for _left-tree_. In [my next
post](https://patshaughnessy.net/2017/12/13/saving-a-tree-in-postgres-using-ltree), I’ll
write about some of these functions and operators: what they do and how to use
them.

But first, where is the LTREE extension? How can you install and start using it? Read on to
find out.

## Testing Whether the LTREE Extension is Installed

Depending on where you downloaded Postgres from and how you installed it, you
may have already installed LTREE with Postgres. To find out, execute this SQL
statement:

<pre>
=> create extension ltree;
CREATE EXTENSION
</pre>

If you see the “CREATE EXTENSION” message like this, then you’re all set! LTREE
was already installed and you just enabled it. Skip to my next post to find out
what it can do and how to use it.

Or if you see:

<pre>
=> create extension ltree;
ERROR:  extension "ltree" already exists
</pre>

…then your Postgres server already had LTREE enabled.

FYI The <span class="code">pg_available_extensions</span> table will show you
all the Postgres extensions that are available and installed in your server:

<pre>
select * from pg_available_extensions;

  name   | default_version | installed_version |                     comment
---------+-----------------+-------------------+-------------------------------------------------
 ltree   | 1.1             | 1.1               | data type for hierarchical tree-like structures
 plpgsql | 1.0             | 1.0               | PL/pgSQL procedural language
(2 rows)
</pre>

As you can see, “ltree” already appears in my server’s list. The value “1.1”
for <span class="code">installed_version</span> indicates that I’ve already
enabled it too. This would have been blank before running the <span
class="code">create extension ltree</span> command above.

I originally installed a local copy of Postgres on my Mac using Homebrew, and I
was happy to find that the Homebrew Postgres formula does include steps to
build and install LTREE, after building the rest of the Postgres server. But I
still needed to enable it using <span class="code">create extension</span>.

## Using LTREE on a Shared Postgres Server

Running the <span class="code">create extension ltree</span> command may fail
with this error message:

<pre>
=> create extension ltree;
ERROR:  permission denied to create extension "ltree"
HINT:  Must be superuser to create this extension.
</pre>

Enabling Postgres extensions requires super user access. If you’re using a
shared Postgres server and don’t have super-user access, you’ll need to find
someone who does. Or you may just need to login to Postgres using the proper
Postgres user account.

## How to Install the LTREE Extension

Running the <span class="code">create extension</span> command may also fail
with this error message:

<pre>
=> create extension ltree;
ERROR:  could not open extension control file "/usr/local/pgsql/share/extension/ltree.control": No such file or directory
</pre>

This error means the LTREE code isn’t even installed on your Postgres server.
If you’re running on Linux and installed Postgres using a package manager, you
may have to install a second package called “postgresql-contrib.”

If you installed Postgres from source yourself, then you will see this error
message because the Postgres Makefile doesn’t compile and install LTREE by
default. Don’t worry! It turns out the Postgres source tree already contains
the code for LTREE and many other extensions in a subdirectory called
“contrib.”

<img src="https://patshaughnessy.net/assets/2017/12/12/ltree-source.png">

Compile it as follows:

<pre>
$ cd /path/to/postgres-9.6.5/contrib/ltree
$ make

gcc -Wall -Wmissing-prototypes -Wpointer-arith -Wdeclaration-after-statement -Wendif-labels -Wmissing-format-attribute -Wformat-security -fno-strict-aliasing -fwrapv -Wno-unused-command-line-argument -O2  -DLOWER_NODE -I. -I. -I../../src/include   -c -o ltree_io.o ltree_io.c

etc…

$ sudo make install

/bin/sh ../../config/install-sh -c -d '/usr/local/pgsql/lib'
/bin/sh ../../config/install-sh -c -d '/usr/local/pgsql/share/extension'
/bin/sh ../../config/install-sh -c -d '/usr/local/pgsql/share/extension'
/usr/bin/install -c -m 755  ltree.so '/usr/local/pgsql/lib/ltree.so'
/usr/bin/install -c -m 644 ./ltree.control '/usr/local/pgsql/share/extension/'
/usr/bin/install -c -m 644 ./ltree--1.1.sql ./ltree--1.0--1.1.sql ./ltree--unpackaged--1.0.sql  ‘/usr/local/pgsql/share/extension/'
</pre>

You can see above the install step copied the ltree.so library into my Postgres
server’s lib directory: /usr/local/pgsql/lib, and ran a couple other commands
as well. Now I can run the <span class="code">create extension ltree</span>
command as shown above. I don’t even need to restart Postgres; it will find and
load ltree.so automatically.

Now that you have LTREE installed and enabled, you can read [my next
post](https://patshaughnessy.net/2017/12/13/saving-a-tree-in-postgres-using-ltree), I’ll
to learn how to use it.
