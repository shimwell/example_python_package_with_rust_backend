[![PyPI](https://img.shields.io/pypi/v/example_python_package_with_rust_backend?&label=PyPI)](https://pypi.org/project/example_python_package_with_rust_backend/)
 


A minimal example of a :snake: Python package with a :crab: Rust backend

One of the great aspects of Rust is that calling Rust functions from Python is easy thanks to [maturin](https://github.com/PyO3/maturin)

This repository also includes automated creation and uploading of wheels to PyPI for distribution to Windows :window: , Mac :green_apple: and Linux :penguin: thanks to [maturin-action](https://github.com/PyO3/maturin-action) and [maturin](https://github.com/PyO3/maturin) for automatically generating the CI yaml.

The package itself is less import and just prints a few messages to the terminal.

## Install the package with pip

```bash
pip install example_python_package_with_rust_backend
```

## Usage
```
import example_python_package_with_rust_backend

returned_value = example_python_package_with_rust_backend.hello_world(
    int_arg=3,
    float_arg=2.01,
    str_arg='hi'
)
print(returned_value)
```

## Testing

Testing in python with pytest, first install the package with testing dependencies

```bash
pip install example_python_package_with_rust_backend [tests]
```

Then test with pytest

```
pytest tests
```