const std = @import("std");
const expect = std.testing.expect;

fn Tree(comptime T: type) type {
    return struct {
        const Self = @This();

        root: ?*Node(T),
        allocator: std.mem.Allocator,
        size: usize,

        fn init(allocator: std.mem.Allocator) Self {
            return Self{
                .root = null,
                .allocator = allocator,
                .size = 0,
            };
        }

        fn deinit(self: *Self) !void {
            var stack = std.ArrayList(*Node(T)).init(self.allocator);
            defer stack.deinit();

            if (self.root) |root| {
                try stack.append(root);
            }

            while (stack.popOrNull()) |node| {
                if (node.left) |leftNode| {
                    try stack.append(leftNode);
                }
                if (node.right) |rightNode| {
                    try stack.append(rightNode);
                }
                self.allocator.destroy(node);
            }
        }

        fn insert(self: *Self, value: T) !void {
            if (self.root != null) {
                var currentNode = self.root;

                while (currentNode) |node| {
                    if (value < node.value) {
                        currentNode = node.left;
                        if (node.left == null) {
                            node.left = (try self.allocator.create(
                                Node(T),
                            )).init(value);
                        }
                    } else {
                        currentNode = node.right;
                        if (node.right == null) {
                            node.right = (try self.allocator.create(
                                Node(T),
                            )).init(value);
                        }
                    }
                }
            } else {
                self.root = (try self.allocator.create(Node(T))).init(value);
            }

            self.size += 1;
        }
    };
}

fn Node(comptime T: type) type {
    return struct {
        const Self = @This();

        value: T,
        left: ?*Self = null,
        right: ?*Self = null,

        fn init(self: *Self, value: T) *Self {
            self.left = null;
            self.value = value;
            self.right = null;
            return self;
        }
    };
}

pub fn main() !void {}

test "debug" {
    var tree = Tree(i32).init(std.testing.allocator);

    try tree.insert(2);
    std.debug.print("aaaaaa tree: {any}\n\n", .{tree.root});
    try tree.insert(1);
    std.debug.print("aaaaaa tree: {any}\n\n", .{tree.root});
    try tree.insert(3);
    std.debug.print("aaaaaa tree: {any}\n\n", .{tree.root});

    try tree.deinit();
}
