require 'erb'
require 'pathname'

class Layout

  attr_reader :page

  def initialize(page)
    @page = page
  end

  def render_to(path)
    puts "Rendering: #{path}"
    output = Pathname.new(path)
    output.dirname.mkpath
    output.write(contents)
  end

  def contents
    erb('layout') do
      erb(page.class.to_s.downcase)
    end
  end

  def erb(template)
    page.instance_eval { ERB.new(File.read("erb/#{template}.erb")).result(binding) }
  end

end
