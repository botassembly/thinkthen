# The Ruby helper's proof, run by test.sh with sentinels in this process.
require_relative "children"
require "tmpdir"
require "open3"

PROBE = 'test -z "${THINKTHEN_SENTINEL+x}" && test -z "${FAKE_SERVICE_API_KEY+x}" && ' \
        'test -z "${ABSENT_0127+x}" && test "$KEPT_0127" = kept && test "$SET_0127" = set'
REFUSED = %w[THINKTHEN_BASE_URL OPENAI_API_KEY GITHUB_TOKEN db_password AWS_SECRET_ACCESS_KEY].freeze

env = Children.env(keep: %w[KEPT_0127 ABSENT_0127], SET_0127: "set")
bad = system(env, "sh", "-c", PROBE, unsetenv_others: true) ? [] : ["the child"]
REFUSED.each do |name|
  Children.env(keep: [name])
  bad << name
rescue ArgumentError => e
  sentence = "a test child may not keep #{name} from the parent: set a THINKTHEN_ value or a fake key explicitly"
  bad << e.message unless e.message == sentence
end
# The owned folders stay under the caller's root. Explicit values override
# the defaults, and nil removes a name from the spawned environment.
Dir.mktmpdir("thinkthen-child-home-") do |root|
  env = Children.env(home: root, XDG_CONFIG_HOME: "#{root}/explicit", XDG_STATE_HOME: nil,
                     KEPT_0127: "kept", SET_0127: "set")
  probe = PROBE + ' && test -z "${XDG_STATE_HOME+x}" && printf "%s\n" "$HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME" "$APPDATA" "$LOCALAPPDATA"'
  out, _, status = Open3.capture3(env, "sh", "-c", probe, unsetenv_others: true)
  expected = ["#{root}/home", "#{root}/explicit", "#{root}/xdg-cache", "#{root}/xdg-config", "#{root}/local"]
  bad << "owned home and explicit override" unless status.success? && out.lines.map(&:chomp) == expected
end
puts "children ruby: #{bad.empty? ? 'ok' : "FAIL #{bad.join(', ')}"}"
exit(bad.empty? ? 0 : 1)
