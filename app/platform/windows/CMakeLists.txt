cmake_minimum_required(VERSION 3.20)
project(BanglaKeyboard VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Rust engine library
set(RUST_TARGET_DIR "${CMAKE_SOURCE_DIR}/../../target")
# Multi-config generators (Visual Studio) leave CMAKE_BUILD_TYPE empty; pick the
# Rust profile dir from the build-time $<CONFIG> instead so Release/Debug both work.
set(RUST_LIB_DIR "${RUST_TARGET_DIR}/$<IF:$<CONFIG:Debug>,debug,release>")

# Locate cargo up-front so custom commands don't rely on MSBuild's inherited PATH.
find_program(CARGO_EXECUTABLE cargo REQUIRED)

# Build Rust engine
add_custom_command(
    OUTPUT "${RUST_LIB_DIR}/engine_ffi.lib"
    COMMAND "${CARGO_EXECUTABLE}" build -p engine-ffi $<$<NOT:$<CONFIG:Debug>>:--release>
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/../.."
    COMMENT "Building Rust engine"
)
add_custom_target(rust_engine DEPENDS "${RUST_LIB_DIR}/engine_ffi.lib")

# TSF DLL
add_library(BanglaKeyboardTSF SHARED
    src/dll_main.cpp
    src/class_factory.cpp
    src/class_factory.h
    src/text_service.cpp
    src/text_service.h
    src/key_event_sink.cpp
    src/composition.cpp
    src/edit_session.cpp
    src/edit_session.h
    src/globals.cpp
    src/globals.h
    src/register.cpp
    src/register.h
    src/bangla_keyboard.def
)

target_include_directories(BanglaKeyboardTSF PRIVATE
    src
    "${CMAKE_SOURCE_DIR}/../../crates/engine-ffi"
)

target_link_directories(BanglaKeyboardTSF PRIVATE "${RUST_LIB_DIR}")
target_link_libraries(BanglaKeyboardTSF PRIVATE
    engine_ffi
    ole32
    oleaut32
    uuid
    advapi32
    userenv
    ws2_32
    bcrypt
    ntdll
)

add_dependencies(BanglaKeyboardTSF rust_engine)

set_target_properties(BanglaKeyboardTSF PROPERTIES
    OUTPUT_NAME "BanglaKeyboard"
    SUFFIX ".dll"
)

# Copy data files post-build
add_custom_command(TARGET BanglaKeyboardTSF POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        "${CMAKE_SOURCE_DIR}/../../data"
        "$<TARGET_FILE_DIR:BanglaKeyboardTSF>/data"
    COMMENT "Copying data files"
)
