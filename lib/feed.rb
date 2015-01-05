require 'builder'
require 'time'

class Feed

  attr_reader :posts

  def initialize(posts)
    @posts = posts[0..9]
  end

  def render(path)
    puts "Rendering: #{path}"
    output = Pathname.new(path)
    output.dirname.mkpath
    output.write(contents)
  end

  def contents
    xml.instruct!
    feed
  end

  def feed
    xml.feed 'xmlns' => 'http://www.w3.org/2005/Atom' do
      xml.title 'Pat Shaughnessy'
      xml.id 'http://patshaughnessy.net'
      xml.updated '2008-09-03T00:00:00Z'
      xml.author { xml.name '' }
      posts.each do |post|
        entry(post)
      end
    end
  end

  def entry(post)
    xml.entry do
      xml.title post.title
      xml.link "href" => absolute(post.url), "rel" => "alternate"
      xml.id absolute(post.url)
      xml.published time_from(post.date)
      xml.updated time_from(post.date)
      xml.category 'ruby'
      xml.author { xml.name '' }
      xml.summary post.summary, "type" => "html"
      xml.content post.body_html, "type" => "html"
    end
  end

  def xml
    @builder ||= Builder::XmlMarkup.new(indent: 2)
  end

  def absolute(url)
    "http://patshaughnessy.net#{url}"
  end

  def time_from(date)
    adjusted_time = date.to_time-4*60*60
    adjusted_time.utc.iso8601
  end

end
