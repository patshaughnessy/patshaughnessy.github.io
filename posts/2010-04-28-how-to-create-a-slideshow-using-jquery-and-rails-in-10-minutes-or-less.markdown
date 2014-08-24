title: How to create a slideshow using JQuery and Rails in 10 minutes or less
date: 2010/04/28

<p>Today I want to demonstrate a JQuery slideshow tool that my friend <a href="http://www.flyingmachinestudios.com/">Daniel Higginbotham</a> wrote: <a href="http://github.com/flyingmachine/electric-slide">Electric Slide</a>. It&rsquo;s tremendously simple to use, while still providing ways to customize and adapt its behavior as you need to.</p>
<p>Suppose you have a long series of slides containing text and/or images that you want to display as a slideshow on a web page; let&rsquo;s use these Mickey images as an example:</p>
<p><img src="http://patshaughnessy.net/assets/2010/4/27/mickey_row.png"/></p>
<p>You might represent these slides using a Rails ActiveRecord model called &ldquo;Slide.&rdquo; If you had one image per slide you could attach it to the slide model with <a href="http://github.com/thoughtbot/paperclip">Paperclip</a>:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="r">class</span> <span class="cl">Slide</span> &lt; <span class="co">ActiveRecord</span>::<span class="co">Base</span>
  has_attached_file <span class="sy">:image</span>
<span class="r">end</span></pre></div>
</div><br>
<table>
  <tr>
    <td>
      <p>The simplest way to display all of these would be to draw a single vertical column on a web page and let the user scroll down to view all of the slides:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="il"><span class="idl">&lt;%</span> <span class="iv">@slides</span>.each <span class="r">do</span> |slide| <span class="idl">%&gt;</span></span>
  <span class="il"><span class="idl">&lt;%=</span> image_tag slide.image.url <span class="idl">%&gt;</span></span>
  <span class="ta">&lt;br</span><span class="ta">/&gt;</span>
<span class="il"><span class="idl">&lt;%</span> <span class="r">end</span> <span class="idl">%&gt;</span></span></pre></div>
</div><br>
<p>However, scrolling can be annoying especially if there are many images or a lot of text. Also, this isn&rsquo;t a slideshow. I don&rsquo;t see each image in the same location as the previous one. It&rsquo;s harder to notice changes between the slides and also harder for me to surprise the user with something funny or unexpected in the following slide since they are all immediately visible.</p></td>
    <td><img src="http://patshaughnessy.net/assets/2010/4/27/mickey_column.png"/></td>
  </tr>
</table>
<p>Another simple solution would be to display each slide on a separate page and provide next/previous links, using code similar to this:</p>
<table>
  <tr>
    <td><img src="http://patshaughnessy.net/assets/2010/4/27/mickey_prev_next.png"></td>
    <td>
<div class="CodeRay">
  <div class="code"><pre><span class="il"><span class="idl">&lt;%=</span> image_tag <span class="iv">@slide</span>.image.url <span class="idl">%&gt;</span></span>
<span class="ta">&lt;br&gt;</span>
<span class="il"><span class="idl">&lt;%=</span> link_to <span class="s"><span class="dl">'</span><span class="k">Prev</span><span class="dl">'</span></span>, <span class="iv">@slide</span>.higher_item <span class="r">unless</span> <span class="iv">@slide</span>.first? <span class="idl">%&gt;</span></span> <span class="en">&amp;nbsp;</span>
<span class="il"><span class="idl">&lt;%=</span> link_to <span class="s"><span class="dl">'</span><span class="k">Next</span><span class="dl">'</span></span>, <span class="iv">@slide</span>.lower_item <span class="r">unless</span> <span class="iv">@slide</span>.last? <span class="idl">%&gt;</span></span>
</pre></div>
</div>      
    </td>
  </tr>
