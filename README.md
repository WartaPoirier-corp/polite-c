# polite-c ![Build status](https://github.com/WartaPoirier-corp/polite-c/workflows/Main%20workflow/badge.svg) [![codecov coverage](https://codecov.io/gh/WartaPoirier-corp/polite-c/branch/main/graph/badge.svg)](https://codecov.io/gh/WartaPoirier-corp/polite-c)

## Installation

### All platforms

Make sure to have the CLang libraries (version 10) installed before

```bash
cargo install --git https://github.com/Wartapoirier-corp/polite-c
```

Or git clone and run `cargo install`

### Linux (Debian based)

Download the latest Linux build from the "Actions" tab, unzip it,
and run

```
sudo apt install ./polite-c_0.1.0_amd64.deb
```

### Linux (other distros)

Install the CLang librairies (`libclang` or something like that), version 10.

Download the latest Linux build, unzip it, and run

```
sudo install polite-c /usr/bin
```

### Windows

Install the LLVM stack (which includes CLang) version 10.0.0 from [this page](https://releases.llvm.org/download.html).
The link is called "Windows (64-bit)".

Then download the latest Windows build from the "Actions" tab,
unzip it and run the `.msi` installer or copy `polite-c.exe`
in a folder that is in your `PATH`.

### Mac OS X

Not 100% how to install CLang, but probably install the XCode developer
tools.

Then download the latest Mac OS X build from the "Actions" tab, unzip it
and copy the `polite-c` file to `/usr/bin`, eventually with `sudo install polite-c /usr/bin`.
