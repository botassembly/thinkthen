# A test child's whole environment, built from nothing (ticket 0127).
# The child gets PATH, the parent's value of each name the caller keeps, and
# the values the caller sets. A secret-shaped or THINKTHEN_ name is never
# kept. Every spawn that takes this also passes `unsetenv_others: true`.
module Children
  SECRET = /\ATHINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH/i

  # PATH, each kept name the parent has, then the values set here.
  def self.env(keep: [], **values)
    env = { "PATH" => ENV.fetch("PATH", "/usr/bin:/bin") }
    keep.each do |name|
      if SECRET.match?(name)
        raise ArgumentError, "a test child may not keep #{name} from the parent: " \
                             "set a THINKTHEN_ value or a fake key explicitly"
      end
      env[name] = ENV[name] if ENV.key?(name)
    end
    env.merge(values.transform_keys(&:to_s))
  end
end