</table>
<p>But this is also annoying since each click takes the user to a separate page, changing the URL and also adding to the browser&rsquo;s history list. Since all of these images are part of a single presentation I&rsquo;d like to display them on one page. Additionally I have to write all of the navigation code: I might use acts_as_list in my model class like in the ERB snippet above, and then write the HTML to display the next/prev links in the proper location, to hide them when necessary, to give them the proper styling, etc. In other words, it&rsquo;s a fair amount of work.</p>
<p><b>Electric Slide</b></p>
<p><a href="http://github.com/flyingmachine/electric-slide/">Electric Slide</a> to the rescue! If you install Daniel&rsquo;s Electric Slide JQuery code file in your app, all you have to do is use a &lt;div class=&ldquo;slide&rdquo;&gt; tag around each image or whatever slide content you have, call a JQuery function when your page is loaded and Electric Slide will take care of the rest:</p>
<table>
  <tr>
    <td><div class="CodeRay">
  <div class="code"><pre><span class="ta">&lt;script</span> <span class="an">type</span>=<span class="s"><span class="dl">&quot;</span><span class="k">text/javascript</span><span class="dl">&quot;</span></span> <span class="an">charset</span>=<span class="s"><span class="dl">&quot;</span><span class="k">utf-8</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
  $(function(){
    $(&quot;#slides&quot;).electricSlide();
  })
<span class="ta">&lt;/script&gt;</span>
<span class="ta">&lt;div</span> <span class="an">id</span>=<span class="s"><span class="dl">&quot;</span><span class="k">slides</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
<span class="il"><span class="idl">&lt;%</span> <span class="iv">@slides</span>.each <span class="r">do</span> |slide| <span class="idl">%&gt;</span></span>
  <span class="ta">&lt;div</span> <span class="an">class</span>=<span class="s"><span class="dl">&quot;</span><span class="k">slide</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
    <span class="il"><span class="idl">&lt;%=</span> image_tag slide.image.url <span class="idl">%&gt;</span></span>
  <span class="ta">&lt;/div&gt;</span>
<span class="il"><span class="idl">&lt;%</span> <span class="r">end</span> <span class="idl">%&gt;</span></span>
<span class="ta">&lt;/div&gt;</span></pre></div>
</div>  
    </td>
    <td width="60">&nbsp;</td>
    <td><img src="http://patshaughnessy.net/assets/2010/4/28/mickey_es1.png"></td>
  </tr>
</table>
<p>With just a little bit of CSS love, the slideshow can look like this (click through the screen shot to see a <a href="http://patshaughnessy.net/mickey_slides_example/index.html">working example</a>):</p>
<table>
  <tr>
    <td><div class="CodeRay">
  <div class="code"><pre><span class="kw">body</span> {
  <span class="ke">background</span>:<span class="cr">#EEEEDD</span>;
}
<span class="cl">.slide</span> {
  <span class="ke">display</span>:<span class="vl">none</span>;
}
<span class="co">#slides</span> <span class="cl">.slide-header</span> {
  <span class="ke">display</span>: <span class="vl">none</span>
}
<span class="co">#slides</span> <span class="cl">.slide-footer</span> {
  <span class="ke">margin</span>:<span class="fl">0</span>;
  <span class="ke">width</span>: <span class="fl">200px</span>;
}
<span class="cl">.slide-footer</span> <span class="cl">.previous</span> {
  <span class="ke">width</span>:<span class="fl">75px</span>;
  <span class="ke">float</span>:<span class="vl">left</span>;
}
<span class="cl">.slide-footer</span> <span class="cl">.next</span> {
  <span class="ke">width</span>:<span class="fl">75px</span>;
  <span class="ke">float</span>:<span class="vl">right</span>;
  <span class="ke">text-align</span>:<span class="vl">right</span>;
}</pre></div>
</div>
    </td>
    <td width="60">&nbsp;</td>
    <td><a href="http://patshaughnessy.net/mickey_slides_example/index.html"><img src="http://patshaughnessy.net/assets/2010/4/28/mickey_es3.png"></a></td>
  </tr>
</table><br></p>
<p>Now all the slides are displayed in a working slideshow inside my single web page! When I click on the previous or next links, I can watch Mickey move around without leaving the page. Let&rsquo;s take a look at how my ERB and CSS code works and what Electric Slide is doing:
<ul><li>First I&rsquo;ve identified each of my slide&rsquo;s content using &lt;div class=&ldquo;slide&rdquo;&gt;. I just included that in my iteration over the slide array. For me, each of these &lt;div&gt; tags contains a Mickey image, but they could contain any HTML content you would like.</li>
<li>Note I set &ldquo;display: none&rdquo; for the &ldquo;.slide&rdquo; style so the slides are initially hidden. This is required by Electric Slide, or else all of the slides will appear initially visible.</li>
<li>Next I added an on-load handler using JQuery syntax: $(function(){  &hellip;. }). JQuery will call this code when the page finishes loading.</li>
<li>In the on-load handler, I call the electricSlide function on my &lt;div id=&ldquo;slides&rdquo;&gt; parent object. This initializes Electric Slide. There are many options I could have passed in here, but today I&rsquo;m just using all of the default values. Check out <a href="http://github.com/flyingmachine/electric-slide">Daniel&rsquo;s github repo</a> for documentation and examples for more info on what to pass in here.</li>
<li>Electric Slide counts up the slides, and displays &ldquo;previous&rdquo; and &ldquo;next&rdquo; links for each one as necessary. I don&rsquo;t need to worry about writing code to display the links or handle clicks on them: the navigation just works.</li>
<li>Electric Slide also creates &lt;div&gt; tags automatically to hold the previous/next links, these are called &lt;div class=&ldquo;slide-header&rdquo;&gt; and &lt;div class=&ldquo;slide-footer&rdquo;&gt;. I&rsquo;ve styled these using CSS to hide the header and to align the links with the edge of the image in the footer.</li></ul></p>
<p><b>Detailed step-by-step example</b></p>
<p>Let&rsquo;s create a new Rails app from scratch, copy the mickey images into it, install Electric Slide and then display them all as a slideshow&hellip; all in 10 minutes or less!</p>
<div class="CodeRay">
  <div class="code"><pre>$ rails mickey-slides
