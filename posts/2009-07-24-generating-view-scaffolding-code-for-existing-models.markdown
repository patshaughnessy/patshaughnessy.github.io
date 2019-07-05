title: "Generating view scaffolding code for existing models"
date: 2009/07/24
tag: View Mapper

<p>(Updated October 2009)</p>
<p>I just rewrote and cleaned up the code I describe below here and published it as a new gem called <a href="http://gemcutter.org/gems/view_mapper">View Mapper on gemcutter.org</a>. See <a href="http://patshaughnessy.net/view_mapper">my usage article</a> for more details. I also rethought and redesigned the commands I describe below&hellip; the part about nested attributes will not work for now. I&rsquo;m planning to reimplement that soon.</p>
<p/>
<p>I&rsquo;ve been thinking for a while that a generator that creates view scaffolding for an existing model or models would be really useful. For example, I want to be able to write this by hand:</p>
<pre>class Group &lt; ActiveRecord::Base
  has_many :people
end
class Person &lt; ActiveRecord::Base
  belongs_to :group
end</pre>
<p>&hellip; and then run a generator and get a working view to edit and display groups of people. Recently I started developing a generator called &ldquo;ViewMapper&rdquo; that does this; see: <a href="http://github.com/patshaughnessy/view_mapper">http://github.com/patshaughnessy/view_mapper</a>.</p>
<p>The idea is that it creates a view that maps to your existing models. Writing model classes is often very easy; using ActiveRecord allows you to write concise, short code files. However, writing a view that displays a form for these models is often very hard, requiring knowledge of a long list of Rails functions, macros, classes, as well as the usual HTML and Javascript. Right now ViewMapper it just a plugin; so you can install it into your app like this:</p>
<pre>$ ./script/plugin install git://github.com/patshaughnessy/view_mapper.git</pre>
<p>Probably it should be packaged as a gem instead… this is still work-in-progress. At the moment it supports:
  <ul>
    <li>Creating simple form based on the columns of an existing model</li>
    <li>Creating a complex form based on the columns of two associated models in a many-one relationship, using nested attributes</li>
  </ul>
  If you&rsquo;re interested and have time, try the examples below or better yet try the generator on some of your own models and let me know what you think.</p>
<p/>
<p/>
<p><u>Example 1:</u> Create a view based on the columns of an existing model</p>
<p>Let&rsquo;s say I have an existing model class that manages a series of person records:</p>
<pre>class Person &lt; ActiveRecord::Base
end</pre>
<p>Now to create the corresponding view with ViewMapper, just run this command:</p>
<pre>$ ./script/generate view_for person
      exists  app/controllers/
      exists  app/helpers/
      create  app/views/people
      exists  app/views/layouts/
      exists  test/functional/
      create  test/unit/helpers/
      exists  public/stylesheets/
      create  app/views/people/index.html.erb
      create  app/views/people/show.html.erb
      create  app/views/people/new.html.erb
      create  app/views/people/edit.html.erb
      create  app/views/layouts/people.html.erb
      create  public/stylesheets/scaffold.css
      create  app/controllers/people_controller.rb
      create  test/functional/people_controller_test.rb
      create  app/helpers/people_helper.rb
      create  test/unit/helpers/people_helper_test.rb
       route  map.resources :people</pre>
<p>If you run your Rails app you will see the usual scaffolding UI:</p>
<img src="http://patshaughnessy.net/assets/2009/7/24/person_new.png">
<p>This looks just like the scaffolding page you would have gotten from the Rails scaffolding generator. In fact, I&rsquo;ve based the &ldquo;view_for&rdquo; generator class (ViewForGenerator) on the existing Rails ScaffoldGenerator class, so it generates the same code.</p>
<p>The difference here is that the scaffolding code was based on the properties of the existing Person model. Here&rsquo;s what the view_for generator did:
  <ul>
    <li>Find the specified model class</li>
    <li>Inspect it and get a list of column names and data types</li>
    <li>Create the scaffolding code as usual</li>
  </ul>
  If you also want to create your model class at the same time, then you don&rsquo;t need ViewMapper; you would just run the standard Rails scaffold generator like this:</p>
