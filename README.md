### Interoperability - Calling Rust code from a C program

1. Create Rust code like the example addition program.

2. Write a C program which uses a function from the shared library.

3. 
```
$ gcc -L./addition/target/debug -llibaddition main.c -o main
$ LD_LIBRARY_PATH=./addition/target/debug ./main
```
