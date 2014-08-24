title: "Besides being faster, what else is new in Bundler 1.1?"
tag: Bundler
date: 2011/11/5

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2011/11/4/jeweler.jpg"></td></tr>
  <tr><td align="center"><small><i>Bundler 1.1 can help you sort through your gems</i></small></td></tr>
</table>
</div>

Three weeks ago I explained [why Bundler 1.1 will be much faster](http://patshaughnessy.net/2011/10/14/why-bundler-1-1-will-be-much-faster) than Bundler 1.0, which is clearly its most important new feature. However, there are a few new commands and options the Bundler team implemented in version 1.1 that can help you better manage and sort through the gems you have installed on your development computer and servers. Bundler 1.1’s new commands will tell you which gems you can upgrade, and also help keep things clean by deleting gems you no longer need.

Today I’m going to do a quick walk through of <span class="code">bundle outdated</span>, <span class="code">bundle clean</span>, and <span class="code">bundle install -—standalone</span>, using examples illustrating how to use each one. After you upgrade Bundler you’ll immediately notice the speed improvement, but also be sure to take the time to learn how Bundler 1.1’s new commands can help you better organize your gems.

## Bundle outdated

The most useful and important new command introduced in Bundler 1.1 is <span class="code">bundle outdated</span>, which will identify the outdated gems in your bundle - these are the gems for which there is a newer version available. For example, let’s upgrade from Bundler 1.0 to 1.1.rc, and then run <span class="code">bundle outdated</span> in a somewhat old Rails 3.0.7 app I have lying around from last year:

<pre type="console">
$ gem install bundler --pre
Successfully installed bundler-1.1.rc
1 gem installed
Installing ri documentation for bundler-1.1.rc...
Installing RDoc documentation for bundler-1.1.rc...

$ cd /path/to/an/old/rails/app

$ bundle outdated
Fetching gem metadata from http://rubygems.org/........

Outdated gems included in the bundle:
  * rake (0.9.2.2 > 0.8.7)
  * activesupport (3.1.1 > 3.0.7)
  * builder (3.0.0 > 2.1.2)
  * i18n (0.6.0 > 0.5.0)
  * activemodel (3.1.1 > 3.0.7)
  * erubis (2.7.0 > 2.6.6)
  * rack (1.3.5 > 1.2.4)
  * rack-mount (0.8.3 > 0.6.14)
  * rack-test (0.6.1 > 0.5.7)
  * actionpack (3.1.1 > 3.0.7)
  * mail (2.3.0 > 2.2.19)
  * actionmailer (3.1.1 > 3.0.7)
  * arel (2.2.1 > 2.0.10)
  * activerecord (3.1.1 > 3.0.7)
  * activeresource (3.1.1 > 3.0.7)
  * aws-ses (0.4.3 > 0.3.2)
  * ffi (1.0.10 > 1.0.9)
  * railties (3.1.1 > 3.0.7)
  * rails (3.1.1 > 3.0.7)
  * mysql2 (0.3.7 > 0.2.13)
</pre>

First we see the “Fetching gem metadata...” message appear as Bundler connects to the RubyGems API and gets the latest gem dependency information. See [my article from 3 weeks ago](http://patshaughnessy.net/2011/10/14/why-bundler-1-1-will-be-much-faster) for more details about this. Next, Bundler lists the gems that are out of date, in other words, gems that have newer versions available and that would be updated if I now ran <span class="code">bundle update</span>. In this example, I can see that all of the Rails 3.0.7 gems are out of date, because the Rails 3.1.1 gems are now available. I can also see that during the last year a newer version of rake was released, along with newer versions of mysql2 and other gems related to Rails 3.1.1.

What’s convenient about this is that Bundler displays the gems it would download and install, but without actually doing it. This gives me the freedom to inspect the list and update just the gems I would like to. In this example, I might want the newer version of mysql2 but maybe I don’t want to upgrade to Rails 3.1.1. In that case I can just run <span class="code">bundle update mysql2</span>. With Bundler 1.0, I would probably have just run <span class="code">bundle update</span> to update all of my gems and hoped for the best. If there was a problem - a failing test or a gem version conflict - then I could have used git to start over again.

By default <span class="code">bundle outdated</span> doesn't include prerelease gem versions in the list. Only newer production ready gem versions are listed. If you do want to see what prerelease gems are available, just add the <span class="code">--pre</span> option like this:

<pre type="console">
$ bundle outdated --pre
Fetching gem metadata from http://rubygems.org/........

Outdated gems included in the bundle (including pre-releases):
  * rake (0.9.3.beta.1 > 0.8.7)
  * activesupport (3.1.1 > 3.0.10)
  * builder (3.0.0 > 2.1.2)
  * i18n (0.6.0 > 0.5.0)
  * activemodel (3.1.1 > 3.0.10)
  * erubis (2.7.0 > 2.6.6)
  * rack (1.3.5 > 1.2.4)
  * rack-mount (0.8.3 > 0.6.14)
  * rack-test (0.6.1 > 0.5.7)
  * actionpack (3.1.1 > 3.0.10)
  * mail (2.3.0 > 2.2.19)
  * actionmailer (3.1.1 > 3.0.10)
  * arel (2.2.1 > 2.0.10)
  * activerecord (3.1.1 > 3.0.10)
  * activeresource (3.1.1 > 3.0.10)
  * aws-ses (0.4.3 > 0.3.2)
  * ffi (1.0.10 > 1.0.9)
  * thor (0.15.0.rc2 > 0.14.6)
  * railties (3.1.1 > 3.0.10)
  * rails (3.1.1 > 3.0.10)
  * delayed_job (3.0.0.pre2 > 2.1.4)
  * haml (3.2.0.alpha.8 > 3.1.3)
  * mysql2 (0.3.7 > 0.2.13)
  * ruby-debug-base (0.10.5.rc1 > 0.10.4)
  * ruby-debug (0.10.5.rc1 > 0.10.4)
</pre>

Now I know there is a new "alpha.8" version of haml and a "pre2" version of delayed_job among other prerelease gems available for me to install.

In my opinion, the <span class="code">--pre</span> option is not just a bell or whistle feature, but actually is a fantastic way to keep an eye on what's out there and coming soon that might impact my application. Bundler 1.1 has given me some really valuable knowledge that would have taken lots of Google searching or RubyGems.org browsing to have found otherwise. Aside from planning ahead for changes I'll need to make in my own app, I can also more easily contribute back to the haml, delayed_job or other projects if I find some failing tests or bugs after upgrading.

## A quick review of bundle install --path

Before I continue with Bundler 1.1’s new features, let’s review an option from Bundler 1.0 that isn’t widely used or understood. Normally when you run <span class="code">bundle install</span> Bundler will download and install the new gems into the same location that <span class="code">gem install</span> would: the “system gem repository.” This is a folder inside your Ruby installation directory; on my laptop using RVM and Ruby 1.8.7 this is: “/Users/pat/.rvm/rubies/ruby-1.8.7-p352/lib/ruby/gems/1.8/gems”.

However, if you provide the <span class="code">--path</span> option you can tell Bundler to install the gems into any directory you would like. For example:

<pre type="console">
$ bundle install --path vendor/bundle
</pre>

...would install the new gems into a directory called "vendor/bundle/ruby/1.8/gems." Most people aren’t aware of this option, because it’s not normally that helpful. For normal development work on your own computer it’s usually fine to just use the system gem repository. But there are a few cases when the <span class="code">--path</span> option can be helpful, especially on a server, for example if:
<ul>
<li>you don’t have write permission to the system gem repository, or</li>
<li>you are using Capistrano with Bundler, in which case this option is enabled automatically, or</li>
<li>some of your applications use Bundler and others don’t, or</li>
<li>you want to be absolutely sure each application’s bundle is independent and separate from every other application’s bundle running on that same server.</li>
</ul>

Once you set the bundle path this way, it’s saved into the .bundle/config file as a persistent setting. This insures that any future bundle commands you run in this application use the same bundle path. You can review this value and all your other config settings using the <span class="code">bundle config</span> command:

<pre type="console">
$ bundle config
{^s:Settings are listed in order of priority. The top value will be used.^}

{^s:disable_shared_gems^}
  Set for your local app (/path/to/an/old/rails/app/.bundle/config): "1"

{^s:path^}
  Set for your local app (/path/to/an/old/rails/app/.bundle/config): "vendor/bundle"
</pre>

To remove the path setting, just use the <span class="code">--system</span> option:

<pre type="console">
$ bundle install --system
</pre>

## Automatically cleaning up old gems when --path is set

Another nice feature of Bundler 1.1 is that it automatically cleans up old, unused gems from your system... if you have set the bundle path using <span class="code">--path</span>. If haven’t run <span class="code">bundle install --path</span> and are just using the system gem repository as usual then Bundler won’t clean up unused gems, since other applications on your system might still use them.

Let's take a look at how this works. Continuing with my old Rails 3.0.7 application example, suppose I decide to upgrade from Rails 3.0.7 to Rails 3.0.10. First editing my Gemfile:

<pre type="console">
gem 'rails', '3.0.10'
</pre>

...and then running <span class="code">bundle update</span>:

<pre type="console">
$ bundle update rails
Fetching gem metadata from http://rubygems.org/........
Using rake (0.8.7) 
Using abstract (1.0.0) 
Installing activesupport (3.0.10) 
Using builder (2.1.2) 
Using i18n (0.5.0) 
Installing activemodel (3.0.10) 

...etc...

Using rspec-rails (2.6.0) 
Using ruby-debug-base (0.10.4) 
Using ruby-debug (0.10.4) 
Removing actionmailer (3.0.7)
Removing actionpack (3.0.7)
Removing activemodel (3.0.7)
Removing activerecord (3.0.7)
Removing activeresource (3.0.7)
Removing activesupport (3.0.7)
Removing rails (3.0.7)
Removing railties (3.0.7)
{^s:Your bundle is updated! Use `bundle show [gemname]` to see where a bundled gem is installed.^}
</pre>

Here we can see that Bundler has installed the new gems for Rails 3.0.10, activesupport, activerecord, etc., but also that it has automatically deleted the old gems that are no longer used! Cleaning up old gems helps to save disk space and to keep your gem repository more organized, reducing clutter as over time you periodically upgrade your gems to newer versions.

## Bundle clean

You can also delete unused gems directly by running the <span class="code">bundle clean</span> command. There’s normally no reason to run this since both <span class="code">bundle install</span> and <span class="code">bundle update</span> will run the cleanup code automatically when you have the <span class="code">--path</span> option set.

If you don’t have <span class="code">--path</span> set and you’re using the system gem repository, then you can still use bundle clean to delete all the gems on your system that aren’t used by the current bundle:

<pre type="console">
$ bundle clean
{^error:Can only use bundle clean when --path is set or --force is set^}
</pre>

Here Bundler is preventing you from accidentally deleting all your gems; if you really want to do this use the <span class="code">--force</span> option... <u>warning, don’t try this at home</u>:

<pre type="console">
$ bundle clean --force
Removing actionmailer (3.1.1)
Removing actionpack (3.1.1)
Removing activemodel (3.1.1)
Removing activerecord (3.1.1)
Removing activeresource (3.1.1)
Removing activesupport (3.1.1)
Removing arel (2.2.1)

etc...
</pre>

As you can see, I’ve just broken all the Rails 3.1.1 apps on my laptop! Again, here Bundler deleted all of my gems that were not included in the current application's bundle, which is this case meant any gems not related to Rails 3.0.10.

Using <span class="code">bundle clean --force</span> might be useful if you know you're going to be working on just one Ruby application for a while, and want to clean things up and save disk space. Reducing clutter in your system gem repository can also help to identify legacy Rails applications that aren't using Bundler and rely accidentally on a gem in the system repo in a way you weren't aware of.

## Bundle install --standalone

This is another new option for Bundler 1.1 which allows you to create a bundle that will work on another server or computer that doesn’t even have Bundler installed. Here’s how it works:

<pre type="console">
$ bundle install --standalone
Using rake (0.8.7) 
Using abstract (1.0.0) 
Using activesupport (3.0.9) 
Using builder (2.1.2) 

... etc...

Using rspec (2.7.0) 
Using rspec-rails (2.7.0) 
Using ruby-debug-base (0.10.4) 
Using ruby-debug (0.10.4) 
{^s:Your bundle is complete! It was installed into ./bundle^}
</pre>

This gives me exactly the same local bundle folder I would have gotten by running <span class="code">bundle install --path ./bundle</span>, except that one additional code file “bundle/bundler/setup.rb” is created:

<pre type="console">
$ find bundle | more
bundle
bundle/bundler
bundle/bundler/setup.rb
bundle/ruby
bundle/ruby/1.8
bundle/ruby/1.8/bin
bundle/ruby/1.8/bin/cdiff

... etc...
</pre>

And if you take a look inside this file you’ll see code that manually adds the lib folder from each of the gems in your bundle to the load path:

<pre type="ruby">
path = File.expand_path('..', __FILE__)
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/rake-0.8.7/lib")
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/abstract-1.0.0/lib")
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/activesupport-3.0.10/lib")
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/builder-2.1.2/lib")
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/i18n-0.5.0/lib")
$:.unshift File.expand_path("#{path}/../ruby/1.8/gems/activemodel-3.0.10/lib")
</pre>

This allows you to run your app on another server that doesn’t even have Bundler installed at all. It's also a good way to get a sense of what Bundler actually does normally: constructing a load path based on the gems in your Gemfile.

## Wait, there's even more...

There are also a number of other more minor changes and bug fixes in Bundler 1.1 that I don’t have time to go into here today - check out the [change log](https://github.com/carlhuda/bundler/blob/master/CHANGELOG.md) and also look out for a newer version of the documentation on [http://gembundler.com](http://gembundler.com).
