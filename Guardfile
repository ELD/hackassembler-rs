# A sample Guardfile
# More info at https://github.com/guard/guard#readme

## Uncomment and set this to only include directories you want to watch
# directories %w(app lib config test spec features) \
#  .select{|d| Dir.exists?(d) ? d : UI.warning("Directory #{d} does not exist")}

## Note: if you are using the `directories` clause above and you are not
## watching the project directory ('.'), then you will want to move
## the Guardfile to a watched dir and symlink it back, e.g.
#
#  $ mkdir config
#  $ mv Guardfile config/
#  $ ln -s config/Guardfile .
#
# and, you'll have to watch "config/Guardfile" instead of "Guardfile"

# Add files and commands to this file, like the example:
#   watch(%r{file/path}) { `command(s)` }
#
guard :shell do
  watch(/src\/(.*).rs/) do |m|
    path = m.first

    mod = path.sub(/src\//, '')
              .sub(/\/mod\.rs/, '.rs')
              .sub(/\/tests\.rs/, '.rs')
              .gsub(/\//, '::')
              .sub(/\.rs/, '::test')

    `cargo test #{mod}`
  end
end
