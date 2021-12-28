title: "Using Rake to Generate a Blog"
date: 2015/1/8
tag: Ruby

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="https://patshaughnessy.net/assets/2015/1/8/power-rake.png"><br/>
  <i>Jim Weirich showing a real power rake at <a href="http://www.confreaks.com/videos/988-goruco2012-power-rake">GORUCO 2012</a></i>
</div>

Last year I needed to replace the software I use to serve this web site.
Instead of just using [Jekyll](https://github.com/jekyll/jekyll),
[Middleman](https://middlemanapp.com), [Nanoc](http://nanoc.ws) or one of the
[many other available
options](https://www.ruby-toolbox.com/categories/static_website_generation), I
decided to implement my own [custom blog
software](https://github.com/patshaughnessy/patshaughnessy.github.io). After a
fair amount of work, I was able to implement a static blog site generator using
only [Rake](https://github.com/jimweirich/rake) and a handful of simple Ruby classes. Although it took a bit longer,
it was a lot of fun and I learned a few tricks which I’d like to pass along
today.

I first got the idea of using Rake as a static site generator from a
presentation called [Power Rake](http://www.confreaks.com/videos/988-goruco2012-power-rake ),
given by the late Jim Weirich at GORUCO 2012. This was one of the first Ruby
conferences I had ever attended, and was also the first time I had ever seen
Jim speak in public. It still stands out in my memory as one of the best
conference presentations I've ever seen. Funny, engaging, interesting, but most
of all _genuine_, Jim had me and the rest of the audience enthralled as he talked
about [Rake](https://github.com/jimweirich/rake), his Ruby reinterpretation of
the old C make utility from the 1970s.

The key idea behind using Rake to generate a static site is to generate and
manipulate files using Rake file tasks. What are file tasks? How are they
different from standard Rake tasks? To find out watch Jim’s presentation, or
read [an excellent series of articles and
screencasts](http://devblog.avdi.org/2014/04/21/rake-part-1-basics/.) by Avdi
Grimm. Today I’ll explain how I used Rake to create this blog. But first, let’s review
what a blog really is.

## A Blog or a Static Web Site?

Most of the blogs in the world consist of a few dynamically generated web pages
served by either [wordpress.com](http://wordpress.com) or
[blogger.com](http://blogger.com). To be honest, I should just use one of these
two free services for my site as well. However, I have a few years worth of
markdown files that contain all of my old content which would be a hassle to
import into whatever format Wordpress or Google uses. Plus using these sites
would be no fun at all; instead, I was looking for an excuse to write some Ruby
code and to learn more about Rake.

What I really needed was an automated process for converting my markdown source
files into a series of static HTML files that were navigable using URL patterns that
readers expect. That is, I wanted a Rake task that would do this:

<img src="https://patshaughnessy.net/assets/2015/1/8/convert-file.png"/>

On the top is one of my markdown files; on the bottom is the HTML version. I
needed a way to generate the bottom file from the top one. I needed to write a Rake
task that would iterate over all of the markdown files in the “posts” directory,
and generate the corresponding HTML files in the proper target directory.  The
markdown file name (“posts/2014-10-13-…”) was a naming convention I used to stay
organized. However, the name and path of the HTML file was what readers would see
in the post’s URL online - for example:
[https://patshaughnessy.net/2014/10/13/following-a-select-statement-through-postgres-internals](https://patshaughnessy.net/2014/10/13/following-a-select-statement-through-postgres-internals).
This was a problem well suited to Rake file tasks, because they allow you to
create a series of dependencies between source and target files.

But before I was ready to use file tasks, I needed to use a few tricks to make
those tasks easier to write.

## Iterating Over Files Using Rake::FileList

Ruby objects are easier to work with than text files are, so the first thing I
decided to do was to write a Ruby class that represented one of my markdown
files. I called it <span class="code">Post</span> because each markdown file represented a single blog
post.

Next, I needed to create a post object for each of the files in the posts
directory, by listing the files and iterating over them. It turns out Rake
provides a very simple way to do this: the <span class="code">Rake::FileList</span> class. To quote [the
documentation](http://ruby-doc.org/stdlib-2.2.0/libdoc/rake/rdoc/Rake/FileList.html):

<br/>

<blockquote>
A FileList is essentially an array with a few helper methods defined to make file manipulation a bit easier.
</blockquote>

I like things that are easier. Here’s how I used <span class="code">FileList</span>:

<img src="https://patshaughnessy.net/assets/2015/1/8/mapping-posts.png"/>

On the left are my markdown files with the corresponding post objects on the right. My
code above first created a <span class="code">FileList</span>, using the <span class="code">posts/*.markdown</span> pattern. You
can think of the <span class="code">FileList</span> as an array of files that match the given pattern.
Once I had this array, I _mapped_ the array to a second array of ruby objects using
<span class="code">Enumerable#map</span>.

## Blog Post Routing

Now that I had a <span class="code">Post</span> object for each source markdown file, I could add methods
to the <span class="code">Post</span> class to make manipulating the markdown files easier. Most
importantly, what I needed to know for each markdown file is where its HTML
should go in the generated site. That is, I needed to know the URL of the post:

<img src="https://patshaughnessy.net/assets/2015/1/8/post-url.png"/>

This did the trick. The <span class="code">date</span> and <span class="code">title</span> methods parsed some metadata values I
saved in the markdown file along with the text. The <span class="code">url</span> method returned a string using
the year/month/day pattern most people are familiar with. The <span class="code">slugize</span> method
removed characters from the title that weren’t compatible with URL strings. As I
explained earlier, the URL is also the file system path for each post’s HTML
file: The single line of code above mapped the posts to an array of strings, each
one the path to an HTML file, the URL of that post appended with a file
extension.

## Grouping Two Arrays Together

Now I had two arrays: <span class="code">Post</span> objects and HTML file paths. I was almost ready to
write a Rake file task that would convert the posts into HTML files. But, as
you'll see in a minute, writing a file task requires two files: a source file
and a target file. Somehow I needed to convert these two separate arrays into a
single array of pairs, like this:

<img src="https://patshaughnessy.net/assets/2015/1/8/zipping.png"/>

As you can see, Ruby’s <span class="code">Enumerable#zip</span> method was
perfect solution. It yielded object pairs, one object taken from the receiver
(<span class="code">html_files</span>) and the other object taken from the
argument (<span class="code">posts</span>). If you pass in 2, 3 or more
arguments, it will yield triplets, quadruplets or n-tuples to the block
instead. I first learned about <span class="code">zip</span> from Jim Weirich’s
2012 Power Rake presentation; he used it in his static web site example in a
very similar way.  Of course, you can use <span class="code">zip</span> to
process multiple arrays for any purpose. It’s one of Ruby’s most beautiful
features I think.

## Writing Rake File Tasks

As you probably know, a standard Rake task runs when you execute the task
directly, or when you run another task that depends on it. A file task,
however, will only execute the Ruby code inside the block if:

* The source file is newer than the target file, or
* The target file doesn’t exist at all.

This behavior is ideal for generating a static web site, or for any other job
that requires generating a file from another file. Rake will build the target
file for the first time if it doesn’t exist, or update it if the source file
has changed.

Now that I had pairs of HTML paths and <span class="code">Post</span> objects,
it was easy for me to write a file task using one of these pairs. Here’s what I
came up with:

<img src="https://patshaughnessy.net/assets/2015/1/8/file-task.png"/>

By calling <span class="code">file</span> inside of the <span class="code">zip</span> block, I created a file task for each one
of the paths in <span class="code">html_files</span>. Now if I created a single, standard Rake task that
depended on the array of html file paths, I could test whether any or all of the
HTML files needed to be generated:

<img src="https://patshaughnessy.net/assets/2015/1/8/rake-posts.png"/>

Now I could generate all of my blog posts with one command: <span class="code">rake posts</span>!

## Rendering Each Post Using ERB

What did the code inside the file task do? It generated the HTML file for a
single post using <span class="code">ERB</span>, using a method I wrote called <span class="code">Layout#render</span>. If you’re
interested, here’s the <span class="code">Layout</span> class ([github](https://github.com/patshaughnessy/patshaughnessy.github.io/blob/master/lib/layout.rb)):

<img src="https://patshaughnessy.net/assets/2015/1/8/layout.png"/>

I won’t explain this line by line, but there were a couple of interesting tricks
here also. First, the <span class="code">contents</span> method used nested calls to
ERB to render a page layout surrounding the post, along
with the article text itself. This required I call <span
class="code">yield</span> somewhere inside my <span class="code">layout.erb</span> file in just the same
way I would in <span class="code">application.html.erb</span> for a Rails app.

The complex line of code at the bottom that uses <span class="code">instance_eval</span> and
<span class="code">binding</span> seems impossible to understand at first. But actually it’s fairly
standard boilerplate Ruby metaprogramming code that evaluates the ERB template
in the context of the <span class="code">page</span> object and the current method.

Let’s take a closer look at this:

<img src="https://patshaughnessy.net/assets/2015/1/8/metaprogramming.png"/>

On the left I show the the <span class="code">page</span> object, an instance of the <span class="code">Post</span> class, in the
center the code running the ERB transformation, and on the right the Ruby call
stack.

The arrow from ERB going to the left represents the use of <span class="code">instance_eval</span>. This
method, built into the Ruby language, resets the <span class="code">self</span> pointer to the
receiver or the <span class="code">page</span> object in this example. This allows the ERB code to access
the instance variables of the <span class="code">page</span> object and the methods of the <span class="code">Post</span> class.

The arrow from ERB going to the right, in turn, represents the call to <span class="code">binding</span>.
The <span class="code">binding</span> method, also part of the Ruby core language, refers to the
current Ruby stack frame allowing the ERB code to access all of the local
variables present there, such as <span class="code">recent_posts</span>.

## My Rakefile

Of course, I’m glossing over some other important details here, such as
generating the index or home page, the RSS feed and a few other things. For
reference, here’s my entire Rakefile ([github](https://github.com/patshaughnessy/patshaughnessy.github.io/blob/master/Rakefile)):

<img src="https://patshaughnessy.net/assets/2015/1/8/rakefile.png"/>

You can see the call to <span class="code">Layout#render</span> and the <span class="code">rake :posts</span> task I described
above. Here are some other coding details, if you’re interested:

* After creating the <span class="code">posts</span> array, I sort it by date, reversed.

* I generate the home page using another file task: <span class="code">index.html</span>, and a <span class="code">HomePage</span> class.

* I generate the RSS in a similar way using a third file task: <span class="code">index.xml</span>, and a <span class="code">Feed</span> class.

## Ideas, Not Code

If you’re interested in using this code for your own site, it’s on [github](https://github.com/patshaughnessy/patshaughnessy.github.io).
However, I wouldn’t recommended using it: It’s always a better idea to use a
well tested, robust free service such as wordpress.com or Jekyll.

Instead of using this code, use the ideas behind it! Take the time to use
<span class="code">Rake::FileList</span> and Rake file tasks in whatever application you’re working on. And
please take the time to watch the [PowerRake presentation](http://www.confreaks.com/videos/988-goruco2012-power-rake ). You’ll learn more about one
of Ruby’s most powerful tools - and you’ll be able spend some time with Jim.
Jim’s bright personality and sense of humor can live on in our memory, at
least.
