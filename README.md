# alinux

> modern linux distribution, focused around the azul.rs GUI toolkit

<p>
  <img src="https://img.shields.io/badge/kernel-6.12.1-blue" />
  <img src="https://img.shields.io/badge/init-custom-green" />
  <img src="https://img.shields.io/badge/compositor-wayland-purple" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" />
</p>

**alinux** (Azul Linux) is a from-scratch Linux distribution built for performance and simplicity. 
Inspired by the Azul.rs GUI framework for GPU-accelerated rendering and a custom layout engine, alinux 
ships mostly with Rust-based applications and tools. The core "operating system" consists of:

- User-friendly installer: introibo
- Login manager / Greeter: alogin
- Wayland Compositor: acomp
- Desktop Environment: ade
- Package manager: apkg (can install packages from AUR, snap, flatpak, etc.)
- A settings application to manage various settings

The idea is to not make a "Server OS" or "OS for corporate installations" like RHEL or OpenSuse or 
whatever, but just make a simple "User OS", that is:

- Core Kernel with drivers, compiled for performance instead of old hardware support
- Pre-installed graphics drivers, media codecs, etc. - Steam should work out-of-the-box
- Preinstalled useful CLI utilities (mostly C/Rust utilities)
- Preinstalled useful GUI utilities (mostly using the Azul GUI toolkit or KDE applications)
- Package manager that can install + update from MANY different sources, but doesn't need deployment functionality (no uploading)
  - This is so that it becomes irrelevant whether a software is available as a pure binary
  - For source-available software like Python libraries, etc. the package manager will either automatically build-on-install with docker or install the source inside of a system-defined directory
  - The window manager then picks up on these applications in the standard directories
  - Yes, this breaks any "security" advancements of flatpak / snap, but adds more possbilities to install software

The scope of the "project" is therefore less to be some big distro and more to be a "template" 
to "build your own Linux distribution by **forking** this repository". The entire ISO gets built
by GitHub Actions, which makes deployment very easy.

This repo therefore hold all the "GUI utilities" that will need to be developed specifically for this
operating system. Otherwise, the build.yml script just clones the other software and builds it from source (with caching).

The system ships with git, gcc, cmake and other development tools out of the box. However, the goal is to not bloat the
system too much, to keep it under 500MB for the final ISO, in order to make it possible to use as a "testing OS"

## Installation

### QEMU 

```bash
wget https://github.com/fschutt/alinux/releases/latest/download/alinux.iso
qemu-system-x86_64 -m 2G -cdrom alinux.iso -enable-kvm
```

### USB Stick

```bash
sudo dd if=alinux.iso of=/dev/sdX bs=1M status=progress
```

## Building from Source

ALinux uses GitHub Actions for reproducible builds. You'll need to either 
manually follow the steps in the `alinux.yml` or just fork the repository

The goal is that you can **make your own distribution** within 10 minutes, 
the goal is not to build the perfect OS for everyone. 

- If you want to tune the performance, look at the `config/kernel.config` file.
- If you want to ship your application by default in the ISO, add it to the GitHub .yml
- The ISO ships with libazul.so out of the box, so it's easy for GUI applications to use that

## Technical Details

### Boot Process

1. **ISOLINUX** loads kernel and initramfs
2. **kernel** initializes hardware, mounts initramfs
3. **init script** mounts filesystems, starts services
4. **seatd** starts for seat management
5. **introibo** runs on first boot for system setup
6. **acomp** is the Wayland compositor
7. **alogin** displays login manager
8. **ade** is the environment which the

## License

While the kernel is GPL-2.0 licensed, alinux itself (the userspace programs, compositor, etc.) are MIT licensed.

- Linux Kernel: GPL-2.0
- BusyBox: GPL-2.0
- Azul GUI framework: MIT
- Rust applications: MIT
