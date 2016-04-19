title: "How does Bundler bundle?"
date: 2011/9/24
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2011/9/24/bundle.jpeg"></td></tr>
</table>
</div>

Every time you start a new Ruby project, probably one of the first things you do is write a Gemfile and then run <span class="code">bundle install</span>. Bundler has truly made all of our lives much easier... we know it will install the proper version of all the gems we need. But how does it actually work? This week I decided to take a closer look at the Bundler code itself to find out. How does it know which gems to install? How does it guarantee each gem’s dependencies will be satisfied? How do I get just the right “bundle” my app needs?

DELIM
<br/>
Today I’m going to explain how Bundler’s “resolver” algorithm works using a series of conceptual diagrams. This is the code that determines exactly which set of gems to install on your laptop or server... i.e. what your “bundle” should be.

## Running Bundler with DEBUG_RESOLVER=1

To keep things simple, let’s suppose I have this small Gemfile in my Ruby app:

<pre type="ruby">
source 'http://rubygems.org'
gem 'uglifier'
</pre>

I chose uglifier as an example not only because of its cool name, but also because it has a fairly simple set of dependencies, which will make it easier to follow how Bundler’s resolver code is working. Now instead of just running <span class="code">bundle install</span> as usual, let’s first set an environment variable that will cause Bundler’s resolver code to write out some debugging information as it runs:

<pre type="console">
$ export DEBUG_RESOLVER=1
$ bundle install 2> output.txt
Fetching source index for http://rubygems.org/
Using multi_json (1.0.3) 
Using execjs (1.2.8) 
Using uglifier (1.0.3) 
Using bundler (1.0.18) 
Your bundle is complete! Use `bundle show [gemname]` to see where a bundled gem is installed.
</pre>

If you run this you should now have a text file called “output.txt” which will contain some information about how Bundler resolved the gem dependencies.

## Visualizing Bundler’s resolver algorithm using GraphViz

