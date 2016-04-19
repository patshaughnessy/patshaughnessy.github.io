require 'test_helper'
root = Pathname.new(__FILE__).dirname.parent.expand_path
require root.join('lib/post.rb')
require root.join('lib/home_page.rb')

describe HomePage do

  let(:posts) { FileList["#{root}/posts/*.markdown"].map{|markdown| Post.new(markdown) } }
  let(:home_page) { HomePage.new(posts.reverse) }

  it 'has posts' do
    home_page.posts.must_equal posts.reverse
  end

  it 'returns a list of posts sorted by date' do
    posts = home_page.posts
    posts.size.must_equal Dir["posts/*"].length
    posts.last.title.must_equal "Why to use TDD with Drupal"
  end

  it 'returns a list of month strings for these posts, but using empty strings for duplicates' do
    months = home_page.month_strings
    size = months.size
    size.must_equal Dir["posts/*"].length
    months[0..9].must_equal [
      "April 2016",
      "January 2016",
      "November 2015",
      "September 2015",
      "June 2015",
      "February 2015",
      "January 2015",
      "December 2014",
      "",
      "November 2014"
    ]
  end

end
