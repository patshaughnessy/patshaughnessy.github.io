title: "The auto_complete plugin refactored to support repeated fields and named scopes"
date: 2008/11/18
url: /repeated_auto_complete
tag: the auto_complete plugin

<p>(Updated June 2009)</p>
<p>This version of auto_complete will support text fields that are repeated more than once on a complex form. It allows you to call text_field_with_auto_complete on the form builder object yielded by fields_for or form_for. This will work for complex forms built with Rails 2.2 or earlier, and for the nested attributes feature introduced in Rails 2.3. Here's an example using nested attributes:</p>
<pre>&lt;% form_for @project do |project_form| %&gt;
  &lt;% project_form.fields_for :tasks do |task_form| %&gt;
    &lt;p&gt;
      &lt;%= task_form.label :name, &quot;Task:&quot; %&gt;
      &lt;%= task_form.text_field_with_auto_complete :name, {},
          { :method =&gt; :get, :skip_style =&gt; true } %&gt;
    &lt;/p&gt;
  &lt;% end %&gt;
&lt;% end %&gt;</pre>
<p>It also allows you to provide a block to auto_complete_for in your controller that filters the drop down pick list in some custom way. For example, this block would display task names for the project the user had selected elsewhere on the same form, using a named scope by_project:</p>
<pre>auto_complete_for :task, :name do | items, params |
  items.by_project(params['project'])
end</pre><br/>
<p><b>Code:</b>&nbsp;&nbsp;<a href="http://github.com/patshaughnessy/auto_complete">http://github.com/patshaughnessy/auto_complete</a></p>
<p><b>Install as a gem:</b></p>
<pre>
gem sources -a http://gemcutter.org
sudo gem install repeated_auto_complete</pre><br/>
<p>&hellip; and in config/environment.rb:
<pre>Rails::Initializer.run do |config|
&hellip;
  config.gem &quot;repeated_auto_complete&quot;
&hellip;
end</pre><br/>
<p><b>Install as a plugin:</b></p>
<pre>script/plugin install git://github.com/patshaughnessy/auto_complete.git</pre><br/>
<p><b>More information:</b>
<ul>
  <li><a href="http://patshaughnessy.net/2009/11/25/scaffolding-for-auto-complete-on-a-complex-nested-form">How to generate scaffolding for auto_complete on a complex form</a></li>
  <li><a href="http://patshaughnessy.net/2009/6/15/auto-complete-for-complex-forms-using-nested-attributes-in-rails-2-3">How the plugin works with Rails 2.3 nested attributes</a></li>
  <li><a href="http://patshaughnessy.net/2009/6/15/repeated-auto-complete-plugin-usage-change">Recent usage changes (June 2009) to enable nested attribute support</a></li>
  <li><a href="http://patshaughnessy.net/2009/1/30/repeated_auto_complete-changes-merged-into-auto_complete">Detailed description of code changes</a></li>
  <li><a href="http://patshaughnessy.net/2009/1/30/sample-app-for-auto-complete-on-a-complex-form">Sample app showing how to use the plugin</a></li>
  <li><a href="http://patshaughnessy.net/2009/4/3/filtering-auto_complete-pick-lists-part-2-using-named-scopes">Explanation of how to use named scopes with auto_complete_for</a></li>
  <li><a href="http://patshaughnessy.net/2009/3/14/filtering-auto_complete-pick-lists">Related article about how to filter auto_complete pick lists</a></li>
  <li><a href="http://patshaughnessy.net/2008/10/21/autocomplete-plugin-doesn-t-work-for-repeated-fields">Why the auto_complete plugin doesn’t work for repeated fields</a></li>
  <li><a href="http://patshaughnessy.net/2008/11/16/testing-is-a-lesson-in-humility">My experience writing unit tests for the modified plugin</a></li>
  <li><a href="http://patshaughnessy.net/2008/10/31/modifying-the-autocomplete-plugin-to-allow-repeated-fields">My original changes to auto_complete from October</a> (no longer used)</li>
</ul></p>
