class HomePage

  attr_reader :posts

  def initialize(posts)
    @posts = posts
  end

  def urls
    posts.map(&:url)
  end

  def titles
    posts.map(&:title)
  end

  def title
    'Pat Shaughnessy'
  end

  def month_strings
    @month_strings ||= months_with_empty_duplicates
  end

  def tag
    nil
  end

  private
  def months_with_empty_duplicates
    posts.map(&:month_string).reduce([]) do |months, month_string|
      if months.include?(month_string)
        months.push("")
      else
        months.push(month_string)
      end
    end
  end

end
