# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.16

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build

# Include any dependencies generated for this target.
include src/CMakeFiles/raylib_static.dir/depend.make

# Include the progress variables for this target.
include src/CMakeFiles/raylib_static.dir/progress.make

# Include the compile flags for this target's objects.
include src/CMakeFiles/raylib_static.dir/flags.make

src/CMakeFiles/raylib_static.dir/core.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/core.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/core.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object src/CMakeFiles/raylib_static.dir/core.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/core.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/core.c

src/CMakeFiles/raylib_static.dir/core.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/core.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/core.c > CMakeFiles/raylib_static.dir/core.c.i

src/CMakeFiles/raylib_static.dir/core.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/core.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/core.c -o CMakeFiles/raylib_static.dir/core.c.s

src/CMakeFiles/raylib_static.dir/models.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/models.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/models.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building C object src/CMakeFiles/raylib_static.dir/models.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/models.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/models.c

src/CMakeFiles/raylib_static.dir/models.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/models.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/models.c > CMakeFiles/raylib_static.dir/models.c.i

src/CMakeFiles/raylib_static.dir/models.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/models.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/models.c -o CMakeFiles/raylib_static.dir/models.c.s

src/CMakeFiles/raylib_static.dir/raudio.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/raudio.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/raudio.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building C object src/CMakeFiles/raylib_static.dir/raudio.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/raudio.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/raudio.c

src/CMakeFiles/raylib_static.dir/raudio.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/raudio.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/raudio.c > CMakeFiles/raylib_static.dir/raudio.c.i

src/CMakeFiles/raylib_static.dir/raudio.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/raudio.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/raudio.c -o CMakeFiles/raylib_static.dir/raudio.c.s

src/CMakeFiles/raylib_static.dir/shapes.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/shapes.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/shapes.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building C object src/CMakeFiles/raylib_static.dir/shapes.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/shapes.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/shapes.c

src/CMakeFiles/raylib_static.dir/shapes.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/shapes.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/shapes.c > CMakeFiles/raylib_static.dir/shapes.c.i

src/CMakeFiles/raylib_static.dir/shapes.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/shapes.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/shapes.c -o CMakeFiles/raylib_static.dir/shapes.c.s

src/CMakeFiles/raylib_static.dir/text.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/text.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/text.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building C object src/CMakeFiles/raylib_static.dir/text.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/text.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/text.c

src/CMakeFiles/raylib_static.dir/text.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/text.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/text.c > CMakeFiles/raylib_static.dir/text.c.i

src/CMakeFiles/raylib_static.dir/text.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/text.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/text.c -o CMakeFiles/raylib_static.dir/text.c.s

src/CMakeFiles/raylib_static.dir/textures.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/textures.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/textures.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Building C object src/CMakeFiles/raylib_static.dir/textures.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/textures.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/textures.c

src/CMakeFiles/raylib_static.dir/textures.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/textures.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/textures.c > CMakeFiles/raylib_static.dir/textures.c.i

src/CMakeFiles/raylib_static.dir/textures.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/textures.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/textures.c -o CMakeFiles/raylib_static.dir/textures.c.s

src/CMakeFiles/raylib_static.dir/utils.c.o: src/CMakeFiles/raylib_static.dir/flags.make
src/CMakeFiles/raylib_static.dir/utils.c.o: /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/utils.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_7) "Building C object src/CMakeFiles/raylib_static.dir/utils.c.o"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/raylib_static.dir/utils.c.o   -c /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/utils.c

src/CMakeFiles/raylib_static.dir/utils.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/raylib_static.dir/utils.c.i"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/utils.c > CMakeFiles/raylib_static.dir/utils.c.i

src/CMakeFiles/raylib_static.dir/utils.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/raylib_static.dir/utils.c.s"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && /usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src/utils.c -o CMakeFiles/raylib_static.dir/utils.c.s

# Object files for target raylib_static
raylib_static_OBJECTS = \
"CMakeFiles/raylib_static.dir/core.c.o" \
"CMakeFiles/raylib_static.dir/models.c.o" \
"CMakeFiles/raylib_static.dir/raudio.c.o" \
"CMakeFiles/raylib_static.dir/shapes.c.o" \
"CMakeFiles/raylib_static.dir/text.c.o" \
"CMakeFiles/raylib_static.dir/textures.c.o" \
"CMakeFiles/raylib_static.dir/utils.c.o"

# External object files for target raylib_static
raylib_static_EXTERNAL_OBJECTS = \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/context.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/init.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/input.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/monitor.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/vulkan.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/window.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_init.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_monitor.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_window.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/xkb_unicode.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_time.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_thread.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/glx_context.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/egl_context.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/osmesa_context.c.o" \
"/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/external/glfw/src/CMakeFiles/glfw_objlib.dir/linux_joystick.c.o"

src/libraylib.a: src/CMakeFiles/raylib_static.dir/core.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/models.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/raudio.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/shapes.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/text.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/textures.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/utils.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/context.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/init.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/input.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/monitor.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/vulkan.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/window.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_init.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_monitor.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/x11_window.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/xkb_unicode.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_time.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/posix_thread.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/glx_context.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/egl_context.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/osmesa_context.c.o
src/libraylib.a: src/external/glfw/src/CMakeFiles/glfw_objlib.dir/linux_joystick.c.o
src/libraylib.a: src/CMakeFiles/raylib_static.dir/build.make
src/libraylib.a: src/CMakeFiles/raylib_static.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_8) "Linking C static library libraylib.a"
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && $(CMAKE_COMMAND) -P CMakeFiles/raylib_static.dir/cmake_clean_target.cmake
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/raylib_static.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
src/CMakeFiles/raylib_static.dir/build: src/libraylib.a

.PHONY : src/CMakeFiles/raylib_static.dir/build

src/CMakeFiles/raylib_static.dir/clean:
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src && $(CMAKE_COMMAND) -P CMakeFiles/raylib_static.dir/cmake_clean.cmake
.PHONY : src/CMakeFiles/raylib_static.dir/clean

src/CMakeFiles/raylib_static.dir/depend:
	cd /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/raylib/src /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src /home/wjanus/Documents/Limonka/target/debug/build/raylib-sys-13c01704cc68a052/out/build/src/CMakeFiles/raylib_static.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : src/CMakeFiles/raylib_static.dir/depend

