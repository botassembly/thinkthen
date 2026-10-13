// Generated named calls from the canonical request schema; do not edit.
const tt = @import("thinkthen.zig");
const inputs = @import("request_generated.zig");
pub const Arguments = struct {
    question: inputs.RequestQuestion,
    input: inputs.RequestInput,
    options: ?inputs.RequestOptions = null,
};
pub fn decide(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .decide, arguments);
}
pub fn choose(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .choose, arguments);
}
pub fn tag(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .tag, arguments);
}
pub fn score(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .score, arguments);
}
pub fn filter(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .filter, arguments);
}
pub fn rank(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .rank, arguments);
}
pub fn find(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .find, arguments);
}
pub fn annotate(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .annotate, arguments);
}
pub fn recognize(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .recognize, arguments);
}
pub fn relate(engine: *tt.Engine, arguments: Arguments) !tt.session.Session {
    return tt.session.startNamed(engine, .relate, arguments);
}
