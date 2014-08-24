title: "Creating associations to existing data part 3: has_many :through scaffolding"
date: 2010/04/04
tag: View Mapper

<p>I just updated <a href="http://patshaughnessy.net/view_mapper">View Mapper</a> to support scaffolding for models in a has_many, :through relationship. It generates a complex form that is a combination of the &ldquo;belongs_to&rdquo; scaffolding from <a href="http://patshaughnessy.net/2010/1/25/creating-associations-to-existing-data-part-1-belongs_to-scaffolding">part 1 of this series</a> and the nested attributes scaffolding <a href="http://patshaughnessy.net/2009/11/9/scaffolding-for-complex-forms-using-nested-attributes">I wrote about in November</a>:</p>
<p><img src="http://patshaughnessy.net/assets/2010/4/4/hmt_form.png"></p>
<p>Based on the programmer/assignment/project example from the <a href="http://api.rubyonrails.org/classes/ActiveRecord/Associations/ClassMethods.html">ActiveRecord documentation page</a>, this form will create a new programmer record and allow the user to add one or more assignments, each of which also has a name text field. For each new assignment the user can also select an existing project record. Here&rsquo;s the Programmer model with the has_many :through association:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="r">class</span> <span class="cl">Programmer</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  has_many <span class="sy">:projects</span>, <span class="sy">:through</span> =&gt; <span class="sy">:assignments</span>
  has_many <span class="sy">:assignments</span>
  accepts_nested_attributes_for <span class="sy">:assignments</span>,
                                <span class="sy">:allow_destroy</span> =&gt; <span class="pc">true</span>,
                                <span class="sy">:reject_if</span> =&gt; proc { |attrs|
                                  attrs[<span class="s"><span class="dl">'</span><span class="k">name</span><span class="dl">'</span></span>].blank? &amp;&amp;
                                  attrs[<span class="s"><span class="dl">'</span><span class="k">project_id</span><span class="dl">'</span></span>].blank?
                                }
<span class="r">end</span></pre></div>
</div><br>
<p>This implements a many-many relationship between programmers and projects; the assignments model is used to map the projects with the programmers. I&rsquo;ve also specified that the programmer model accepts_nested_attributes_for assignments&hellip; more on that below.</p>
<p>You can now use the &ldquo;view_for&rdquo; generator from View Mapper to generate the form above for your models using a new view called &ldquo;has_many_existing:&rdquo;</p>
<div class="CodeRay">
  <div class="code"><pre>$ sudo gem install view_mapper
$ script/generate view_for programmer --view has_many_existing:projects</pre></div>
</div><br>
<p><b>Assumptions and requirements</b></p>
<p><u>Nested Attributes:</u> the form above works by using ActiveRecord&rsquo;s nested attributes feature to save multiple assignments for a single programmer. Therefore, you need to be sure you call accepts_nested_attributes_for in your target model; if you forget to do this, you&rsquo;ll get an error from View Mapper:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate view_for programmer --view has_many_existing:projects
    warning  Model Programmer does not accept nested attributes
    for model Assignment.</pre></div>
</div><br>
<p>To fix this problem you can use the code I showed above:</p>

<div class="CodeRay">
  <div class="code"><pre><span class="r">class</span> <span class="cl">Programmer</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
   has_many <span class="sy">:projects</span>, <span class="sy">:through</span> =&gt; <span class="sy">:assignments</span>
   has_many <span class="sy">:assignments</span>
<div class='container'>   accepts_nested_attributes_for <span class="sy">:assignments</span>,
                                 <span class="sy">:allow_destroy</span> =&gt; <span class="pc">true</span>,
                                 <span class="sy">:reject_if</span> =&gt; proc { |attrs|
                                   attrs[<span class="s"><span class="dl">'</span><span class="k">name</span><span class="dl">'</span></span>].blank? &amp;&amp;
                                   attrs[<span class="s"><span class="dl">'</span><span class="k">project_id</span><span class="dl">'</span></span>].blank?
                                 }<span class='overlay'></span></div> <span class="r">end</span></pre></div>
</div><br>
<p>The options I&rsquo;ve specified here tell ActiveRecord it is allowed to delete assignment records (when the user clicks &ldquo;remove&rdquo; in the form) and to avoid creating empty assignment records if all of their attributes are blank (if the user clicked &ldquo;Add Assignment&rdquo; an extra time).</p>
<p>Or if you prefer you can generate the entire target model including the nested attributes call using the scaffold_for_view generator like this &ndash; specify the new model&rsquo;s columns using the same syntax as the standard Rails scaffold generator:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate scaffold_for_view programmer
                  first_name:string last_name:string
                  --view has_many_existing:projects</pre></div>
</div><br>
<p>It&rsquo;s easy to overlook one very elegant detail here about ActiveRecord&rsquo;s nested attribtues feature: note that &ldquo;project_id&rdquo; is one of the nested attributes, generated by each of the project select list boxes. (They are implemented with collection_select; see <a href="http://patshaughnessy.net/2010/1/25/creating-associations-to-existing-data-part-1-belongs_to-scaffolding">part 1 of this series</a>). Now when the new programmer form is submitted all of the associations for each assignment &ndash; and for the new programmer &ndash; are setup. In other words, after you save the new programmer record this way you can immediately access the associated projects through assignments: &ldquo;programmer.projects&rdquo; &ndash; very cool! And it's all seamless: I don't have to write any code in my controller to associate the projects or assignments with the new programmer.</p>
<p><u>Correct associations among your models:</u> if you forget to put the proper associations in your three models the has_many :through behavior will not work. You need to have six associations setup among your three models like this:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="r">class</span> <span class="cl">Programmer</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  has_many <span class="sy">:projects</span>, <span class="sy">:through</span> =&gt; <span class="sy">:assignments</span>
  has_many <span class="sy">:assignments</span>
