// `without_integer_time_returns_badarg` in unit tests
// `with_integer_time_without_unit_from_unit_errors_badarg` in unit tests
// `with_integer_time_with_unit_from_unit_without_unit_to_unit_errors_badarg` in unit tests
test_stdout!(with_small_integer_time_valid_units_returns_converted_value, "true\n2000000000\n500000000\n500000000000\n500000000000000000\n500000000000\n500000000000\n5000000000\n1000000000\n1000000000000\n1000000000000000000\n1000000000000\n1000000000000\n5000000\n1000000\n1000000000\n1000000000000000\n1000000000\n1000000000\n5000\n1000\n1000000\n1000000000000\n1000000\n1000000\n5\n1\n1000\n1000000000\n1000\n1000\n5000000\n1000000\n1000000000\n1000000000000000\n1000000000\n1000000000\n5000000\n1000000\n1000000000\n1000000000000000\n1000000000\n1000000000\n");
test_stdout!(with_big_integer_time_with_unit_from_unit_with_unit_to_unit_returns_converted_value, "true\n2000000000000000000\n500000000000000000\n500000000000000000000\n500000000000000000000000000\n500000000000000000000\n500000000000000000000\n5000000000000000000\n1000000000000000000\n1000000000000000000000\n1000000000000000000000000000\n1000000000000000000000\n1000000000000000000000\n5000000000000000\n1000000000000000\n1000000000000000000\n1000000000000000000000000\n1000000000000000000\n1000000000000000000\n5000000000000\n1000000000000\n1000000000000000\n1000000000000000000000\n1000000000000000\n1000000000000000\n5000000000\n1000000000\n1000000000000\n1000000000000000000\n1000000000000\n1000000000000\n5000000000000000\n1000000000000000\n1000000000000000000\n1000000000000000000000000\n1000000000000000000\n1000000000000000000\n5000000000000000\n1000000000000000\n1000000000000000000\n1000000000000000000000000\n");
