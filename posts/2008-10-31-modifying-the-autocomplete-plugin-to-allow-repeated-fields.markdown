title: "Modifying the auto_complete Plugin to Allow Repeated Fields"
date: 2008/10/31
url: /2008/10/31/modifying-the-autocomplete-plugin-to-allow-repeated-fields
tag: the auto_complete plugin

<p>Update March 2009: I reimplemented the code from this article in a better way and posted it in a fork of auto_complete on github; see: <a href="http://patshaughnessy.net/repeated_auto_complete">http://patshaughnessy.net/repeated_auto_complete</a>.... however, the basic ideas below still apply.</p>
<a href="http://patshaughnessy.net/2008/10/21/autocomplete-plugin-doesn-t-work-for-repeated-fields">Last week I ran into trouble</a> trying to use the auto_complete plugin like this for a form containing a single group name field, but a series of repeated people name fields:
<pre>&lt;p&gt;
  Group &lt;%= f.label :name %&gt;&lt;br /&gt;
  &lt;%= text_field_with_auto_complete :group, :name,
                                       {}, {:method =&gt; :get} %&gt;&lt;/p&gt;
&lt;% for person in @group.people %&gt;
  &lt;% fields_for "group[person_attributes][]", person do |person_form| %&gt;
    &lt;p&gt;
      Person &lt;%= person_form.label :name %&gt;&lt;br /&gt;
     &lt;%= text_field_with_auto_complete :person, :name, {:index =&gt; nil},
                                      {:method =&gt; :get}  %&gt;&lt;/p&gt;
    &lt;% unless person.new_record? %&gt;
      &lt;%= person_form.hidden_field :id, :index =&gt; nil %&gt;
    &lt;% end %&gt;
  &lt;% end %&gt;
&lt;% end %&gt;</pre>
<p>I want to just display a text field repeated once for each person record. The problem is that the text_field_with_auto_complete macro returns HTML and Javascript that doesn’t work when it’s repeated many times. How can I to get this to work?</p>
Let’s start by writing the code we would like to use, or wished could work someday. The key detail in the form above is the name of the object in fields_for:
<pre>fields_for "group[person_attributes][]", person do |person_form|</pre>
<p>This name is used on the server to mass-assign the attributes of all of the person records to the parent group record. The only way text_field_with_auto_complete could possibly work is if it generated an &lt;input&gt; tag with the desired name: “group[person_attributes][][name]”. I could just pass this value into text_field_with_auto_complete, but that not be very DRY since I would have to repeat the name more than once.</p>
What if I could just call text_field_with_auto_complete directly on the person_form object yielded by fields_for? For example:
<pre>&lt;% fields_for "group[person_attributes][]", person do |person_form| %&gt;
  &lt;%= person_form.text_field_with_auto_complete :person, :name %&gt;
</pre>
<p>Then this call could generate an &lt;input&gt; tag with a name generated from the fields_for object name and my form code would remain very simple, readable and maintainable.</p>
I found a way to do this following John Ford’s example (<a href="http://www.aldenta.com/2006/09/19/writing-a-custom-formbuilder-in-rails/">Writing a Custom FormBuilder in Rails</a>) by adding a new version of form_for and fields_for to ActionView. First I modified the auto_complete plugin’s init.rb file and added a line to mixin a new form helper module into ActionView like this:
<pre>ActionView::Base.send :include, AutoCompleteFormHelper</pre>
Then our new AutoCompleteFormHelper class will look like this:
<pre>module AutoCompleteFormHelper
  [:form_for, :fields_for, :form_remote_for, :remote_form_for].each do |meth|
    src = &lt;&lt;-end_src
      def auto_complete_#{meth}(object_name, *args, &amp;proc)
        options = args.last.is_a?(Hash) ? args.pop : {}
        options.update(:builder =&gt; AutoCompleteFormBuilder)
        #{meth}(object_name, *(args &amp;lt;&amp;lt; options), &amp;proc)
      end
    end_src
    module_eval src, __FILE__, __LINE__
  end
end</pre>
<p>What this does is create new ActionView methods called “auto_complete_form_for,” “auto_complete_fields_for,” etc. These methods simply call the original form_for and fields_for but pass in an additional option “:builder” that indicates ActionView should yield a different class to the form block… a new class I will write called “AutoCompleteFormBuilder,” instead of the original FormBuilder class. Now by writing AutoCompleteFormBuilder we have the ability to implement the behavior we need from text_field_with_auto_complete.</p>
<p>Here’s what I ended up with:</p>
<pre>class AutoCompleteFormBuilder &lt; ActionView::Helpers::FormBuilder
  def text_field_with_auto_complete(object, method, tag_options = {},
                                    completion_options = {})
    @template.text_field_with_auto_complete(
      "#{object}_#{Object.new.object_id}",
      method,
      { :name => "#{@object_name}[#{method}]" }.update(tag_options),
      { :param_name => "#{object}[#{method}]",
        :url => { :action => "auto_complete_for_#{object}_#{method}" }
      }.update(completion_options)
    )
  end  
end</pre>
This simply calls the original text_field_with_auto_complete with slightly different parameters:
<ul>
  <li>We generate a unique number and add it as a suffix to the id attribute that will be generated for the &lt;input&gt; tag, and referenced in the Javascript:
    <pre>"#{object}_#{Object.new.object_id}"</pre>
    Now each &lt;input&gt; tag will have a unique id attribute, and the script.aculo.us autocomplete Javascript will work unmodified. Without this change, all of the
    repeated &lt;input&gt; tags would have the same id, and the autocomplete code would fail trying to find the ambiguous id on the page.
    </li>
  <li>We pass in the object name from the fields_for or forms_for declaration as the name attribute for the &lt;input&gt; tag:
    <pre>{ :name => "#{@object_name}[#{method}]" }.update(tag_options)</pre>
    Now our model code works as originally intended when the form is submitted, i.e. the "person_attributes" value will be mass-assigned to our child model objects.
    </li>
  <li>We get the autocomplete behavior to work by telling it explicitly to use the simple “person[name]” object name in both the parameter name and URL for the AJAX request:
    <pre>{ :param_name => "#{object}[#{method}]",
  :url => { :action => "auto_complete_for_#{object}_#{method}" }
}.update(completion_options)</pre>
This allow us to continue to use the same simple declaration in the controller that handles the auto complete requests:
<pre>auto_complete_for :person, :name</pre>
</li>
</ul>
<p>Next step: Post this on GitHub.</p>