$ cd mickey-slides</pre></div>
</div><br>
<p>Go ahead and download the Mickey images from my site; or feel free to use any images you have instead:</p>
<div class="CodeRay">
  <div class="code"><pre>$ curl -O http://patshaughnessy.net/assets/2010/4/28/mickey-images.tar.gz
$ tar zxvf mickey-images.tar.gz 
images/
images/mickey1.jpg
images/mickey2.jpg
images/mickey3.jpg
images/mickey4.jpg
images/mickey5.jpg
images/mickey6.jpg</pre></div>
</div><br>
<p>Now let&rsquo;s use <a href="http://patshaughnessy.net/view_mapper">View Mapper</a> to create my slide model with an &ldquo;image&rdquo; attachment:</p>
<div class="CodeRay">
  <div class="code"><pre>$ sudo gem install view_mapper
$ ./script/generate scaffold_for_view slide --view paperclip:image
       error  The Paperclip plugin does not appear to be installed.</pre></div>
</div><br>
<p>Oh yea&hellip; I forgot to install Paperclip; let&rsquo;s do that now also and then repeat the View Mapper command:</p>
<div class="CodeRay">
  <div class="code"><pre>$ ./script/plugin install git://github.com/thoughtbot/paperclip.git
$ ./script/generate scaffold_for_view slide --view paperclip:image
$ rake db:migrate</pre></div>
</div><br>
<p>Next let&rsquo;s load the images into our database using Paperclip in the Rails console:</p>
<div class="CodeRay">
  <div class="code"><pre>$ ./script/console 
Loading development environment (Rails 2.3.5)
&gt;&gt; Dir.glob('images/*.jpg').each do |filename|
?&gt; Slide.create :image =&gt; File.new(filename)
&gt;&gt; end
=&gt; [&quot;images/mickey1.jpg&quot;, &quot;images/mickey2.jpg&quot;, &quot;images/mickey3.jpg&quot;,
&quot;images/mickey4.jpg&quot;, &quot;images/mickey5.jpg&quot;, &quot;images/mickey6.jpg&quot;]</pre></div>
</div><br>
<p>Now we just need to install Electric Slide &ndash; let&rsquo;s just get Daniel&rsquo;s entire github repo including the examples and documentation:</p>
<div class="CodeRay">
  <div class="code"><pre>$ cd ..
$ git clone git://github.com/flyingmachine/electric-slide.git
$ cd mickey-slides</pre></div>
</div><br>
<p>And now let&rsquo;s replace our default Rails prototype javascript files with JQuery and the Electric Slide code from Daniel&rsquo;s repository:</p>
<div class="CodeRay">
  <div class="code"><pre>$ rm -rf public/javascripts
$ cp -r ../electric-slide/javascripts public/javascripts</pre></div>
</div><br>
<p>Now edit app/views/layouts/slides.html.erb and include Electric Slide, JQuery and a new style.css file by replacing the existing stylesheet_link_tag &lsquo;scaffold&rsquo; line with the lines highlighted below:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="dt">&lt;!DOCTYPE html PUBLIC &quot;-//W3C//DTD XHTML 1.0 Transitional//EN&quot;
       &quot;http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd&quot;&gt;</span>

