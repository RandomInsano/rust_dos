# Rust DOS (Creating a DOS executable with Rust)

It is possible to create a DOS executable or 1st stage bootloader with Rust.  
This is a quick demo of creating COM executable for DOS.

Currently implemented DOS INT 21 functions:

| Code | Description                        | Code | Description
|------|------------------------------------|------|------------------------------------|
| 01 ✓ | Read character from STDIN          | 35   | Get interrupt vector               |
| 02 ✓ | Write character to STDOUT          | 36   | Get free disk space                |
| 05   | Write character to printer         | 39 ✓ | Create subdirectory                |
| 06   | Console Input/Output               | 3A ✓ | Remove subdirectory                |
| 07   | Direct character read, no echo     | 3B ✓ | Change current working directory   |
| 08   | Char read from STDIN, no echo      | 3C   | Create file                        |
| 09 ✓ | Write string to STDOUT             | 3D ~ | Open file                          |
| 0A   | Buffered input                     | 3E ✓ | Close file                         |
| 0B   | Get STDIN status                   | 3F ✓ | Read file                          |
| 0C   | Flush buffer from STDIN            | 40 ✓ | Write file                         |
| 0D   | Disk reset                         | 41   | Delete file                        |
| 0E   | Select default drive               | 42 ✓ | Seek file                          |
| 19   | Get current default drive          | 43 ~ | Get/set file attributes            |
| 25   | Set interrupt vector               | 47   | Get current directory              |
| 2A ✓ | Get system date                    | 4C ✓ | Exit program                       |
| 2B ✓ | Set system date                    | 4D   | Get return code                    |
| 2C ✓ | Get system time                    | 54   | Get verify flag                    |
| 2D ✓ | Set system time                    | 56   | Rename file                        |
| 2E   | Set verify flag                    | 57 ~ | Get/set file date                  |
| 30 ✓ | Get DOS version                    |      |                                    |

Legend:
✓ = All features implemented
~ = Partial features implemented

Reference material:
* [DOS INT 21h - DOS Function Codes](http://spike.scu.edu.au/~barry/interrupts.html#ah36)
* [Registers in x86 Assembly](https://www.cs.uaf.edu/2017/fall/cs301/lecture/09_11_registers.html)


## Building
You need a binutils and llvm-tools-preview.

```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Then you can build the project by running:

```shell
cargo build --release
```

To create a COM executable for DOS, run:

```shell
cargo objcopy --release -- -O binary --binary-architecture=i386:x86 rust_dos.com
```

## Running

### QEMU

You can copy `rust_dos.com` to your DOS image.

examples on Linux

```shell
$ sudo partx -av freedos.img
partition: none, disk: freedos.img, lower: 0, upper: 0
Trying to use '/dev/loop1' for the loop device
/dev/loop1: partition table type 'dos' detected
range recount: max partno=1, lower=0, upper=0
/dev/loop1: partition #1 added
$ sudo mount /dev/loop1p1 /mnt
$ sudo cp rust_dos.com /mnt/
$ sudo umount /mnt
$ sudo partx -dv /dev/loop1
```

Then, you can test it using QEMU:

```shell
qemu-system-i386 freedos.img -boot c
```

You can use the `println!` macro. 
Below is an example of HelloWorld:

![sample](https://github.com/o8vm/rust_dos/blob/images/rust_dos_hello.png)

### DOSBox

First install DOSBox. Some examples if you like using package managers:

#### Debian / Ubuntu

```
sudo apt install dosbox
```

#### macOS (Homebrew)

```
brew install dosbox
```

#### Windows (Chocolatey)

```
choco install dosbox
```

Once installed, you can launch DOSBox and give it the path to your executable. For example, you can just give it the current working directory like the following:

```
dosbox .
```

And this will open DOSBox and have the "C:\" drive be the current working directory. It's usually good to do this from another console so you don't have to close DOSBox every time you want to compile your application again.

### Others
dpkey module steals key input processing from DOS and converts scan code to ascii code.  
about scan code: see [PS/2 Keyboard - OSDev Wiki](https://wiki.osdev.org/PS/2_Keyboard).

![sample2](https://github.com/o8vm/rust_dos/blob/images/dpkey.gif)
