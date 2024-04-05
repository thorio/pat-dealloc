# pat-dealloc

![GitHub License](https://img.shields.io/github/license/thorio/pat-dealloc?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/thorio/pat-dealloc?style=flat-square)
[![AUR Version](https://img.shields.io/aur/version/pat-dealloc-dkms?style=flat-square)](https://aur.archlinux.org/packages/pat-dealloc-dkms)

A kernel module and helper utility to remove PAT entries. May cause issues if used incorrectly, so proceed with caution.  
Intended as a less dodgy workaround for this issue: https://gitlab.freedesktop.org/drm/amd/-/issues/2794

Also my first kernel module and first (sort of) useful thing written in C.

## Usage

### WARNING
The kernel module **DOES NOT CARE** what you give it: as long as it parses it will try to `memtype_free` the address range(s) you supply, even if that memory is still in use by a driver. Doing this while anything is actively using that memory *might* result in memory corruption. **USE AT YOUR OWN RISK**.

Best to unload/stop relevant drivers before running this.

### Standalone
Write a string like this `0x000000f81c650000-0x000000f81c651000` to `/dev/pat_dealloc` in a single write call, without a newline. The module will stupidly attempt to free the given range, check kernel logs.

### With the helper
Run `pat-dealloc raw --start 0x000000f81c650000 --end 0x000000f81c651000` to accomplish the same as the example above.

Alternatively, run `pat-dealloc pci --address 0000:01:00.0` to free all reserved PAT entries for the given PCI device.

You can append `--load` to either of these to automatically load the kernel module for you.

## Building

You will need rust set up and linux headers installed. The makefile was written for arch and might need tweaking on other distros (PRs welcome).  
You will also need to re-compile the module for every new kernel you run, which can be automated with [dkms](https://wiki.archlinux.org/title/Dynamic_Kernel_Module_Support) or similar.

`cargo build` to compile the userspace helper. `./target/debug/pat-dealloc` as root to run it.

`cd` into `module` and run `make` to compile the kernel module. Then `insmod pat_dealloc.ko` as root.
