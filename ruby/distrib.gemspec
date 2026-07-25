Gem::Specification.new do |gem|
  gem.version            = File.read('VERSION').chomp
  gem.date               = File.mtime('VERSION').strftime('%Y-%m-%d')

  gem.name               = "distrib"
  gem.homepage           = "https://distrib.sh"
  gem.license            = "Unlicense"
  gem.summary            = "Distrib for Ruby"
  gem.description        = "Distrib helps you distribute your software."
  gem.metadata           = {
    'bug_tracker_uri'   => "https://github.com/artob/distrib/issues",
    'changelog_uri'     => "https://github.com/artob/distrib/blob/master/CHANGES.md",
    'documentation_uri' => "https://rubydoc.info/gems/distrib",
    'homepage_uri'      => "https://distrib.sh",
    'source_code_uri'   => "https://github.com/artob/distrib",
  }

  gem.author             = "Arto Bendiken"
  gem.email              = "arto@bendiken.net"

  gem.platform           = Gem::Platform::RUBY
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob('lib/**/*.rb')
  gem.bindir             = %q(bin)
  gem.executables        = %w()

  gem.required_ruby_version = '>= 4.0'
  gem.add_development_dependency 'rake',  '~> 13'
  gem.add_development_dependency 'rspec', '~> 3.13'
  gem.add_development_dependency 'yard' , '~> 0.9'
end
