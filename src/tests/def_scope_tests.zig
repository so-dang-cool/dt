const std = @import("std");
const expectEqualStrings = std.testing.expectEqualStrings;

const dt = @import("dt_test_utils.zig").dt;

test "basic_def" {
    const source =
        \\ [1 +] \inc def
        \\ 1 inc print
        \\ \inc def? print
    ;
    try expectEqualStrings("2true", (try dt(&.{source})).stdout);
}

test "basic_do_bang_def" {
    const source =
        \\ [[] \empty-quote :   ] do!
        \\
        \\ "empty-quote" def? "do! should define in parent context" assert-true
    ;
    try expectEqualStrings("", (try dt(&.{source})).stderr);
    try expectEqualStrings("", (try dt(&.{source})).stdout);
}

test "basic_do_def" {
    const source =
        \\ [[] \empty-quote :   ] do
        \\
        \\"empty-quote" undef? "do should NOT define in parent context" assert-true
    ;
    try expectEqualStrings("", (try dt(&.{source})).stderr);
    try expectEqualStrings("", (try dt(&.{source})).stdout);
}

test "basic_colon_do" {
    const source =
        \\ 3 "banana"
        \\
        \\ # Print banana three times
        \\ [[n str]: [str print] n times] do
        \\
        \\ "n" undef? "do must not leak definitions, but n was defined" assert-true
        \\ "str" undef? "do must not leak definitions, but str was defined" assert-true
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("banana" ** 3, res.stdout);
}

test "colon_in_times" {
    const source =
        \\ 1
        \\ [ [n] :
        \\     n println
        \\     n 2 *
        \\ ] 7 times
        \\ drop
        \\
        \\ \n undef? "times must not leak definitions, but n was defined" assert-true
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("1\n" ++ "2\n" ++ "4\n" ++ "8\n" ++ "16\n" ++ "32\n" ++ "64\n", res.stdout);
}

test "colon_in_each" {
    const source =
        \\ [ 1 2 3 4 5 ]
        \\ [ \n : n n * println ] each
        \\
        \\ \n undef? "each must not leak definitions, but n was defined" assert-true
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("1\n" ++ "4\n" ++ "9\n" ++ "16\n" ++ "25\n", res.stdout);
}

test "colon_in_map" {
    const source =
        \\ ["apple" "banana" "cereal"]
        \\ [[food]: food upcase] map print
        \\
        \\ \food undef? "map must not leak definitions, but food was defined" assert-true
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("[ \"APPLE\" \"BANANA\" \"CEREAL\" ]", res.stdout);
}

test "colon_in_filter" {
    const source =
        \\ [[1 "banana"] [2 "banana"] [3 "banana"] [4 "bananas make a bunch!"]]
        \\ [...[n str]: n even? str len odd?] filter ... print
        \\
        \\ \n   undef? "filter must not leak definitions, but n was defined" assert-true
        \\ \str undef? "filter must not leak definitions, but str was defined" assert-true
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("[ 4 \"bananas make a bunch!\" ]", res.stdout);
}

test "shadowing_in_do" {
    const source =
        \\ [ 5 ] "fav-number" def
        \\
        \\ fav-number println
        \\
        \\ [
        \\     [ 8 ] "fav-number" def
        \\     fav-number println
        \\ ] do
        \\
        \\ fav-number println
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("5\n" ++ "8\n" ++ "5\n", res.stdout);
}

test "shadowing_colon_in_do" {
    const source =
        \\ [ 6 ] "fav-number" def
        \\
        \\ fav-number println
        \\
        \\ 2 [ [ fav-number ] : fav-number println ] do
        \\
        \\ fav-number println
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("6\n" ++ "2\n" ++ "6\n", res.stdout);
}

test "shadowing_in_times" {
    const source =
        \\ [ 1 ] \n def
        \\
        \\ n [ \n :   n println   n 1 + ] 3 times
        \\ drop
        \\
        \\ n println
    ;

    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("1\n" ++ "2\n" ++ "3\n" ++ "1\n", res.stdout);
}

test "shadowing_in_each" {
    const source =
        \\ [ "pepperoni" ] \pizza def
        \\
        \\ pizza println
        \\
        \\ [ "cheese" "bbq" "combo" ] [ \pizza : pizza println ] each
        \\
        \\ pizza println
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("pepperoni\n" ++ "cheese\n" ++ "bbq\n" ++ "combo\n" ++ "pepperoni\n", res.stdout);
}

test "shadowing_in_map" {
    const source =
        \\ [ "banana" ] \x def
        \\
        \\ x println
        \\
        \\ [ 1 2 3 ] [ \x :   x 2.0 / ] map println
        \\
        \\ x println
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("banana\n" ++ "[ 0.5 1 1.5 ]\n" ++ "banana\n", res.stdout);
}

test "shadowing_in_filter" {
    const source =
        \\ [ "whee" ] \happy-word def
        \\
        \\ happy-word println
        \\
        \\ [ "yay" "hurray" "whoo" "huzzah" ] [ \happy-word :   happy-word len   4 gt? ] filter println
        \\
        \\ happy-word println
    ;
    const res = (try dt(&.{source}));
    try expectEqualStrings("", res.stderr);
    try expectEqualStrings("whee\n" ++ "[ \"hurray\" \"huzzah\" ]\n" ++ "whee\n", res.stdout);
}
