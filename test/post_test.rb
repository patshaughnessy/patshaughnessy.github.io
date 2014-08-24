require 'test_helper'
require Pathname.new(__FILE__).dirname.expand_path.join('../lib/post.rb')

describe Post do

  let(:articles_path) { Pathname.new(__FILE__).expand_path.dirname.join('../posts') }
  let(:article) { articles_path.join('2008-09-03-drupal-tdd-1.markdown') }
  let(:article_title_containing_colon) { articles_path.join('2009-04-03-filtering-auto_complete-pick-lists-part-2-using-named-scopes.markdown') }
  let(:article_quoted_title) { articles_path.join('2013-06-03-ruby-python-java-c-and-programmer-happiness.markdown') }
  let(:post) { Post.new(article_quoted_title) }

  it 'has a source file' do
    post.source_file.must_equal article_quoted_title
  end

  it 'parses the headers' do
    post.headers.must_equal({
      title: "\"Ruby, Python, Java, C and Programmer Happiness\"",
      date: "2013/6/3",
      tag: "RubySource"
    })
  end

  describe 'when the title contains a colon' do

    before do
      post.source_file = article_title_containing_colon
    end

    it 'parses the title' do
      post.title.must_equal 'Filtering auto_complete pick lists – part 2: using named scopes'
    end

  end

  describe 'when the title is not quoted' do

    before do
      post.source_file = article
    end

    it 'parses the title' do
      post.title.must_equal 'Why to use TDD with Drupal'
    end

  end

  it 'parses the date' do
    post.date.must_equal Date.parse('2013-6-3')
  end

  it 'returns a month-year string for the home page' do
    post.month_string.must_equal "June 2013"
  end

  it 'formats the date properly' do
    post.formatted_date.must_equal 'June 3rd 2013'
  end

  it 'parses the tag' do
    post.tag.must_equal 'RubySource'
  end

  describe 'when no url header is specified' do

    it 'constructs the url from the date and title' do
      post.url.must_equal '/2013/6/3/ruby-python-java-c-and-programmer-happiness'
    end

  end

  describe 'when a url header is specified' do

    before do
    post.stubs(:headers).returns({
      title: "Why to use TDD with Drupal",
      date: "2008/9/3",
      tag: "Drupal",
      url: "/2008/9/3/drupal-tdd-1"
    })
    end

    it 'uses the url header' do
      post.url.must_equal '/2008/9/3/drupal-tdd-1'
    end

  end

  it 'has a hard coded config' do
    post.config.must_equal({
      disqus: 'patshaughnessy'
    })
  end

  it 'passes Ruby code snippets to CodeRay' do
    options = {:wrap => :div, :bold_every => false, :line_numbers => false, :css => :class}
    code_ray = mock
    CodeRay.expects(:scan).with('puts "Journey to the Center of JRuby".upcase', :ruby).returns(code_ray)
    code_ray.expects(:html).with(options).returns('[CODE]')
    post.body_html_with_code_snippets('before<pre type="ruby">puts "Journey to the Center of JRuby".upcase</pre>after').must_equal 'before[CODE]after'
  end

  it 'passes console code snippets to CodeRay without a type' do
    options = {:wrap => :div, :bold_every => false, :line_numbers => false, :css => :class}
    code_ray = mock
    CodeRay.expects(:scan).with('console text', :text).returns(code_ray)
    code_ray.expects(:html).with(options).returns('[CODE]')
    post.body_html_with_code_snippets('before<pre type="console">console text</pre>after').must_equal 'before[CODE]after'
  end

  it 'removes the delimeter string' do
    post.body_html_delim_removed('somethingDELIM_something_else').must_equal 'something_something_else'
  end

  it 'converts body markdown to html' do
    post.body_html.must_equal <<HTML
<div style=\"float: left; padding: 7px 30px 10px 0px\">
  <img src=\"http://patshaughnessy.net/assets/2013/6/3/python-cropped.png\">
</div>




<p>&nbsp;</p>


<blockquote>
  “Ruby is designed to make programmers happy.” - Yukihiro “Matz” Matsumoto
</blockquote>


<p>Not everyone might agree, but as a Rubyist I think Matz achieved his design
goal. There’s something intangible about Ruby’s syntax that makes it fun,
rewarding and easy to use – something that makes me happy. I thought it would
be fun to compare Ruby with a few other languages by looking at how different
open source developers implemented the same method or function in each
language. How do the languages differ? Do they make you equally happy?</p>

<p>And what better example to look at than inside of Ruby itself! Today I’m going
to look at how Ruby’s Hash#fetch method is implemented in Ruby (by Rubinius),
Python (by Topaz), Java (by JRuby) and finally in C (by standard Ruby 2.0). Of
course, there are many other programming languages out there, even other
versions of Ruby, but looking at a small slice of Ruby internals gives us an
interesting example and allows us to compare apples with apples.</p>

<p>Read the <a href=\"http://rubysource.com/ruby-python-java-c-and-programmer-happiness/\">full article on RubySource.com</a>.</p>
HTML
  end

end
