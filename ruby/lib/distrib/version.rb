# This is free and unencumbered software released into the public domain.

module Distrib; end
module Distrib::VERSION; end

module Distrib::VERSION
  FILE = File.expand_path('../../../VERSION', __FILE__)
  STRING = File.read(FILE).chomp.freeze
  MAJOR, MINOR, PATCH, EXTRA = STRING.split('.').map(&:freeze)
end # Distrib::VERSION
