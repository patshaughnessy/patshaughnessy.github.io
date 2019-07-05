title: "Repeated auto complete plugin usage change"
date: 2009/06/15
tag: the auto_complete plugin

<p>I&rsquo;ve forked the auto_complete plugin to support repeated text fields in a complex form; see <a href="http://patshaughnessy.net/repeated_auto_complete">http://patshaughnessy.net/repeated_auto_complete</a> for more details.</p>
<p>If you had downloaded my plugin in the past, I&rsquo;ve just made a couple of changes that will require some simple code changes to your app:
  <ul>
    <li>You no longer need to or are able to use &ldquo;auto_complete_form_for&rdquo; or &ldquo;auto_complete_fields_for.&rdquo; I decided this was confusing and unnecessary. Now my plugin just mixes the text_field_with_auto_complete method right  into the standard FormBuilder class. Just use form_for or fields_for as usual.</li>
    <li>I also dropped the object name parameter from text_field_with_auto_complete. Since text_field_with_auto_complete is a method of the form builder, the target object is indicated by the surrounding call to fields_for or form_for and so doesn&rsquo;t need to be repeated. Now using form.text_field_with_auto_complete is very similar to using form.text_field or the other form builder methods: you just need to specify the column/field name.</li>
  </ul>
</p>
<p>So if you are using my old plugin with a Rails 2.2 or earlier app like this:</p>
<pre>&lt;% for person in @group.people %&gt;
  &lt;% <b>auto_complete_</b>fields_for &quot;group[person_attributes][]&quot;, person do |form| %&gt;
    Person &lt;%= person_form.label :name %&gt;&lt;br /&gt;
    &lt;%= form.text_field_with_auto_complete <b>:person,</b> :name, {},
                                           {:method =&gt; :get}  %&gt;
  &lt;% end %&gt;
&lt;% end %&gt;</pre>
<p>&hellip; you should drop &ldquo;auto_complete_&rdquo; and &ldquo;:person&rdquo; and just use code like this instead:</p>
<pre>&lt;% for person in @group.people %&gt;
  &lt;% fields_for &quot;group[person_attributes][]&quot;, person do |form| %&gt;
    Person &lt;%= person_form.label :name %&gt;&lt;br /&gt;
    &lt;%= form.text_field_with_auto_complete :name, {},
                                           {:method =&gt; :get}  %&gt;
  &lt;% end %&gt;
&lt;% end %&gt;</pre>
<p>And if you have Rails 2.3 or later and are using nested attributes, this would become:</p>
<pre>&lt;% form_for @group do |group_form| -%&gt;
  &lt;% group_form.fields_for :people do |person_form| %&gt;
    Person &lt;%= person_form.label :name %&gt;&lt;br /&gt;
    &lt;%= person_form.text_field_with_auto_complete :name, {},
          { :method => :get, :skip_style => true } %>
  &lt;% end %&gt;
&lt;% end %&gt;</pre>
