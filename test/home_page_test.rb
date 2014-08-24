require 'test_helper'
root = Pathname.new(__FILE__).dirname.parent.expand_path
require root.join('lib/post.rb')
require root.join('lib/home_page.rb')

describe HomePage do

  let(:posts) { FileList["#{root}/posts/*.markdown"].map{|markdown| Post.new(markdown) } }
  let(:home_page) { HomePage.new(posts) }

  it 'has posts, saved in a reverse order' do
    home_page.posts.must_equal posts.reverse
  end

  it 'returns a list of posts sorted by date' do
    posts = home_page.posts
    posts.size.must_equal 120
    posts.first.title.must_equal "A Rule of Thumb for Strong Parameters"
    posts.last.title.must_equal "Why to use TDD with Drupal"
  end

  it 'returns a list of month strings for these posts, but using empty strings for duplicates' do
    months = home_page.month_strings
    size = months.size
    size.must_equal 120
    months[0..7].must_equal [
      "June 2014",
      "April 2014",
      "February 2014",
      "January 2014",
      "",
      "",
      "December 2013",
      "November 2013"
    ]
  end

end
