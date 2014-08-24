class String
  def slugize
    self.downcase.gsub(/&/, 'and').gsub(/(\s+)|\./, '-').gsub(/[^a-z0-9\-_]/, '')
  end
end

class Fixnum
  def ordinal
    # 1 => 1st
    # 2 => 2nd
    # 3 => 3rd
    # ...
    case self % 100
      when 11..13; "#{self}th"
    else
      case self % 10
        when 1; "#{self}st"
        when 2; "#{self}nd"
        when 3; "#{self}rd"
        else    "#{self}th"
      end
    end
  end
end
