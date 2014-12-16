guard :shell do
  watch(%r{^posts/(.+).markdown$}) do |m|
    `DRAFT=1 rake`
  end
end
