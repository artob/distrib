# This is free and unencumbered software released into the public domain.

require 'rubygems/specification'

module Distrib; end
module Distrib::Ruby; end

module Distrib::Ruby::Gemspec
  ##
  # Builds up a `Gem::Specification`.
  #
  # @example
  #   require 'distrib/ruby/gemspec'
  #
  #   Distrib::Ruby::Gemspec.build!(__FILE__) do |gemspec|
  #     gemspec.add_dependency 'liquid'
  #   end
  #
  # @param gemspec_path [String] the path to the gemspec file
  # @param kwargs [Hash] keyword arguments for the gemspec
  # @yield [Gem::Specification] the gemspec object
  # @return [Gem::Specification]
  def self.build!(gemspec_path, **kwargs, &block)
    gemspec_path = Pathname(gemspec_path)

    gemspec = Gem::Specification.new do |s|
      s.name = (kwargs[:name] || gemspec_path.basename('.*')).to_s
      s.version = (kwargs[:version] || File.read('VERSION').chomp).to_s
      s.date = (kwargs[:date] || File.mtime('VERSION').strftime('%Y-%m-%d')).to_s
      s.license = 'Unlicense' if File.exist?('UNLICENSE')

      authors = (File.readlines('AUTHORS') rescue []).map(&:chomp)
      s.authors = authors.map { |entry| entry[/^\*\s+(.+?)\s+</, 1] }
      s.email = authors.map { |entry| entry[/<([^>]+)>/, 1] }

      s.files = %w[AUTHORS CHANGES.md README.md UNLICENSE VERSION]
      s.files.reject! { |f| !File.exist?(f) }
      s.files += Dir['lib/**/*.rb']
      s.files += Dir['ext/**/*.{rb,rs,lock,toml}']
      s.extensions = Dir['ext/*/extconf.rb']

      s.required_ruby_version = '>= 4.0'  # just a default

      s.add_dependency 'rb_sys' if File.exist?('Cargo.toml')

      s.add_development_dependency 'distrib',       '~> 0' unless s.name == 'distrib'
      s.add_development_dependency 'rake',          '~> 13'
      s.add_development_dependency 'rake-compiler', '~> 1.3' if File.exist?('Cargo.toml')
      s.add_development_dependency 'rspec',         '~> 3.13' if Dir.exist?('spec')
      s.add_development_dependency 'yard' ,         '~> 0.9'
    end

    yield gemspec if block_given?

    gemspec.metadata = (gemspec.metadata || {}).transform_keys(&:to_s)

    if !gemspec.metadata.has_key?('documentation_uri')
      gemspec.metadata['documentation_uri'] =
        "https://rubydoc.info/gems/#{gemspec.name}"
    end

    if !gemspec.metadata.has_key?('homepage_uri')
      gemspec.metadata['homepage_uri'] =
        gemspec.homepage || "https://rubygems.org/gems/#{gemspec.name}"
    end

    gemspec
  end
end # Distrib::Ruby::Gemspec
