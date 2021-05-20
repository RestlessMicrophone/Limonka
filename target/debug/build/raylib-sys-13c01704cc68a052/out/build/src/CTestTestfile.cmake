# CMake generated Testfile for 
# Source directory: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src
# Build directory: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(pkg-config--static "/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/../cmake/test-pkgconfig.sh" "--static")
set_tests_properties(pkg-config--static PROPERTIES  _BACKTRACE_TRIPLES "/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/CMakeLists.txt;212;add_test;/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/CMakeLists.txt;0;")
subdirs("external/glfw")
