title: "Why Bundler 1.1 will be much faster"
date: 2011/10/14
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/10/14/cheetah.jpg"></td></tr>
  <tr><td align="center"><small><i>Bundler is about to get a lot faster...</i></small></td></tr>
</table>
</div>

If you’ve worked on a Rails 3 application during the past year or so you’ve probably noticed that running <span class="code">bundle install</span> or <span class="code">bundle update</span> can often take a long, long time to finish. The terminal seems to hang for 30 seconds or more after displaying “Fetching source index for http://rubygems.org/.” Well I have some good news for you: the smart people behind Bundler and RubyGems.org have come up with a solution, and the new version of Bundler about to be released is a lot faster! Today I’m going to take a look at exactly why Bundler 1.1 is so much faster - at what exactly the RubyGems/Bundler teams did to speed it up. DELIM

<br/><b>Update November 2011:</b> I had a lot of fun doing a presentation on this topic at the <a href="http://bostonrb.org/">Boston.rb</a> group meeting on November 15th; if you're interested in learning more about Bundler 1.1 you can watch the <a href="http://bostonrb.org/presentations/month/November-2011">online video</a> or <a href="http://www.slideshare.net/pat_shaughnessy/why-bundler-11-will-be-much-faster">download the slides</a>.

## Why Bundler 1.0 was slow

