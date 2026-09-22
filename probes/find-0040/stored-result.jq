if (.meta | type) == "object" and (.meta | has("cached"))
then .meta.replayed = .meta.cached | del(.meta.cached)
else .
end