<span class="r">end</span>

<span class="r">class</span> <span class="cl">Project</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  has_many <span class="sy">:programmers</span>, <span class="sy">:through</span> =&gt; <span class="sy">:assignments</span>
  has_many <span class="sy">:assignments</span>
<span class="r">end</span>

<span class="r">class</span> <span class="cl">Assignment</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  belongs_to <span class="sy">:project</span>
  belongs_to <span class="sy">:programmer</span>
<span class="r">end</span></pre></div>
</div><br>
<p>View Mapper will help you out by displaying an error message if you&rsquo;re missing one of these:</p>
<div class="CodeRay">
<div class="code"><pre>$ script/generate scaffold_for_view programmer name:string
                  --view has_many_existing:projects
   warning  Model Project does not contain a has_many association for Assignment.</pre></div>
</div><br>
<p>&hellip;or if you&rsquo;re missing one of the corresponding foreign key columns in the &ldquo;through&rdquo; model:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate scaffold_for_view programmer name:string
                  --view has_many_existing:projects
    warning  Model Assignment does not contain a foreign key for Programmer.</pre></div>
  </div><br>
<p><u>Has many existing model identified by name attribute:</u> In the form above, the Project records were identified in the select list boxes using their &ldquo;name&rdquo; attribute. Therefore, you need to insure that your existing model has a name column or method; if it does not View Mapper will display an error message like this:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate scaffold_for_view programmer name:string
                  --view has_many_existing:projects
    warning  Model Project does not have a name attribute.</pre></div>
  </div><br>
<p>To fix this problem, add a &ldquo;name&rdquo; method to your existing model, or else you can specify that View Mapper use a different attribute (e.g. &ldquo;code&rdquo;) instead with this syntax:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate scaffold_for_view programmer name:string
                  --view has_many_existing:projects[code]</pre></div>
</div><br>
<p><u>Associated model name method in through model:</u> The last requirement is that the through model, Assignment in this example, have a method (&ldquo;project_name&rdquo;) to display the name of its associated existing model. View Mapper requires this to avoid putting this code into the view:</p>
<div class="CodeRay">
  <div class="code"><pre> <span class="r">class</span> <span class="cl">Assignment</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
   belongs_to <span class="sy">:project</span>
   belongs_to <span class="sy">:programmer</span>
<div class='container'>   <span class="r">def</span> <span class="fu">project_name</span>
     project.name <span class="r">if</span> project
   <span class="r">end</span><span class='overlay'></span></div> <span class="r">end</span>
</pre></div>
</div><br>
<p>If you forget this method, View Mapper will remind you with this error message:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate scaffold_for_view programmer name:string
                  --view has_many_existing:projects[code]
     warning  Model Assignment does not have a method project_code.</pre></div>
</div><br>
<p><b>Detailed Example</b>
<p>Here&rsquo;s a step by step example of how to create a Rails application from scratch that contains the has_many :through scaffolding:</p>
<div class="CodeRay">
  <div class="code"><pre>$ rails hmt_example</pre></div>
</div><br>
<p>Here&rsquo;s our model to hold the existing data:</p>
<div class="CodeRay">
  <div class="code"><pre>$ cd hmt_example
$ script/generate model project code:string</pre></div>
</div><br>
<p>And the through model to associate projects with programmers; note I&rsquo;ve included integer attributes as the foreign keys for both the existing model and the new model:</p>
<div class="CodeRay">
  <div class="code"><pre>$ script/generate model assignment name:string
                        project_id:integer programmer_id:integer
$ rake db:migrate</pre></div>
</div><br>
<p>Next edit the new models and enter the required associations along with the project_code method:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="r">class</span> <span class="cl">Project</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  has_many <span class="sy">:programmers</span>, <span class="sy">:through</span> =&gt; <span class="sy">:assignments</span>
  has_many <span class="sy">:assignments</span>
<span class="r">end</span>

<span class="r">class</span> <span class="cl">Assignment</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  belongs_to <span class="sy">:programmer</span>
  belongs_to <span class="sy">:project</span>
  <span class="r">def</span> <span class="fu">project_code</span>
    project.code <span class="r">if</span> project
  <span class="r">end</span>
<span class="r">end</span>
</pre></div>
</div><br>
<p>Now we&rsquo;re ready to create the programmer has_many :through scaffolding; note I&rsquo;ve specified &ldquo;code&rdquo; as the attribute to use to identify each project:</p>
<div class="CodeRay">
  <div class="code"><pre>$ sudo gem install view_mapper
$ script/generate scaffold_for_view programmer
                  first_name:string last_name:string
                  --view has_many_existing:projects[code]
$ rake db:migrate</pre></div>
</div><br>
<p>Note this won&rsquo;t work yet for a has_and_belongs_to_many association; dealing with that is next on my View Mapper to do list.</p>
