title: "View Mapper for Rails 3: ScaffoldHub"
date: 2011/3/13
tag: ScaffoldHub

Back in 2009 I wrote a gem called [View Mapper](http://patshaughnessy.net/2009/10/1/view-mapper-generate-complex-view-code-for-your-models) which created different variations on the standard Rails scaffolding user interface. This was cool because it could show you how to use a certain plugin or gem, and it even worked with existing models and their associations.

This year I’ve been thinking about how to upgrade View Mapper to work with Rails 3 properly... and in the end I decided not to upgrade View Mapper at all but instead to create something entirely new: [ScaffoldHub](http://scaffoldhub.org).

![ScaffoldHub](http://patshaughnessy.net/assets/2011/3/13/scaffoldhub.png)

This is both a web site and a gem... the [http://scaffoldhub.org](http://scaffoldhub.org) site displays a gallery of variations on the Rails scaffolding generator, while a single scaffoldhub gem allows you to run any of these “scaffolds” inside your own app!

## Scaffolding: learn by example

Each scaffolding variation might use a certain JQuery plugin (e.g. autocomplete or date picker) or Rails gem (e.g. Paperclip or Will Paginate). This is a great way to get a jump start on learning how to use that particular plugin or gem with Rails. In seconds you something working in your app, and then you can take a look at the generated scaffolding code in detail to see how it works and adapt it to your needs.

As as example, here’s the screen shot shown on scaffoldhub.org for the “autocomplete scaffold:”
￼
![ScaffoldHub](http://patshaughnessy.net/assets/2011/3/13/autocomplete.png)

Once you have the ScaffoldHub gem installed, all you need to do is type in the command line shown on the web site to create that version of the scaffolding right inside your Rails 3 app. See my example below for more details.

## Community Driven

I intend ScaffoldHub.org to be community driven: instead writing all the scaffolds myself, anyone will be able to post their own version of Rails scaffolding and end users will see it on the site along with the author’s gravitar, immediately available for use.

The variety and usefulness of the scaffolds won’t be limited by my time or imagination (both in very short supply!) but instead anyone with a good idea and a few hours will be able to post a new scaffold. There won’t be any hidden agenda or preference for coding style or tools; if you disagree or just don’t like the way a certain version of the scaffolding user interface works, just post a newer, better version!

## Easy to use a scaffold; easy to write a scaffold

The key design goals I have in mind for ScaffoldHub are:

* To make using different scaffolds tremendously easy for end users. The web site will show you a screen shot of exactly what you’ll get after running each scaffold. Then you will run the scaffoldhub generator the same way you use the standard Rails scaffold generator: a single “rails generate” command.

* To also make it easy for anyone to write a new scaffold: no Rails generator code to write; no need to create and publish a new gem. You will just post a single code file called the “scaffold spec” along with your ERB templates and you’re done. All end users will be able to immediately start using it without the need to install a new version of the scaffoldhub gem, or any other gem.

## Detailed example

To get things started, I just posted a single new scaffold on ScaffoldHub.org: the [AutoComplete scaffold](http://scaffoldhub.org/scaffolds/autocomplete). This scaffold looks the same as the standard Rails scaffolding code, except you can use type ahead/autocomplete on one of the text fields. I implemented it using the [JQuery autocomplete plugin](http://docs.jquery.com/Plugins/autocomplete).

To run it, first just install the ScaffoldHub gem in your Rails 3 app by editing your Gemfile and adding this line:

<div class="CodeRay">
  <div class="code"><pre>gem <span class="s"><span class="dl">'</span><span class="k">scaffoldhub</span><span class="dl">'</span></span></pre></div>
</div>

And of course then install it with bundler:

<div class="CodeRay">
  <div class="code"><pre>$ bundle install
Fetching source index for http://rubygems.org/
Using rake (0.8.7) 
Using abstract (1.0.0) 

... etc...

Using railties (3.0.5) 
Using rails (3.0.5) 
<div class='container'>Using scaffoldhub (0.0.3)<div class='overlay'></div></div>Using sqlite3 (1.3.3) 
<span class="s">Your bundle is complete! Use `bundle show [gemname]` to see where a bundled gem is installed.</span>
</pre></div>
</div>


Now with the gem installed you will have a new generator called “scaffoldhub” available that uses the same syntax as the standard Rails scaffold generator, except for one additional option which specifies which scaffold to use. For example, to create a person model with a name field, you would use:

<div class="CodeRay">
  <div class="code"><pre>rails generate scaffoldhub person name:string --scaffold autocomplete:name</pre></div>
</div>

Here the new --scaffold option indicates we want to use the autocomplete scaffold from scaffoldhub.org, and that we want the autocomplete behavior on the name field.

When you run the scaffoldhub command above, you’ll see this output:

<div class="CodeRay">
  <div class="code"><pre>$ rails generate scaffoldhub person name:string --scaffold autocomplete:name
      invoke  active_record
      <span class="s">create</span>    db/migrate/20110313231546_create_people.rb
    <span class="s">download</span>    http://scaffoldhub.org/scaffolds/autocomplete/spec
    <span class="s">download</span>    https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/model.rb
      <span class="s">create</span>    app/models/person.rb
      invoke    test_unit
      <span class="s">create</span>      test/unit/person_test.rb
      <span class="s">create</span>      test/fixtures/people.yml
       <span class="s">route</span>  resources :people
      invoke  scaffold_controller
    <span class="s">download</span>    https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/controller.rb
      <span class="s">create</span>    app/controllers/people_controller.rb
      invoke    erb
      <span class="s">create</span>      app/views/people
    <span class="s">download</span>      https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/_form.html.erb
      <span class="s">create</span>      app/views/people/_form.html.erb
    <span class="s">download</span>      https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/edit.html.erb
      <span class="s">create</span>      app/views/people/edit.html.erb
    <span class="s">download</span>      https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/index.html.erb
      <span class="s">create</span>      app/views/people/index.html.erb

...etc...</pre></div>
</div>

This looks similar to what the Rails scaffold generator displays, except you’ll notice a number of “download” lines. These indicate that the scaffoldhub gem is downloading the code needed to generate the autocomplete scaffolding. The first download line:

<div class="CodeRay">
  <div class="code"><pre><span class="s">download</span>    http://scaffoldhub.org/scaffolds/autocomplete/spec</pre></div>
</div>

...actually gets the list of generator template files for autocomplete from scaffoldhub.org, while the remaining download lines:

<div class="CodeRay">
  <div class="code"><pre><span class="s">download</span>    https://github.com/patshaughnessy/scaffolds/raw/master/autocomplete/templates/model.rb
<span class="s">create</span>      app/models/person.rb</pre></div>
</div>

...get the actual generator template files.

## More to come...

The next thing on my list to do is implement the login and contribute user interface for scaffoldhub.org. This should be straightforward, because all it will do is save the URL of the “scaffold spec” file I mentioned above, along with a screen shot and account information about who is posting the scaffold. Stay tuned here for more information and get ready to write your own scaffold!
