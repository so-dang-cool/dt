const std = @import("std");
const expectEqual = std.testing.expectEqual;
const expectEqualStrings = std.testing.expectEqualStrings;

const dt = @import("dt_test_utils.zig").dt;

test "true" {
    const res = try dt(&.{ "true", "print" });
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("true", res.stdout);
    try expectEqual(@as(u8, 0), res.term.Exited);
}

test "false" {
    const res = try dt(&.{ "false", "print" });
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("false", res.stdout);
    try expectEqual(@as(u8, 0), res.term.Exited);
}

test "not" {
    try expectEqualStrings("true", (try dt(&.{"false not print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"true not print"})).stdout);
}

test "bool_equality" {
    const first_result = try dt(&.{"true true eq? print"});
    try expectEqualStrings("", first_result.stderr);
    try expectEqualStrings("true", first_result.stdout);
    try expectEqualStrings("true", (try dt(&.{"false false eq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"true false neq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"false true neq? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"true true neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"false false neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"true false eq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"false true eq? print"})).stdout);
}

test "numeric_equality" {
    try expectEqualStrings("true", (try dt(&.{"1 1 eq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1.0 eq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 1 eq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 1.0 eq? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"1 2 eq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 2.0 eq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 2 eq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 2.0 eq? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"1 1 neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 1.0 neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1 neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1.0 neq? print"})).stdout);

    try expectEqualStrings("true", (try dt(&.{"1 2 neq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 2.0 neq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 2 neq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 2.0 neq? print"})).stdout);
}

test "string_equality" {
    try expectEqualStrings("true", (try dt(&.{"\"apple\" \"apple\" eq? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"\"apple\" \"orange\" neq? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"\"apple\" \"apple\" neq? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"\"apple\" \"orange\" eq? print"})).stdout);
}

test "quote_of_many_types_equality" {
    try expectEqualStrings("true", (try dt(&.{"[ true 2 -3.4e5 \"6\" [ print ] ]\n[ true 2 -3.4e5 \"6\" [ print ] ]\neq? print\n"})).stdout);
}

test "comparison_lt" {
    try expectEqualStrings("true", (try dt(&.{"1 2 lt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1.1 lt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 2 lt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 2.2 lt? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"1 1 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 1.0 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1.0 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 0 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 0.9 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.1 1 lt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.1 0.9 lt? print"})).stdout);
}

test "comparison_lte" {
    try expectEqualStrings("true", (try dt(&.{"1 2 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1.1 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 2 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 1.1 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1.0 lte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 1 lte? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"1 0 lte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 0.9 lte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.1 1 lte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.1 0.9 lte? print"})).stdout);
}
test "comparison_gt" {
    try expectEqualStrings("true", (try dt(&.{"2 1 gt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 1 gt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"2 1.1 gt? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"2.2 1.1 gt? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"1 1 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 1.0 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1.0 1.0 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"0 1 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"0.9 1 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 1.1 gt? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"0.9 1.1 gt? print"})).stdout);
}

test "comparison_gte" {
    try expectEqualStrings("true", (try dt(&.{"2 1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"2 1.1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.1 1.1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1.0 1 gte? print"})).stdout);
    try expectEqualStrings("true", (try dt(&.{"1 1.0 gte? print"})).stdout);

    try expectEqualStrings("false", (try dt(&.{"0 1 gte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"0.9 1 gte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"1 1.1 gte? print"})).stdout);
    try expectEqualStrings("false", (try dt(&.{"0.9 1.1 gte? print"})).stdout);
}
