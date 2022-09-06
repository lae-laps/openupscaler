# OpenUpscaler
OpenUpscaler is a free and open-source image manipulation software. Its main purpose is increasing the quality of images through different methods. The project is provided with a GUI frontend written in [rust](https://www.rust-lang.org/es), but most of the backend modules are written and provided as command line utilities written in python with [OpenCV](https://opencv.org/). The algorithmic filters and transformations are however written in rust for concurrency and speed purposes.

# Installation

## Linux

Install the [python interpreter](https://www.python.org/downloads/)
Install a [rust toolchain](https://www.rust-lang.org/learn/get-started)
Clone the repository

```git clone https://github.com/lae-laps/openupscaler```

Run the Makefile installer inside the directory

```cd openupscaler && make install```

## Mac-OS

For the time being, linux build instructions are aplicable in Mac-OS

## Windows

As this is a project in development, there is still no support for windows. However, pre-release binaries will be provided soon, and for the time being the project can be compiled from source.

