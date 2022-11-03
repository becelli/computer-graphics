import os
import sys


class OS:
    def __init__(self):
        self.rust_build = None
        self.copy_lib = None
        self.execute_python = None
        self.compile_python = None

    def lib_build(self):
        os.system(self.rust_build)
        os.system(self.copy_lib)

    def dev_build(self):
        os.system(self.execute_python)

    def release_build(self):
        os.system(self.rust_build)
        os.system(self.copy_lib)
        os.system(self.compile_python)

    def install_deps(self):
        print("Try to install dependencies...")
        print("python -m pip install -r requirements.txt")
        try:
            os.system("python -m pip install -r requirements.txt")
        except:
            print("Failed to install dependencies")
            print("Please install dependencies manually")
            sys.exit(1)


class Linux(OS):
    def __init__(self):
        self.rust_build = "cargo build --release --manifest-path=./library/Cargo.toml"
        self.copy_lib = "cp ./library/target/release/libcglib.so ./cglib.so"
        self.execute_python = "python3 main.pyw"
        self.compile_python = "nuitka3 main.pyw --follow-imports"


class Windows(OS):
    def __init__(self):
        self.rust_build = "cargo build --release --manifest-path=./library/Cargo.toml"
        self.copy_lib = "copy library\\target\\release\\cglib.dll cglib.pyd"
        self.execute_python = "python .\main.pyw"
        self.compile_python = "nuitka .\main.pyw --follow-imports"


class Mac(OS):
    def __init__(self):
        self.rust_build = "cargo build --release --manifest-path=./library/Cargo.toml"
        self.copy_lib = "cp ./library/target/release/libcglib.dylib ./cglib.so"
        self.execute_python = "python3 main.pyw"
        self.compile_python = "nuitka3 main.pyw --follow-imports"


if __name__ == "__main__":
    # Recognize the OS
    system = sys.platform
    if sys.platform == "linux":
        system = Linux()
    elif sys.platform == "win32":
        system = Windows()
    elif sys.platform == "darwin":
        system = Mac()
    else:
        print("Unsupported OS")
        sys.exit(1)

    if len(sys.argv) > 1:
        if sys.argv[1] == "dev":
            system.dev_build()
        if sys.argv[1] == "devlib":
            system.lib_build()
            system.dev_build()

        if sys.argv[1] == "lib":
            system.lib_build()

        elif sys.argv[1] == "release":
            system.release_build()
        elif sys.argv[1] == "install":
            system.install_deps()
        else:
            print("Unsupported argument")
            sys.exit(1)
    else:
        print("No argument given. Available arguments: dev, release")
        sys.exit(1)