To make this debug text file easier to understand, I decided to write some Ruby code to parse it and then use [GraphViz](http://www.graphviz.org/) to output a graphical representation of the information there. [As I discussed last week](http://patshaughnessy.net/2011/9/17/bundlers-best-kept-secret), GraphViz is the library that Bundler uses to implement the <span class="code">bundle viz</span> command, which displays a graphical chart showing your gem dependencies.

Now let’s take a look at how Bundler’s resolver code actually works. We’ll start the process with two lists:

![step 1](http://patshaughnessy.net/assets/2011/9/24/simple1.png)

On the left are the gems activated so far, which is an empty list at first. This will eventually be the return value of the resolver algorithm. On the right are the requirements from the Gemfile, which in my simple example is just the single uglifier gem.

The resolver algorithm’s first step is to take the requirement, uglifier >=0, and see whether it is satisfied by any of the gems already activated. It also checks whether any of the activated gems conflict with the requirement. Since there are no activated gems yet, Bundler looks for the latest version of uglifier that matches the requirement from rubygems.org; here it is:

![step 2](http://patshaughnessy.net/assets/2011/9/24/simple2.png)

As I wrote this, the latest version of uglifier was 1.0.3. Bundler also downloads uglifier’s dependencies from rubygems. Now it “activates” uglifier 1.0.3, adding it to the list of activated gems:

![step 3](http://patshaughnessy.net/assets/2011/9/24/simple3.png)

Bundler also adds the dependencies for the new gem to the list of requirements, as if they were written in the Gemfile directly. You can see here that multi_json and execjs were added to the list on the right.

Now Bundler simply iterates and repeats the same process. The next requirement is popped off the top of the list on the right, checked against the activated gems, and a matching gem is found in the source index that was downloaded from rubygems.org. It turns out that the latest version of multi_json is 1.0.3 and has no dependencies at all. Also since there is no other version of multi_json already activated, Bundler just activates it:

![step 4](http://patshaughnessy.net/assets/2011/9/24/simple4.png)

And now we repeat the process again, fetching a gem that matches the execjs >= 0.3.0 requirement on rubygems.org:

![step 5](http://patshaughnessy.net/assets/2011/9/24/simple5.png)

This turns out to be execjs 1.2.8, again the latest version at the time I wrote this. It has a single dependency on multi_json ~> 1.0. Now repeating the process, Bundler activates execjs 1.2.8, which didn’t appear in the activated list yet, and adds multi_json ~> 1.0 to the requirements list:

![step 6](http://patshaughnessy.net/assets/2011/9/24/simple6.png)

Finally Bundler checks the latest requirement and finds that there is a gem that was activated that already matches this requirement: multi_json 1.0.3. So there’s no need to download a new version of multi_json - this requirement is already satisfied.

Now Bundler is done - the list on the left becomes our “bundle,” and the list is written out to a file called Gemfile.lock, along with dependency information:

<pre type="console">
$ cat Gemfile.lock
GEM
  remote: http://rubygems.org/
  specs:
    execjs (1.2.8)
      multi_json (~> 1.0)
    multi_json (1.0.3)
    uglifier (1.0.3)
      execjs (>= 0.3.0)
      multi_json (>= 1.0.2)

PLATFORMS
  ruby

DEPENDENCIES
  uglifier
</pre>

The information here is used by bundler when your app is deployed onto different servers or laptops to insure that Bundler activates the exact same list of gems.

## But what if there’s a conflict?

The example above is the best case scenario: there was a very simple Gemfile with just a single gem and only a few dependencies that didn’t conflict. To make things more interesting, let’s add a new requirement to my example Gemfile:

<pre type="ruby">
source 'http://rubygems.org'
gem 'multi_json', '1.0.1'
gem 'uglifier'
</pre>

And now I’ll run <span class="code">bundle install</span> again:

<pre type="console">
$ bundle install
Fetching source index for http://rubygems.org/
You have requested:
  multi_json = 1.0.1

The bundle currently has multi_json locked at 1.0.3.
Try running `bundle update multi_json`
</pre>

Oops - what happened here was that my bundle was already locked, and used multi_json 1.0.3. That is to say, Bundler loaded the Gemfile.lock file I showed above and found that it contained a different version of multi_json than what I added to the Gemfile.

Let’s try again, but this time use <span class="code">bundle update</span> instead, so that the Gemfile.lock is discarded and recreated after the resolver code is done:

<pre type="console">
$ bundle update 2> output2.txt
Fetching source index for http://rubygems.org/
Using multi_json (1.0.1) 
Using execjs (1.2.8) 
Using uglifier (0.5.2) 
Using bundler (1.0.18) 
Your bundle is updated! Use `bundle show [gemname]` to see where a bundled gem is installed.
</pre>

Here you can see that I got a different set of gems... Bundler was able to resolve the request for multi_json 1.0.1 by downloading and activating a different, older version of uglifier (0.5.2 vs. 1.0.3) - very cool! But how does this work? How does Bundler know to do this?

Let’s repeat the same process again by starting with the same two columns, the activated gems on the left and the requirements on the right:

![step 1](http://patshaughnessy.net/assets/2011/9/24/conflict1.png)

Now my Gemfile has two requirements, so you see two entries on the right to start with. Repeating the same resolver algorithm, Bundler finds a matching version of multi_json on rubygems.org. This has to be version multi_json 1.0.1, since I specified that exact version in the Gemfile’s requirement.

Since this is the first gem to be processed, Bundler just activates it immediately. There are also no dependencies, so the list of requirements just gets smaller.

![step 2](http://patshaughnessy.net/assets/2011/9/24/conflict2.png)

Uglifier >= 0 is the next requirement so Bundler will find the latest version of this gem available on rubygems.org:

![step 3](http://patshaughnessy.net/assets/2011/9/24/conflict3.png)

Just as above, Bundler finds there are two dependencies on execjs and multi_json. Now it activates uglifier 1.0.3 and adds it to the activated gems, and also adds the two dependencies to the requirements list:

![step 4](http://patshaughnessy.net/assets/2011/9/24/conflict4.png)

This leaves multi_json >= 1.0.2 as the next requirement, but before Bundler checks for the latest version on rubygems.org, it will first check the list of gems that were already activated: uglifier 1.0.3 and multi_json 1.0.1. Now Bundler has a conflict! It can’t activate any version of multi_json >= 1.0.2 since multi_json 1.0.1, was already activated.

To resolve the conflict, Bundler now returns to the gem which depended on multi_json >= 1.0.2, in this case uglifier 1.0.3. Discarding uglifier 1.0.3, Bundler uses the source index from rubygems.org to iterate through the older versions of the problem gem (uglifier) in descending order, running the same process with each one. To keep things simple I’ll skip a couple of intermediate versions and jump to this one next:

![step 5](http://patshaughnessy.net/assets/2011/9/24/conflict5.png)

Now Bundler tries the same process with this older version of the gem: it activates it and adds its dependencies to the list of requirements:

![step 6](http://patshaughnessy.net/assets/2011/9/24/conflict6.png)

Next Bundler returns to the same process as usual, and checks whether the next requirement conflicts with any previously activated gems. Multi_json 1.0.1 meets the multi_json >= 0 requirement and doesn’t conflict.

Now Bundler is free to continue the process, finding the newest version of execjs, and getting its dependencies.

## To learn more...

In today’s example, I simplified the resolver algorithm somewhat; to see the actual code Bundler uses for resolving gem dependencies just go to [resolver.rb on github](https://github.com/carlhuda/bundler/blob/master/lib/bundler/resolver.rb). In a real world resolver task, the dependency graph can be much more complex; for example it can be a deeply nested tree of gems that depend on gems that depend on other gems. If there’s a conflict in a more complex scenario, Bundler might have to iterate through a series of parent gems to find which gem needs to be replaced to resolve the conflict. Bundler has other features, such as being able to specify different gem sets and requirements for different environments, that make the actual code more complicated also.

I hope this exercise was a fun way to get a glimpse into the “magic” behind Bundler... giving you some understanding of what’s really happening the next time you run <span class="code">bundle install</span>.
