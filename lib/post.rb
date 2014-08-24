require 'rdiscount'
require 'coderay'
require 'pathname'
require 'date'

require Pathname.new(__FILE__).dirname.join('core_ext.rb')

class Post

  attr_accessor :source_file

  def initialize(src)
    @source_file = src
  end

  def body_html
    body_html_with_code_snippets(body_html_delim_removed(body_lines_as_html))
  end

  def body_html_with_code_snippets(html)
    html.gsub(/<pre type="(.*?)">(.*?)<\/pre>/m) do
      CodeRay.scan($2, code_type($1)).html(:wrap => :div, :bold_every => false, :line_numbers => false, :css => :class)
    end
  end

  def body_html_delim_removed(html)
    html.sub('DELIM', '')
  end

  def headers
    header_lines.map {|line| line.split(':') }.reduce({}) do |headers, key_value|
      add_header(headers, key_value) if key_value.size > 1
      headers
    end
  end

  def add_header(headers, key_value)
    key = key_value.first.to_sym
    value = key_value.size == 2 ? key_value[1] : key_value[1..-1].join(':')
    headers[key] = value.strip
  end

  def title
    title = headers[:title]
    title =~ /^"(.*)"$/ ? $1 : title
  end

  def date
    Date.parse(headers[:date])
  end

  def month_string
    "#{Date::MONTHNAMES[date.month]} #{date.year}"
  end

  def tag
    headers[:tag]
  end

  def url
    headers[:url] || "/#{date.year}/#{date.month}/#{date.day}/#{title.slugize}"
  end

  def formatted_date
    date.strftime("%B #{date.day.ordinal} %Y")
  end

  def config
    { disqus: 'patshaughnessy' }
  end

  def render_to(path)
    Renderer.new(self).render_to(path)
  end

  def erb_context
    binding
  end

  private
  def body_lines_as_html
    RDiscount.new(body_lines.join, :smart).to_html
  end

  def body_lines
    lines.drop_while { |line| is_header?(line) }
  end

  def header_lines
    lines.take_while { |line| is_header?(line) }
  end

  def lines
    File.read(source_file).lines
  end

  def is_header?(line)
    line =~ /^[a-z]+:/ || line == "\n"
  end

  def code_type(pre_tag_type_attrib)
    if pre_tag_type_attrib == 'console'
      :text
    else
      pre_tag_type_attrib.to_sym
    end
  end

end
