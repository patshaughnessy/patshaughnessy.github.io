require 'erb'
require 'pathname'

class Layout

  attr_reader :post

  def initialize(post)
    @post = post
  end

  def render_to(path)
    puts "Rendering: #{path}"
    output = Pathname.new(path)
    output.dirname.mkpath
    output.write(contents)
  end

  def contents
    erb('layout') do
      erb('article')
    end
  end

  def erb(template)
    post.instance_eval { ERB.new(File.read("erb/#{template}.erb")).result(binding) }
  end

end
