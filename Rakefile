require './lib/post'
require './lib/layout'
require 'rake/testtask'

Rake::TestTask.new do |t|
  t.libs = ['lib','test']
  t.test_files = Dir.glob("test/**/*_test.rb").sort
  t.verbose = true
end

markdown_files = FileList['posts/*.markdown']
posts = markdown_files.map {|markdown| Post.new(markdown) }
html_files = posts.map {|post| ".#{post.url}.html" }

html_files.zip(posts, markdown_files).each do |html, post, markdown|
  file html => markdown do
    Layout.new(post).render_to(html)
  end
end

task :posts => html_files
task :home_page
task :feed

task :default => [:posts, :home_page, :feed]
