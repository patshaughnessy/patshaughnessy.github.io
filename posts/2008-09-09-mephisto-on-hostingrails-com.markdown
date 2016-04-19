title: Mephisto Setup on HostingRails.com
date: 2008/09/09
url: /2008/9/9/mephisto-on-hostingrails-com
tag: Ruby

In the past I’ve setup blog sites using [B2Evolution](http://b2evolution.net/), [WordPress](http://wordpress.org/) and other PHP frameworks and had no trouble at all. Last week I setup this web site using [Mephisto](http://mephistoblog.com/) on a Rails hosting service ([hostingrails.com](http://www.hostingrails.com/)), and expected to get it up and running in just a few minutes; after all, if installing and configuring a PHP blog site in 2005 was easy, certainly doing the same in 2008 with better technology (Rails) should be even easier.

Wrong!

First I created a MySQL database on my production server and called it “mephisto”. Then I set the char set and collation settings using the command line:<pre># mysql -u myusername -p
mysql> ALTER DATABASE mydatabasename DEFAULT CHARACTER SET utf8
  COLLATE utf8_unicode_ci</pre>

Since I’m not ready to write my own Rails blog site, I decided to download and setup the latest version of [Mephisto (0.8 “Drax”)](http://mephistoblog.com/download). When I tried to download the TAR file from the 0.8 Drax link using curl I got HTML that redirected me to a different location:

<pre># cat master.tar.gz
&lt;html>&lt;body>You are being &lt;a href="http://github.com/tarballs/technoweenie-
mephisto-90e2cc253d94e2e544bc8b21f361c7360c1e9baa.tar.gz">redirected&lt;/a>.&lt;/body>&lt;/html></pre>

Probably this is standard behavior by github and I should have used that directly. I need to stop thinking like a PHP developer! Regardless, it wasn’t much trouble to download the actual Mephisto TAR file:

<pre># curl -O http://github.com/tarballs/
  technoweenie-mephisto-90e2cc253d94e2e544bc8b21f361c7360c1e9baa.tar.gz
# tar xvf technoweenie-mephisto-90e2cc253d94e2e544bc8b21f361c7360c1e9baa.tar.gz
# mv technoweenie-mephisto-90e2cc253d94e2e544bc8b21f361c7360c1e9baa mephisto</pre>

No big deal; so far this was a very similar experience to setting up a PHP web site: create the database, download the code and you’re almost done. Next, I entered the proper values in database.yml and tried to initialize the database:

<pre># rake db:bootstrap RAILS_ENV=production
(in /home/myusername/mephisto)
Your config/boot.rb is outdated: Run "rake rails:update".</pre>

It was at this point that my problems started. First I wasn’t sure why I received this error message at all. Next I tried to use rails:update to fix it as instructed and ran into further trouble. A couple of hours and 5 or 10 blog posts later I discovered the solution [here in this great write up](http://groups.google.com/group/MephistoBlog/browse_thread/thread/a03e8ac350382e0e). It turns out that Mephisto 0.8 does not work with Rails 2.1 and I needed to downgrade to Rails 2.0.2. Since I’m using a shared server on hostingrails.com I used rails:freeze:edge to get the proper version of Rails installed in my app’s folder:

<pre># rake rails:freeze:edge RELEASE=2.0.2
Downloading Rails from http://dev.rubyonrails.org/archives/rails_2.0.2.zip
Unpacking Rails
Updating current scripts, javascripts, and configuration settings</pre>

At this point I tried to initialize the database again but this time got a different error:

<pre># rake db:bootstrap RAILS_ENV=production
(in /home/myusername/mephisto)
rake aborted!
undefined method 'install_gem_spec_stubs' for #&lt;Rails::Initializer:0xb7c652e8></pre>

As [explained here](http://groups.google.com/group/MephistoBlog/browse_thread/thread/a03e8ac350382e0e), I still had to manually copy one file from Rails to my application which rails:freeze:edge didn’t handle properly, using this command from my app's folder:

<pre># cp vendor/rails/railties/environments/boot.rb config/boot.rb</pre>

Finally I was able to run rake db:bootstrap successfully! The last bit of configuration I had to do was to add this line to config/environment.rb in order to insure hostingrails.com used the production configuration:

<pre># Uncomment below to force Rails into production mode when
# you don't control web/app server and can't set it the proper way
ENV['RAILS_ENV'] ||= 'production'</pre>

After creating a symlink and doing some other [standard configuration for hostingrails.com](http://www.hostingrails.com/forums/wiki_thread/1), I was finally able to get my site up and running. It was a lot harder than configuring a PHP site and ruined a whole evening, but in the end wasn’t all that bad.
