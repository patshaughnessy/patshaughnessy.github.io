title: "View Mapper: Scaffolding for your models and plugins"
date: 2009/10/01
url: /2009/10/1/view-mapper-generate-complex-view-code-for-your-models
tag: View Mapper

<p>View Mapper will generate scaffolding illustrating how to write view code using a specified plugin or feature with your existing models. It can also generate new models.</p>
<p>A couple simple examples:</p>
<pre>script/generate view_for office --view auto_complete:address</pre>
<p>&hellip; will generate Rails scaffolding code that displays a form for an existing &ldquo;office&rdquo; model, with auto complete on the &ldquo;address&rdquo; field.</p>
<pre>script/generate scaffold_for_view office address:string code:string
                                  --view auto_complete:address</pre>
<p>&hellip; will generate the same form, but also create the &ldquo;office&rdquo; model class file, a migration file containing the &ldquo;address&rdquo; and &ldquo;code&rdquo; columns, and other standard scaffolding files as well.</p>
<p>The idea behind View Mapper is that it&rsquo;s easy to write simple, concise model classes representing your domain objects using ActiveRecord, but very hard to implement the corresponding views using a combination of HTML, Javascript Rails helper functions, routes, controllers, etc. If you&rsquo;re not very familiar with a certain plugin you want to use in your app, View Mapper can help you get started in the right direction by generating a working example with scaffolding code.</p>
<p>If you&rsquo;re developing a Rails plugin or gem it&rsquo;s easy to write your own View Mapper module for your plugin&rsquo;s users to call with View Mapper.</p>
<p/>
<p/>
<p><b>Code:</b>&nbsp;&nbsp;&nbsp;<a href="http://github.com/patshaughnessy/view_mapper">http://github.com/patshaughnessy/view_mapper</a></p>
<p/>
<p/>
<p><b>Install:</b></p>
<pre>sudo gem install view_mapper</pre>
<p/>
<p/>
<p><b>Usage:</b></p>
<p>Two generators are provided, called view_for and scaffold_for_view:</p>
<p/>
<pre>script/generate view_for model [ --view view_name:param ]</pre>
    <p>This will generate the specified view code for an existing model. The view_for generator will look for your model, inspect all of its columns and then generate standard Rails scaffolding containing a form field for each existing column.</p>
    <p>If you also specify a view, then a custom view will be created using the specified Rails feature or plugin, using the specified parameter.</p>
<p/>
<pre>script/generate scaffold_for_view model attributes [ --view view_name:param ]</pre>
    <p>If you don&rsquo;t specify a view, then this command is identical to the standard Rails scaffold generator.</p>
    <p>If you do specify a view, then the entire working set of a model, views and controller will be generated to implement the specified Rails feature or plugin, using the specified parameter.</p>
<p/>
<p/>
<p><b>Views:</b></p>
<p>Right now, I&rsquo;ve implemented eight views:
<ul>
  <li><a href="http://patshaughnessy.net/2009/10/1/auto_complete-scaffolding">auto_complete</a>: Uses the standard Rails auto_complete plugin to implement type ahead behavior for the specified field.</li>
  <li><a href="http://patshaughnessy.net/2009/10/16/paperclip-scaffolding">paperclip</a>: Uses the Paperclip plugin to upload and download file attachments.</li>
  <li><a href="http://patshaughnessy.net/2009/11/9/scaffolding-for-complex-forms-using-nested-attributes">has_many</a>: Displays a complex form to edit two or more associated models.</li>  <li><a href="http://patshaughnessy.net/2009/11/25/scaffolding-for-auto-complete-on-a-complex-nested-form">has_many_auto_complete</a>: This is the same as has_many but also uses the auto_complete plugin to implement type ahead behavior for each text field. This view requires you to install <a href="http://patshaughnessy.net/repeated_auto_complete">my fork of the Rails auto_complete plugin</a>.</li>
  <li><a href="http://patshaughnessy.net/2010/1/25/creating-associations-to-existing-data-part-1-belongs_to-scaffolding">belongs_to</a>: Generates scaffolding that allows you to select an existing, associated model.</li>
  <li><a href="http://patshaughnessy.net/2010/2/13/creating-associations-to-existing-data-part-2-belongs_to-with-auto_complete">belongs_to_auto_complete</a>: Generates scaffolding that allows you to select an existing, associated model using auto_complete.</li>
  <li><a href="http://patshaughnessy.net/2010/4/4/creating-associations-to-existing-data-part-3-has_many-through-scaffolding">has_many_existing</a>: Generates scaffolding for a complex form to edit two models that have a has_many, :through association with a third model. Use this if you have a many-many relationship with existing data.</li>  
  <li>(Default) If no view is specified, then standard Rails scaffold code will be generated.</li>
</ul>
<p>I&rsquo;ll be implementing more views in the coming weeks and months. There is also an API for implementing your own View Mapper module, for example to generate code illustrating how to use a plugin or gem you are working on. In the future I&rsquo;ll document this as well.</p>
