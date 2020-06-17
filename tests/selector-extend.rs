#![cfg(test)]

#[macro_use]
mod macros;

test!(
    simple_attribute_equal,
    "a {\n  color: selector-extend(\"[c=d]\", \"[c=d]\", \"e\");\n}\n",
    "a {\n  color: [c=d], e;\n}\n"
);
test!(
    simple_attribute_unequal_name,
    "a {\n  color: selector-extend(\"[c=d]\", \"[e=d]\", \"f\");\n}\n",
    "a {\n  color: [c=d];\n}\n"
);
test!(
    simple_attribute_unequal_value,
    "a {\n  color: selector-extend(\"[c=d]\", \"[c=e]\", \"f\");\n}\n",
    "a {\n  color: [c=d];\n}\n"
);
test!(
    simple_attribute_unequal_operator,
    "a {\n  color: selector-extend(\"[c=d]\", \"[c^=e]\", \"f\");\n}\n",
    "a {\n  color: [c=d];\n}\n"
);
test!(
    simple_class_equal,
    "a {\n  color: selector-extend(\".c\", \".c\", \"e\");\n}\n",
    "a {\n  color: .c, e;\n}\n"
);
test!(
    simple_class_unequal,
    "a {\n  color: selector-extend(\".c\", \".d\", \"e\");\n}\n",
    "a {\n  color: .c;\n}\n"
);
test!(
    simple_id_equal,
    "a {\n  color: selector-extend(\"#c\", \"#c\", \"e\");\n}\n",
    "a {\n  color: #c, e;\n}\n"
);
test!(
    simple_id_unequal,
    "a {\n  color: selector-extend(\"#c\", \"#d\", \"e\");\n}\n",
    "a {\n  color: #c;\n}\n"
);
test!(
    simple_placeholder_equal,
    "a {\n  color: selector-extend(\"%c\", \"%c\", \"e\");\n}\n",
    "a {\n  color: %c, e;\n}\n"
);
test!(
    simple_placeholder_unequal,
    "a {\n  color: selector-extend(\"%c\", \"%d\", \"e\");\n}\n",
    "a {\n  color: %c;\n}\n"
);
test!(
    simple_type_equal,
    "a {\n  color: selector-extend(\"c\", \"c\", \"e\");\n}\n",
    "a {\n  color: c, e;\n}\n"
);
test!(
    simple_type_unequal,
    "a {\n  color: selector-extend(\"c\", \"d\", \"e\");\n}\n",
    "a {\n  color: c;\n}\n"
);
test!(
    simple_type_and_universal,
    "a {\n  color: selector-extend(\"c\", \"*\", \"d\");\n}\n",
    "a {\n  color: c;\n}\n"
);
test!(
    simple_type_explicit_namespace_and_type_explicit_namespace_equal,
    "a {\n  color: selector-extend(\"c|d\", \"c|d\", \"e\");\n}\n",
    "a {\n  color: c|d, e;\n}\n"
);
test!(
    simple_type_explicit_namespace_and_type_implicit_namespace,
    "a {\n  color: selector-extend(\"c|d\", \"d\", \"e\");\n}\n",
    "a {\n  color: c|d;\n}\n"
);
test!(
    simple_type_explicit_namespace_and_type_empty_namespace,
    "a {\n  color: selector-extend(\"c|d\", \"|d\", \"e\");\n}\n",
    "a {\n  color: c|d;\n}\n"
);
test!(
    simple_type_explicit_namespace_and_type_universal_namespace,
    "a {\n  color: selector-extend(\"c|d\", \"*|d\", \"e\");\n}\n",
    "a {\n  color: c|d;\n}\n"
);
test!(
    simple_type_empty_namespace_and_type_explicit_namespace_equal,
    "a {\n  color: selector-extend(\"|c\", \"d|c\", \"e\");\n}\n",
    "a {\n  color: |c;\n}\n"
);
test!(
    simple_type_empty_namespace_and_type_implicit_namespace,
    "a {\n  color: selector-extend(\"|c\", \"c\", \"d\");\n}\n",
    "a {\n  color: |c;\n}\n"
);
test!(
    simple_type_empty_namespace_and_type_empty_namespace,
    "a {\n  color: selector-extend(\"|c\", \"|c\", \"d\");\n}\n",
    "a {\n  color: |c, d;\n}\n"
);
test!(
    simple_type_empty_namespace_and_type_universal_namespace,
    "a {\n  color: selector-extend(\"|c\", \"*|c\", \"d\");\n}\n",
    "a {\n  color: |c;\n}\n"
);
test!(
    simple_type_universal_namespace_and_type_explicit_namespace_equal,
    "a {\n  color: selector-extend(\"*|c\", \"d|c\", \"d\");\n}\n",
    "a {\n  color: *|c;\n}\n"
);
test!(
    simple_type_universal_namespace_and_type_implicit_namespace,
    "a {\n  color: selector-extend(\"*|c\", \"c\", \"d\");\n}\n",
    "a {\n  color: *|c;\n}\n"
);
test!(
    simple_type_universal_namespace_and_type_empty_namespace,
    "a {\n  color: selector-extend(\"*|c\", \"|c\", \"d\");\n}\n",
    "a {\n  color: *|c;\n}\n"
);
test!(
    simple_type_universal_namespace_and_type_universal_namespace,
    "a {\n  color: selector-extend(\"*|c\", \"*|c\", \"d\");\n}\n",
    "a {\n  color: *|c, d;\n}\n"
);
test!(
    simple_pseudo_class_no_arg_equal,
    "a {\n  color: selector-extend(\":c\", \":c\", \"e\");\n}\n",
    "a {\n  color: :c, e;\n}\n"
);
test!(
    simple_pseudo_class_no_arg_unequal,
    "a {\n  color: selector-extend(\":c\", \":d\", \"e\");\n}\n",
    "a {\n  color: :c;\n}\n"
);
test!(
    simple_pseudo_class_no_arg_and_element,
    "a {\n  color: selector-extend(\":c\", \"::c\", \"e\");\n}\n",
    "a {\n  color: :c;\n}\n"
);
test!(
    simple_pseudo_element_no_arg_and_element_equal,
    "a {\n  color: selector-extend(\"::c\", \"::c\", \"e\");\n}\n",
    "a {\n  color: ::c, e;\n}\n"
);
test!(
    simple_pseudo_element_no_arg_and_class,
    "a {\n  color: selector-extend(\"::c\", \":c\", \"e\");\n}\n",
    "a {\n  color: ::c;\n}\n"
);
test!(
    simple_pseudo_class_arg_equal,
    "a {\n  color: selector-extend(\":c(@#$)\", \":c(@#$)\", \"e\");\n}\n",
    "a {\n  color: :c(@#$), e;\n}\n"
);
test!(
    simple_pseudo_class_arg_unequal_name,
    "a {\n  color: selector-extend(\":c(@#$)\", \":d(@#$)\", \"e\");\n}\n",
    "a {\n  color: :c(@#$);\n}\n"
);
test!(
    simple_pseudo_class_arg_unequal_arg,
    "a {\n  color: selector-extend(\":c(@#$)\", \":c(*&^)\", \"e\");\n}\n",
    "a {\n  color: :c(@#$);\n}\n"
);
test!(
    simple_pseudo_class_arg_unequal_no_arg,
    "a {\n  color: selector-extend(\":c(@#$)\", \":c\", \"e\");\n}\n",
    "a {\n  color: :c(@#$);\n}\n"
);
test!(
    simple_pseudo_class_arg_and_element,
    "a {\n  color: selector-extend(\":c(@#$)\", \"::c(@#$)\", \"e\");\n}\n",
    "a {\n  color: :c(@#$);\n}\n"
);
test!(
    simple_pseudo_element_arg_and_element_equal,
    "a {\n  color: selector-extend(\"::c(@#$)\", \"::c(@#$)\", \"e\");\n}\n",
    "a {\n  color: ::c(@#$), e;\n}\n"
);
test!(
    simple_pseudo_element_arg_and_class,
    "a {\n  color: selector-extend(\"::c(@#$)\", \":c(@#$)\", \"e\");\n}\n",
    "a {\n  color: ::c(@#$);\n}\n"
);
test!(
    complex_parent_without_grandparents_simple,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e\");\n}\n",
    "a {\n  color: .c .d, .e .d;\n}\n"
);
test!(
    complex_parent_without_grandparents_complex,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e .f\");\n}\n",
    "a {\n  color: .c .d, .e .f .d;\n}\n"
);
test!(
    complex_parent_without_grandparents_list,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e, .f\");\n}\n",
    "a {\n  color: .c .d, .e .d, .f .d;\n}\n"
);
test!(
    complex_parent_with_grandparents_simple,
    "a {\n  color: selector-extend(\".c .d .e\", \".d\", \".f\");\n}\n",
    "a {\n  color: .c .d .e, .c .f .e;\n}\n"
);
test!(
    complex_parent_with_grandparents_complex,
    "a {\n  color: selector-extend(\".c .d .e\", \".d\", \".f .g\");\n}\n",
    "a {\n  color: .c .d .e, .c .f .g .e, .f .c .g .e;\n}\n"
);
test!(
    complex_parent_with_grandparents_list,
    "a {\n  color: selector-extend(\".c .d .e\", \".d\", \".f, .g\");\n}\n",
    "a {\n  color: .c .d .e, .c .f .e, .c .g .e;\n}\n"
);
test!(
    complex_trailing_combinator_child,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e >\");\n}\n",
    "a {\n  color: .c .d, .e > .d;\n}\n"
);
test!(
    complex_trailing_combinator_sibling,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e ~\");\n}\n",
    "a {\n  color: .c .d, .e ~ .d;\n}\n"
);
test!(
    complex_trailing_combinator_next_sibling,
    "a {\n  color: selector-extend(\".c .d\", \".c\", \".e +\");\n}\n",
    "a {\n  color: .c .d, .e + .d;\n}\n"
);
test!(
    list_partial_no_op,
    "a {\n  color: selector-extend(\"c, d\", \"d\", \"e\");\n}\n",
    "a {\n  color: c, d, e;\n}\n"
);
// todo: https://github.com/sass/sass-spec/tree/master/spec/core_functions/selector/extend/simple/pseudo/selector/
