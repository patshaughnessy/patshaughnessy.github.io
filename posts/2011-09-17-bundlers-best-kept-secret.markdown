title: "Bundler’s Best Kept Secret"
date: 2011/9/17
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/9/17/gem_graph_cropped.png"></td></tr>
  <tr><td align="center"><small><i>Part of a gem dependency network graph</i></small></td></tr>
</table>
</div>

This week I just discovered Bundler’s best kept secret: the <span class="code">bundle viz</span> command will generate a network graph showing the dependencies among all the different gems used by your Ruby app. For example, the image on the left is a portion of the gem dependency graph for a vanilla Rails 3.1 app. [Click here](https://patshaughnessy.net/assets/2011/9/17/gem_graph.png) to see the entire, uncropped dependency graph. The gems actually called out in your Gemfile are displayed in grey, while other gems included through dependencies only are shown in white. Finally gem groups are shown as rectangles along the top.

To try this on your own app, you’ll first need to install the [GraphViz](http://www.graphviz.org/) library, which is what Bundler uses to generate the graph. On my Mac Lion laptop, I used Homebrew:

<pre type="console">
$ brew install graphviz
</pre>

You can also install it with Macports, and on the [GraphViz web site](http://www.graphviz.org/Download.php) there are executable binaries available for the Linux, Mac SnowLeopard and Leopard and also Windows platforms. Once you have the GraphViz library installed, you’ll also need to install the ruby-graphviz gem (or add it to your Gemfile):

<pre type="console">
$ gem install ruby-graphviz
</pre>

Then you can go ahead and run:

<pre type="console">
$ bundle viz
/Users/pat/apps/path/to/my_app/gem_graph.png
</pre>

... and it will display the path to a new “gem_graph.png” file. Open this up and you’ll see your gem dependency network graph; very cool! There are also options to include each gem's version number (<span class="code">--version</span>) or dependency requirements (<span class="code">--requirements</span>) in the chart:

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/9/17/gem_graph_version.png"></td></tr>
  <tr><td align="center"><small>bundle viz --version</small></td></tr>
</table>
</div>

<div style="float: right; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/9/17/gem_graph_requirements.png"></td></tr>
  <tr><td align="center"><small>bundle viz --requirements</small></td></tr>
</table>
</div>

<div style="clear: both"></div>

## Example: Displaying ActiveRecord associations with GraphViz

Using the ruby-graphviz gem from Ruby is actually quite easy, and you can use the Bundler source code in [graph.rb](https://github.com/carlhuda/bundler/blob/1-0-stable/lib/bundler/graph.rb) as a working example to learn from.

First you need to create a new GraphViz object like this:

<pre type="ruby">
graph_viz = GraphViz::new('Gemfile', {:concentrate => true, :normalize => true, :nodesep => 0.55})
graph_viz.edge[:fontname] = graph_viz.node[:fontname] = 'Arial, Helvetica, SansSerif'
graph_viz.edge[:fontsize] = 12
</pre>

These are the options that Bundler uses for the font, font size and other layout attributes; you can take a look at the [documentation on the GraphViz site](http://www.graphviz.org/doc/info/attrs.html) to learn about all of the other options available.

As an example today, we can display nodes for all of the ActiveRecord models in your Rails app using code like this:

<pre type="ruby">
models = {}
each_model do |model|
  name = model.to_s
  models[name] = graph_viz.add_node(name, { :shape => 'box3d',
                                            :fontsize => 16,
                                            :style => 'filled',
                                            :fillcolor => '#B9B9D5' } )
end
</pre>

Here I’m calling graph_viz.add_node for each model, and then saving the nodes in a hash called “models.” The <span class="code">each_model</span> method will look like this:

<pre type="ruby">
def each_model
  ObjectSpace.each_object(Class) do |klass|
    yield klass if klass.ancestors.include?(ActiveRecord::Base) && klass != ActiveRecord::Base
  end
end
</pre>

This just iterates over all of the Ruby classes loaded in the system, and yields just those that are subclasses of ActiveRecord::Base. To try this code yourself, you’ll have to run with RAILS_ENV=production to force Rails to preload all of the model classes.

Once you have the nodes on the chart, the next step is to draw the lines between them using the <span class="code">add_edge</span> method. Looking in [graph.rb](https://github.com/carlhuda/bundler/blob/1-0-stable/lib/bundler/graph.rb) you can see how Bundler calls <span class="code">add_edge</span> for each gem’s dependencies. For our ActiveRecord associations example, we’ll use code like this:

<pre type="ruby">
each_model do |model|
  model_name = model.to_s
  model.reflect_on_all_associations.each do |assoc|
    assoc_name = assoc.name.to_s.singularize.camelize
    graph_viz.add_edge(models[model_name],
                       models[assoc_name],
                       { :weight => 2 }
                      ) unless models[assoc_name].nil?
  end
end
</pre>

This uses the reflect_on_all_associations method to find all of the associations for the given model, and then passes in the source and destination nodes after looking them up in the models hash.

Finally, we just ask GraphViz to generate the network graph and save it into an image file like this:

<pre type="ruby">
graph_viz.output( :png => 'activerecord_associations.png' )
</pre>

For a simple has_many/belongs_to assocation like this: 

<pre type="ruby">
class Group < ActiveRecord::Base
  has_many :people
end

class Person < ActiveRecord::Base
  belongs_to :group
end
</pre>

...you’ll get a network graph that looks like this:

![page scope](https://patshaughnessy.net/assets/2011/9/17/activerecord_associations.png)

Here’s a more real world example - this is the network graph I get for the ActiveRecord associations in the BostonRB web site [http://bostonrb.org](http://bostonrb.org):

![page scope](https://patshaughnessy.net/assets/2011/9/17/bostonrb_models.png)