Before we get to Bundler 1.1, why was Bundler 1.0 so slow? What was happening when it displayed “Fetching source index...” and why did it take so long? Nick Quaranto posted a [nice write up back in January](http://robots.thoughtbot.com/post/2729333530/fetching-source-index-for-http-rubygems-org) about this; take a look at that article if you’re interested in learning more about the details. As Nick explained, what happens every time you run Bundler is that it needs to download and process the entire list of gems from RubyGems.org - since there are 10,000s of gems this can take a long, long time. Nick also hints at the solution that I'm about to describe below.

Also since what Bundler does is determine which gems should be included in your application - your “bundle” - based on which gems depend on which other gems, it needs to download all of the gem dependency information as well. Downloading and processing all of this information is what takes 30 seconds or more, depending on your network connection and CPU.

## Is Bundler 1.1 actually faster?

The best way to find out if the new version of Bundler is any faster is just to install it and try it out:

<pre type="console">
$ gem install bundler --pre
Successfully installed bundler-1.1.rc
1 gem installed
Installing ri documentation for bundler-1.1.rc...
Installing RDoc documentation for bundler-1.1.rc...
$ cd /path/to/my/favorite/rails/app
$ bundle update
Fetching gem metadata from http://rubygems.org/.........
Using rake (0.9.2) 
Using multi_json (1.0.3) 
Using activesupport (3.1.1) 
Using builder (3.0.0) 

etc...

Using sass-rails (3.1.4) 
Using sqlite3 (1.3.4) 
Using uglifier (1.0.3) 
Your bundle is updated! Use `bundle show [gemname]` to see where a bundled gem is installed.
</pre>

If you try this, you’ll notice two dramatic differences:

<ol>
  <li>It’s MUCH faster: for me this took a total of only 4 seconds, instead of 30 seconds or more using Bundler 1.0 - a dramatic improvement!</li>
  <li>Instead of “Fetching source index...” and a long delay, I see “Fetching gem metadata from http://rubygems.org/.........” with the dots gradually appearing, almost as if I were running specs.</li>
</ol>

But how does this actually work? And what does “Fetching gem metadata...” mean? What's happening as those dots gradually appear? Let’s take a closer look...

## The RubyGems.org API

We can get some idea of what’s going on here by trying the <span class="code">bundle update</span> command again with the <span class="code">--verbose</span> option enabled:

<pre type="console">
$ bundle update --verbose
Fetching gem metadata from http://rubygems.org/
Query List: ["rails", "sqlite3", "json", "sass-rails", "coffee-rails", "uglifier", "jquery-rails"]
Query Gemcutter Dependency Endpoint API: rails sqlite3 json sass-rails coffee-rails uglifier jquery-rails
Fetching from: http://rubygems.org/api/v1/dependencies?gems=rails,sqlite3,json,sass-rails,coffee-rails,uglifier,jquery-rails
HTTP Success
Query List: ["bundler", "railties", "actionmailer", "activeresource", "activerecord", "actionpack", "activesupport", "rake", "actionwebservice", "ffi", "sprockets", "tilt", "sass", "coffee-script", "multi_json", "execjs", "therubyracer", "thor"]
Query Gemcutter Dependency Endpoint API: bundler railties actionmailer activeresource activerecord actionpack activesupport rake actionwebservice ffi sprockets tilt sass coffee-script multi_json execjs therubyracer thor
Fetching from: http://rubygems.org/api/v1/dependencies?gems=bundler,railties,actionmailer,activeresource,activerecord,actionpack,activesupport,rake,actionwebservice,ffi,sprockets,tilt,sass,coffee-script,multi_json,execjs,therubyracer,thor
HTTP Success

etc...
</pre>

Ah - now we can see how Bundler 1.1 works: it’s calling an HTTP API provided by RubyGems.org to obtain the dependency information for a short list of specific gems, instead of downloading the entire source index. If you paste one of those URLs into a browser you can see the dependencies for the specified gems encoded in some complex format that resembles JSON. It turns out that it’s not JSON but instead a string generated by Ruby’s [Marshal library](http://ruby-doc.org/core-1.8.7/Marshal.html).

To get a sense of how the RubyGems API works, I wrote this simple script that uses [HTTParty](http://httparty.rubyforge.org/) and Marshal to display the dependencies for any gem you specify:

<pre type="ruby">
require 'rubygems'
require 'httparty'

class RubyGemsApi
  include HTTParty
  base_uri 'rubygems.org'

  def self.info_for(gems)
    res = get('/api/v1/dependencies', :query => { :gems => gems })
    Marshal.load(res)
  end

  def self.display_info_for(gems)
    info_for(gems).each do |info|
      puts "#{info[:name]} version #{info[:number]} dependencies: #{info[:dependencies].inspect}"
    end
  end
end

RubyGemsApi.display_info_for(ARGV[0])
</pre>

Running this using my favorite example gem “uglifier:”

<pre type="console">
$ ruby parse_rubygems_api.rb uglifier
uglifier version 1.0.3 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 1.0.2 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 1.0.1 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 1.0.0 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 0.5.4 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 0.5.3 dependencies: [["multi_json", ">= 1.0.2"], ["execjs", ">= 0.3.0"]]
uglifier version 0.5.2 dependencies: [["multi_json", ">= 0"], ["execjs", ">= 0.3.0"]]
uglifier version 0.5.1 dependencies: [["json", ">= 0"], ["execjs", ">= 0"]]
uglifier version 0.5.0 dependencies: [["json", ">= 0"], ["execjs", "~> 0.1.0"]]
uglifier version 0.4.0 dependencies: [["therubyracer", "~> 0.8.0"]]
uglifier version 0.3.0 dependencies: [["therubyracer", ">= 0.8.0"]]
uglifier version 0.2.0 dependencies: []
uglifier version 0.1.1 dependencies: []
uglifier version 0.1.0 dependencies: []
</pre>

Notice that the response includes the dependencies for each version of the gem, not just the latest version. This is necessary since Bundler’s resolver algorithm might need to use an older version of the gem, depending on what other gems are included in the bundle.

You can also provide a list of comma separated gem names, such as what we see Bundler doing in the verbose output above:

<pre type="console">
$ ruby parse_rubygems_api.rb rails,sqlite3,json,sass-rails,coffee-rails,uglifier,jquery-rails
</pre>

This HTTP API call is very fast - less than a second - and provides all the information Bundler needs to have, and nothing more.

## Visualizing Bundler 1.1’s algorithm for downloading dependencies

Let’s use the same simple example Gemfile that I did three weeks ago in my article [How does Bundler bundle](https://patshaughnessy.net/2011/9/24/how-does-bundler-bundle):

<pre type="ruby">
source 'http://rubygems.org'
gem 'uglifier’
</pre>

And I’ll draw a simple diagram that represents the contents of this Gemfile; in other words just a single gem:

![single gem](https://patshaughnessy.net/assets/2011/10/14/one.png)

Next, I’ll run <span class="code">bundle update --verbose</span> in the folder containing this Gemfile and take a look at the output; but this time I’ll intersperse the output text with more diagrams showing what’s actually going on inside Bundler’s dependency-fetch algorithm:

<pre type="console">
$ bundle update --verbose
Fetching gem metadata from http://rubygems.org/
</pre>

![first HTTP request](https://patshaughnessy.net/assets/2011/10/14/two.png)

<pre type="console">
Query List: ["uglifier"]
Query Gemcutter Dependency Endpoint API: uglifier
Fetching from: http://rubygems.org/api/v1/dependencies?gems=uglifier
HTTP Success
</pre>

We can see here the first thing that happens is a HTTP request to determine the dependencies of the “uglifier” gem, the only gem in my Gemfile. You can see the results of this HTTP request above in the output of the parse_rubygems_api.rb script: that there are four gems that all of the different versions of uglifier depend on:

![first HTTP request result](https://patshaughnessy.net/assets/2011/10/14/three.png)

The latest version of uglifier depends on json, execjs and multi_json, while some older versions of uglifier depend on “therubyracer” gem.

The next thing Bundler will do is send a second HTTP request to RubyGems.org asking for the dependencies of these four gems:

![second HTTP request](https://patshaughnessy.net/assets/2011/10/14/four.png)

<pre type="console">
Query List: ["multi_json", "execjs", "json", "therubyracer"]
Query Gemcutter Dependency Endpoint API: multi_json execjs json therubyracer
Fetching from: http://rubygems.org/api/v1/dependencies?gems=multi_json,execjs,json,therubyracer
HTTP Success
</pre>

If you’re interested you can run parse_rubygems_api.rb to see the results of this request. Here’s a diagram of what RubyGems returns:

![second HTTP request result](https://patshaughnessy.net/assets/2011/10/14/five.png)

This time we can see that the “execjs” gem has some versions that depend on “multi_json” and that “therubyracer” depends on “libv8.” Finally, Bundler continues by sending a third HTTP request to RubyGems.org to get the dependencies of libv8 - multi_json is not included since we already have its info:

![third HTTP request result](https://patshaughnessy.net/assets/2011/10/14/six.png)

<pre type="console">
Query List: ["libv8"]
Query Gemcutter Dependency Endpoint API: libv8
Fetching from: http://rubygems.org/api/v1/dependencies?gems=libv8
HTTP Success
</pre>

This time RubyGems.org returns an empty set, since libv8 has no dependencies at all:

![third HTTP request result](https://patshaughnessy.net/assets/2011/10/14/seven.png)

<pre type="console">
Query List: []
Unmet Dependencies: 
</pre>

Now Bundler has all the dependency information it needs, so it continues by running the resolver algorithm and then outputs the gems that were included in my new bundle:

<pre type="console">
Using multi_json (1.0.3) from /Users/pat/.rvm/gems/ruby-1.8.7-p352/specifications/multi_json-1.0.3.gemspec 
Using execjs (1.2.9) from /Users/pat/.rvm/gems/ruby-1.8.7-p352/specifications/execjs-1.2.9.gemspec 
Using uglifier (1.0.3) from /Users/pat/.rvm/gems/ruby-1.8.7-p352/specifications/uglifier-1.0.3.gemspec 
Using bundler (1.1.rc) from /Users/pat/.rvm/gems/ruby-1.8.7-p352/specifications/bundler-1.1.rc.gemspec 
</pre>
