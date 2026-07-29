# See: https://docs.ruby-lang.org/en/4.0/Gem/Specification.html

require 'distrib/ruby/gemspec'

Distrib::Ruby::Gemspec.build!(__FILE__) do |gemspec|
  gemspec.summary     = "Distrib for Ruby"
  gemspec.description = "Distrib helps you distribute your software."
  gemspec.homepage    = "https://distrib.sh"
  gemspec.metadata    = {
    :source_code_uri  => "https://github.com/artob/distrib",
    :bug_tracker_uri  => "https://github.com/artob/distrib/issues",
    :changelog_uri    => "https://github.com/artob/distrib/blob/master/CHANGES.md",
  }.transform_keys(&:to_s)
end
