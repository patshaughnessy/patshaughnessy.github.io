require 'test_helper'
require Pathname.new(__FILE__).dirname.expand_path.join('../lib/post.rb')
require Pathname.new(__FILE__).dirname.expand_path.join('../lib/layout.rb')

class Layout

  alias :erb_orig :erb

  def erb(template, page)
    if block_given?
      template.must_equal 'layout'
      '[OUTER_HTML]' + yield
    else
      template.must_equal 'post'
      '[INNER_HTML]'
    end
  end

end

class CompiledErb

  def result(binding)
    binding.eval("source_file").must_equal '/path/to/some_article1.html'
    binding.eval("recent_posts.collect{|recent| recent.source_file }").must_equal [
      '/path/to/some_article5.html',
      '/path/to/some_article4.html',
      '/path/to/some_article3.html',
      '/path/to/some_article2.html'
    ]
  end

end

describe Layout do

  5.times do |n|
    let("post#{n+1}".to_sym) { Post.new("/path/to/some_article#{n+1}.html") }
  end

  before do
    post1.stubs(:tag).returns('Ruby')
    post2.stubs(:tag).returns('Ruby')
    post3.stubs(:tag).returns('Ruby')
    post4.stubs(:tag).returns('Ruby')
    post5.stubs(:tag).returns('Ruby')
  end

  let(:posts) { [ post5, post4, post3, post2, post1 ] }
  let(:layout) { Layout.new(posts) }
  let(:output_file) { Pathname.new(__FILE__).dirname.join('support/erb_output_dir/erb_output.html') }

  it 'has a page' do
    layout.posts.must_equal posts
  end

  it 'creates the directory containing the output file' do
    output_file.dirname.rmtree
    layout.stubs(:contents)
    layout.stubs(:puts)
    layout.render(post1, output_file)
    output_file.dirname.exist?.must_equal true
  end

  it 'renders the layout and expects it to yield to render the post' do
    layout.contents(post1).must_equal '[OUTER_HTML][INNER_HTML]'
  end

  it 'passes the contents of template to ERB and returns the result' do
    compiled_erb = mock
    ERB.expects(:new).with(File.read('erb/post.erb')).returns(compiled_erb)
    compiled_erb.expects(:result).returns("HTML")
    layout.erb_orig('post', post1).must_equal "HTML"
  end

  it 'evaluates the template in the context of the proper post, and makes recent_posts available' do
    ERB.expects(:new).returns(CompiledErb.new)
    layout.erb_orig('post', post1)
  end

end
