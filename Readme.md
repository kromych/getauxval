# Get the auxialiary vector on Linux

Shows how to read it without using `/proc/pid/auxv` (e.g. the proc FS might be not mounted just yet).
The `glibc` and `musl` both define the `__environ` symbol which allows to find the auxiliary vector
in the memory.

The glibc loader can print the auxv data to see if they match up with what this code prints:

```sh
LD_SHOW_AUXV=1 target/debug/getauxval 
```

```text
AT_SYSINFO_EHDR:      0x7ffe8b563000
AT_MINSIGSTKSZ:       2032
AT_HWCAP:             bfebfbff
AT_PAGESZ:            4096
AT_CLKTCK:            100
AT_PHDR:              0x55d8f6e37040
AT_PHENT:             56
AT_PHNUM:             14
AT_BASE:              0x7fb037e51000
AT_FLAGS:             0x0
AT_ENTRY:             0x55d8f6e3f8c0
AT_UID:               1000
AT_EUID:              1000
AT_GID:               1000
AT_EGID:              1000
AT_SECURE:            0
AT_RANDOM:            0x7ffe8b4906f9
AT_HWCAP2:            0x2
AT_EXECFN:            target/debug/getauxval
AT_PLATFORM:          x86_64

Hello, world!
env address 0x7ffe8b490478

0x21 = 0x7ffe8b563000
0x33 = 0x7f0
0x10 = 0xbfebfbff
0x6 = 0x1000
0x11 = 0x64
0x3 = 0x55d8f6e37040
0x4 = 0x38
0x5 = 0xe
0x7 = 0x7fb037e51000
0x8 = 0x0
0x9 = 0x55d8f6e3f8c0
0xb = 0x3e8
0xc = 0x3e8
0xd = 0x3e8
0xe = 0x3e8
0x17 = 0x0
0x19 = 0x7ffe8b4906f9
0x1a = 0x2
0x1f = 0x7ffe8b492fe1
0xf = 0x7ffe8b490709
```
