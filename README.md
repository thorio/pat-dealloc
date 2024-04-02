# pat-dealloc

![GitHub License](https://img.shields.io/github/license/thorio/pat-dealloc?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/thorio/pat-dealloc?style=flat-square)

Very much WIP, intended as a dodgy workaround for this issue: https://gitlab.freedesktop.org/drm/amd/-/issues/2794

Also my first kernel module and first (sort of) useful thing written in C.

## Usage

`cargo build` to compile the userspace helper. `./target/debug/pat-dealloc` as root to run it.

`cd` into `module`, run `make load` to compile and load the kernel module. It will ask for root to load the module.

Either use the helper (`--help`) or alternatively `echo -n 0x000000f81c650000-0x000000f81c651000 > /dev/pat_dealloc` as root.
The kernel module **DOES NOT CARE** what you put here, as long as it parses it will try to `memtype_free` whatever address range you give it, even if that memory is still in use by a driver. **USE WITH CAUTION**.

Use the helper to free all memtypes for a particular PCI device.
