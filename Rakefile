require './lib/post'
require './lib/home_page'
require './lib/feed'
require './lib/layout'
require 'rake/testtask'

Rake::TestTask.new do |t|
  t.libs = ['lib','test']
  t.test_files = Dir.glob("test/**/*_test.rb").sort
  t.verbose = true
end

markdown_files = FileList['posts/*.markdown']
posts = markdown_files.map {|markdown| Post.new(markdown) }
sorted_posts = posts.sort {|p1, p2| p1.date <=> p2.date }.reverse
html_files = sorted_posts.map {|post| ".#{post.url}.html" }

html_files.zip(sorted_posts).each do |html, post|
  file html => post.source_file do
    Layout.new(sorted_posts).render(post, html)
  end
end
task :posts => html_files

file 'index.html' => markdown_files do
  home_page = HomePage.new(sorted_posts)
  Layout.new(sorted_posts).render(home_page, 'index.html')
end
task :home_page => 'index.html'

file 'index.xml' => markdown_files do
  Feed.new(sorted_posts).render('index.xml')
end
task :feed => 'index.xml'

task :default => [:posts, :home_page, :feed]
