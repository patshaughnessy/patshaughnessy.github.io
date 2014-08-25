title: "Database storage for Paperclip: rewritten to use a single table"
date: 2009/04/14
tag: Paperclip
url: /paperclip-database-storage

<p>Back in February I wrote an <a href="http://patshaughnessy.net/2009/2/19/database-storage-for-paperclip">implementation of a new storage module for Paperclip</a> that supports saving file attachments in a database table. My original implementation saved the file attachments in a separate database table, which was internally managed using a &ldquo;has_many&rdquo; relationship from the target model.</p>
<p>This month I decided to rewrite the code to use a simpler, single table approach. Instead of saving the files in a separate table, each file is saved in a BLOB column in the same table as the target model. This makes the database storage module easier and more intuitive to use, and moves it closer to the original intent of the Paperclip design.</p>
<p>Code: <a href="http://github.com/patshaughnessy/paperclip">http://github.com/patshaughnessy/paperclip</a></p>
<p>New usage:</p>
<ol>
  <li>Install and use my version of the plugin:
    <pre>script/plugin install git://github.com/patshaughnessy/paperclip.git</pre>
  </li>
  <li>In your model specify the &quot;database&quot; storage option; for example:
    <pre>has_attached_file :avatar, :storage =&gt; :database</pre>
    The file will be stored in a column called [attachment name]_file (e.g. &quot;avatar_file&quot;) by default. To specify a different BLOB column name, use :column, like this:
    <pre>has_attached_file :avatar,
                  :storage =&gt; :database,
                  :column =&gt; &#x27;avatar_data&#x27;</pre>
    </li>
  <li>If you have defined different styles, these files will be stored in additional columns called [attachment name]_[style name]_file (e.g. &quot;avatar_thumb_file&quot;) by default. To specify different column names for each style, use :column in the style definition, like this:</li>
  <pre>has_attached_file :avatar,
                  :storage => :database,
                  :styles => { 
                    :medium => {
                      :geometry => "300x300>",
                      :column => 'medium_file'
                    },
                    :thumb => {
                      :geometry => "100x100>",
                      :column => 'thumb_file'
                    }
                  }</pre>
  <li>You need to create these new columns in your migrations or you&#x27;ll get an exception. Example:
    <pre>add_column :users, :avatar_file, :binary
add_column :users, :avatar_medium_file, :binary
add_column :users, :avatar_thumb_file, :binary</pre>
    Note the migration for the &quot;binary&quot; column type will not work for LONGBLOBs in MySQL. Here&#x27;s an example migration for MySQL:
    <pre>execute 'ALTER TABLE users ADD COLUMN avatar_file LONGBLOB'
execute 'ALTER TABLE users ADD COLUMN avatar_medium_file LONGBLOB'
execute 'ALTER TABLE users ADD COLUMN avatar_thumb_file LONGBLOB'</pre>
    </li>
  <li>
    To avoid performance problems loading all of the BLOB columns every time you access your ActiveRecord object, a class method is provided on your model called &ldquo;select_without_file_columns_for.&rdquo; This is set to a hash that will instruct ActiveRecord::Base.find to load all of the columns <u>except</u> the BLOB/file data columns, for example:
<pre>{:select=&gt;&quot;id,name,avatar_file_name,avatar_content_type,...&quot;}</pre>
    If you&rsquo;re using Rails 2.3, you can specify this as a default scope:
    <pre>default_scope select_without_file_columns_for(:avatar)</pre>
    Or if you&rsquo;re using Rails 2.1 or 2.2 you can use it to create a named scope:
    <pre>named_scope :without_file_data, select_without_file_columns_for(:avatar)</pre>
    </li>
  <li>By default, URLs will be set to this pattern:
    <pre>/:relative_root/:class/:attachment/:id?style=:style</pre>
    Example:
    <pre>/app-root-url/users/avatars/23?style=original</pre>
    The idea here is that to retrieve a file from the database storage, you will need some controller&#x27;s code to be executed. Once you pick a controller to use for downloading, you can add this line to generate the download action for the default URL/action (the plural attachment name), &quot;avatars&quot; in this example:
      <pre>downloads_files_for :user, :avatar</pre>
      Or you can write a download method manually if there are security, logging or other requirements. If you prefer a different URL for downloading files you can specify that in the model; e.g.:
      <pre>has_attached_file :avatar,
                  :storage =&gt; :database,
                  :url =&gt;&#x27;/users/show_avatar/:id/:style&#x27;</pre>
    </li>
    <li>Add a route for the download to the controller which will handle downloads, if necessary. The default URL, /:relative_root/:class/:attachment/:id?style=:style, will be matched by the default route: :controller/:action/:id</li>
</ol>
<p>For now, I’ve overwritten my code from February. I believe this implementation is cleaner and easier to use; if anyone did download and use my code from February let me know and I can help you migrate the file data from the separate table back to columns in primary table. I will be writing a script to do this for my own application.</p>
<p>When I have time in the next few days or weeks, I’ll post a sample application that illustrates all of these steps and shows exactly how to do all of this. If anyone has any questions or suggestions please let me know.</p>
