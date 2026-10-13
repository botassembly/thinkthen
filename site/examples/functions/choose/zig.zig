const std = @import("std");
const tt = @import("thinkthen");
const c = tt.c;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| {
            tt.releaseFailure(allocator, failure);
            return error.NoEngine;
        },
    };
    defer engine.deinit();

    const question = "Which team owns this?";
    const teams: tt.inputs.AuthoredOptions = .{ .object = .{
        .entries = &.{
            .{ .key = "billing", .value = .{
                .string = "Invoices, fees, and refunds.",
            } },
            .{ .key = "shipping", .value = .{
                .string = "Parcels and delivery.",
            } },
            .{ .key = "account", .value = .{
                .string = "Logins and passwords.",
            } },
        },
    } };
    const parcel = "My parcel went to the wrong address.";
    const which_team: tt.inputs.RequestDefinition = .{
        .Authored_choose = .{
            .choose = .{ .string = question },
            .options = teams,
        },
    };
    var call = try tt.session.choose(&engine, .{
        .question = .{
            .definition = .{ .value = which_team },
        },
        .input = .{ .text = .{ .text = parcel } },
    });
    defer call.deinit();

    const choose_row =
        c.THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1;
    var team: []const u8 = "";

    while (true) {
        var packet = switch (try call.read()) {
            .pending => {
                var ready: u32 = undefined;
                _ = c.thinkthen_session_wait(
                    call.raw,
                    c.THINKTHEN_SESSION_WAIT_RESULT_V1,
                    50,
                    &ready,
                );
                continue;
            },
            .end => break,
            .packet => |value| value,
        };
        defer packet.deinit();
        if (packet.view.kind != choose_row) continue;
        const row = packet.view.data.choose_row.*.value.*;
        const name =
            tt.session.optional(row.value) orelse continue;
        const text = tt.session.bytes(name);
        team = try allocator.dupe(u8, text);
    }
    std.debug.assert(std.mem.eql(u8, team, "shipping"));
}
