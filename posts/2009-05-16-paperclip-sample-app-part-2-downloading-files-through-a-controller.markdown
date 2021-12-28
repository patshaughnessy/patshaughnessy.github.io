title: "Paperclip sample app part 2: downloading files through a controller"
date: 2009/05/16
tag: Paperclip

<p><a href="https://patshaughnessy.net/2009/4/30/paperclip-sample-app">Last time</a> I wrote about how to quickly setup a Rails application using scaffolding that allows users to upload image files and then display them using the <a href="http://www.thoughtbot.com/projects/paperclip">Paperclip plugin</a>. Paperclip does the simplest thing possible by default: it saves the file attachments right on the file system of your web server, allowing you to download them to users easily using Apache or whatever web server you have installed.</p>
<p>Today I&rsquo;d like to take that sample app one step further and show how to use a Rails controller to download the files, instead of directly through Apache. To get the finished code just go to <a href="http://github.com/patshaughnessy/paperclip-sample-app">http://github.com/patshaughnessy/paperclip-sample-app</a> and look at the &ldquo;part2&rdquo; folder.</p>
<p>There are a variety of reasons why you might want to do this, including:
  <ul>
    <li>Security: you don&rsquo;t want to expose all of the file attachments to all users of your web site. Instead, you want to implement some business rules and show some files to some users, but not to others.</li>
    <li>Auditing/logging: you want to keep track of who is viewing which files, or how many times they are viewing them.</li>
    <li>You want to hide the actual location of the files from users, and instead map the files to URLs in some other pattern or manner.</li>
    <li>Or, you might want not want to store the files on your web server&rsquo;s file system at all, but instead in a database table or somewhere else. In Part 3 of this series, I&rsquo;ll show how to do this&hellip;</li>
  </ul>
</p>
<p>The common thread here is that you want to execute some Ruby code every time a users accesses a file, and the way to do that is by routing the download requests through a controller.</p>
<p>Let&rsquo;s pick up where we left off <a href="https://patshaughnessy.net/2009/4/30/paperclip-sample-app">last time</a>:</p>
<p><img src="https://patshaughnessy.net/assets/2009/4/30/mickey-index.png"></p>
<p>If we take a look at some of the HTML source for this page:</p>
<pre>&lt;h1&gt;Listing users&lt;/h1&gt;
&lt;table&gt;
  &lt;tr&gt;
    &lt;th&gt;Photo&lt;/th&gt;
    &lt;th&gt;Name&lt;/th&gt;
    &lt;th&gt;Email&lt;/th&gt;
  &lt;/tr&gt;
&lt;tr&gt;
    &lt;td&gt;&lt;img alt=&quot;Mickey-mouse&quot;
      src=&quot;<b>/system/avatars/1/thumb/mickey-mouse.jpg?1242395876</b>&quot; /&gt;&lt;/td&gt;
    &lt;td&gt;Mickey Mouse&lt;/td&gt;
    &lt;td&gt;mickey@disney.com&lt;/td&gt;
    &lt;td&gt;&lt;a href=&quot;/users/1&quot;&gt;Show&lt;/a&gt;&lt;/td&gt;
    &lt;td&gt;&lt;a href=&quot;/users/1/edit&quot;&gt;Edit&lt;/a&gt;&lt;/td&gt;</pre>
<p>&hellip; we can see that Paperclip&rsquo;s &ldquo;url&rdquo; function which we called in index.html.erb is returning a pointer to the actual location of the file on the web server&rsquo;s hard drive, under the public/system folder:</p>
<pre>$ find public/system
public/system
public/system/avatars
public/system/avatars/1
public/system/avatars/1/original
public/system/avatars/1/original/mickey-mouse.jpg
public/system/avatars/1/small
public/system/avatars/1/small/mickey-mouse.jpg
public/system/avatars/1/thumb
public/system/avatars/1/thumb/mickey-mouse.jpg</pre>
<p>Now let&rsquo;s say that we want to implement some simple security around these images&hellip;. reason #1 from my list above. The first thing we&rsquo;ll need to do, then, is to remove the image files from the public folder and instead save them in some non-public place on our web server, for example:</p>
<pre>$ mkdir non-public
$ mv public/system non-public/.</pre>
<p>Now let&rsquo;s double check that Apache can&rsquo;t find the files in their new location:<br/>
  <img src="https://patshaughnessy.net/assets/2009/5/15/mickey-index-missing.png"></p>
<p>Great! We see a missing image as expected. No users can see the files unless we run some bit of Ruby code to enable access. Now&hellip; how can we use a controller to download the files through Rails, instead of through Apache? The first thing we need to do is add a route to routes.rb for accessing the files.</p>
<p>If you open up routes.rb and look at what the scaffolding generator created for us, you&rsquo;ll see this:</p>
<pre>ActionController::Routing::Routes.draw do |map|
  map.resources :users
  ...etc...
  map.connect ':controller/:action/:id'
  map.connect ':controller/:action/:id.:format'
