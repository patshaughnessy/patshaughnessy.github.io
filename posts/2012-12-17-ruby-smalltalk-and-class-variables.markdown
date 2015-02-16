title: "Ruby, Smalltalk and Class Variables"
date: 2012/12/17

<div style="float: left; padding: 17px 30px 10px 0px;
line-height:16px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td align="center"><img src="http://patshaughnessy.net/assets/2012/12/17/bluebook-and-ruby.png"></td></tr>
    <tr><td align="center"><i>Many of the ideas behind Ruby’s object model<br/>were developed for Smalltalk in the 1970s.</i></td></tr>
  </table>
</div>

A couple weeks ago [an article by Ernie
Miller](http://erniemiller.org/2012/11/29/ruby-tidbit-include-vs-extend-with-module-class-variables/)
got me interested in how class variables work in Ruby. After doing a bit of
research, I found that class variables have been a perennial source of
confusion. In fact, John Nunemaker wrote an article called [Class and Instance
Variables
In Ruby](http://railstips.org/blog/archives/2006/11/18/class-and-instance-variables-in-ruby/)
way back in 2006 that still applies today. The fundamental problem with class
variables in Ruby is that they are shared among a class and all of its
subclasses - as John explained six years ago, this can lead to confusion and
unexpected behavior.

But for me the interesting question here is: “Why?” Why does Ruby share a single
value across all of the subclasses? Why have a distinction between “class
variables” and “class instance variables?” Where do these ideas come from? It
turns out the answer is simple: class variables in Ruby work the same way class
variables work in a much older language called
[Smalltalk](http://en.wikipedia.org/wiki/Smalltalk). Smalltalk was invented in
the early 1970s by the renown computer scientist [Alan
Kay](http://en.wikipedia.org/wiki/Alan_Kay) and a group of his colleagues
working at the [Xerox PARC](http://www.parc.com) laboratory. With
Smalltalk, Alan Kay didn’t just invent a programming language; he also
conceived of the entire concept of object oriented programming (OOP) and
implemented it for the first time.  While not in very widespread use now,
Smalltalk has influenced many other object oriented programming languages that
are used widely today - most importantly Objective C and Ruby.

Today I’m going to look at how class variables work in Smalltalk, and compare and
contrast that against how they work in Ruby. As you’ll see, I found that class
variables aren’t the only idea Ruby took from Smalltalk. Much of Ruby’s object
model design was taken from Smalltalk as well.

## Class variables in Ruby

First, let’s quickly review what a class variable is, and how they work in Ruby. Using
[John Nunemaker’s example from
2006](http://railstips.org/blog/archives/2006/11/18/class-and-instance-variables-in-ruby/),
here’s a simple Ruby class, <span class="code">Polygon</span>, that contains a single class variable,
<span class="code">@@sides</span>:

<pre type="ruby">
class Polygon
  @@sides = 10
  def self.sides
    @@sides
  end
end

puts Polygon.sides
=> 10
</pre>

This is simple enough: <span class="code">@@sides</span> is a variable that any class or instance
method of <span class="code">Polygon</span> can access. Here the <span class="code">sides</span> class method returns it. At a
conceptual level, internally Ruby associates the <span class="code">@@sides</span> variable with the
same memory structure used to represent the <span class="code">Polygon</span> class:

<img src="http://patshaughnessy.net/assets/2012/12/17/polygon.png"/>

The confusion comes in when you define a subclass; again here is another one of
John Nunemaker’s examples:

<pre type="ruby">
class Triangle < Polygon
  @@sides = 3
end

puts Triangle.sides
#=> 3
puts Polygon.sides
#=> 3
</pre>

Notice both class variables, <span class="code">Triangle.sides</span> and <span class="code">Polygon.sides</span>, were changed to
3. In fact, internally Ruby creates a single variable that both classes share:

<img src="http://patshaughnessy.net/assets/2012/12/17/polygon-and-triangle.png"/>

I may write in more detail about the details of Ruby’s internal implementation
of class variables in an upcoming blog post, but for today I’ll just use these
very simple diagrams. Instead, now let’s switch gears and learn more about
Smalltalk….

## What is Smalltalk?

As I said above, Alan Kay invented Smalltalk along with object oriented
programming while working at Xerox PARC in the early 1970s. This is the same
laboratory that also invented the personal computer, the graphical user
interface, and the Ethernet among many other things. Object oriented
programming actually seems to be one of their less important inventions!

In Smalltalk, Kay introduced terminology and ideas that we all take for granted
today. Every value in Smalltalk, including language constructs such as code
blocks, is an object. A Smalltalk program consists of these objects and the
way they interact; to call a particular Smalltalk function, you “send a
message” to the object that implements that function. In Smalltalk, functions
are known as “methods.” An object implements a series of methods. All of this
should sound very familiar, of course.

<div style="float: left; padding: 17px 30px 10px 0px;
line-height:16px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td align="center"><img src="http://patshaughnessy.net/assets/2012/12/17/children.png"></td></tr>
    <tr><td align="center"><i>

In the 1970s, Alan Kay envisioned a laptop/tablet he called
the<br/>
“Dynabook” would run Smalltalk. He and his team actually built
a<br/>
computer called the “Interim Dynabook” and used it to teach<br/>
programming to middle school children.

    </i></td></tr>
  </table>
</div>

From the very beginning, Kay’s conception of OOP included the idea of an
object’s “class.” An object’s class described a series of behaviors (methods)
each instance of that class exhibited.  Smalltalk also implemented the concept
of polymorphism, which allows the developer to define “subclasses” that share
the behaviors of their “superclass.” All of these terms we use often today were
coined by Kay and his colleagues 40 years ago.

Smalltalk, however, is more than just a programming language; it’s an entire
graphical development environment. I think of Smalltalk as a precursor to
Visual Studio or XCode, invented before Microsoft or Apple even existed, in a
world where computers were found only in academic or government settings! One
other impressive goal Alan Kay and the Smalltalk team had from the beginning
was to use their visual environment as a teaching tool for school children.
It’s a truly amazing story.

To learn more about the history and origin of Smalltalk, I would highly
recommend reading <i>The Early History Of Smalltalk</i>
([html](http://www.smalltalk.org/smalltalk/TheEarlyHistoryOfSmalltalk_Abstract.html)
or [original
pdf](http://www.smalltalk.org/downloads/papers/SmalltalkHistoryHOPL.pdf) or
[easier to read pdf, but missing some
diagrams](http://samizdat.cc/shelf/documents/2004/08.02-historyOfSmalltalk/historyOfSmalltalk.pdf)),
a retrospective account Kay wrote later in the 1990s. It’s a fascinating
narrative of how Kay and his colleagues borrowed ideas from even earlier, but
with the combination of hard work, creativity and pure talent managed to take a
large step forward and revolutionize the computer science world of their day,
and ours.

Alan Kay created the first working version of Smalltalk in 1972 - in his own
words, here is how it happened:

<blockquote style="line-height:16px">
I had expected that the new Smalltalk would be an iconic language and would
take at least two years to invent, but fate intervened. One day, in a typical
PARC hallway bullsession, Ted Kaehler, Dan Ingalls, and I were standing around
talking about programming languages. The subject of power came up and the two
of them wondered how large a language one would have to make to get great
power. With as much panache as I could muster, I asserted that you could define
the "most powerful language in the world" in "a page of code." They said, "Put
up or shut up." Ted went back to CMU but Dan was still around egging me on. For
the next two weeks I got to PARC every morning at four o'clock and worked on
the problem until eight, when Dan, joined by Henry Fuchs, John Shoch, and Steve
Purcell showed up to kibbitz the morning's work.  I had originally made the
boast because McCarthy's self-describing LISP interpreter was written in
itself. It was about "a page", and as far as power goes, LISP was the whole
nine-yards for functional languages. I was quite sure I could do the same for
object-oriented languages….
</blockquote>

Here Kay referred to [John
McCarthy](http://en.wikipedia.org/wiki/John_McCarthy_\(computer_scientist\)), who
invented LISP about 10 years earlier.  It took Kay only eight early mornings of
work to finish the first version of Smalltalk:

<blockquote style="line-height:16px">
The first few versions had flaws that were soundly criticized by the group. But
by morning 8 or so, a version appeared that seemed to work….
</blockquote>

I wish I could be as creative, dedicated and productive as Alan Kay and his
Xerox PARC colleagues were 40 years ago!

## Class variables in Smalltalk 

To find out how class variables actually work in Smalltalk, I
installed [GNU
Smalltalk](http://smalltalk.gnu.org), a command line based
version of the language which is easy to download and run on a
Linux box. Initially I found Smalltalk to be very strange and
unfamiliar; it’s syntax seems a bit odd and weird at first
glance. For example, you need to remember to end each command
with a period, and also to define a method you only need to
specify a list of arguments… without a method name! I suppose the
first argument is the method name or vice-versa. But after a
couple of days I became accustomed to the idiosyncratic syntax,
and the language began to make more sense to me.

Here is the same <span class="code">Polygon</span> class again - now I have Smalltalk on the left, and
Ruby on the right:

<pre type="smalltalk" style="display: inline-block; width: 40%;">
Object subclass: Polygon [
  Sides := 10.
]

Polygon class extend [
  sides [ ^Sides ]
]

Polygon sides printNl.
=> 10
</pre>
<pre type="ruby" style="display: inline-block; width: 40%;">
class Polygon
  @@sides = 10
  def self.sides
    @@sides
  end
end


puts Polygon.sides
#=> 10
</pre>

Here’s a quick explanation of what the Smalltalk code does:

* <span class="code">Object subclass: Polygon</span> - this means send the <span class="code">subclass</span>
message to the <span class="code">Object</span> class and pass in the name <span class="code">Polygon</span>. It
creates a new class, which is a subclass of the <span class="code">Object</span> class.
This is analogous to <span class="code">class Polygon &lt; Object</span> in Ruby. Of
course, in Ruby specifying <span class="code">Object</span> as the superclass is
unnecessary.

* <span class="code">Sides := 10.</span> - this declares a class variable <span class="code">Sides</span>, and
assigns it a value. Ruby instead uses the <span class="code">@@sides</span> syntax.

* <span class="code">Polygon class extend</span> - this “extends” the <span class="code">Polygon</span> class;
i.e., it opens up the <span class="code">Polygon</span> class and allows me to add a class
method.  In Ruby I use <span class="code">class Polygon; def self.sides</span>.

* The <span class="code">printNl</span> method prints a value to the console; it works
the same way as <span class="code">puts</span> in Ruby, except <span class="code">printNl</span> is a method of
the <span class="code">Sides</span> object. Imagine calling <span class="code">@@sides.puts</span> in Ruby!

Aside from the superficial syntax differences, if you take a step back and
think about this, it’s striking how similar Smalltalk and Ruby really are! Not
only do both languages share the same class variable concept, but I wrote the
<span class="code">Polygon</span> class, declared a class variable and printed it out exactly the same
way in both languages. In fact, you can think of Ruby as a newer version of
Smalltalk with a simpler, easier to use syntax!

As I said at the top, Smalltalk shares class variables among subclasses the
same way Ruby does. Here’s how I would declare the Triangle subclass in
Smalltalk and Ruby:

<pre type="smalltalk" style="display: inline-block; width: 40%;">
Polygon subclass: Triangle [
]
Triangle class extend [
  set_sides: num [ Sides := num ]
]
Polygon sides printNl.
=> 10 

</pre>
<pre type="ruby" style="display: inline-block; width: 40%;">
class Triangle < Polygon
  def self.sides=(num)
    @@sides = num
  end
end

puts Triangle.sides
#=> 10
</pre>

Here I declare the <span class="code">Triangle</span> subclass and a method to set the class variable’s
value. Now let’s try changing the value of the class variable from the
subclass:

<pre type="smalltalk" style="display: inline-block; width: 40%;">
Triangle set_sides: 3.
Triangle sides printNl.
=> 3</pre>
<pre type="ruby" style="display: inline-block; width: 40%;">
Triangle.sides = 3
puts Triangle.sides
=> 3</pre>

No surprise; by calling the <span class="code">set_slides</span> class method (<span class="code">sides=</span> in Ruby) I can
update the value. But notice since both <span class="code">Polygon</span> and <span class="code">Triangle</span> share the same
class variable, it’s changed for <span class="code">Polygon</span> also:

<pre type="smalltalk" style="display: inline-block; width: 40%;">
Polygon sides printNl.
=> 3
</pre>
<pre type="ruby" style="display: inline-block; width: 40%;">
puts Polygon.sides
=> 3
</pre>

Again, we’ve seen Ruby and Smalltalk behave in exactly the same way.

One way the two languages differ is that Smalltalk does allow you to create a
separate class variable for each subclass, if you want. By repeating the class
variable definition and the accessor class method in both classes they become
separate variables, at least in GNU Smalltalk which I was using:

<div class="CodeRay">
  <div class="code"><pre>Object <span class="r">subclass:</span> Polygon [
  Sides := <span class="cl">10</span>.
]

Polygon <span class="r">class</span> extend [
  sides [ ^Sides ]
]

Polygon <span class="r">subclass:</span> Triangle [
  Sides := <span class="cl">3</span>.
]

Triangle <span class="r">class</span> extend [
  sides [ ^Sides ]
]

Polygon sides printNl.
>= 10

Triangle sides printNl.
>= 3
</pre></div>
</div>

This isn’t true in Ruby; as we saw above <span class="code">@@sides</span> will always refer to the
same value.

## Class instance variables

In Ruby if you want to keep a separate value for each class, then you need to
use a class instance variable instead of a class variable. What does this mean?
Let’s take a look at another one of John Nunemaker’s examples:

<pre type="ruby" style="display: inline-block; width: 40%;">
class Polygon
  def self.sides
    @sides
  end
  @sides = 8
end

puts Polygon.sides
#=> 8
</pre>

Now since I used the <span class="code">@sides</span> notation instead of <span class="code">@@sides</span>, Ruby created an
instance variable instead of a class variable:

<img
src="http://patshaughnessy.net/assets/2012/12/17/polygon-instance.png"/>

Conceptually there’s no difference, until I create the <span class="code">Triangle</span> subclass again:

<pre type="ruby">
class Triangle < Polygon
  @sides = 3
end
</pre>

Now each class has its own copy of the value <span class="code">@sides</span>:

<img
src="http://patshaughnessy.net/assets/2012/12/17/polygon-and-triangle-instance.png"/>

Now let’s try the same thing in Smalltalk. In Smalltalk to declare an instance
variable you call the <span class="code">instanceVariableNames</span> method on a class:

<pre type="smalltalk" style="display: inline-block; width: 40%;">
Object subclass: Polygon [
]

Polygon instanceVariableNames: 'Sides '!

Polygon extend [
  sides [ ^Sides ]
]
</pre>
<pre type="ruby" style="display: inline-block; width: 40%;">
class Polygon
  def sides
    @sides
  end
end



</pre>

Here I’ve created a new class <span class="code">Polygon</span>, a subclass of <span class="code">Object</span>. Then I send
the <span class="code">instanceVariableNames</span> message to this new class, telling Smalltalk to
create a new instance variable called <span class="code">Sides</span>. Finally, I reopen the <span class="code">Polygon</span>
class and add the <span class="code">sides</span> method to it. Again I show the corresponding Ruby
code on the right.

However, here <span class="code">Sides</span> and <span class="code">@sides</span> are instance variables of <span class="code">Polygon</span> objects,
and not of the <span class="code">Polygon</span> class. To create a class instance variable in Smalltalk,
you instead have to send the <span class="code">class</span> message to <span class="code">Polygon</span> first before calling
<span class="code">instanceVariableNames</span> or <span class="code">extend</span>, like this:

<pre type="smalltalk" style="display: inline-block; width: 50%;">
Object subclass: Polygon [
]

Polygon class instanceVariableNames: 'Sides '!

Polygon class extend [
  sides [ ^Sides ]
]
</pre>
<pre type="ruby" style="display: inline-block; width: 30%;">
class Polygon
  def self.sides
    @sides
  end
end



</pre>

Again, notice that the Smalltalk and Ruby code snippets are really just two
different ways of expressing the same commands. In Smalltalk you say <span class="code">Polygon
class extend [ sides...</span> while in Ruby you say <span class="code">class Polygon; def self.sides</span>.
To me Ruby seems to be a more succinct version of Smalltalk.

## Metaclasses in Smalltalk and Ruby

<div style="float: right; padding: 17px 0px 10px 30px;
line-height:16px">
  <table cellpadding="0" cellspacing="0" border="0">
    <tr><td align="center"><img
    src="http://patshaughnessy.net/assets/2012/12/17/metaphysics.png"></td></tr>
    <tr><td align="center"><i>
This diagram, taken from Alan Kay’s fascinating article <a
href="http://www.smalltalk.org/downloads/papers/SmalltalkHistoryHOPL.pdf">The
Early<br/>
History Of Smalltalk</a>, resembles the class hierarchy Ruby
would<br/>
use 20 years later! 
    </i></td></tr>
  </table>
</div>

Let’s take another look at the line of code I used above to create an instance
variable in Smalltalk:

<div class="CodeRay">
  <div class="code"><pre>Polygon <span class="r">instanceVariableNames:</span> <span class="pc">'Sides '</span>!
</pre></div>
</div>

Translating from Smalltalk into English, this means:

* Take the <span class="code">Polygon</span> class,

* send it a message called <span class="code">instanceVariableNames</span>,

* and pass the string <span class="code">Sides</span> as a parameter.

Again, this is how you define instance variables in Smalltalk. That is, now
when I create instances of the <span class="code">Polygon</span> class, they will each have a <span class="code">Sides</span>
instance variable. Saying the same thing in a different way, to give all
polygon instances an instance variable, I call a method on the <span class="code">Polygon</span> class.

As I explained above, to create a class instance variable in Smalltalk you have
to use the <span class="code">class</span> keyword, like this:

<div class="CodeRay">
  <div class="code"><pre>Polygon class <span class="r">instanceVariableNames:</span> <span class="pc">'Sides '</span>!
</pre></div>
</div>

This code literally means: call the <span class="code">instanceVariableNames</span> method on the
<span class="code">Polygon</span> class’s class. Following the same pattern, now all instances of the
<span class="code">Polygon</span> class will contain a class instance variable. But what is the “class of
the <span class="code">Polygon</span> class” in Smalltalk? What does this mean? Spending just a moment at
the GNU Smalltalk REPL we can find out:

<div class="CodeRay">
  <div class="code"><pre>$ gst
GNU Smalltalk ready

st> Polygon printNl.
=> Polygon

st> Polygon class printNl.
=> Polygon class
</pre></div>
</div>

I first display the <span class="code">Polygon</span> class object, and I get “Polygon”. Displaying the
class of the <span class="code">Polygon</span> class, I get “Polygon class.” But what type of object is
this? Let’s call <span class="code">class</span> on it:

<div class="CodeRay">
  <div class="code"><pre>st> Polygon class class printNl.
=> Metaclass
</pre></div>
</div>

Ah… so the class of a class is a <span class="code">Metaclass</span>. Above, when I called
<span class="code">instanceVariableNames</span> to create a class instance variable, I was actually
using the <span class="code">Polygon</span> metaclass, an instance of the <span class="code">Metaclass</span> class.

Here’s a diagram showing how these classes are all related in Smalltalk:

<img
src="http://patshaughnessy.net/assets/2012/12/17/metaclasses-smalltalk.png"/>

By now, it should be no surprise if I tell you internally Ruby uses the same
model. Here’s how classes work inside of Ruby:

<img
src="http://patshaughnessy.net/assets/2012/12/17/metaclasses-ruby.png"/>

In Ruby whenever you create a class, Ruby internally creates a corresponding
new class called the “metaclass.” Unlike Smalltalk, Ruby doesn’t use this for
class instance variables, but only to keep track of class methods. Also, Ruby
doesn’t have a <span class="code">Metaclass</span> class, but instead all metaclasses are simply
instances of the <span class="code">Class</span> class.

In Ruby the metaclass is a hidden, mysterious concept. Ruby silently creates it
without telling you and doesn’t expose the metaclass in the language directly.
In Smalltalk, however, the metaclasses are not hidden at all and instead play a
large role in the language. Creating a class instance variable, as I did above,
is just one example of using a metaclass in Smalltalk. Another good example is
the way you add class methods by calling <span class="code">extend</span>.

When you ask for a class’s class in Ruby, you simply get <span class="code">Class</span>. Ruby doesn’t
tell you about the metaclass:

<pre type="console">
$ irb
> class Polygon; end
> Polygon.class
Class
</pre>

To see a Ruby metaclass, you have to use a trick instead:

<pre type="console">
$ irb
> class Polygon
>   def self.metaclass
>     class << self
>       self
>     end
>   end
> end
=> nil
> Polygon.metaclass
=> #<Class:Polygon>
</pre>

“#<Class:Polygon>” is the metaclass of <span class="code">Polygon</span>. This syntax means
the metaclass is “an instance of <span class="code">Class</span> for the <span class="code">Polygon</span> class,” or
the metaclass for <span class="code">Polygon</span>.

<p></p>
<blockquote>
* Quoted text and images from: The Early History Of Smalltalk, by Alan Kay, © 1993 Association for Computing Machinery
</blockquote>
