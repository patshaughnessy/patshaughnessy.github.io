title: "Learning from the masters part 2: three metaprogramming best practices"
date: 2011/12/20
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/20/mozart.jpg"></td></tr>
  <tr><td align="center"><small><i>Mozart’s Requiem Mass in D minor would be<br/>hard to understand without a translation from Latin</i></small></td></tr>
</table>
</div>

Metaprogramming has always been one of the most intimidating features of the Ruby language - it’s considered very “advanced” and can often be very difficult to read and understand. However, I agree completely with what [Russ Olsen](http://jroller.com/rolsen/) said about metaprogramming on [this Thursday's Ruby Rogues podcast](http://rubyrogues.com/033-rr-book-club-eloquent-ruby/): that Ruby metaprogramming really isn’t all that hard to understand, that it is just another moderately difficult programming concept any Ruby developer can learn and use.

[Like I did two weeks ago](https://patshaughnessy.net/2011/12/6/learning-from-the-masters-some-of-my-favorite-rails-commits) I decided to take a look at the Rails source code, this time to see whether there were any good examples of metaprogramming that I could learn from.DELIM What techniques did the Rails core team use with metaprogramming that I could use in my own work? I found that I was impressed by how the Rails team <i>thought</i> about metaprogramming - the philosophy and coding practices they used - rather than by the actual metaprogramming techniques themselves.

Today I’m going to discuss three of the metaprogramming practices I found in the Rails source code - read on to learn more about these best practices and how to use them in your code.

<strong>TL;DR:</strong> Be sure to provide a translated, annotated version of your metaprogramming in comments and to raise exceptions with accurate method name, file and line number information. Also when using <span class="code">method_missing</span> be sure to call <span class="code">super</span> and to implement <span class="code">respond_to?</span>.

## Provide a translation

<div style="float: right; padding: 15px 0px 10px 30px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/20/translate.jpg"></td></tr>
</table>
</div>

One of the most commonly used and well known examples of metaprogramming in Rails is the way ActiveRecord implements dynamic finder and scope methods. For example, suppose I have an ActiveRecord model called “Shirt” with a color field. I can get all the red shirt records just by calling <span class="code">Shirt.find_all_by_color('red')</span>. ActiveRecord also supports dynamic scope methods that return a scope object; for example calling <span class="code">Shirt.scoped_by_color('red')</span> will return a scope that you can then chain with other methods.

Rails implements dynamic finder and scope methods using <span class="code">method_missing</span>. Since I didn’t actually write a method called “scoped_by_color” in my Shirt class, the Ruby interpreter calls <span class="code">method_missing</span> and gives ActiveRecord a chance to handle the missing method call. Here’s a small part of ActiveRecord’s <span class="code">method_missing</span> implementation for these dynamic finder and scope methods - in edge Rails you can find the complete method_missing implementation in ActiveRecord::DynamicMatchers (see [dynamic_matchers.rb](https://github.com/rails/rails/blob/master/activerecord/lib/active_record/dynamic_matchers.rb#L24)) or in Rails 3.1 and earlier you’ll find this in ActiveRecord::Base (see [base.rb](https://github.com/rails/rails/blob/3-1-stable/activerecord/lib/active_record/base.rb#L1051)):

<pre type="ruby">
def method_missing(method_id, *arguments, &block)

...etc... 

  self.class_eval <<-METHOD, __FILE__, __LINE__ + 1
    def self.#{method_id}(*args)
      attributes = Hash[[:#{attribute_names.join(',:')}].zip(args)]
      scoped(:conditions => attributes)
    end
  METHOD
  send(method_id, *arguments)

...etc...
</pre>

Hmm - what the heck does this mean? I’ve removed a lot of the surrounding code from method_missing, but this snippet is still very confusing. When I first saw this code, I was definitely intimidated! What in the world is going on here? Maybe metaprogramming is not for me after all!

Then I noticed a very helpful comment right next to the actual production code - just to the right, actually:

<pre type="ruby">
# def self.scoped_by_user_name_and_password(*args)
#   attributes = Hash[[:user_name, :password].zip(args)]
#
#   scoped(:conditions => attributes)
# end
</pre>

I view this comment as a translation, sort of a “clear text” version of the actual metaprogramming code above, that uses two example attributes “user_name” and “password.” Immediately I can make the mental leap that for my case, the Shirt model with a single “color” attribute, the generated code would be:


<pre type="ruby">
# def self.scoped_by_color(*args)
#   attributes = Hash[[:color].zip(args)]
#
#   scoped(:conditions => attributes)
# end
</pre>

This is still far from obvious, but after looking up the meaning of the [Array.zip method](http://www.ruby-doc.org/core-1.9.3/Array.html#method-i-zip) and experimenting with the Hash[] syntax in the console:

<pre type="console">
> args = ["red"]
 => ["red"] 
> [:color].zip(args)
 => [[:color, "red"]] 
> Hash[ [[:color, "red"]] ]
 => {:color=>"red"} 
</pre>

...I can finally get to a good understanding of what the original metaprogramming was doing. It was defining a new class method called <span class="code">scoped_by_color</span> that simply calls <span class="code">scoped(:conditions => {:color=>"red"})</span> - giving me the same behavior I would have gotten by using an anonymous scope directly:

<pre type="console">
> Shirt.scoped(:conditions => {:color => 'red'})
  Shirt Load (0.2ms)  SELECT "shirts".* FROM "shirts" WHERE "shirts"."color" = 'red'
 => [#&lt;Shirt id: 1, color: "red", created_at... >] 
</pre>

I found this translated code comment to be so helpful that I believe it’s a metaprogramming best practice that we should all learn from! The idea isn’t new, actually. In fact [Yehuda Katz](http://yehudakatz.com/), by all accounts a master of Ruby programming, [originally mentioned the idea](http://yehudakatz.com/2008/12/29/another-rails-2x3-update) of “annotating metaprogramming” while he was working on Rails 3 back in 2008, saying he was inspired by Michael Klishin (antares), and the next day [Xavier Noria](http://hashref.com/) went ahead and added the annotations to the Rails source code in this [one large commit](https://github.com/rails/rails/commit/a2270ef2594b97891994848138614657363f2806).

But wait a minute... aren’t code comments frowned upon and largely unnecessary, especially in a language as expressive as Ruby is? Also comments instantly become a maintenance problem - as soon as you change your code the comments are now out of date. However, in the case of metaprogramming, I think this is a small price to pay. I’m impressed that the Rails team took some time to make their code more readable and understandable for other developers, and not just for the Rails interpreter. We should all do the same whenever we use metaprogramming.

## Plan ahead for accidents

<div style="float: right; padding: 15px 0px 10px 30px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/20/accident.jpg"></td></tr>
</table>
</div>

Using the same <span class="code">Shirt.scoped_by_color</span> example, let’s suppose I forget to pass in the color parameter (note: if you try this yourself you won’t get this error unless you’re using edge Rails. In Rails 3.1 and earlier a warning is put into the log file instead.):

<pre type="console">
> Shirt.scoped_by_color
ArgumentError: wrong number of arguments (0 for 1)
 from .../dynamic_matchers.rb:29:in `scoped_by_color'
</pre>

You can see I now get a fairly obvious exception message: I passed in zero arguments when the <span class="code">scoped_by_color</span> method expects 1. But wait a minute: there actually is no “scoped_by_color” method in my Shirt class! This error message is actually generated dynamically by the following code, also in the same <span class="code">method_missing</span> implementation in [dynamic_matchers.rb](https://github.com/rails/rails/blob/master/activerecord/lib/active_record/dynamic_matchers.rb#L24):

<pre type="ruby">
def method_missing(method_id, *arguments, &block)

...etc...

  if arguments.size < attribute_names.size
    method_trace = "#{__FILE__}:#{__LINE__}:in `#{method_id}'"
    backtrace = [method_trace] + caller
    raise ArgumentError, "wrong number of arguments (#{arguments.size} for #{attribute_names.size})", backtrace
  end

...etc...

end
</pre>

This isn’t very glamourous code; all it’s doing is generating a string containing the dynamic method name, <span class="code">scoped_by_color</span> in this example, along with file name and line number information and manually adding it to a backtrace array. Then it raises an exception using the manually generated backtrace.

What I like about this, however, is that the Rails team spent some time thinking about what might go wrong with <span class="code">scoped_by_color</span>. They realized that not passing in enough parameters might be a very common mistake, especially with a dynamically generated method, and wrote some code to anticipate that situation and display a more helpful error message. Because of this, my Rails log file will display dynamic method names such as <span class="code">scoped_by_color</span> in exception backtraces just the same as it does any other method.

I call this practice “planning ahead for accidents.” You should probably do this for any code, but for metaprogramming it’s especially important since it’s often not at all obvious how many or which parameters to pass in to generated code. If you’re using metaprogramming, anything at all you can do to make life easier for developers, including yourself, who will later use your code is worthwhile. When you can’t simply look at a method definition it can be very hard to understand how to call that method.

There’s another simple example of planning ahead for accidents you may not have noticed in the code I showed above:

<pre type="ruby">
self.class_eval <<-METHOD, __FILE__, __LINE__ + 1
  def self.#{method_id}(*args)
etc...
METHOD
</pre>

Here by passing <span class="code">\_\_FILE\_\_</span> and <span class="code">\__LINE__+1</span> into the call to <span class="code">class_eval</span>, the Rails team has insured that any errors or exceptions that occur inside this new, generated method will contain the proper file and line number information.

## Be a good neighbor

Here’s some more code from the same file, [dynamic_matchers.rb](https://github.com/rails/rails/blob/master/activerecord/lib/active_record/dynamic_matchers.rb#L24) in edge Rails, containing the <span class="code">method_missing</span> implementation of dynamic finder and scope methods:

<pre type="ruby">
def respond_to?(method_id, include_private = false)
  if match = DynamicFinderMatch.match(method_id)
    return true if all_attributes_exists?(match.attribute_names)
  elsif match = DynamicScopeMatch.match(method_id)
    return true if all_attributes_exists?(match.attribute_names)
  end

  super
end
</pre>

Here we can see that ActiveRecord implements <span class="code">respond_to?</span> to indicate which dynamic finder and scope methods <span class="code">method_missing</span> will handle. It returns true if the method_id matches one or more of the model’s attributes, and false if not. This is important since other code in your application that uses the Shirt model, for example, might call <span class="code">respond_to?</span> to determine whether or not Shirt has a color attribute.

<div style="float: right; padding: 15px 0px 10px 30px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/20/neighborhood.jpg"></td></tr>
</table>
</div>

It might seem that there’s not a tremendous amount of value here - after all, I know that my Shirt model handles find_by_color and find_all_by_color, etc., and I’ll probably never call <span class="code">Shirt.respond_to?</span> myself directly. However, keep in mind that other developers might work with the same application in the future, and also there may be gems or other 3rd party code that use my Shirt model from a more generalized perspective and that might call <span class="code">respond_to?</span> on it.

I consider this to be one example of another metaprogramming best practice: “Being a good neighbor.” Here the Rails team has taken some extra time to be sure that the Shirt model will work well and cooperate with other nearby code in the same application.

Another example of “being a good neighbor” is the way the Rails team calls <span class="code">super</span> in the case where a missing method actually does not match one of the Shirt model’s attributes:

<pre type="ruby">
def method_missing(method_id, *arguments, &block)
  if match = (DynamicFinderMatch.match(method_id) || DynamicScopeMatch.match(method_id))

...etc...

  else
    super
  end
end
</pre>

Just as <span class="code">respond_to?</span> returns false if the method_id doesn’t match one or more of the attributes, <span class="code">method_missing</span> will call <span class="code">super</span> so Ruby will call any other <span class="code">method_missing</span> functions that might be defined elsewhere. If ActiveRecord didn’t call <span class="code">super</span> when it’s <span class="code">method_missing</span> didn’t handle the method call, then no other modules included into the Shirt class or it's superclasses would receive a <span class="code">method_missing</span> call. This is really less of a best practice and more of an absolute requirement when using <span class="code">method_missing</span>: you always, always need to call <span class="code">super</span> when you don’t handle the call.

Although implementing <span class="code">respond_to?</span> and calling <span class="code">super</span> are specific to <span class="code">method_missing</span>, for me being a good neighbor more generally means you need to think about the larger system around you - your neighborhood. This is especially true of metaprogramming, since something as simple as forgetting to call <span class="code">super</span> can have a profound effect on other parts of your application.

## Conclusion

Metaprogramming is a very powerful tool and can be a great way to solve complex problems with elegant, concise code. Don’t be afraid of using it! But when you do use metaprogramming be sure to:

<ul>
  <li>Take the extra time to provide a translated version of your generated code - this will make it much, much easier for future developers to read and understand your code.</li>
  <li>Be sure that any error exceptions or errors you generate use the proper method name, file name and line number information so other developers don’t have to go on a wild goose chase to understand where a problem lies.</li>
  <li>When using <span class="code">method_missing</span> always be sure to implement <span class="code">respond_to?</span> and to call <span class="code">super</span> when you don’t handle the method call, otherwise you might break code that surrounds and uses your class.</li>
</ul>

I’m sure there are other good examples of metaprogramming best practices elsewhere in Rails and in many other projects - what do you think? What else should Ruby developers keep in mind while using metaprogramming?
