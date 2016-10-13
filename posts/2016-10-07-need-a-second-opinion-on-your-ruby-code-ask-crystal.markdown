title: "Need a Second Opinion on Your Ruby Code? Ask Crystal"
date: 2016/10/7
tag: Ruby

<div style="float: left; padding: 0px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2016/10/7/x-ray1.jpg"><br/>
  <i>Running the Crystal compiler on your Ruby code<br/>is like asking a second doctor for their opinion.</i><br/>
</div>

When it comes to your health, you don’t hesitate to get a second opinion.
Doctors don’t always agree, and a second doctor’s appointment is always time
well spent when it comes to staying healthy.

But what about your code? A code review is similar to going to see a doctor:
Someone examines your code, looks for potential problems and hopefully gives
you some advice you can take away. Sadly, however, we don’t always have the
time or opportunity for a real code review.

Recently I’ve been learning about the [Crystal programming
language](https://crystal-lang.org), a variation on Ruby syntax implemented on
the [LLVM platform](http://llvm.org). What’s interesting about Crystal is that
it uses static types while at the same time retaining much of Ruby’s original
elegance and natural feel. The two languages are so similar, in fact, it’s
possible to use the Crystal compiler to parse your Ruby code after making just
a few superficial changes. This can be a great way to get helpful feedback on
your Ruby code, a free code review from a dramatically different perspective.

Using a compiler for one language on code from another sounds crazy. Will it
really work? To find out, let’s look at a simple example.

## Rock Stars

Here’s a Ruby class that represents the lead singer of a rock band, and a
couple of methods that use it:

<pre type="ruby">
class Singer

  attr_reader :band, :first_name, :last_name

  def initialize(band, first_name, last_name)
    @band = band
    @first_name = first_name
    @last_name = last_name
  end 

end

def lead_singer_for(band, singers)
  singers.find{|s| s.band == band}
end

def longest_last_name(singers)
  singers.map{|s| s.last_name}.max_by{|name| name.size }
end
</pre>

This is similar to Ruby code I write everyday: small classes containing a few
instance variables, and short, simple methods. With some test data we can try
out this code to see if it works:

<pre type="ruby">
lead_singers = [ 
  Singer.new("The Rolling Stones", "Mick", "Jagger"),
  Singer.new("Queen", "Freddie", "Mercury"),
  Singer.new("The Doors", "Jim", "Morrison")
]

singer = lead_singer_for('The Doors', lead_singers)
puts "#{singer.first_name} #{singer.last_name}"
# => Jim Morrison

puts longest_last_name(lead_singers)
# => Morrison
</pre>

Everything works well. On a real project I’d express this as a series of
Minitest expectations, and seeing green I’d go ahead and check it into Git on a
branch and ask a colleague for a code review.

But what if no one is around or even awake in my time zone? Or what if I’m
working alone on this? Well, I’d have to review my own code alone.

##  Code Reviewing Yourself

I believe in the medical world doctors have a legal or at least an ethical
prohibition on treating themselves, for obvious reasons. And just as giving
yourself a physical exam makes no sense, reviewing your own code doesn’t
either. You don’t have perspective on what you wrote, especially just after you
finish writing it. Usually a fresh pair of eyes will see mistakes that you
can’t see.

But in this case I have no choice - I decide to review my own code before
checking it in. And right away I find a problem: I call `find` but never consider
whether the return value could be `nil`:

<pre type="ruby">
def lead_singer_for(band, singers)
  singers.find{|s| s.band == band}
end
</pre>

In my test, I happened to pick a band name that existed in the test data set,
but if I misspell it or look for a different band, I would get an error:

<pre type="ruby">
singer = lead_singer_for('Doors', lead_singers)
puts "#{singer.first_name} #{singer.last_name}"
# => undefined method `first_name' for nil:NilClass (NoMethodError)
</pre>

I make this sort of mistake quite often, actually. In fact, I do it so often
that checking for `nil` after calling `find` is part of my mental checklist for
code reviews.

## Superficial Syntax Differences: Crystal vs. Ruby 

But suppose I was tired or in a rush; I might not have noticed the call to
`find`. And often forgetting to check for a `nil` return value isn’t as obvious as
it is here in this example. What if there was a way to find code issues the
Ruby interpreter doesn’t report? Imagine if this code review could happen before
my code is ever deployed or used?

There is; we just need to run my Ruby code through the Crystal compiler:

<pre class="console">
$ cp lead_singers.rb lead_singers.cr
$ crystal lead_singers.cr
</pre>

What? Pat, this is nuts. Crystal, while superficially similar to Ruby, is a
very different language. How in the world can I use a compiler written for one
language on code written in another?

Well, you’re right. I run into a syntax error immediately:

<pre class="console">
$ crystal lead_singers.cr 
Syntax error in ./lead_singers.cr:27: unterminated char literal, use double quotes for strings

singer = lead_singer_for('Doors', lead_singers)
                         ^
</pre>

The most common difference of all between Crystal and Ruby is that Crystal uses
only double quotes for string literals, while Ruby allows either single or
double quotes. ([Some people
think](https://www.viget.com/articles/just-use-double-quoted-ruby-strings) Ruby
should limit us to double quotes also.) A quick search and replace solves this
problem:

<pre type="ruby">
singer = lead_singer_for("Doors", lead_singers)
</pre>

Let’s compile again:

<pre class="console">
$ crystal lead_singers.cr 
Error in ./lead_singers.cr:3: undefined method 'attr_reader'

  attr_reader :band, :first_name, :last_name
  ^~~~~~~~~~~
</pre>

We’ve run into another difference: Crystal uses the `property` keyword
(actually a macro) instead of `attr_reader`, `attr_writer` and `attr_accessor`.
Easy enough to fix:

<pre type="ruby">
class Singer

  property :band, :first_name, :last_name

  def initialize(band, first_name, last_name)
    @band = band
    @first_name = first_name
    @last_name = last_name
  end 

end
</pre>

Now let’s try again. Compiling my Ruby code using Crystal for a third time, I
get:

<pre class="console">
$ crystal lead_singers.cr
Error in ./lead_singers.cr:22: instantiating 'Singer:Class#new(String, String, String)'

  Singer.new("The Rolling Stones", "Mick", "Jagger"),
         ^~~

instantiating 'Singer#initialize(String, String, String)'

in ./lead_singers.cr:6: Can't infer the type of instance variable '@band' of Singer

The type of a instance variable, if not declared explicitly with
`@band : Type`, is inferred from assignments to it across
the whole program.

The assignments must look like this:

  1. `@band = 1` (or other literals), inferred to the literal's type
  2. `@band = Type.new`, type is inferred to be Type
  3. `@band = Type.method`, where `method` has a return type
     annotation, type is inferred from it
  4. `@band = arg`, with 'arg' being a method argument with a
     type restriction 'Type', type is inferred to be Type
  5. `@band = arg`, with 'arg' being a method argument with a
     default value, type is inferred using rules 1, 2 and 3 from it
  6. `@band = uninitialized Type`, type is inferred to be Type
  7. `@band = LibSome.func`, and `LibSome` is a `lib`, type
     is inferred from that fun.
  8. `LibSome.func(out @band)`, and `LibSome` is a `lib`, type
     is inferred from that fun argument.

Other assignments have no effect on its type.

Can't infer the type of instance variable '@band' of Singer

    @band = band
    ^~~~~
</pre>

Oh my God, I’ve made a mistake so terrible the Crystal compiler has given me an
error message an entire page long! This is never going to work.  As you might
guess, I’ve fixed all of the superficial syntax issues. Now my Ruby code is
essentially Crystal code. This error is telling me I haven’t picked a type for
one of my instance variables, which I’ll do next.

But let’s stop for a moment to review what I’ve changed so far:

* First, I replaced single quotes with double quotes for all of my string literals.
* Then, I changed `attr_reader` to `property`.

There are a few other superficial differences you’ll run into between Ruby and
Crystal. Here are a few more I’ve come across:

* `include?` is called `includes?` in Crystal. This reads better in English, but I
  suppose Crystal looses a bit of that charming Japanese style we’ve come to
  love in Ruby.
* The `Symbol#to_proc` syntax doesn’t work in Crystal, for example `map(&:method)`.
  Instead they’ve invented a new syntax for that idiom which doesn’t exist in
  Ruby: `map(&.method)`. [The Crystal team explains
  why](https://crystal-lang.org/2013/09/15/to-proc.html) on their blog.
* Declaring an empty array `[]` or hash `{}` requires a type definition, like
  this: `[] of Int32`.

The syntax changes I had to deal with are quite small. In fact, it’s amazing
the two languages are so similar. In just a few minutes I can change my code
from Ruby, a dynamic language running with an interpreter, to Crystal, a
statically typed language that compiles to LLVM byte code and later native
machine language.

## Think About Which Types to Use

<div style="float: right; padding: 0px 0px 50px 30px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2016/10/7/x-ray2.png"><br/>
  <i>Like an X-Ray, Crystal can find problems with<br/> your Ruby code hidden underneath the surface.</i>
</div>

Of course, now that I’m using a language with static types I have to pick types
for my variables. If you've ever used an older, statically typed language like
Java or C, you know how tedious and verbose this can be. In fact, avoiding
static types is why many of us started to use Ruby in the first place.

But one of Crystal’s strengths is that it can guess which type to use for each
value in your code based on a series of rules. I don’t have to explicitly write
the type for every variable, method argument or return value in my code. This
might even be a preview of [how Ruby might work in the
future](http://confreaks.tv/videos/rubyconf2014-opening-keynote).

However, in some cases Crystal can’t guess which type to use. That’s what
happened here. Take the time to read through the page-long error message; it’s
quite helpful. It explains all of the patterns the Crystal compiler looked for
in my code, `@band = 1`, `@band = Type.new` etc. But because my assignment `@band =
band` didn’t fall into any of these categories, Crystal couldn’t figure out what
type of value `@band` represents:

<pre class="console">
in ./lead_singers.cr:6: Can't infer the type of instance variable '@band' of Singer
</pre>

To fix this, I’ll just declare the type of my `@band` variable right where I
declare it, along with my two other instance variables:

<pre type="ruby">
class Singer

  property band : String
  property first_name : String
  property last_name : String
    
  def initialize(band, first_name, last_name)
    @band = band 
    @first_name = first_name
    @last_name = last_name
  end

end 
</pre>

Notice here I use `property` three times, specifying each variable’s
name and type. My three variables, `band`, `first_name` and `last_name` are all
strings, so I just need to tell Crystal this using a more verbose declaration.

Now we should be good to go! Let's try compiling again:

<pre class="console">
$ crystal lead_singers.cr         
Error in ./lead_singers.cr:30: undefined method 'first_name' for Nil (compile-time type
is (Singer | Nil))

puts "#{singer.first_name} #{singer.last_name}"
               ^~~~~~~~~~

================================================================================

Nil trace:

  ./lead_singers.cr:29

    singer = lead_singer_for("Doors", lead_singers)
    ^~~~~~

  ./lead_singers.cr:29

    singer = lead_singer_for("Doors", lead_singers)
             ^~~~~~~~~~~~~~~

  ./lead_singers.cr:15

    def lead_singer_for(band, singers)
        ^~~~~~~~~~~~~~~

  ./lead_singers.cr:16

      singers.find{|s| s.band == band}
              ^~~~

  /Users/pat/bllvm/crystal/src/enumerable.cr:228

      def find(if_none = nil)


  /Users/pat/bllvm/crystal/src/enumerable.cr:232

        if_none
        ^~~~~~~

  /Users/pat/bllvm/crystal/src/enumerable.cr:228

      def find(if_none = nil)
                         ^
</pre>


Ugh; more trouble. Another page-long error message. Maybe I should just forget
all about Crystal and go back to writing Ruby.

## Understanding a Crystal Nil Trace

Instead, I decide to take some time to understand what Crystal is telling me. I
focus at the beginning of the Crystal error message:

<pre class="console">
Error in ./lead_singers.cr:30: undefined method 'first_name' for Nil (compile-time type
is (Singer | Nil))

puts "#{singer.first_name} #{singer.last_name}"
               ^~~~~~~~~~
</pre>

This looks unfamiliar to me, a Ruby developer, at first. The message is similar
to the error I saw earlier in Ruby when I didn’t check the return value for
`find`. Recall that was “undefined method `first_name' for nil:NilClass
(NoMethodError)”. Crystal seems to be telling me the same thing: “undefined
method 'first_name' for Nil.”

And it is. But instead of giving me a runtime exception, Crystal is giving me a
compile time error based on types. Ruby didn’t report the problem until I ran
my Ruby code, when Ruby actually tried to call the `first_name` method on the
`NilClass` class. But Crystal’s compiler has found the problem before my code was
ever run. It knows the `Nil` class doesn’t have a `first_name` method at compile
time.

But why does Crystal think there is a `Nil` class in my code? I just told it my
three instance variables are strings:

<pre type="ruby">
  property band : String
  property first_name : String
  property last_name : String
</pre>

What the Crystal compiler did is quite interesting! While compiling my code, it
saw that I use the `@band` instance variable in the `lead_singer_for` method:

<pre type="ruby">
def lead_singer_for(band, singers)
  singers.find{|s| s.band == band}
end
</pre>

Internally, the Crystal compiler now has to decide what type `lead_singer_for`
returns. That’s obvious, isn’t it? It should return a `Singer`. The call to `find`
returns a `Singer` object, the first element of the `singers` array which matches
the band, the element for which the block returns `true`.

But what if the band name doesn’t match any singers? What if the block never
returns `true` for any element in the array? As we know from Ruby, in that case
`lead_singer_for` would return `nil`. So `lead_singer_for` might return `nil` or it
might return a singer.

Crystal’s type system has a solution for this situation: a [union
type](https://crystal-lang.org/2013/09/23/type-inference-part-1.html).
Crystal decides `lead_singer_for` returns a `(Singer | Nil)` type, which it
mentions in the error message. Now when I use this return value, Crystal’s
compiler knows to check whether the `first_name` and `last_name` methods are
defined for every class in that union type: `Singer` and `Nil`.

The rest of the long error message is known as a “Nil trace.” To help us
understand what is wrong, Crystal backtracks through the code starting from
where the missing method was found to where the offending type was introduced.
You can read the Nil trace above for yourself. It starts with:

<pre class="console">
 ./lead_singers.cr:29

    singer = lead_singer_for("Doors", lead_singers)
    ^~~~~~
</pre>

And reading down you can see where the `Nil` type was actually introduced:

<pre class="console">
  /Users/pat/bllvm/crystal/src/enumerable.cr:228

      def find(if_none = nil)
</pre>

As you can see, the `Nil` type is a default value passed to the `Enumerable#find`
method, which I call in `lead_singer_for`. Crystal’s standard library is
entirely implemented using Crystal. This means if I’m curious (and I am) I can
read how Crystal implements all of the `Enumerable` methods. I could even go
and experiment with the language by modifying them.

In fact, the Crystal compiler itself is implemented with Crystal! Interested in
learning about how a real world compiler works but don’t have time to learn C
or C++? Read the Crystal source code.

## Think Twice About Which Types to Use

Now back to my example. I’m done, right? Recall in my Ruby code I added a check
for the return value of `lead_singer_for`:

<pre type="ruby">
singer = lead_singer_for("Doors", lead_singers)
if singer
  puts "#{singer.first_name} #{singer.last_name}"
else
  puts "Not found"
end
</pre>

The same fix will work for Crystal. The Crystal compiler is clever enough to
know that inside the first branch of the if-statement the type of `singer` is
`Singer` and not `Nil`. And in the second, else branch it is `Nil` and not `Singer`. It
splits up the union type again depending on the syntax of my program. Amazing.

But before I declare victory, this business about the `(Singer | Nil)` union type
has got me thinking… Crystal decided that a `nil` value can be introduced by my
code in a certain scenario. But maybe `nil` should be a valid value for one of my
variables? After all, I’m dealing with rock stars. Sometimes rock stars become
so famous they decide they don’t need a last name any more. What about lead
singers like String, Bono or Prince? How would I represent them in my test data
set?

The answer is obvious: their singer objects would have a `nil` `last_name` value. I
would create them like this:

<pre type="ruby">
Singer.new("The Police", "Sting", nil)
</pre>

In Ruby, this would have worked just fine. But Crystal objects:

<pre class="console">
$ crystal lead_singers.cr      
Error in ./lead_singers.cr:26: instantiating 'Singer:Class#new(String, String, Nil)'

  Singer.new("The Police", "Sting", nil),
         ^~~

instantiating 'Singer#initialize(String, String, Nil)'

in ./lead_singers.cr:10: instance variable '@last_name' of Singer must be String, not Nil

    @last_name = last_name
    ^~~~~~~~~~
</pre>

What do I do now? How can I save a `nil` last name in my `Singer` class? The
instance variables are strings and cannot hold `nil`.

The answer is I picked the wrong type for `last_name`. To accommodate
super-famous singers, I need to use the same union type we saw earlier:

<pre type="ruby">
class Singer

  property band : String
  property first_name : String
  property last_name : (String | Nil)

  def initialize(band, first_name, last_name)
    @band = band
    @first_name = first_name
    @last_name = last_name
  end 

end
</pre>

Now I can create the Sting object no problem:

<pre type="ruby">
Singer.new("The Police", "Sting", nil)
</pre>

Finally, we’re ready to compile my Ruby and move on!

<pre class="console">
$ crystal lead_singers.cr
Error in ./lead_singers.cr:37: instantiating 'longest_last_name(Array(Singer))'

puts longest_last_name(lead_singers)
     ^~~~~~~~~~~~~~~~~

in ./lead_singers.cr:20: undefined method 'size' for Nil (compile-time type is (String | Nil))

  singers.map{|s| s.last_name}.max_by{|name| name.size }
                                                  ^~~~

================================================================================

Nil trace:

  ./lead_singers.cr:20

      singers.map{|s| s.last_name}.max_by{|name| name.size }
                                           ^~~~
</pre>

Once again the Crystal compiler has stopped me in my tracks. When will I ever
get this right? Is this another Ruby vs. Crystal difference? Another subtlety
of the Crystal type system I need to learn about?

## Static Types Reveal a Hidden Problem

No. Crystal has found a real problem with my Ruby code, a problem I never
noticed. Because Sting doesn’t have a last name, the `longest_last_name` method
runs into a problem:

<pre type="ruby">
def longest_last_name(singers)
  singers.map{|s| s.last_name}.max_by{|name| name.size }
end
</pre>

The first call to `map` returns an array of last names, which now will contain
`nil`. Then I pass that array into `max_by` which converts the names into
corresponding name lengths, and then returns the longest name.

Now that I know where to look, it’s easy to see the problem: `max_by` will pass
`nil` to the second block for Sting’s missing last name, and the block will then
try to call the `size` method on `nil`. Easy enough to fix:

<pre type="ruby">
def longest_last_name(singers)
  singers.map{|s| s.last_name}.compact.max_by{|name| name.size }
end
</pre>

Using `compact`, I remove the `nil` element from the array of names, meaning the
`size` method will never be called on `nil`. Of course, now that I’m thinking about
`nil` values and the `longest_last_name` method, I realize that maybe all the
singers are super-famous and have no last names, or possibly there were no
singers to begin with. I tighten up my code even more:

<pre type="ruby">
def longest_last_name(singers)
  singers_with_last_names = singers.map{|s| s.last_name}.compact
  unless singers_with_last_names.empty?
    singers_with_last_names.max_by{|name| name.size }
  end
end

last_name = longest_last_name(lead_singers)
if last_name
  puts last_name
else
  puts "Not found"
end
</pre>

Now everything works!

One interesting footnote here: Ruby allows me to get away without checking for
an empty array using `unless`. In Ruby if I call `max_by` on an empty array it
simply returns `nil`, meaning there is no maximum value at all. But Crystal is
even more strict: It raises an runtime exception "Empty enumerable
(Enumerable::EmptyError)". In a sense this is going a bit overboard, because `nil`
seems to me a valid result in this case. But on the other hand, calling `max_by`
on an empty array might be an indication of other problems in my code. Crystal
brings that to my attention, but with a runtime exception not a compile error.
Crystal reports runtime errors for other cases as well, for example looking for
a value in a hash when the key doesn’t exist:

<pre type="ruby">
hash = { "a" => 123 } 
puts hash["b"]
# => Missing hash key: :b (KeyError)
</pre>

The Crystal compiler expects a higher level of quality and thoroughness in my
code than Ruby does, it seems to me.

## Conclusion

There are two important concepts I took away from this exercise. First, using
Ruby we depend on the completeness of our test suite in order to find and avoid
mistakes. Precisely which values you choose for your test data set is very
important. If I had thought of using Sting when I originally wrote my tests, I
would have found the missing last name problem right away.  But I didn’t.

Second, the most tedious and time consuming part of converting from Ruby to
Crystal, choosing a type for each value in my code, is of course the most
valuable step in the process. It wasn’t until I tried using `(String | Nil)` for
the `@last_name` variable that the Crystal compiler found the missing last name
problem for me.

You still may not be convinced. This was obviously a very contrived example and
using the Crystal compiler on real world Ruby code won’t be easy. I agree. It
would be pointless to try to compile a large Rails application using Crystal.

But look over your code. I would guess there are a few important methods or
classes which are central to your application’s behavior and logic. Take an
hour or two and copy and paste those important lines of code into a separate
file, stub out any dependencies, and run it through the Crystal compiler. Take
the time to convert your code to use static types. Take the time to think
carefully about which types of values your code should be able to handle.

Bring your important Ruby code to the Crystal compiler for a second opinion.
You might be surprised by what Crystal finds.
