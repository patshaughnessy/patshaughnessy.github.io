title: "A Rule of Thumb for Strong Parameters"
date: 2014/6/16
tag: Ruby

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="https://patshaughnessy.net/assets/2014/6/16/security.jpg"><br/>
  <i>It can be hard to open the Strong Parameters<br/>door and let permitted values into your Rails 4 app.</i>
</div>

Last week I banged my head against the wall for a few hours trying to figure
out how to whitelist input values for my Rails 4 app using the [Strong
Parameters](http://edgeguides.rubyonrails.org/action_controller_overview.html#strong-parameters)
feature. Calling <span class="code">permit</span> when you have a simple
attributes hash for a single new object is straightforward, but figuring out
how to call <span class="code">permit</span> for a complex, nested set of attributes can be nearly
impossible.

<b>TL;DR:</b> Use this rule of thumb when trying to figure out how to whitelist nested attributes:

<blockquote>To Permit a Hash, Pass an Array<br/>
  To Permit an Array, Pass a Hash</blockquote>

After studying the problem for a while, I noticed this curious pattern: To whitelist a simple hash of attributes, you pass
<span class="code">permit</span> an array. To whitelist an array of nested objects, you pass it a hash,
including that hash in the surrounding array.

Not familiar with Strong Parameters yet? What do I mean by “whitelist?” Why do
you need this rule of thumb? My post today will explain.

## To Permit a Hash, Pass an Array

Rails 4 requires you to whitelist or authorize input values for your app. This
important new feature, known as Strong Parameters, adds an extra layer of
security that prevents attackers from posting harmful or garbage information to
your site.

Here’s how it works: Suppose you have a <span class="code">Post</span> model
with title and body columns. Using Rails 4, you would write a create controller
action like this:

<img src="https://patshaughnessy.net/assets/2014/6/16/simple_controller.png"><br/>

Here you first tell Rails which attributes are allowed for new post objects -
title and body in this example - and then you create the new post. This is
simple enough and quite readable. You are telling Rails: “data for a post is
required and it’s attributes may only include title and body attributes.”

<img src="https://patshaughnessy.net/assets/2014/6/16/diagram1.png"><br/>

In the diagram above you can see the post attribute hash on the left, and the
arguments for <span class="code">permit</span> on the right. Notice the arguments are actually a
single array (internally Rails processes the arguments as an array). Each key/value pair on the left maps to an array element on the
right. You permit a hash by passing an array.

## Strong Parameters Confusion

However, now suppose you add a second model to your app, <span
  class="code">Comment</span>. Let’s suppose a post has many comments, and each
comment has a single text attribute.

<img src="https://patshaughnessy.net/assets/2014/6/16/has-many.png"><br/>

Because your app is a REST-ful JSON service (what else do people use Rails for
these days?) you have a requirement to create a post and its comments from a
single JSON string:

<img src="https://patshaughnessy.net/assets/2014/6/16/json.png"><br/>

Because the comment array is named “comments” you assume ActiveRecord will
create the associated comment models along with the new post. That is,
ActiveRecord should call <span class="code">comments=</span> on the new post
and pass in the comment attributes. But it doesn’t work. Instead, using the
code from earlier you get:

<img src="https://patshaughnessy.net/assets/2014/6/16/warning.png"><br/>

Ah - you forgot to whitelist the comments attribute. You try adjusting the call
to <span class="code">permit</span>:

<img src="https://patshaughnessy.net/assets/2014/6/16/permit-comments.png"><br/>

It still doesn’t work. You get the same warning:

<img src="https://patshaughnessy.net/assets/2014/6/16/warning.png"><br/>

Clearly Rails isn’t listening! You just told it comments is permitted - why
does Rails give you the same warning again? Maybe you need to create the
comment objects yourself, as a separate step:

<img src="https://patshaughnessy.net/assets/2014/6/16/create-comments.png"><br/>

Now things are even worse: Rails raises an exception!

<img src="https://patshaughnessy.net/assets/2014/6/16/exception.png"><br/>

The problem here is that the <span class="code">Comment</span> model is
complaining that you haven’t whitelisted its attributes. Somehow each
ActiveRecord model has knowledge about which attributes were whitelisted and
which weren’t!

If you happened to know that Rails 4 saves the parameters inside an <span
  class="code">ActionController::Parameters</span> object, you could try
creating a separate instance of this class for each comment, and whitelist it's text attribute directly:

<img src="https://patshaughnessy.net/assets/2014/6/16/permit-comments2.png"><br/>

But you’ll still get the same “Unpermitted parameters” warning when you try to
create the post next - not to mention that your code has become incredibly
confusing and verbose. What’s going on here? There must be some way of creating
nested objects without warnings or exceptions.

## To Permit an Array, Pass a Hash

The solution is to permit all the post and comment attributes with a single
call to <span class="code">ActionController::Parameters#permit</span>, like
this:

<img src="https://patshaughnessy.net/assets/2014/6/16/nested-solution.png"><br/>

What? What does the complex argument list to <span class="code">permit</span> mean? How in the world
would anyone know to pass that in?

My rule of thumb can help. In this example, you are permitting an array of
comments by passing a hash. Imagine if your app received a post with two
comments, like this:

<img src="https://patshaughnessy.net/assets/2014/6/16/diagram2.png"><br/>

On the left you see the nested attributes for the post and its comments. Rails
has parsed this for you from a JSON string. On the right are the arguments you
pass to <span class="code">permit</span>. Notice how the hash of post
attributes contains an array for the comments - in the call to <span class="code">permit</span> you
replace this array with a hash! To permit an array, pass a hash.

This hash, in turn, contains an array listing the attributes of each comment
object (just <span class="code">[:text]</span> here).

## Whitelisting Rails Nested Attributes

Note: if your Rails app was in fact a web site, you might use the Rails nested
attributes feature with a complex HTML form. In this case, you would declare
that posts accept nested attributes for comments, directing ActiveRecord to
automatically create the comment objects for you:

<img src="https://patshaughnessy.net/assets/2014/6/16/accepts-nested.png"><br/>

To make this work you will need to adjust your call to <span
class="code">permit</span> slightly:

<img src="https://patshaughnessy.net/assets/2014/6/16/nested-attribs-permit.png"><br/>

With <span class="code">accepts_nested_attribute_for</span>, Rails expects the
comments to be saved as <span class="code">comments_attributes</span>. Also,
each comment must have an <span class="code">id</span> attribute in order for
the web forms to update existing comments properly. (Also, Rails represents the
comment array as a hash, with the id as the key for each comment.)

But the syntax is almost the same, and the same rule applies: to permit an
array of comments you call <span class="code">permit</span> with a hash.

## Under the Hood

Internally, Rails uses a trick to tell ActiveRecord which attributes were
permitted and which weren’t. Here’s how it works:

<img src="https://patshaughnessy.net/assets/2014/6/16/hashes.png"><br/>

On the left is a normal hash - you can create an ActiveRecord model using this
hash without worrying about whitelisting parameters. On the right is an <span
class="code">ActionController::Parameters</span> object; notice it looks almost
the same.  <span class="code">ActionController::Parameters</span> is actually a
subclass of <span class="code">Hash</span> (via <span
class="code">ActiveSupport::HashWithIndifferentAccess</span>).

But on the right notice that the <span
  class="code">ActionController::Parameters</span> object contains a <span class="code">permitted?</span>
method. This tells ActiveRecord whether the attributes in that hash were
whitelisted or not.

When you create a new post or any <span class="code">ActiveRecord::Base</span>
object, code inside of Rails checks whether the attributes hash implements the
<span class="code">permitted?</span> method or not:

<img src="https://patshaughnessy.net/assets/2014/6/16/internals1.png"><br/>

Because a normal hash doesn’t respond to <span class="code">permitted?</span>,
ActiveRecord creates the new post without complaining.

However, if you try to create a post from an <span
  class="code">ActionController::Parameters</span> object, Rails finds the
<span class="code">permitted?</span> method:

<img src="https://patshaughnessy.net/assets/2014/6/16/internals2.png"><br/>

If you didn’t whitelist this hash using a call to <span class="code">permit</span>, <span class="code">permitted?</span> will
return <span class="code">false</span> and Rails will raise the <span class="code">ActiveModel::ForbiddenAttributesError</span>
exception.

## Still Confused? Just Use a Normal Hash

If all of this is still confusing you - if you’re still having trouble figuring
out the call to <span class="code">permit</span> properly for your complex JSON input - then remember you
can always just use normal hashes instead of <span class="code">ActionController::Parameters</span>
objects.

For example:

<img src="https://patshaughnessy.net/assets/2014/6/16/normal-hashes.png"><br/>

Here you are whitelisting or permitting each attribute manually by copying them
into normal hashes. Since <span class="code">Hash</span> doesn't implement <span
class="code">permitted?</span>, ActiveRecord won’t raise a <span
class="code">ActiveModel::ForbiddenAttributesError</span> exception.