<span class="ta">&lt;html</span> <span class="an">xmlns</span>=<span class="s"><span class="dl">&quot;</span><span class="k">http://www.w3.org/1999/xhtml</span><span class="dl">&quot;</span></span> <span class="an">xml:lang</span>=<span class="s"><span class="dl">&quot;</span><span class="k">en</span><span class="dl">&quot;</span></span> <span class="an">lang</span>=<span class="s"><span class="dl">&quot;</span><span class="k">en</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
<span class="ta">&lt;head&gt;</span>
  <span class="ta">&lt;meta</span> <span class="an">http-equiv</span>=<span class="s"><span class="dl">&quot;</span><span class="k">content-type</span><span class="dl">&quot;</span></span> <span class="an">content</span>=<span class="s"><span class="dl">&quot;</span><span class="k">text/html;charset=UTF-8</span><span class="dl">&quot;</span></span> <span class="ta">/&gt;</span>
  <span class="ta">&lt;title&gt;</span>Slides: <span class="il"><span class="idl">&lt;%=</span> controller.action_name <span class="idl">%&gt;</span></span><span class="ta">&lt;/title&gt;</span>
  <span class="il"><span class='container'><span class="idl">&lt;%=</span> stylesheet_link_tag <span class="s"><span class="dl">'</span><span class="k">scaffold</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">styles</span><span class="dl">'</span></span> <span class="idl">%&gt;</span><span class='overlay'></span></span></span>
  <span class="il"><span class='container'><span class="idl">&lt;%=</span> javascript_include_tag <span class="s"><span class="dl">'</span><span class="k">jquery</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">jquery.sizes</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">jquery.electric-slide</span><span class="dl">'</span></span>, <span class="s"><span class="dl">'</span><span class="k">jquery-ui</span><span class="dl">'</span></span> <span class="idl">%&gt;</span><span class='overlay'></span></span></span>
<span class="ta">&lt;/head&gt;</span>

<span class="ta">&lt;body&gt;</span>

<span class="ta">&lt;p</span> <span class="an">style</span>=<span class="s"><span class="dl">&quot;</span><span class="k">color: green</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span><span class="il"><span class="idl">&lt;%=</span> flash[<span class="sy">:notice</span>] <span class="idl">%&gt;</span></span><span class="ta">&lt;/p&gt;</span>

<span class="il"><span class="idl">&lt;%=</span> <span class="r">yield</span> <span class="idl">%&gt;</span></span>

<span class="ta">&lt;/body&gt;</span>
<span class="ta">&lt;/html&gt;</span>
</pre></div>
</div><br>
<p>Now we can add the CSS code snippet from above into our app &ndash; copy this into a new file called public/stylesheets/styles.css, which we just included above in the layout:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="kw">body</span> {
  <span class="ke">background</span>:<span class="cr">#EEEEDD</span>;
}
<span class="cl">.slide</span> {
  <span class="ke">display</span>:<span class="vl">none</span>;
}
<span class="co">#slides</span> <span class="cl">.slide-header</span> {
  <span class="ke">display</span>: <span class="vl">none</span>
}
<span class="co">#slides</span> <span class="cl">.slide-footer</span> {
  <span class="ke">margin</span>:<span class="fl">0</span>;
  <span class="ke">width</span>: <span class="fl">200px</span>;
}
<span class="cl">.slide-footer</span> <span class="cl">.previous</span> {
  <span class="ke">width</span>:<span class="fl">75px</span>;
  <span class="ke">float</span>:<span class="vl">left</span>;
}
<span class="cl">.slide-footer</span> <span class="cl">.next</span> {
  <span class="ke">width</span>:<span class="fl">75px</span>;
  <span class="ke">float</span>:<span class="vl">right</span>;
  <span class="ke">text-align</span>:<span class="vl">right</span>;
}</pre></div>
</div><br>
<p>And finally replace the scaffolding slide index view, app/views/slides/index.html.erb, with the code from above:</p>
<div class="CodeRay">
  <div class="code"><pre><span class="ta">&lt;script</span> <span class="an">type</span>=<span class="s"><span class="dl">&quot;</span><span class="k">text/javascript</span><span class="dl">&quot;</span></span> <span class="an">charset</span>=<span class="s"><span class="dl">&quot;</span><span class="k">utf-8</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
  $(function(){
      $(&quot;#slides&quot;).electricSlide();
  })
<span class="ta">&lt;/script&gt;</span>
<span class="ta">&lt;div</span> <span class="an">id</span>=<span class="s"><span class="dl">&quot;</span><span class="k">slides</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
<span class="il"><span class="idl">&lt;%</span> <span class="iv">@slides</span>.each <span class="r">do</span> |slide| <span class="idl">%&gt;</span></span>
  <span class="ta">&lt;div</span> <span class="an">class</span>=<span class="s"><span class="dl">&quot;</span><span class="k">slide</span><span class="dl">&quot;</span></span><span class="ta">&gt;</span>
    <span class="il"><span class="idl">&lt;%=</span> image_tag slide.image.url <span class="idl">%&gt;</span></span>
  <span class="ta">&lt;/div&gt;</span>
<span class="il"><span class="idl">&lt;%</span> <span class="r">end</span> <span class="idl">%&gt;</span></span>
<span class="ta">&lt;/div&gt;</span>
</pre></div>
</div><br>
<p>Now start up your server at look for the Mickey slide show at <a href="http://localhost:3000/slides">http://localhost:3000/slides</a>!</p>
