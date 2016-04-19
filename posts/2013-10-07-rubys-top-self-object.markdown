title: "Ruby’s Top Self Object"
date: 2013/10/7
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px; text-align: center; margin-top: 20px">
  <img src="http://patshaughnessy.net/assets/2013/9/15/smalltalk-and-ruby.png"><br/>
  <i>Creating functions without a class is<br/>
  awkward in Smalltalk, but simple in Ruby.</i>
</div>

Much of Ruby’s implementation of objects and classes is modeled after
Smalltalk, one of the original object oriented languages first built in the
late 1960s. Just like with Smalltalk, Ruby’s <span class="code">Object</span> class is the root of your
program’s class hierarchy, and all Ruby classes are instances of the <span class="code">Class</span>
class. Smalltalk blocks and Ruby blocks also both support using anonymous
functions as closures.

But in one interesting way Ruby and Smalltalk differ. Ruby allows you to define
simple functions at the top level scope. This enables Ruby to serve as a
_scripting language_. Using Ruby, it’s very easy to combine a few functions
together in a small script to accomplish some simple command line task. At the
same time, Ruby’s has Smalltalk’s sophisticated OO design at the ready, waiting
for you to use it. When your script gets a bit more complex, you can easily
turn it into a more organized, object oriented program.

How does Ruby do this? Before your script starts to run, Ruby automatically
creates a hidden object known as the top self object, an instance of the <span class="code">Object</span>
class. This object serves as the default receiver for top level methods. Today
we’ll see how this object – the object we didn’t even know we were using –
allows us to write simple functions in an object oriented language.

Read the full article on [sitepoint.com](http://www.sitepoint.com/rubys-top-self-object/).
