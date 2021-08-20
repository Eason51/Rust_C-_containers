Each single rust experiment is implemented in /src/bin, the corresponding release binary executables are generated in /target/release, and they can be directly executed.

Each set of c++ container experiments are implemented in /cppSrc/, the Makefile in each folder can generate the corresponding executable from source files. 

The executable in /cppSrc/unorderedMapRead/ needs to be linked with boost::timer and boost::chrono library files.

All experiments statistics are recorded in /data
