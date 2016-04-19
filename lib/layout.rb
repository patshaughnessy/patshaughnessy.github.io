require 'erb'
require 'pathname'

class Layout

  attr_reader :posts

  def initialize(posts)
    @posts = posts
  end

  def render(page, path)
    puts "Rendering: #{path}"
    output = Pathname.new(path)
    output.dirname.mkpath
    output.write(contents(page))
  end

  def contents(page)
    erb('layout', page) do
      erb(page.class.to_s.downcase, page)
    end
  end

  def erb(template, page)
    recent_posts = find_recent_posts(page)
    page.instance_eval { ERB.new(File.read("erb/#{template}.erb")).result(binding) }
  end

  def find_recent_posts(page)
    @recent ||= posts.reject{|post| post == page}.select{|post| post.tag == page.tag}.take(4) if page.tag
  end

end
