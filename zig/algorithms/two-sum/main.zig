const std = @import("std");
const expect = @import("std").testing.expect;

pub fn main() !void {}

/// time O(n)
/// space O(n)
fn twoSum(allocator: std.mem.Allocator, numbers: []i32, target: i32) ![2]usize {
    var cache = std.hash_map.AutoHashMap(i32, usize).init(allocator);
    defer cache.deinit();

    for (numbers, 0..) |number, i| {
        if (cache.get(number)) |j| {
            return [_]usize{ j, i };
        }

        var difference = std.math.absInt(target - number) catch unreachable;

        try cache.put(difference, i);
    }

    unreachable;
}

test "twoSum" {
    {
        var array = [_]i32{ 2, 7, 11, 15 };
        var result = try twoSum(std.testing.allocator, &array, 9);
        try expect(std.mem.eql(usize, &result, &[_]usize{ 0, 1 }));
    }

    {
        var array = [_]i32{ 3, 2, 4 };
        var result = try twoSum(std.testing.allocator, &array, 6);
        try expect(std.mem.eql(usize, &result, &[_]usize{ 1, 2 }));
    }

    {
        var array = [_]i32{ 3, 3 };
        var result = try twoSum(std.testing.allocator, &array, 6);
        try expect(std.mem.eql(usize, &result, &[_]usize{ 0, 1 }));
    }
}
