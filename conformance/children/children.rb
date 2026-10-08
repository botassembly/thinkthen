# A test child's whole environment, built from nothing (ticket 0127).
# The child gets PATH, the parent's value of each name the caller keeps, and
# the values the caller sets. A secret-shaped or THINKTHEN_ name is never
# kept. Every spawn that takes this also passes `unsetenv_others: true`.
module Children
  SECRET = /\ATHINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH/i

  # Tool children omit home. Product children supply their owned root;
  # these paths preserve the Ruby fixtures' home and xdg-* locations.
  # Explicit values still win, including nil to unset a name at spawn.
  def self.env(keep: [], home: nil, **values)
    env = { "PATH" => ENV.fetch("PATH", "/usr/bin:/bin") }
    keep.each do |name|
      if SECRET.match?(name)
        raise ArgumentError, "a test child may not keep #{name} from the parent: " \
                             "set a THINKTHEN_ value or a fake key explicitly"
      end
      env[name] = ENV[name] if ENV.key?(name)
    end
    if home
      env.merge!("HOME" => File.join(home, "home"),
                 "XDG_CONFIG_HOME" => File.join(home, "xdg-config"),
                 "XDG_CACHE_HOME" => File.join(home, "xdg-cache"),
                 "XDG_STATE_HOME" => File.join(home, "xdg-state"),
                 "APPDATA" => File.join(home, "xdg-config"),
                 "LOCALAPPDATA" => File.join(home, "local"))
    end
    env.merge(values.transform_keys(&:to_s))
  end
end