end</pre>
<p>The map.resources line indicates that our application supports a series of routes that handle the four REST actions: GET, POST, PUT and DELETE. The best way to get a handle on how the routes work is by running &ldquo;rake routes&rdquo; to list out all the URL patterns that Rails will match on:</p>
<pre>$ rake routes
(in /Users/pat/rails-apps/paperclip-sample-app)
    users GET    /users(.:format)          {:action=&gt;&quot;index&quot;, :controller=&gt;&quot;users&quot;}
          POST   /users(.:format)          {:action=&gt;&quot;create&quot;, :controller=&gt;&quot;users&quot;}
 new_user GET    /users/new(.:format)      {:action=&gt;&quot;new&quot;, :controller=&gt;&quot;users&quot;}
edit_user GET    /users/:id/edit(.:format) {:action=&gt;&quot;edit&quot;, :controller=&gt;&quot;users&quot;}
     user GET    /users/:id(.:format)      {:action=&gt;&quot;show&quot;, :controller=&gt;&quot;users&quot;}
          PUT    /users/:id(.:format)      {:action=&gt;&quot;update&quot;, :controller=&gt;&quot;users&quot;}
          DELETE /users/:id(.:format)      {:action=&gt;&quot;destroy&quot;, :controller=&gt;&quot;users&quot;}
                 /:controller/:action/:id           
                 /:controller/:action/:id(.:format)</pre>
<p>The last two lines are the &ldquo;default routes,&rdquo; which connect any URL matching the pattern controller/action/id to the corresponding controller. We could just go ahead and use the default routes, but to learn a bit more about how map.resources works, let&rsquo;s create a new URL pattern for our users that we&rsquo;ll use in a minute to download the avatar file attachments with. Edit routes.rb and add the :member parameter in bold:</p>
<pre>ActionController::Routing::Routes.draw do |map|
  map.resources :users<b>, :member => { :avatars => :get }</b>
etc...</pre>
<p>If we now re-run rake routes, we can see that a new URL pattern was created for us that we can use to download the avatar images via a GET request:</p>
<pre>$ rake routes
(in /Users/pat/rails-apps/paperclip-sample-app)
      users GET    /users(.:format)          {:action=&gt;&quot;index&quot;, :controller=&gt;&quot;users&quot;}
            POST   /users(.:format)          {:action=&gt;&quot;create&quot;, :controller=&gt;&quot;users&quot;}
   new_user GET    /users/new(.:format)      {:action=&gt;&quot;new&quot;, :controller=&gt;&quot;users&quot;}
  edit_user GET    /users/:id/edit(.:format) {:action=&gt;&quot;edit&quot;, :controller=&gt;&quot;users&quot;}
<b>avatars_user GET   /users/:id/avatars(.:format) {
                                              :action=&gt;&quot;avatars&quot;,
                                              :controller=&gt;&quot;users&quot;
                                             }</b>
       user GET    /users/:id(.:format)      {:action=&gt;&quot;show&quot;, :controller=&gt;&quot;users&quot;}
            PUT    /users/:id(.:format)      {:action=&gt;&quot;update&quot;, :controller=&gt;&quot;users&quot;}
            DELETE /users/:id(.:format)      {:action=&gt;&quot;destroy&quot;, :controller=&gt;&quot;users&quot;}
                   /:controller/:action/:id           
                   /:controller/:action/:id(.:format)</pre>
<p>Now to get the avatar for user 7, for example, we can issue a URL like this:</p>
<pre>http://localhost:3000/users/7/avatars</pre>
<p>&hellip; and the request will be routed to the &ldquo;avatars&rdquo; action in the &ldquo;users&rdquo; controller (plural since a user might have more than one style of avatar). So now let&rsquo;s go right ahead and implement the avatars method and add some code to download a file to the client. The way to do that is to use <a href="http://api.rubyonrails.org/classes/ActionController/Streaming.html">ActionController::Streaming::send_file</a>. It&rsquo;s simple enough; we just need to pass the file&rsquo;s path to send_file as well as the MIME content type which the client uses as a clue for deciding how to display the file, and that&rsquo;s it! Let&rsquo;s hard code these values for now and see if it all works (update the path here for your machine):</p>
<pre>class UsersController &lt; ApplicationController
  def avatars
    send_file &#x27;/path/to/non-public/system/avatars/1/original/mickey-mouse.jpg&#x27;,
      :type =&gt; &#x27;image/jpeg&#x27;
  end</pre>
<p>Now if you type <a href="http://localhost:3000/users/1/avatars">http://localhost:3000/users/1/avatars</a> into your browser you should see the mickey image again. If not, then double check your code changes, where the files are actually located on the hard drive now and also try stopping and reloading the Rails app since changes to routes.rb are cached and are only loaded when Rails is initialized.</p>
<p>Instead of hard coding the path in the avatars method, we obviously need to be able to handle requests for any avatar file attachment for any user record. Before we enhance our code to do this, let&rsquo;s take a few minutes to configure Paperclip and tell it where the files are now stored on the file system, and which URL we have configured our routes.rb file to use. This will make our work coding in the controller a lot easier, and also indicate to Paperclip where new file attachments should be uploaded to. To do this, we need to add a couple of parameters to our call to has_attached_file in our User model (user.rb), shown here in bold (again, update the path for your machine):</p>
<pre>class User &lt; ActiveRecord::Base
  has_attached_file :avatar,
    :styles =&gt; { :thumb =&gt; &quot;75x75&gt;&quot;, :small =&gt; &quot;150x150&gt;&quot; },
    <b>:path =&gt; &#x27;/path/to/non-public/system/avatars/1/original/mickey-mouse.jpg&#x27;,
    :url =&gt; &#x27;users/1/avatars&#x27;</b>
