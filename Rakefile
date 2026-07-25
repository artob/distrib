abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

DISTRIB_SUBCOMMANDS = %w[list convert parse]

task default: %w[codegen]

desc "Generate .config/readmer/*.sh-session files"
task codegen: %w[.config/readmer/distrib.sh-session] +
  DISTRIB_SUBCOMMANDS.map { ".config/readmer/distrib-#{it}.sh-session" }.to_a

([nil] + DISTRIB_SUBCOMMANDS).each do |subcommand|
  command = subcommand ? "distrib #{subcommand} --help" : "distrib"
  filename = command.delete_suffix(' --help').gsub(' ', '-')
  desc "Generate .config/readmer/#{filename}.sh-session"
  file ".config/readmer/#{filename}.sh-session" do |t|
    File.open(t.name, 'w') do |f|
      f.puts "$ #{command}"
      f.puts `#{command} 2>&1`
    end
  end
end
