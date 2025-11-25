cmake_minimum_required(VERSION 3.10)
project(MyRustProject LANGUAGES NONE)

# Tell CMake how to build using cargo
add_custom_target(Main ALL
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# Optional: Install the compiled binary
install(PROGRAMS
    ${CMAKE_CURRENT_SOURCE_DIR}/target/release/Main
    DESTINATION bin
)
