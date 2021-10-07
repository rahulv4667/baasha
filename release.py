from sys import stderr, stdout
import platform
import shutil
import os
import subprocess

VERSION = "0.0.1"
RELEASE_FOLDER = "baasha-releases"
FILE_PREFIX = "baasha-"+VERSION+"-"

def rust_target_build_path(target_triple: str):
    return os.path.relpath('../target/'+target_triple+'/release/baasha')

def build_target(target_triple: str):

    # adding target using `rustup`
    process = subprocess.Popen(
        ['rustup', 'target', 'add', target_triple],
        stderr = subprocess.PIPE
    )

    if process.wait() != 0:
        print('Error occured while trying to add support for `{}`'.format(target_triple))
        for line in process.stderr.readlines():
            print(line.__str__())
        return

    # building target
    process = subprocess.Popen(
        ['cargo', 'build', '--release', '--target', target_triple],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )

    # checking if there is any error.
    if process.wait() != 0:
        print('Error occured while trying to build for `{}`'.format(target_triple))
        for line in process.stderr.readlines():
            print(line.__str__())
        return

    # changing to releases directory
    os.chdir(RELEASE_FOLDER)

    # creating the release directory
    executable_file_path = rust_target_build_path(target_triple)   
    # print('Rust target build path: ', executable_file_path.__str__())
    target_build_folder = "baasha-"+target_triple
    if not os.path.exists(target_build_folder):
        os.mkdir(target_build_folder)

    # copying files to release directory
    shutil.copyfile(executable_file_path.__str__(), target_build_folder+"/baasha")
    shutil.copyfile("../runtime.c", target_build_folder+"/runtime.c")

    # making zip of release direcotry
    shutil.make_archive("baasha-"+target_triple, "zip", target_build_folder)

    os.chdir('..')






if __name__== "__main__":
    ostream = os.popen("rustc --print target-list")
    output = ostream.readlines()
    print('Total targets: ', len(output))
    
    
    # output = list(
    #     filter(
    #         lambda x : 
    #             x.__contains__('gnu'),
    #             # x.__contains__('apple') or 
    #             # x.__contains__('linux') or 
    #             # x.__contains__('windows'),
    #             # x.__contains__('netbsd') or
    #             # x.__contains__('freebsd'), 
    #         output
    #     )
    # )

    ##########################
    # output = [
    #     "x86_64-unknown-linux-gnu",
    #     # "x86_64-pc-linux-gnu"
    #     "x86_64-"
    # ]

    # ##### Tier 1 #####
    # output = [
    #     "aarch64-unknown-linux-gnu",
    #     "i686-pc-windows-gnu",
    #     "i686-pc-windows-msvc",
    #     "i686-unknown-linux-gnu",
    #     "x86_64-apple-darwin",
    #     "x86_64-pc-windows-gnu",
    #     "x86_64-pc-windows-msvc",
    #     "x86_64-unknown-linux-gnu"
    # ]
    
    output = [ 
        "x86_64-unknown-linux-gnu"
    ]
    
    print('Total targets: ', len(output))

    if not os.path.exists(RELEASE_FOLDER):
        os.mkdir(RELEASE_FOLDER)
    output = output[:10]

    for i, out in enumerate(output):
        out = out.strip('\n')
        print('Building target `{}`'.format(out))
        build_target(out)
        print('Build complete for target `{}`'.format(out))
    # os.system("cargo rustc --print target-list")
