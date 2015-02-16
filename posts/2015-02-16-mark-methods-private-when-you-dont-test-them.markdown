title: "Mark Methods Private When You Don’t Test Them"
date: 2015/2/16

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2015/2/16/plaza-de-la-merced.jpg"><br/>
  <i>My father in law once lived in same building where Picasso<br/>was born, near the Plaza de la Merced in Málaga, Spain.</i>
</div>

In Ruby and many other languages, you write private methods to implement
internal logic you don’t want to expose. You want the freedom to rename,
repurpose or even delete them without worrying about impacting anything else.
The <span class="code">private</span> keyword signals other developers: Don’t rely on this; don’t call
it; it might change. This is especially important when writing framework or
library code that many other developers will use.

But which methods should you make private? Sometimes this is obvious; sometimes
it isn’t. A good rule of thumb to use is: If you’re not testing a method, it
should be private.

<br/>
<br/>

But wait a minute! Aren’t we supposed to test everything? Isn’t 100% code
coverage the nirvana every Ruby developer seeks? Let me clarify. You should
mark methods private when you test them indirectly by calling the other, public
methods in the same class. Use the <span class="code">private</span> keyword to help organize your code,
to remind yourself what you still need to test, and what you don’t.

## Three Paintings

A simple example will make this clear. Suppose I have a class that describes a
painting:

<pre type="ruby">
Painting = Struct.new(:name, :year)
</pre>

Now I can create a list of three paintings in a <span
class="code">Minitest::Spec</span> file like this:

<pre type="ruby">
let(:one) { Painting.new("Spanish Couple In Front Of Inn", 1900) }
let(:two) { Painting.new("Guernica", 1937) }
let(:three) { Painting.new("Petite Fleurs", 1958) }
let(:paintings) { [one, two, three] }
</pre>

Suppose my first requirement is to return the first painting from the list.
Simple enough:

<pre type="ruby" style="display: inline-block; width: 200px;">
def first(list)
  list.first
end
</pre>
<pre type="ruby" style="display: inline-block;">
it "should return the first element" do
  first(paintings).must_equal one
end
</pre>

I just call <span class="code">Array#first</span> and I’m done. Returning the
rest of the list is slightly more interesting:

<pre type="ruby" style="display: inline-block; width: 200px;">
def rest(list)
  _, *rest = list
  rest
end
</pre>
<pre type="ruby" style="display: inline-block">
it "returns the rest of the elements" do
  rest(paintings).must_equal [two, three]
end

</pre>

Using [a trick I learned from
Avdi](http://devblog.avdi.org/2010/01/31/first-and-rest-in-ruby/), <span
class="code">rest</span> always returns an array even if the input list was
empty or had only one element. So far, so good. I’ve written two methods and
two tests:

<img src="http://patshaughnessy.net/assets/2015/2/16/two-tests.png"/>

## A New Requirement

Now suppose my business requirement changes slightly and I instead need to
return the first painting sorted alphabetically by name. Once again, it’s not
hard to do.

<pre type="ruby" style="display: inline-block; width: 300px;">
def first(list)
  list.sort do |p1, p2|
    p1.name <=> p2.name
  end.first
end
</pre>
<pre type="ruby" style="display: inline-block">
it "should return the first element" do
  first(paintings).name.must_equal "Guernica"
end


</pre>

And I need rest to use the same sort order, so I repeat the call to sort:

<pre type="ruby" style="display: inline-block; width: 300px;">
def rest(list)
  _, *rest = list.sort do |p1, p2|
    p1.name <=> p2.name
  end 
  rest
end
</pre>
<pre type="ruby" style="display: inline-block">
it "returns the rest of the elements" do
  rest(paintings).map(&:name).must_equal [
    "Petite Fleurs",
    "Spanish Couple In Front Of Inn"
  ]
end
</pre>

I’ve implemented new behavior, but still have two methods and two tests:

<img src="http://patshaughnessy.net/assets/2015/2/16/two-tests.png"/>

## Extracting a Method

Because both of my methods are covered by tests, I’m free to refactor them. I
decide to extract a new method <span class="code">sorted_by_name</span>:

<pre type="ruby" style="display: inline-block; width: 300px;">
def first(list)
  sorted_by_name(list).first
end

def rest(list)
  _, *rest = sorted_by_name(list)
  rest
end




def sorted_by_name(list)
  list.sort do |p1, p2|
    p1.name <=> p2.name
  end
end
</pre>

<pre type="ruby" style="display: inline-block; width: 400px;">
it "returns the element with the first name" do
  first(paintings).name.must_equal "Guernica"
end

it "returns the rest after the first name" do
  rest(paintings).map(&:name).must_equal [
    "Petite Fleurs",
    "Spanish Couple In Front Of Inn"
  ]
end







</pre>

Here I’ve simply moved the call to sort into a utility method called
<span class="code">sorted_by_name</span>. Now <span class="code">first</span>
and <span class="code">rest</span> both call <span
class="code">sorted_by_name</span>, making the code a bit clearer and DRY-er. But
now I have three methods and only two tests:

<img src="http://patshaughnessy.net/assets/2015/2/16/two-tests-three-methods.png"/>

## Mark Methods Private When You Don’t Test Them

Notice I didn’t bother writing a test for <span class="code">sorted_by_name</span>. I know it works
because my other tests still pass. The existing tests are sufficient; I am
testing <span class="code">sorted_by_name</span> indirectly. Because I extracted <span class="code">sorted_by_name</span> from
<span class="code">first</span> and <span class="code">rest</span>, because I
refactored my code without adding any new behavior, no new test were required.

In this scenario, take the time to mark the new, untested method as private:

<pre type="ruby" style="display: inline-block; width: 300px;">
def first(list)
  sorted_by_name(list).first
end

def rest(list)
  _, *rest = sorted_by_name(list)
  rest
end



private

def sorted_by_name(list)
  list.sort do |p1, p2|
    p1.name <=> p2.name
  end
end
</pre>

<pre type="ruby" style="display: inline-block; width: 400px;">
it "returns the element with the first name" do
  first(paintings).name.must_equal "Guernica"
end

it "returns the rest after the first name" do
  rest(paintings).map(&:name).must_equal [
    "Petite Fleurs",
    "Spanish Couple In Front Of Inn"
  ]
end








</pre>

The <span class="code">private</span> keyword here reminds me I’ve already tested <span
class="code">sorted_by_name</span>, that I don’t need to write new tests for
it. Now <span class="code">private</span> is helping me organize my code; it’s helping me remember
which methods I don’t need to test… and which methods are missing important
tests.

<img src="http://patshaughnessy.net/assets/2015/2/16/two-tests-three-methods-private.png"/>

If my tests don’t need to know about <span class="code">sorted_by_name</span>, then certainly other
developers don’t. It should be private. Marking it private reminds me that it
is being tested indirectly, that I didn’t just forget to write a test for it.
Marking it private tells other developers about what I’ve learned from my own
test suite.
