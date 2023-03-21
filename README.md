# GeographicLib PHP extension

A bridge between [GeographicLib library](https://geographiclib.sourceforge.io/) in C and a corresponding PHP extension.

This project aims to port the [GeographicLib library](https://geographiclib.sourceforge.io/) to PHP using Rust as an extension building language.

> **Note:** This is, for now, not a total port of GeographicLib into PHP, but just the geodesic polygon area calculation using the C function `geod_polygon_compute()`.
>
> The full port will be available soon.

## The process
1. The build script *(see [build.rs](./build.rs))* will build the C library into a archive using `clang` and `ar` to forge an `.a` archive.
2. Then, the build will generate a binding of the C code for Rust using `bindgen`.
2. Finally, The rust library will be compiled used middleware code. *(see [lib.rs](./src/lib.rs) and [geographiclib.rs](./src/external/geographiclib.rs))*

## Usage
You can use the included makefile to manipulate this project.

Here is a description of the make targets :
| target | description |
|--------|-------------|
| `make` | will process unit tests and then build the project for release. |
| `make build` | will build the extension in `debug` mode. |
| `make release` | will build the extension in `release` mode. |
| `make test` | will start the whole unit testing. |
| `make test-extension` | will compile the project and use the compiled `.so` extension and a test script to validate that the extension is valid. |
| `make install` | will use `cargo-php` to install the compiled extension into your curent PHP. |
| `make install` | will use `cargo-php` to uninstall a prebious compiled extension from your curent PHP. |
| `make stubs` | will use `cargo-php` to generate PHP code stubs *(i.e. autocompletion skeleton)* for your IDE. |
| `make clean` | will clean all build files. |

## Live testing & installation
You can test the compiled extension using `make test-extension`.

Alternatively, you can add the compiled extension to your PHP ini.

To find your `php.ini` file :
```bash
php -i | grep 'Configuration File'
```

Then to install the extension
```bash
echo "extension=/absolute/path/to/libgeographiclib_php_extension.so" >> "/path/to/your/php.ini"
```