end</pre>
<p>Just to take one step at a time, I&rsquo;ve hard coded the URL and path again here in the model. But now we can generalize our code in UserController to handle any user, like this:</p>
<pre>def avatars
  user = User.find(params[:id])
  send_file user.avatar.path, :type =&gt; user.avatar_content_type
end</pre>
<p>Now we can test <a href="http://localhost:3000/users/1/avatars">http://localhost:3000/users/1/avatars</a> again to be sure that we haven&rsquo;t broken anything. If it&rsquo;s all working, lets&rsquo; proceed to clean up the hard coding in user.rb. It turns out that Paperclip uses the same interpolations idea that we saw above in routes.rb. So I can use symbols like :rails_root, :id, :style, etc., and they will be evaluated to the values I expect and need. Here&rsquo;s the finished code in my model:</p>
<pre>
has_attached_file :avatar,
  :styles =&gt; { :thumb =&gt; &quot;75x75&gt;&quot;, :small =&gt; &quot;150x150&gt;&quot; },
  :path =&gt; 
    &#x27;:rails_root/non-public/system/:attachment/:id/:style/:basename.:extension&#x27;,
  :url =&gt; &#x27;/:class/:id/:attachment&#x27;                    
</pre>
<p>If we open up the console and take a look at our user object there, we can see that Paperclip is substituting the correct values for each of the symbols I&rsquo;ve provided:</p>
<pre>$ ./script/console
Loading development environment (Rails 2.3.2)
&gt;&gt; User.first.avatar.url
=&gt; &quot;/users/1/avatars?1242395876&quot;       (time stamp appended here automatically)
&gt;&gt; User.first.avatar.path
=&gt; &quot;/Users/pat/.../non-public/system/avatars/1/original/mickey-mouse.jpg&quot;</pre>
<p>Now let&rsquo;s go back and test our original application again with our new code:
  <ul>
    <li>The new route in routes.rb</li>
    <li>The new action in UserController</li>
    <li>And the :url and :path parameters added to has_attached_file in the User model.</li>
  </ul><br/>
  <img src="https://patshaughnessy.net/assets/2009/5/15/mickey-index-large.png"></p>
</p>
<p>Oops&hellip; we see the large image again. It doesn&rsquo;t work! What happened? Well, it turns our that we forgot one detail in our new avatars method in UserController. Our code always returns the default, or &ldquo;original&rdquo; style of the file attachment, but in the users index view we actually display the thumbnail image using image_tag user.avatar.url(:thumb). So our controller code needs to be able to handle requests for other styles as well. To do this, we need to pass the requested style somehow. The simplest thing to do is just to add the style as a URL parameter to the download request, like this:</p>
<pre>
  http://localhost:3000/users/1/avatar?style=thumb
</pre>
<p>(We could also add the style to the URL's path, but that would require another change to routes.rb.) To make this work, first I need to tell Paperclip about how I want to handle the style value in the URL:</p>
<pre>
has_attached_file :avatar,
  :styles =&gt; { :thumb =&gt; &quot;75x75&gt;&quot;, :small =&gt; &quot;150x150&gt;&quot; },
  :path =&gt; 
    &#x27;:rails_root/non-public/system/:attachment/:id/:style/:basename.:extension&#x27;,
  :url =&gt; &#x27;/:class/:id/avatar<b>?style=:style</b>&#x27;
</pre>
<p>And now I need to handle this new parameter in my controller:</p>
<pre>def avatars
  user = User.find(params[:id])
  <b>style = params[:style] ? params[:style] : &#x27;original&#x27;</b>
  send_file user.avatar.path<b>(style)</b>,
            :type =&gt; user.avatar_content_type
end</pre>
<p>I don&rsquo;t need to change anything in index.html.erb since there we call image_tag user.avatar.url(:thumb) which picks up the new URL pattern from Paperclip.</p>
<p>And now, if I&rsquo;ve got all of this correct, I should be able to finally see the thumbnail image again on the index page:<br/>
  <img src="https://patshaughnessy.net/assets/2009/4/30/mickey-index.png"></p>
<p>And finally we have files being downloaded by Ruby code present in the UserController class. If we actually wanted to implement security, logging or some other sort of logic we would just add code to the avatars method in UserController. For example, avatars could return a 401 (unauthorized) error if the user wasn&rsquo;t logged in, or didn&rsquo;t have access to view Mickey&rsquo;s image for some reason.</p>
<p>That&rsquo;s it for now; next time I&rsquo;ll modify this sample app once more to demonstrate how we can store the image files in a database table, instead of in the &ldquo;non-public&rdquo; folder or anywhere on the file system.</p>
