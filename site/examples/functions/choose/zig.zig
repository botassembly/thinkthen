const std = @import("std");
const tt = @import("thinkthen");

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
    var call = switch (try tt.calls.choose(&engine, .{
        .question = .{
            .definition = .{ .value = which_team },
        },
        .input = tt.inputs.RequestInputText{ .text = parcel },
    })) {
        .ok => |result| result,
        .failed => |failure| {
            defer tt.releaseFailure(allocator, failure);
            return error.CallFailed;
        },
    };
    defer call.deinit();

    const team = (call.value) orelse return error.NotSure;
    std.debug.assert(std.mem.eql(u8, team, "shipping"));
}
