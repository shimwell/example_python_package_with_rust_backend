import example_python_package_with_rust_backend

def test_retunred_value():
    returned_value = example_python_package_with_rust_backend.hello_world(
        int_arg=3,
        float_arg=2.01,
        str_arg='hi'
    )
    assert returned_value == 9