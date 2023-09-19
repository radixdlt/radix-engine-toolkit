import platform
import ctypes
import sys
import os
import re

def _uniffi_load_indirect() -> ctypes.CDLL:
    """
    This is how we find and load the dynamic library provided by the component.
    The dynamic library is assumed to exist right beside the code and it's name if the same as the 
    target triple.

    Currently, the supported architectures are:
    * x86-64 and Arm64 Apple Darwin
    * x86-64 and Arm64 Linux GNU
    * x86-64 Win64
    """

    def library_file_name() -> str:
        is_x86: bool = platform.machine() in ("AMD64", "x86_64")
        is_arm: bool = platform.machine() == "arm64"
        system: str = platform.system()

        if is_x86 and system == "Darwin":
            return "x86_64-apple-darwin"
        elif is_arm and system == "Darwin":
            return "aarch64-apple-darwin"
        elif is_x86 and system == "Linux":
            return "x86_64-unknown-linux-gnu"
        elif is_arm and system == "Linux":
            return "aarch64-unknown-linux-gnu"
        elif is_x86 and system == "Windows":
            return "x86_64-pc-windows-gnu"
        else:
            raise NotImplemented(f"No implementation of the Radix Engine Toolkit is available on your platform. Information detected: is_x86: {is_x86}, is_arm: {is_arm}, os: {system}")

    file_name: str = library_file_name()
    path: str = os.path.join(os.path.dirname(__file__), file_name)
    return ctypes.cdll.LoadLibrary(path)

def main() -> None:
    # The first arg is the path of the file which we want to do the replacement for.
    path: str = sys.argv[1]
    
    # The regex expression used to read this function
    regex: str = r'(def _uniffi_load_indirect\(\)\s*(->\s*.*)?:[\d\w\s\n:{}.\[\]=\(\)#,$`\'\"\-*\/><]*^$)'

    # Open THIS file and read the function definition through the regex expression.
    with open(os.path.abspath(__file__), 'r') as file:
        new_func_def: str = re.findall(regex, file.read(), re.MULTILINE)[0][0]

    # Open the replacement file, read it, apply regex replacement to it, and then write it again.
    with open(path, 'r') as file:
        content: str = file.read()
        old_func_def: str = re.findall(regex, content, re.MULTILINE)[0][0]

    new_content: str = content.replace(old_func_def, new_func_def)
    with open(path, 'w') as file:
        file.write(new_content)

if __name__ == "__main__":
    main()