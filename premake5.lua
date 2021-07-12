workspace "baasha"
    configurations {"Debug", "Release"}

project "baasha"
    kind "ConsoleApp"
    language { "C", "C++" }
    targetdir "bin/%{cfg.buildcfg}"

    includedirs { 
        "/usr/bin/llvm-12/**", 
        "/usr/include/llvm-c-12/**", 
        "/usr/include/clang/12/include/**" }

    files { "**.hpp", "**.cpp" }

    buildcommands {
        "clang++-12 -g -O3 -Xlinker --export-dynamic main.cpp `llvm-config-12 --cxxflags --ldflags --system-libs --libs core` -o baasha"
    }

    filter "configurations:Debug"
        defines { "DEBUG_VERSION", "IS_DEBUG" }
        symbols "On"

    filter "configurations:Release"
        defines { "RELEASE_VERSION", "IS_RELEASE" }
        optimize "On"