<pre>$ ./script/generate scaffold person name:string age:integer</pre>
<p>ViewMapper provides you with the flexibility to create the view scaffolding after the fact for a model you or someone else has already created in your app.</p>
<p/>
<p/>
<p><u>Example 2:</u> Create a view using a complex form based on two associated models in a many-one relationship, using Rails 2.3 nested attributes</p>
<p>Let&rsquo;s suppose you have two related models that describe groups of people:</p>
<pre>class Group &lt; ActiveRecord::Base
  has_many :people
end
class Person &lt; ActiveRecord::Base
  belongs_to :group
end</pre>
<p>Again, writing model classes is really easy. With just a few lines of code I have told ActiveRecord to manage two separate tables and their relationship.</p>
<p>Now you can use ViewMapper to create a complex form for creating and editing groups and people at the same time like this:</p>
<pre>$ ./script/generate nested_attributes_view_for group containing:people
       error  Model &#x27;Group&#x27; does not accept nested attributes
              for child models &#x27;people&#x27;</pre>
<p>This error indicates that my Group model is missing the &ldquo;accepts_nested_attributes_for&rdquo; directive. Since the generator creates view code that relies on the nested attributes feature, it won&rsquo;t let you create the view for a model that is missing this line. This sort of helpful error message is possible since ViewMapper is inspecting an existing model class, and not just creating new scaffold code.</p>
<p>Now if we add the missing line (suddenly my model is less concise... nested attributes aren't that easy to use yet!):</p>
<pre>class Group &lt; ActiveRecord::Base
  has_many :people
  <b>accepts_nested_attributes_for :people,
    :allow_destroy =&gt; true,
    :reject_if =&gt; proc { |attrs| attrs[&#x27;name&#x27;].blank? &amp;&amp; attrs[&#x27;age&#x27;].blank? }</b>
end</pre>
<p>&hellip;we can re-run the generator:</p>
<pre>$ ./script/generate nested_attributes_view_for group containing:people
      exists  app/controllers/
      exists  app/helpers/
      create  app/views/groups
      exists  app/views/layouts/
      exists  test/functional/
      create  test/unit/helpers/
      exists  public/stylesheets/
      create  app/views/groups/index.html.erb
      create  app/views/groups/show.html.erb
      create  app/views/groups/new.html.erb
      create  app/views/groups/edit.html.erb
      create  app/views/layouts/groups.html.erb
      create  public/stylesheets/scaffold.css
      create  app/controllers/groups_controller.rb
      create  test/functional/groups_controller_test.rb
      create  app/helpers/groups_helper.rb
      create  test/unit/helpers/groups_helper_test.rb
       route  map.resources :groups</pre>
<p>And now if we run our app, we get view scaffolding illustrating how to create and edit people and groups all at the same time:</p>
<img src="http://patshaughnessy.net/assets/2009/7/24/group_new.png">
<p>If I save the group&hellip;</p>
<img src="http://patshaughnessy.net/assets/2009/7/24/group_show.png">
<p>And re-editing the same group:</p>
<img src="http://patshaughnessy.net/assets/2009/7/24/group_edit.png">
<p>More to come&hellip; as I said this is work-in-progress. As a next step I&rsquo;m planning to improve the scaffolding code for nested attributes by adding javascript to dynamically add/delete the child objects. But after that I'm thinking about:
  <ul>
    <li>Adding a &ldquo;paperclip_view_for&rdquo; generator that creates a file upload form for a model that contains a &ldquo;has_attached_file&rdquo; directive.</li>
    <li>Adding a &ldquo;auto_complete_view_for&rdquo; generator that adds &ldquo;auto_complete_for&rdquo; to the controller and uses &ldquo;text_field_for_auto_complete&rdquo; in the form.</li>
    <li>Adding the ability for you to extend this with your own scaffolding code.</li>
  </ul>
</p>
