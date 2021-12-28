title: "4 tips for how to customize a Toto blog site"
date: 2011/1/23
tag: Toto

During the week between Christmas and New Year’s I finally took the time to redesign and redeploy this site on Heroku using [Toto](https://github.com/cloudhead/toto), a very simple Rack blog engine. Sorry if I spammed your RSS inbox with duplicate articles.

What makes Toto fantastic is that it includes just enough code to run a very simple blog site... and nothing else. [Alexis Sellier’s](https://github.com/cloudhead) code is concise and elegant; you can see it all in a single Ruby file: [toto.rb](https://github.com/cloudhead/toto/blob/master/lib/toto.rb). It leaves you a clean, blank slate to add anything special or custom you might need. In fact, I chose to use Toto instead of [Jekyll](https://github.com/mojombo/jekyll), [Nesta](https://github.com/gma/nesta) or other blog engine alternatives only because it looked like it would be fun to understand how it works and customize it.

Here are 4 tips to keep in mind as you work with Toto, based on my experience building this site:

* [Use bundler instead of a Heroku .gems file](#tip1)
* [Run with Shotgun](#tip2)
* [Use Rack::Rewrite for supporting shortened or obsolete URLs](#tip3)
* [Take the time to learn and use Toto’s Riot/RR test suite](#tip4)

## <a id="tip1"></a>Use bundler instead of a Heroku .gems file

The best way to get started with Toto is to take a look at the [GitHub readme page](https://github.com/cloudhead/toto#readme). Basically what you do is clone the [example blog template](https://github.com/cloudhead/dorothy) called “Dorothy” like this:

<div class="CodeRay"><pre>$ git clone git://github.com/cloudhead/dorothy.git myblog
$ cd myblog</pre></div>

Now you have a super-simple Rack app called “myblog” that you could deploy to Heroku immediately if you wanted to. It comes with a rackup file (config.ru) and also a .gems file that tells Heroku which gems to install:

<div class="CodeRay"><pre>$ cat .gems
builder
rdiscount
toto</pre></div>

You can see your new blog site will use builder (to generate the RSS XML feed), rdiscount (to parse the markdown content) and the toto gem, which is the blog engine itself. However, before proceeding I would take a few minutes to add bundler to your app and replace the .gems file with a Gemfile. Besides feeling a bit more modern and similar to Rails 3, using Bundler will make it easy to use a customized Toto gem from your hard drive or your github repo. Here’s how to do it...

First, in the myblog folder edit config.ru and add the 2 lines at the top that I’ve highlighted:

<div class="CodeRay"><pre><div class='container'>require <span class="s"><span class="dl">'</span><span class="k">bundler</span><span class="dl">'</span></span> 
<span class="co">Bundler</span>.setup<span class='overlay'></span></div>
require <span class="s"><span class="dl">'</span><span class="k">toto</span><span class="dl">'</span></span> 
 
<span class="c"># Rack config</span> 
use <span class="co">Rack</span>::<span class="co">Static</span>, <span class="sy">:urls</span> =&gt; [<span class="s"><span class="dl">'</span><span class="k">/css</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">/js</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">/images</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">/favicon.ico</span><span class="dl">'</span></span>], <span class="sy">:root</span> =&gt; <span class="s"><span class="dl">'</span><span class="k">public</span><span class="dl">'</span></span> 
use <span class="co">Rack</span>::<span class="co">CommonLogger</span> 
 
etc...</pre></div>

And now replace the .gems file

<div class="CodeRay"><pre>$ rm .gems</pre></div>

...with a Gemfile:

<div class="CodeRay"><pre>source <span class="s"><span class="dl">&quot;</span><span class="k">http://rubygems.org</span><span class="dl">&quot;</span></span> 
 
gem <span class="s"><span class="dl">&quot;</span><span class="k">builder</span><span class="dl">&quot;</span></span> 
gem <span class="s"><span class="dl">&quot;</span><span class="k">rdiscount</span><span class="dl">&quot;</span></span> 
gem <span class="s"><span class="dl">&quot;</span><span class="k">toto</span><span class="dl">&quot;</span></span>, <span class="sy">:path</span> =&gt; <span class="s"><span class="dl">'</span><span class="k">../customized_toto</span><span class="dl">'</span></span></pre></div>

Here along with the builder and rdiscount gems, I’ve indicated to bundler that it should use a copy of Toto on my local hard drive in a folder called “customized_toto.” At this point you should fork the Toto repo on GitHub and clone it on your machine into the “customized_toto” folder for example:

<div class="CodeRay"><pre>$ cd ..
$ git clone git@github.com:yourname/toto.git customized_toto</pre></div>

And finally install the bundle for your blog app:

<div class="CodeRay"><pre>$ cd myblog
$ bundle install
Fetching source index for http://rubygems.org/
Using builder (3.0.0) 
Using rack (1.2.1) 
Using rdiscount (1.6.5) 
Using toto (0.4.9) from source at /Users/pat/apps/customized_toto 
Using bundler (1.0.0) 
<span class="s">Your bundle is complete! Use `bundle show [gemname]` to see where a bundled gem is installed.</span></pre></div>

When you’re ready to push your site to Heroku, you will just replace the “:path =>” location with your github repo URL, using the “:git” option. For example, here’s the Gemfile from this site’s Heroku app:

<div class="CodeRay"><pre>source <span class="s"><span class="dl">&quot;</span><span class="k">http://rubygems.org</span><span class="dl">&quot;</span></span> 
 
gem <span class="s"><span class="dl">&quot;</span><span class="k">builder</span><span class="dl">&quot;</span></span> 
gem <span class="s"><span class="dl">&quot;</span><span class="k">rdiscount</span><span class="dl">&quot;</span></span> 
gem <span class="s"><span class="dl">&quot;</span><span class="k">toto</span><span class="dl">&quot;</span></span>, <span class="sy">:git</span> =&gt; <span class="s"><span class="dl">'</span><span class="k">https://github.com/patshaughnessy/toto.git</span><span class="dl">'</span></span></pre></div>

Heroku now supports bundler, and will automatically run bundle install on the server after you push your blog site there. Without bundler, using different versions of the Toto gem would be much more of a hassle.

## <a id="tip2"></a>Run with Shotgun

Normally you would startup your Toto blog site like any other rack app, with the “rackup” command:

<div class="CodeRay"><pre>$ cd myblog
$ rackup</pre></div>

Rackup will look for config.ru in the current directory, and launch your application on port 9292. However, the problem with this is that the application code is cached between HTTP requests, similar to how a Rails application works in production. If you’re planning on modifying the Toto blog engine code and not just your blog’s content, then you would have to stop and restart the server each time you made a code change.

For development, it’s much more convenient to use Shotgun instead. This is a version of the rackup command that will create an entirely new process for every HTTP request. What this means while you’re working with the Toto code is that you can make any sort of code change you’d like and just refresh your browser to see it work. You don’t need to worry about anything being cached.

You just need to install it, and run “shotgun” instead of “rackup:”

<div class="CodeRay"><pre>$ gem install shotgun
$ cd myblog
$ shotgun</pre></div>

Now you’ll find your blog at [http://locahost:9393](http://locahost:9393) instead of port 9292.

## <a id="tip3"></a>Use Rack::Rewrite for supporting shortened or obsolete URLs

As Dmitry Fadeyev pointed out in his helpful post [Getting Started With Toto, a Tiny WordPress Killer](http://fadeyev.net/2010/05/10/getting-started-with-toto/), if you’re migrating from a different blog engine the [Rack-Rewrite](https://github.com/jtrupiano/rack-rewrite) gem by John Trupiano is a great way to redirect URL patterns from your old site into what Toto expects. You can also use it like me to shorten URLs for some posts. Rack-Rewrite is a helpful piece of rack middleware that applies Apache rewrite rules to HTTP requests coming through the Rack stack. This means it can be used to redirect or change URL patterns before they hit the Toto code, and avoid the need to change Toto at all for this purpose.

With bundler it’s very easy to add Rack-Rewrite... all you need to do is add a line to your Gemfile like this:

<div class="CodeRay"><pre>source <span class="s"><span class="dl">&quot;</span><span class="k">http://rubygems.org</span><span class="dl">&quot;</span></span> 
 
gem <span class="s"><span class="dl">&quot;</span><span class="k">builder</span><span class="dl">&quot;</span></span> 
gem <span class="s"><span class="dl">&quot;</span><span class="k">rdiscount</span><span class="dl">&quot;</span></span> 
<span class='container'>gem <span class="s"><span class="dl">&quot;</span><span class="k">rack-rewrite</span><span class="dl">&quot;</span></span><span class='overlay'></span></span>
gem <span class="s"><span class="dl">&quot;</span><span class="k">toto</span><span class="dl">&quot;</span></span>, <span class="sy">:git</span> =&gt; <span class="s"><span class="dl">'</span><span class="k">https://github.com/yourname/toto.git</span><span class="dl">'</span></span></pre></div>

... and then run bundle install again:

<div class="CodeRay"><pre>$ bundle install
Using builder (3.0.0) 
Using rack (1.2.1) 
<span class='container'>Using rack-rewrite (1.0.2)<span class='overlay'></span></span>
Using rdiscount (1.6.5) 
Using toto (0.4.9) from https://github.com/yourname/toto.git (at master) 
Using bundler (1.0.0) 
<span class="s">Your bundle is complete! Use `bundle show [gemname]` to see where a bundled gem is installed.</span></pre></div>

You use a simple Ruby DSL in config.ru to specify the rewrite/redirect rules to Rack-Rewrite; here’s an example from my site:

<div class="CodeRay"><pre>
require <span class="s"><span class="dl">'</span><span class="k">rack/rewrite</span><span class="dl">'</span></span> 
use <span class="co">Rack</span>::<span class="co">Rewrite</span> <span class="r">do</span> 
  r301 <span class="s"><span class="dl">'</span><span class="k">/code_buddy</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">/2010/12/13/codebuddy-see-your-ruby-stack-come-alive</span><span class="dl">'</span></span> 
<span class="r">end</span>
</pre></div>

This redirects requests for the “/code_buddy” page to the actual article’s full path. With rack-rewrite I’m able to do this without writing a single line of code.

## <a id="tip4"></a>Take the time to learn and use Toto’s Riot/RR test suite
 
As I mentioned above, Toto’s source code is very concise and elegant... an entire blog application in just 200 or 300 lines of code. Toto’s test suite is equally concise and elegant. Alexis Sellier chose to use [Riot](https://github.com/thumblemonks/riot), an alternative to RSpec, and also [RR](https://github.com/btakita/rr) for mocking and stubbing. The result is a super fast and effective test suite. To use it you’ll probably just need to install Riot, like this:

<div class="CodeRay"><pre>$ gem install riot
Fetching: rr-1.0.2.gem (100%)
Fetching: riot-0.12.1.gem (100%)
Successfully installed rr-1.0.2
Successfully installed riot-0.12.1
2 gems installed</pre></div>

Now from your Toto folder (e.g. “customized_toto”) you can run the test suite just by running “rake:”

<div class="CodeRay"><pre>$ rake
(in /Users/pat/apps/customized_toto)
All dependencies seem to be installed.
/Users/pat/.rvm/rubies/ruby-1.8.7-p302/bin/ruby -I"lib:lib:test" "/Users/pat/.rvm/gems/ruby-1.8.7-p302/gems...
Toto GET /
  + <span class="s">asserts returns a 200 is equal to 200</span>
  + <span class="s">asserts body is not empty</span>
 
etc...</pre></div>

However, I actually got this exception the first time I started working with the Toto Riot tests:

<div class="CodeRay"><pre>/test/../lib/toto.rb:174:in `to_xml': ./test/../lib/ext/ext.rb:43:in `utc': time out of range (ArgumentError)
	from ./test/../lib/ext/ext.rb:43:in `iso8601'
etc...</pre></div>

The problem here is that one of the test articles uses a date in the distant past (1900), which Ruby 1.8.7 chokes on... if you run Ruby 1.9 it will work fine. If you want to use Ruby 1.8.7, you just need to change the test article’s date to something more recent like 1990:

<div class="CodeRay"><pre>$ cd test/articles
$ git mv 1900-05-17-the-wonderful-wizard-of-oz.txt <span class="container">1990<span class="overlay"></span></span>-05-17-the-wonderful-wizard-of-oz.txt</pre></div>

... and also type the corresponding article date inside the test article file like this:

<div class="CodeRay"><pre>title: The Wonderful Wizard of Oz
date: 17/05/<span class="container">1990<span class="overlay"></span></span>

_Once upon a time_...</pre></div>

... and you’ll need to update the corresponding test in test/toto_test.rb on line 58 to use the same date:

<div class="CodeRay"><pre>context <span class="s"><span class="dl">&quot;</span><span class="k">GET a single article</span><span class="dl">&quot;</span></span> <span class="r">do</span> 
  setup { <span class="iv">@toto</span>.get(<span class="s"><span class="dl">&quot;</span><span class="k">/<span class="container">1990<span class="overlay"></span></span>/05/17/the-wonderful-wizard-of-oz</span><span class="dl">&quot;</span></span>) }
  asserts(<span class="s"><span class="dl">&quot;</span><span class="k">returns a 200</span><span class="dl">&quot;</span></span>)                { topic.status }.equals <span class="i">200</span> 
  asserts(<span class="s"><span class="dl">&quot;</span><span class="k">content type is set properly</span><span class="dl">&quot;</span></span>) { topic.content_type }.equals <span class="s"><span class="dl">&quot;</span><span class="k">text/html</span><span class="dl">&quot;</span></span> 
  should(<span class="s"><span class="dl">&quot;</span><span class="k">contain the article</span><span class="dl">&quot;</span></span>)           { topic.body }.includes_html(<span class="s"><span class="dl">&quot;</span><span class="k">p</span><span class="dl">&quot;</span></span> =&gt; <span class="rx"><span class="dl">/</span><span class="k">&lt;em&gt;Once upon a time&lt;em&gt;</span><span class="dl">/</span></span>)
<span class="r">end</span></pre></div>

... and now the tests should pass. For me I was able to run all 61 tests in 0.15 seconds. But even better is that the slow Rails startup time I’m used to is gone; after typing “rake” the tests run and are finished almost immediately... another advantage to working with a simple Rack app vs. something more complex such as Rails.

## What next?

Now that you've got Bundler setup with your version of Toto and the test suite working, you're ready to add something something new to Toto. Next week, I'll walk through how I added support for tagging/categories of articles, for example to get this page to work: [https://patshaughnessy.net/tags/view-mapper](https://patshaughnessy.net/tags/view-mapper)
