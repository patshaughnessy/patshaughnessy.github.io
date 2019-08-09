title: "Getting started with Ruby metaprogramming"
date: 2010/02/20
tag: Ruby

<p>The <a href="http://github.com/rails/auto_complete">Rails auto_complete plugin</a> was my first exposure to Ruby metaprogramming. It&rsquo;s code was simple enough for a Rails beginner like me to understand, but also just complex enough for me to learn something new. Specifically, I ran into metaprogramming when I took a close look at the &ldquo;auto_complete_for&rdquo; method and tried to figure out how it worked. I won&rsquo;t spend any time here explaining what the auto_complete plugin does or what it&rsquo;s used for, beyond to say that if you add this line to one of your controllers:</p>

<pre type="ruby">
class CategoriesController < ApplicationController

  auto_complete_for :category, :name

  etc…
</pre>

<p><br>&hellip; a method called &ldquo;auto_complete_for_category_name&rdquo; will be automatically generated in that controller that will return a list of category records that have a name matching a given search query. This is very cool, and is a typical example of Ruby on Rails magic: you add one line to a class in your application and suddenly an entire feature or behavior is added, customized to the data and objects in your app!</p>
<p>This sort of thing is really what makes Ruby on Rails so amazing&hellip; but how does it work? Let&rsquo;s take a look at the implementation of auto_complete_for method:</p>

<a name='code_snippet0'></a>

<div id='frame'>
  <div id='header'>
    <div id='header_left'>
      
      
    </div>
    <div id='header_right'>
      <a href="#code_snippet1" title="Next code snippet">replace object with :category</a>
      <a href="#code_snippet1" class="icon" title="Next code snippet"></a>

    </div>
  </div>


</div>

<div id='frame'>
tbd
</div>

