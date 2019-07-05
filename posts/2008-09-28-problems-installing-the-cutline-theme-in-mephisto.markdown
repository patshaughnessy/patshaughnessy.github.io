title: "Problems Installing the Cutline Theme in Mephisto"
date: 2008/09/28
tag: Ruby

My next step was to switch away from the standard “simpla” theme to another more interesting one. Once again I expected this to be a very easy task but ended up spending hours debugging and troubleshooting Mephisto and my blog setup on HostingRails.com.

I started by downloading the theme onto my server:
<pre>cd themes/site-1/
mkdir cutline
curl http://mephisto-themes.nanorails.com/download?theme
	=cutline.mephisto-themes.nanorails.com > cutline/cutline.zip
cd cutline
unzip cutline.zip</pre>

Now when I logged into the Mephisto console and viewed Design->Manage Themes I saw the new theme. The first problem occurred when I clicked on the new theme and selected “Use Theme." I expected to see the new theme or a message in the admin console, but instead got a page full of text:
<pre>$("dialog-box").update("\u003Cdiv id=\"theme-info\" class=\"clearfix\"\u003E\n  \u003Ch3\u003E\n    Cutline (rhtml) (v1.1)by \u003Ca
href=\"http://cutline.tubetorial.com/\"\u003EPearsonified\u003C/a\u003E\n    \u003Ca href=\"#\" id=\"close-dialog\" onclick=\"Dialog.close();
return false;\"\u003Eclose\u003C/a\u003E\n  \u003C/h3\u003E\n  \u003Cdiv id=\"screenshot\"\u003E\n    \u003Cimg
src=\"/admin/themes/preview_for/cutline\" alt=\"Theme preview\" title=\"Cutline (rhtml) (v1.1)\" /\u003E\n    \u003Cp\u003EToday, you can put
an end to boring default styling and step into a world where your posts will become more artistic, more engaging, and more
compelling!\u003C/p\u003E\n  \u003C/div\u003E\n  \u003Cdiv id=\"theme-options\"\u003E\n    \u003Cul\u003E\n          \u003Cli\u003E\u003Ca
href=\"/admin/themes/change_to/cutline\"\u003EUse theme\u003C/a\u003E\u003C/li\u003E\n      \u003Cli\u003E\u003Ca
href=\"/admin/design?theme=cutline\"\u003EEdit theme\u003C/a\u003E\u003C/li\u003E\n          \u003Cli\u003E\u003Ca
href=\"/admin/themes/export/cutline\"\u003EDownload theme\u003C/a\u003E\u003C/li\u003E\n          \u003Cli\u003E\u003Ca href=\"#\"
onclick=\"if (confirm('Are you sure you wish to delete this theme stored at \\'themes/site-1/cutline\\'?')) { new
Ajax.Request('/admin/themes/destroy/cutline', {asynchronous:true, evalScripts:true, method:'delete'}); }; return false;\"\u003EDelete
theme\u003C/a\u003E\u003C/li\u003E\n        \u003C/ul\u003E\n  \u003C/div\u003E\n\u003C/div\u003E");
Dialog.current.layout();</pre>

It looks like the parent window from the admin console was redirected to “/admin/themes/show/cutline” and the meta data/contents of the theme dialog box intended for some javascript was displayed as plain text to me. After staring at this for a while I started looking for help on the Internet and eventually found an article explaining that I needed to make a code change to app/controllers/admin/themes_controller.rb near line 5:

<pre>before_filter :protect_action, :only => [:export, :change_to, :rollback,
 :import]</pre>

...should be changed to:

<pre>before_filter :protect_action, :only => [:export, :rollback]</pre>

<p>One to do item here is to check whether the latest, edge version of Mephisto contains this fix already or not. I’m sure it does. Even after making the change on hostingrails.com I struggled for some more time before realizing that I had to kill any dispatch.fcgi processes running on the server before a code change would take effect. This wasn’t a problem in my local, development setup. Passenger must load and cache Ruby code after the first request. This would be interesting to explore further.</p>

<p>I tried again and this time: success! Finally I was able to switch to the new theme. Excited, I quickly took a look at my new site and saw... nothing! More troubleshooting revealed this error in my production.log file:</p>

<pre>SystemExit (Define INLINEDIR or HOME in your environment and try again):
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:93:in 'abort'
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:93:in 'rootdir'
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:107:in 'directory'
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:277:in 'so_name'
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:317:in 'load_cache'
...ruby/gems/1.8/gems/RubyInline-3.7.0/lib/inline.rb:653:in 'inline'
...ruby/gems/1.8/gems/image_science-1.1.3/lib/image_science.rb:84
...ruby/site_ruby/1.8/rubygems/custom_require.rb:32:in 'gem_original_require'
...ruby/site_ruby/1.8/rubygems/custom_require.rb:32:in 'require'
...rails/activesupport/lib/active_support/dependencies.rb:496:in 'require'
...rails/activesupport/lib/active_support/dependencies.rb:342:in 'new_constants_in'
...rails/activesupport/lib/active_support/dependencies.rb:496:in 'require'
...plugins/attachment_fu/lib/technoweenie/attachment_fu/processors/image_science_processor.rb:1
...ruby/site_ruby/1.8/rubygems/custom_require.rb:27:in 'gem_original_require'
etc...</pre>

Some further searching on the Internet revealed that the problem here was that an image library called <a href="http://seattlerb.rubyforge.org/ImageScience.html">ImageScience</a> requires something called <a href="http://rubyforge.org/projects/rubyinline">RubyInline</a>. Looking at the stack trace, it seems that the “attachment_fu” plugin uses the ImageScience library. Probably this is used by Mephisto to produce the thumbnail preview images in the Admin->Assets page. The solution in this case is to create a folder on the server for RubyInline, and to set this value near the top of Mephisto’s environment.rb file. Along with my change from last week to force the production environment, the top of config/environment.rb looks like this:

<pre># Uncomment below to force Rails into production mode when
# you don't control web/app server and can't set it the proper way
ENV['RAILS_ENV'] ||= 'production'
ENV['INLINEDIR'] ||= 'inline'</pre>

I tried again to see my site, and this time still saw a blank page. View->Source revealed that the Ruby code in the view was not being executed. Finally I installed the “mephisto_erb_templates” plugin as instructed by <a href="http://mephisto-themes.nanorails.com/2007/3/21/cutline-theme">http://mephisto-themes.nanorails.com/2007/3/21/cutline-theme</a>.

<pre>script/plugin install http://svn.nanorails.com/plugins/mephisto_erb_templates/</pre>

Now the theme worked properly. I made a few cosmetic CSS tweaks, picked my own image and ended up with this page.
