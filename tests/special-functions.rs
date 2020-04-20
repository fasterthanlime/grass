#![cfg(test)]

#[macro_use]
mod macros;

test!(
    calc_whitespace,
    "a {\n  color: calc(       1      );\n}\n",
    "a {\n  color: calc( 1 );\n}\n"
);
test!(
    calc_multiple_args,
    "a {\n  color: calc(1, 2, a, b, c);\n}\n"
);
test!(
    calc_does_not_evaluate_arithmetic,
    "a {\n  color: calc(1 + 2);\n}\n"
);
test!(
    calc_evaluates_interpolated_arithmetic,
    "a {\n  color: calc(#{1 + 2});\n}\n",
    "a {\n  color: calc(3);\n}\n"
);
test!(calc_retains_silent_comment, "a {\n  color: calc(//);\n}\n");
test!(
    calc_retains_multiline_comment,
    "a {\n  color: calc(/**/);\n}\n"
);
test!(calc_nested_parens, "a {\n  color: calc((((()))));\n}\n");
test!(
    element_whitespace,
    "a {\n  color: element(       1      );\n}\n",
    "a {\n  color: element( 1 );\n}\n"
);
test!(
    element_multiple_args,
    "a {\n  color: element(1, 2, a, b, c);\n}\n"
);
test!(
    element_does_not_evaluate_arithmetic,
    "a {\n  color: element(1 + 2);\n}\n"
);
test!(
    element_evaluates_interpolated_arithmetic,
    "a {\n  color: element(#{1 + 2});\n}\n",
    "a {\n  color: element(3);\n}\n"
);
test!(
    element_retains_silent_comment,
    "a {\n  color: element(//);\n}\n"
);
test!(
    element_retains_multiline_comment,
    "a {\n  color: element(/**/);\n}\n"
);
test!(
    element_nested_parens,
    "a {\n  color: element((((()))));\n}\n"
);
test!(
    expression_whitespace,
    "a {\n  color: expression(       1      );\n}\n",
    "a {\n  color: expression( 1 );\n}\n"
);
test!(
    expression_multiple_args,
    "a {\n  color: expression(1, 2, a, b, c);\n}\n"
);
test!(
    expression_does_not_evaluate_arithmetic,
    "a {\n  color: expression(1 + 2);\n}\n"
);
test!(
    expression_evaluates_interpolated_arithmetic,
    "a {\n  color: expression(#{1 + 2});\n}\n",
    "a {\n  color: expression(3);\n}\n"
);
test!(
    expression_retains_silent_comment,
    "a {\n  color: expression(//);\n}\n"
);
test!(
    expression_retains_multiline_comment,
    "a {\n  color: expression(/**/);\n}\n"
);
test!(
    expression_nested_parens,
    "a {\n  color: expression((((()))));\n}\n"
);
test!(
    progid_whitespace,
    "a {\n  color: progid:(       1      );\n}\n",
    "a {\n  color: progid:( 1 );\n}\n"
);
test!(
    progid_multiple_args,
    "a {\n  color: progid:(1, 2, a, b, c);\n}\n"
);
test!(
    progid_does_not_evaluate_arithmetic,
    "a {\n  color: progid:(1 + 2);\n}\n"
);
test!(
    progid_evaluates_interpolated_arithmetic,
    "a {\n  color: progid:(#{1 + 2});\n}\n",
    "a {\n  color: progid:(3);\n}\n"
);
test!(
    progid_retains_silent_comment,
    "a {\n  color: progid:(//);\n}\n"
);
test!(
    progid_retains_multiline_comment,
    "a {\n  color: progid:(/**/);\n}\n"
);
test!(
    progid_nested_parens,
    "a {\n  color: progid:((((()))));\n}\n"
);
test!(
    progid_values_after_colon,
    "a {\n  color: progid:apple.bottoM..jeans.boots();\n}\n"
);
error!(
    progid_number_after_colon,
    "a {\n  color: progid:ap1ple.bottoM..jeans.boots();\n}\n", "Error: expected \"(\"."
);
