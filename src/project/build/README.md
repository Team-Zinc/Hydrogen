# Build
This is where the magic happens. Inside the `configure` directory is a collection of commonly used (across-language) configuration stuff.

Inside `link` are functions used for calling the linker and the archiver.

Inside `langs` are language directories (like `c` and `cpp`) which then in turn have compiler backends inside of them (such as `gcc`, `g++`, and `clang`).  

So far, the only languages implemented are `c` and `c++`, and the compilers `gcc` and `g++`. However, I want to support `objective-c`, `objective-c++` `fortran`, and `d` in the future.