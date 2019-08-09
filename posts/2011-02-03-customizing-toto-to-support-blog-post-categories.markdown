title: "Customizing Toto to support blog post categories"
date: 2011/2/3
tag: Toto

On this blog I categorize my posts using a few tags, such as [http://patshaughnessy.net/tags/paperclip](http://patshaughnessy.net/tags/paperclip) or [http://patshaughnessy.net/tags/view-mapper](http://patshaughnessy.net/tags/view-mapper) for example. Today I’m going to walk through how I customized the Toto blog engine to display these category pages.

Of course I could have gotten categories for free using a different blog engine such as [Jekyll](https://github.com/mojombo/jekyll) or [Nesta](https://github.com/gma/nesta), but to be honest when I saw Toto’s code for the first time I fell in love with it: concise, elegant and simple. I just couldn’t wait to try to understand how it works and modify it to do something new. I suppose it really is “the 10 second blog-engine for hackers!”

I’ll go about this in four steps... you can follow the same pattern if you’d like to add something custom to your Toto site:

* [Write new Riot tests](#step1)
* [Write a new ERB template](#step2)
* [Add a new route](#step3)
* [Implement your new behavior](#step4)

## <a id="step1"></a>Step 1: Writing new Riot tests

Toto comes with a fast, effective test suite written using [Riot](https://github.com/thumblemonks/riot) and [RR](https://github.com/btakita/rr). If you plan to customize Toto, be sure to take advantage of Alexis Sellier’s existing tests to be sure you don’t break anything. Take a look at [my last post](http://patshaughnessy.net/2011/1/23/4-tips-for-how-to-customize-a-toto-blog-site) for some tips on how to get the tests working for Ruby 1.8.7.

I’ll start today by writing a few new Riot tests specifying exactly what behavior I mean by “blog post categories.”  I added this code to test/toto_test.rb in my local Toto gem folder:

<pre type="ruby">
context "GET a tag page" do
  setup { @toto.get('/tags/the-wizard-of-oz') }
  asserts("returns a 200")                         { topic.status }.equals 200
  asserts("body is not empty")                     { not topic.body.empty? }
  should("includes only the entries for that tag") { topic.body }.includes_elements("li.entry", 2)
  should("has access to @tag")                     { topic.body }.includes_html("#tag" => /The Wizard of Oz/)
end
</pre>

The syntax might be a bit unfamiliar if you’re used to RSpec, but you can probably figure out that the behavior I’m looking for is:

* that GET requests to “/tags/the-wizard-of-oz” return a valid web page that is not empty, and
* that this page contains a list of two articles, just the two that are tagged with “the-wizard-of-oz,” and
* that the human readable version of the tag, “The Wizard of Oz,” appears on this new web page.

I’d like to be able to categorize my posts by adding a new “tag” attribute to the YAML header of each article. So I’ll also have to update a couple of the test articles inside the Toto test suite to make these tests work properly... First I’ll edit test/articles/1990-05-17-the-wonderful-wizard-of-oz.txt and add a tag value like this:

<pre>title: The Wonderful Wizard of Oz
date: 17/05/1990
tag: The Wizard of Oz
_Once upon a time_...</pre>

Next, I’ll add the same “tag: The Wizard of Oz” to some other test article, for example test/articles/2001-01-01-two-thousand-and-one.txt.

Now running the tests of course I get failures:

<pre>$ rake
(in /Users/pat/rails-apps/toto)

...etc... 

Toto GET a tag page
  - <span class="an">asserts returns a 200: expected 200, not 404 (on line 115 in ./test/toto_test.rb)</span>
  + <span class="s">asserts body is not empty</span>
  - <span class="an">should includes only the entries for that tag: expected &lt;font style='font-size:300%'&rt;toto, we're not...</span>
  - <span class="an">should has access to @tag: expected &lt;font style='font-size:300%'&rt;toto, we're not in Kansas anymore (404)</span>

...etc...

62 passes, 3 failures, 0 errors in 0.148593 seconds</pre>

Here Toto is returning a 404 page not found error since I haven’t implemented anything related to the new category page yet.

## <a id="step2"></a>Step 2: Write a new ERB template

Another nice thing about Toto is that the article page template and other templates are implemented with ERB by default, which is very familiar and easy to use. I’ll continue now by writing a new ERB file called “tag.rhtml” in tests/templates to render my new category page:

<pre><span class="ta">&lt;h1</span> <span class="an">id</span>=<span class="s"><span class="dl">&quot;</span><span class="k">tag</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>Articles about <span class="il"><span class="idl">&lt;%=</span> tag <span class="idl">%&gt;</span></span><span class="ta">&lt;/h1&gt;</span> 
<span class="il"><span class="idl">&lt;%</span> <span class="r">for</span> entry <span class="r">in</span> archives <span class="idl">%&gt;</span></span> 
  <span class="ta">&lt;li</span> <span class="an">class</span>=<span class="s"><span class="dl">&quot;</span><span class="k">entry</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span><span class="il"><span class="idl">&lt;%=</span> entry.title <span class="idl">%&gt;</span></span><span class="ta">&lt;/li&gt;</span> 
<span class="il"><span class="idl">&lt;%</span> <span class="r">end</span> <span class="idl">%&gt;</span></span></pre>

Note I’ve used two HTML details here that correspond to the way I wrote the Riot tests above:

* My &lt;h1&gt; tag uses id=“tag” and contains the tag string <%= tag %>, and:
* Each of the matching articles is listed in an &lt;li&gt; tag with class=“entry.”

Of course, the tag.rhtml code I use on my actual blog site which you’re reading now is a bit more verbose:

<pre>
&lt;table id="archive-table"> 
  &lt;% if archives.length > 0 %> 
    &lt;% last_month = nil %> 
    &lt;% for entry in archives %> 
      &lt;tr> 
        &lt;td align="right"> 
          &lt;% month = entry.date.split.first %> 
          &lt;% if last_month.nil? || month != last_month %> 
            &lt;%= month %> 
            &lt;%= entry.date.split.last %> 
          &lt;% end %> 
          &lt;% last_month = month %> 
        &lt;/td> 
        &lt;td> 
          &lt;a href="&lt;%= entry.path %>">&lt;%= entry.title %>&lt;/a> 
        &lt;/td> 
      &lt;/tr> 
    &lt;% end %> 
  &lt;% end %> 
&lt;/table> 
</pre> 

There’s just a bit of logic around displaying the month and year text, and also a couple of CSS cues for my blog’s stylesheet. But to get the tests to pass you just need the barebones tag.rhtml file I showed above.

Let’s try the tests again:

<pre>$ rake
(in /Users/pat/rails-apps/toto)

...etc... 

Toto GET a tag page
  - <span class="an">asserts returns a 200: expected 200, not 404 (on line 115 in ./test/toto_test.rb)</span>
  + <span class="s">asserts body is not empty</span>
  - <span class="an">should includes only the entries for that tag: expected &lt;font style='font-size:300%'&rt;toto, we're not...</span>
  - <span class="an">should has access to @tag: expected &lt;font style='font-size:300%'&rt;toto, we're not in Kansas anymore (404)</span>

...etc...

62 passes, 3 failures, 0 errors in 0.148593 seconds</pre>

It’s still failing because I haven’t changed Toto’s routing method to find the new ERB file yet... let’s do that next.

## <a id="step3"></a>Step 3: Add a new route

Toto implements routing using the Toto::Site.go method. Here’s what that looks like:

<pre type="ruby">
def go route, env = {}, type = :html
  route << self./ if route.empty?
  type, path = type =~ /html|xml|json/ ? type.to_sym : :html, route.join('/')
  context = lambda do |data, page|
    Context.new(data, @config, path, env).render(page, type)
  end

  body, status = if Context.new.respond_to?(:"to_#{type}")
    if route.first =~ /\d{4}/
      case route.size
        when 1..3
          context[archives(route * '-'), :archives]
        when 4
          context[article(route), :article]
        else http 400
      end
    elsif respond_to?(path)
      context[send(path, type), path.to_sym]
    elsif (repo = @config[:github][:repos].grep(/#{path}/).first) &&
          !@config[:github][:user].empty?
      context[Repo.new(repo, @config), :repo]
    else
      context[{}, path.to_sym]
    end
  else
    http 400
  end

rescue Errno::ENOENT => e
  return :body => http(404).first, :type => :html, :status => 404
else
  return :body => body || "", :type => type, :status => status || 200
end
</pre>

Let’s take a few minutes to understand this method... this bit of code is really interesting. It plays the same role that routes.rb does in a Rails application. When Toto gets an HTTP request, it converts the path string into an array of strings and passes that into the “go” method as the first parameter: “route.” For example, if a user requests http://patshaughnessy.net/2011/1/23/4-tips-for-how-to-customize-a-toto-blog-site, then route will be set to:

<div class="CodeRay"> 
  <div class="code"><pre>[<span class="s"><span class="dl">&quot;</span><span class="k">2011</span><span class="dl">&quot;</span></span>, <span class="s"><span class="dl">&quot;</span><span class="k">1</span><span class="dl">&quot;</span></span>, <span class="s"><span class="dl">&quot;</span><span class="k">23</span><span class="dl">&quot;</span></span>, <span class="s"><span class="dl">&quot;</span><span class="k">4-tips-for-how-to-customize-a-toto-blog-site</span><span class="dl">&quot;</span></span>]</pre></div> 
</div> 

The code above is basically a switch statement that decides which RHTML template file to use based on this route array. So for viewing a single blog post, route will contain 4 strings and this highlighted code will be called:

<div class="CodeRay"> 
  <div class="code"><pre>...etc...
 
<span class="r">case</span> route.size
  <span class="r">when</span> <span class="i">1</span>..<span class="i">3</span> 
    context[archives(route * <span class="s"><span class="dl">'</span><span class="k">-</span><span class="dl">'</span></span>), <span class="sy">:archives</span>]
<div class='container'>  <span class="r">when</span> <span class="i">4</span> 
    context[article(route), <span class="sy">:article</span>]<span class='overlay'></span></div>  <span class="r">else</span> http <span class="i">400</span> 
<span class="r">end</span> 
 
...etc...</pre></div> 
</div> 

...returning this value:

<div class="CodeRay"> 
  <div class="code"><pre>context[article(route), <span class="sy">:article</span>]</pre></div> 
</div>

Toto next evaluates this by calling the “context” lambda defined near the top of the “go” method:

<div class="CodeRay"> 
  <div class="code"><pre>context = lambda <span class="r">do</span> |data, page|
  <span class="co">Context</span>.new(data, <span class="iv">@config</span>, path, env).render(page, type)
<span class="r">end</span></pre></div> 
</div>

This eventually runs ERB on the selected RHTML file and returns the result as the response body sent to the user. The “:article” symbol indicates Toto should use the tempates/pages/article.rhtml template. The “Context” class represents the context inside of which the ERB template is evaluated, containing both the data (the article object in this case) and the configuration values specified in config.ru.

So to get Toto to use my new tag.rhtml template, I need to return this from the “go” routing method:

<div class="CodeRay"> 
  <div class="code"><pre>context[data, <span class="sy">:tag</span>]</pre></div> 
</div>

... where “data” contains a hash of the values I used in the tag.rhtml file. For example:

<div class="CodeRay"> 
  <div class="code"><pre>context[{ <span class="sy">:tag</span> =&gt; <span class="s">'The Wizard of Oz'</span>, <span class="sy">:archives</span> =&gt; [ ...array of tagged articles... ] }, <span class="sy">:tag</span>]</pre></div> 
</div> 

Here’s how to do it - I’ll repeat the entire “go” method again:

<pre type="ruby">
def go route, env = {}, type = :html
  route << self./ if route.empty?
  type, path = type =~ /html|xml|json/ ? type.to_sym : :html, route.join('/')
  context = lambda do |data, page|
    Context.new(data, @config, path, env).render(page, type)
  end

  body, status = if Context.new.respond_to?(:"to_#{type}")
    if route.first =~ /\d{4}/
      case route.size
        when 1..3
          context[archives(route * '-'), :archives]
        when 4
          context[article(route), :article]
        else http 400
      end

    elsif route.first == 'tags' && route.size == 2
      if (data = archives('', route[1])).nil?
        http 404
      else
        context[data, :tag]
      end

    elsif respond_to?(path)
      context[send(path, type), path.to_sym]
    elsif (repo = @config[:github][:repos].grep(/#{path}/).first) &&
          !@config[:github][:user].empty?
      context[Repo.new(repo, @config), :repo]
    else
      context[{}, path.to_sym]
    end
  else
    http 400
  end

rescue Errno::ENOENT => e
  return :body => http(404).first, :type => :html, :status => 404
else
  return :body => body || "", :type => type, :status => status || 200
end
</pre>

You can see I match on routes.first == ‘tags’ and that there’s exactly one more level to the path; for example /tags/the-wizard-of-oz. Then I call the existing archives method, and pass in the tag from the URL as a parameter:

<div class="CodeRay"> 
  <div class="code"><pre>data = archives(<span class="s"><span class="dl">'</span><span class="dl">'</span></span>, route[<span class="i">1</span>])</pre></div> 
</div>

Finally if this is not nil, I return the desired context value:

<div class="CodeRay"> 
  <div class="code"><pre>context[data, <span class="sy">:tag</span>]</pre></div> 
</div>

I’ll get to the archives method next, but in the meantime let’s run the tests again:

<pre>$ rake
(in /Users/pat/rails-apps/toto)

...etc... 

Toto GET a tag page
  - <span class="an">asserts returns a 200: expected 200, not 404 (on line 115 in ./test/toto_test.rb)</span>
  + <span class="s">asserts body is not empty</span>
  - <span class="an">should includes only the entries for that tag: expected &lt;font style='font-size:300%'&rt;toto, we're not...</span>
  - <span class="an">should has access to @tag: expected &lt;font style='font-size:300%'&rt;toto, we're not in Kansas anymore (404)</span>

...etc...

62 passes, 3 failures, 0 errors in 0.148593 seconds</pre>

Still failing - but at least I know I haven’t broken any of the other existing routes since all of the other 62 tests still pass!

## <a id="step4"></a>Step 4: Implement your new behavior

Finally, we need to actually filter the list of articles on the given tag somehow. I’ll do this by modifying the existing Toto::Site.archives method. Here’s the original version of the method:

<div class="CodeRay"> 
  <div class="code"><pre><span class="r">def</span> <span class="fu">archives</span> filter = <span class="s"><span class="dl">&quot;</span><span class="dl">&quot;</span></span> 
  entries = ! <span class="pc">self</span>.articles.empty??
    <span class="pc">self</span>.articles.select <span class="r">do</span> |a|
      filter !~ <span class="rx"><span class="dl">/</span><span class="k">^</span><span class="ch">\d</span><span class="k">{4}</span><span class="dl">/</span></span> || <span class="co">File</span>.basename(a) =~ <span class="rx"><span class="dl">/</span><span class="k">^</span><span class="il"><span class="idl">#{</span>filter<span class="idl">}</span></span><span class="dl">/</span></span> 
    <span class="r">end</span>.reverse.map <span class="r">do</span> |article|
      <span class="co">Article</span>.new article, <span class="iv">@config</span> 
    <span class="r">end</span> : []
 
  <span class="r">return</span> <span class="sy">:archives</span> =&gt; <span class="co">Archives</span>.new(entries, <span class="iv">@config</span>)
<span class="r">end</span></pre></div> 
</div> 

This code is called to generate the index page for a Toto blog (e.g. [http://patshaughnessy.net](http://patshaughnessy.net)), and also the year/month archives pages by providing a value for the filter parameter. It returns a single Archives object which contains an array of Article objects.

And here’s my version:

<div class="CodeRay"> 
  <div class="code"><pre><span class="r">def</span> <span class="fu">archives</span> filter = <span class="s"><span class="dl">&quot;</span><span class="dl">&quot;</span></span><span class='container'>, tag = <span class="pc">nil</span><span class='overlay'></span></span>
  entries = ! <span class="pc">self</span>.articles.empty??
    <span class="pc">self</span>.articles.select <span class="r">do</span> |a|
      filter !~ <span class="rx"><span class="dl">/</span><span class="k">^</span><span class="ch">\d</span><span class="k">{4}</span><span class="dl">/</span></span> || <span class="co">File</span>.basename(a) =~ <span class="rx"><span class="dl">/</span><span class="k">^</span><span class="il"><span class="idl">#{</span>filter<span class="idl">}</span></span><span class="dl">/</span></span> 
    <span class="r">end</span>.reverse.map <span class="r">do</span> |article|
      <span class="co">Article</span>.new article, <span class="iv">@config</span> 
    <span class="r">end</span> : []
 
<div class='container'>  <span class="r">if</span> tag.nil?
    { <span class="sy">:archives</span> =&gt; <span class="co">Archives</span>.new(entries, <span class="iv">@config</span>) }
  <span class="r">else</span> 
    tagged = entries.select <span class="r">do</span> |article|
      article_tag = article[<span class="sy">:tag</span>]
      article_tag &amp;&amp; article_tag.slugize == tag
    <span class="r">end</span> 
    { <span class="sy">:tag</span> =&gt; tagged.first[<span class="sy">:tag</span>], <span class="sy">:archives</span> =&gt; tagged } <span class="r">if</span> tagged.size &gt; <span class="i">0</span> 
  <span class="r">end</span><span class='overlay'></span></div>
<span class="r">end</span></pre></div> 
</div> 

My changes are highlighted; I started by adding a new parameter called “tag,” which will contain the tag string we are looking for. If this is not nil, then before returning the entire list of articles I select only the articles that have article[:tag].slugize equal to the tag from the URL. "slugize" means to convert from the human readable version to the short URL version. Finally I return a hash containing two values: the actual (non-slug) tag string, and also the list of matching articles. If no articles have the requested tag, then I return nil.

Re-running the tests again:

<pre>$ rake
(in /Users/pat/rails-apps/toto)

...etc... 

Toto GET a tag page
  + <span class="s">asserts returns a 200 is equal to 200</span>
  + <span class="s">asserts body is not empty</span>
  + <span class="s">should includes only the entries for that tag</span>
  + <span class="s">should has access to @tag</span>
 
...etc... 

65 passes, 0 failures, 0 errors in 0.158935 seconds</pre>

... they pass! Now I’m ready to edit my real blog application, add the new tag.rhtml template to it, and try out my category page.

In the next few days I’ll post these changes on github, possibly in a pull request so others can use the feature also. But for me the real fun is in understanding how Toto works, and how easy it is to add your own custom pages to it.

