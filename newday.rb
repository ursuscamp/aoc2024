#!/usr/bin/env ruby

require 'fileutils'

# Check if a day number is provided
if ARGV.empty?
  puts 'Please provide a day number (1-25)'
  exit 1
end

# Pad the day number with a leading zero if needed
day = format('%02d', ARGV[0].to_i)
day_no_zero = ARGV[0].to_i

# Directories
SRC_DIR = 'src'
INPUTS_DIR = 'inputs'
TEMPLATE_FILE = 'new_day.tmpl'
MAIN_RS = File.join(SRC_DIR, 'main.rs')

# Create day directory
FileUtils.mkdir_p(File.join(SRC_DIR, "day#{day}"))

# Copy template to new module file and replace day placeholder
template_content = File.read(TEMPLATE_FILE)
module_content = template_content.gsub('%%DAY%%', day_no_zero.to_s)
File.write(File.join(SRC_DIR, "day#{day}", 'mod.rs'), module_content)

# Read main.rs content
main_rs_content = File.read(MAIN_RS).lines

# Add use statement if not already present
use_statement = "mod day#{day};"
unless main_rs_content.include?(use_statement)
  # Find the last 'mod' statement and insert after it
  last_mod_line = main_rs_content.rindex { |line| line.start_with?('mod ') }
  main_rs_content.insert(last_mod_line + 1, "#{use_statement}\n")
end

main_rs_content = main_rs_content.join

# Update match statement
match_statement = "        #{day_no_zero} => day#{day}::run(example)?,"
main_rs_content.gsub!(/match day \{.*?^\s*_\s*=>/m) do |match|
  match_lines = match.lines
  catch_all_line = match_lines.pop
  match_lines << "#{match_statement}\n" << catch_all_line
  match_lines.join
end

# Write updated main.rs
File.write(MAIN_RS, main_rs_content)

# Create input files
FileUtils.touch(File.join(INPUTS_DIR, "#{day}.txt"))
FileUtils.touch(File.join(INPUTS_DIR, "#{day}e.txt"))

puts "Added day #{day_no_zero} to the project:"
puts "- Created #{SRC_DIR}/day#{day}/mod.rs (replaced %%DAY%% with #{day_no_zero})"
puts "- Added use statement to #{MAIN_RS}"
puts "- Added match arm to #{MAIN_RS}"
puts "- Created #{INPUTS_DIR}/#{day}.txt"
puts "- Created #{INPUTS_DIR}/#{day}e.txt"
