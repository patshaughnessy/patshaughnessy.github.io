require 'test_helper'
require Pathname.new(__FILE__).dirname.expand_path.join('../lib/post.rb')
require Pathname.new(__FILE__).dirname.expand_path.join('../lib/layout.rb')

class Layout

  alias :erb_orig :erb

  def erb(template)
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
    binding.eval("source_file").must_equal '/path/to/some_article.html'
  end

end

describe Layout do

  let(:post) { Post.new('/path/to/some_article.html') }
  let(:layout) { Layout.new(post) }
  let(:output_file) { Pathname.new(__FILE__).dirname.join('support/erb_output_dir/erb_output.html') }

  it 'has a page' do
    layout.page.must_equal post
  end

  it 'creates the directory containing the output file' do
    output_file.dirname.rmtree
    layout.stubs(:contents)
    layout.stubs(:puts)
    layout.render_to(output_file)
    output_file.dirname.exist?.must_equal true
  end

  it 'renders the layout and expects it to yield to render the post' do
    layout.contents.must_equal '[OUTER_HTML][INNER_HTML]'
  end

  it 'passes the contents of template to ERB and returns the result' do
    compiled_erb = mock
    ERB.expects(:new).with(File.read('erb/post.erb')).returns(compiled_erb)
    compiled_erb.expects(:result).returns("HTML")
    layout.erb_orig('post').must_equal "HTML"
  end

  it 'passes the proper binding to ERB' do
    ERB.expects(:new).returns(CompiledErb.new)
    layout.erb_orig('post')
  end

end
