# ALinux

> A blazingly fast, minimal Linux distribution powered by the Azul GUI framework and modern Rust applications

<p align="center">
  <img src="https://img.shields.io/badge/kernel-6.12.1-blue" />
  <img src="https://img.shields.io/badge/init-custom-green" />
  <img src="https://img.shields.io/badge/compositor-wayland-purple" />
  <img src="https://img.shields.io/badge/language-rust-orange" />
  <img src="https://img.shields.io/badge/license-MIT-lightgrey" />
</p>

**ALinux** (Azul Linux) is a from-scratch Linux distribution built for performance and simplicity. Inspired by the Azul GUI framework which uses Mozilla WebRender for GPU-accelerated rendering with CSS/DOM-based layouts, ALinux ships exclusively with Rust-based applications and tools, delivering a modern, fast, and efficient computing experience.

---

## 🎯 Philosophy

- **Performance First**: CachyOS-inspired kernel optimizations, 1000Hz scheduler, aggressive CPU tuning
- **Rust Everything**: Every shipped application is written in Rust for memory safety and speed
- **Azul GUI Native**: All graphical applications use the Azul framework for consistent, GPU-accelerated UIs
- **Minimal by Design**: Zero bloat - only essential tools and libraries included
- **CLI-Focused**: Powerful command-line tools with optional graphical interfaces

---

## ✨ Features

### 🚀 Core System

- **Optimized Linux Kernel 6.12.1**
  - CachyOS performance patches
  - CPU-specific optimizations (ZEN4, Alder Lake, etc.)
  - 1000Hz scheduler for ultra-responsive desktop experience
  - Full DRM/KMS support for Intel, AMD, and NVIDIA graphics
  
- **Custom Wayland Compositor (acomp)**
  - Minimal, purpose-built compositor
  - GPU-accelerated rendering via Azul/WebRender
  - Native Rust implementation for maximum performance
  
- **libazul.so Graphics Library**
  - Mozilla WebRender rendering engine for hardware-accelerated 2D graphics
  - CSS/DOM-based layout system with flexbox support
  - Functional composition over inheritance
  - Efficient re-rendering only when necessary

### 🎨 Graphical Applications

#### **Introibo Installer**
Windows-inspired first-boot experience with a beautiful, GPU-accelerated installation wizard. Configure your system, partition drives, and set preferences through an intuitive Azul-powered interface.

#### **Alogin Manager**
Sleek Wayland-native login manager featuring:
- Smooth animations and transitions
- Session management
- User switching
- Auto-login configuration

### 🛠️ Essential Rust CLI Tools

ALinux ships with modern Rust alternatives to classic Unix tools:

#### **File Management**
- **eza** - Modern ls replacement with colors, icons, and Git integration
- **fd** - Fast and user-friendly alternative to find
- **yazi** - Blazing fast terminal file manager based on async I/O
- **zoxide** - Smarter cd command supporting all major shells
- **bat** - cat clone with syntax highlighting and Git integration

#### **System Monitoring**
- **procs** - Modern replacement for ps with color-coded, structured process views
- **bottom** - System monitor with graphical visualization widgets for CPU, RAM, disk, and temperature
- **bandwhich** - Network utilization monitor by process, connection, and remote IP
- **zenith** - Like top/htop but with zoomable charts, network, and disk usage

#### **Text Processing**
- **ripgrep** - Lightning-fast grep alternative that respects .gitignore
- **sd** - Intuitive find & replace CLI (sed alternative)
- **tealdeer** - Very fast implementation of tldr for simplified man pages

#### **Development Tools**
- **delta** - Enhanced git diff viewer with extensive styling options
- **starship** - Minimal, blazing-fast, infinitely customizable shell prompt
- **cargo** - Rust package manager and build system
- **rustc** - Rust compiler with full toolchain

#### **Utilities**
- **ouch** - Painless compression and decompression
- **dua-cli** - Fast tool to learn about disk usage
- **grex** - Generate regular expressions from test cases
- **hyperfine** - Command-line benchmarking tool

### 🔧 System Tools

- **BusyBox** - Essential Unix utilities in a single binary
- **seatd** - Minimal seat management daemon for Wayland
- **Git** - Version control for AUR package building
- **GCC/Make** - Build tools for compiling from source

---

## 📦 Installation

### Quick Start with QEMU

```bash
# Download the latest ISO
wget https://github.com/yourusername/alinux/releases/latest/download/alinux.iso

# Test in virtual machine
qemu-system-x86_64 -m 2G -cdrom alinux.iso -enable-kvm
```

### Install to USB Drive

```bash
sudo dd if=alinux.iso of=/dev/sdX bs=1M status=progress
```

### Install to Disk

Boot from the ISO and run the Introibo installer:

1. **Welcome Screen** - Choose language and keyboard layout
2. **Disk Partitioning** - Automatic or manual partitioning
3. **User Setup** - Create your user account
4. **Package Selection** - Choose additional Rust applications
5. **Install** - System copies to disk with optimizations
6. **Reboot** - Remove installation media and boot into ALinux

---

## 🏗️ Building from Source

ALinux uses GitHub Actions for reproducible builds. To build locally:

```bash
git clone https://github.com/yourusername/alinux.git
cd alinux

# Trigger GitHub Actions build
git commit --allow-empty -m "Build ISO"
git push

# Or build locally (requires Docker)
./scripts/build-local.sh
```

### Repository Structure

```
alinux/
├── .github/workflows/
│   └── build.yml              # CI/CD pipeline
├── config/
│   ├── kernel.config          # Kernel configuration
│   ├── init                   # Init script
│   ├── isolinux.cfg           # Bootloader config
│   └── ...                    # System config files
├── libazul/                   # Azul graphics library
├── introibo/                  # Installer GUI
├── alogin/                    # Login manager
├── acomp/                     # Wayland compositor
└── README.md
```

---

## ⚙️ Configuration

### Kernel Optimization

Edit `config/kernel.config` to target your specific CPU:

```bash
# AMD Ryzen 7000 series
CONFIG_MZEN4=y

# AMD Ryzen 5000 series
CONFIG_MZEN3=y

# Intel 12th gen (Alder Lake)
CONFIG_MALDERLAKE=y

# Intel 13th/14th gen (Raptor Lake)
CONFIG_MRAPTORLAKE=y

# Generic modern x86-64 (safest)
CONFIG_GENERIC_CPU=y
```

### Adding Rust Applications

Create a new directory for your app:

```bash
mkdir myapp
cd myapp

# Create main.rs with your Rust code
cat > main.rs << 'EOF'
fn main() {
    println!("Hello from myapp!");
}
EOF
```

Update `.github/workflows/build.yml` to include your app:

```yaml
- name: Build myapp
  run: |
    cd myapp
    rustc -O --target x86_64-unknown-linux-musl \
      -o ../programs-build/myapp main.rs
```

---

## 🎨 Developing Azul Applications

### Hello World Example

```rust
use azul::prelude::*;

struct MyApp {
    counter: usize,
}

extern "C" fn render(data: &mut RefAny, _: &mut LayoutInfo) -> StyledDom {
    let app = data.downcast_ref::<MyApp>()?;
    
    Dom::body()
        .with_child(
            Dom::text(format!("Count: {}", app.counter))
                .with_inline_style("font-size: 24px;")
        )
        .with_child(
            Button::new("Increment")
                .onmouseup(increment, data.clone())
                .dom()
        )
        .style(Css::empty())
}

extern "C" fn increment(data: &mut RefAny, _: &mut CallbackInfo) -> Update {
    let mut app = data.downcast_mut::<MyApp>()?;
    app.counter += 1;
    Update::RefreshDom
}

fn main() {
    let app = App::new(RefAny::new(MyApp { counter: 0 }), AppConfig::default());
    app.run(WindowCreateOptions::new(render));
}
```

### Building Against libazul.so

```bash
# Link your application against the system Azul library
rustc -O -L /usr/lib -lazul -o myapp main.rs

# Or with Cargo
cargo build --release
```

---

## 📚 Package Management

### Installing from Source

ALinux includes `git`, `gcc`, and `make` for building packages:

```bash
# Clone a Rust project
git clone https://github.com/user/awesome-rust-tool.git
cd awesome-rust-tool

# Build and install
cargo build --release
sudo cp target/release/awesome-rust-tool /usr/bin/
```

### AUR Compatibility (Coming Soon)

While not Arch-based, ALinux aims to support AUR package building:

```bash
# Future functionality
alinux-install package-name
```

---

## 🔬 Technical Details

### Boot Process

1. **ISOLINUX** loads kernel and initramfs
2. **Kernel** initializes hardware, mounts initramfs
3. **Init script** mounts filesystems, starts services
4. **seatd** starts for seat management
5. **Introibo** runs on first boot for system setup
6. **acomp** starts Wayland compositor
7. **alogin** displays login manager
8. **User session** starts with your choice of shell/environment

### System Architecture

```
┌─────────────────────────────────────┐
│     Azul GUI Applications           │
│  (Introibo, Alogin, custom apps)    │
├─────────────────────────────────────┤
│         libazul.so                  │
│    (WebRender + CSS/DOM)            │
├─────────────────────────────────────┤
│    acomp (Wayland Compositor)       │
├─────────────────────────────────────┤
│      Wayland Protocol               │
├─────────────────────────────────────┤
│    DRM/KMS (Direct Rendering)       │
├─────────────────────────────────────┤
│   Linux Kernel 6.12.1 (Optimized)   │
└─────────────────────────────────────┘
```

### Performance Characteristics

- **Boot Time**: ~3-5 seconds to login screen (SSD)
- **Memory Usage**: ~150MB idle (with compositor)
- **ISO Size**: ~80-120MB compressed
- **Install Size**: ~300-500MB minimal system

---

## 🤝 Contributing

We welcome contributions! Here's how:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-tool`
3. **Add your Rust application** to the appropriate directory
4. **Update the CI workflow** to build your app
5. **Test locally** or let CI build it
6. **Submit a pull request**

### Contribution Guidelines

- All applications must be written in Rust
- Graphical apps should use the Azul framework
- Follow Rust naming conventions (`snake_case` for files)
- Include a README.md in your app directory
- Add yourself to CONTRIBUTORS.md

---

## 📊 Comparison

| Feature | ALinux | Alpine | Arch | Ubuntu |
|---------|---------|---------|------|--------|
| Init System | Custom | OpenRC | systemd | systemd |
| Package Manager | Source/Cargo | apk | pacman | apt |
| Default Language | Rust | C | Mixed | Mixed |
| ISO Size | ~100MB | ~130MB | ~800MB | ~3GB |
| Boot Time | 3-5s | 5-8s | 10-15s | 20-30s |
| GUI Framework | Azul | GTK/Qt | GTK/Qt | GNOME |
| Compositor | acomp | Various | Various | Mutter |

---

## 🐛 Troubleshooting

### Black screen after boot
- Your GPU driver may not be included. Edit `config/kernel.config`:
  - Intel: Ensure `CONFIG_DRM_I915=y`
  - AMD: Ensure `CONFIG_DRM_AMDGPU=y`
  - NVIDIA: Ensure `CONFIG_DRM_NOUVEAU=y`

### Compositor won't start
```bash
# Check if Wayland socket exists
echo $WAYLAND_DISPLAY

# Manually start compositor
/usr/bin/acomp &

# Check logs
dmesg | grep drm
```

### Programs won't link against libazul.so
```bash
# Add library path
export LD_LIBRARY_PATH=/usr/lib:$LD_LIBRARY_PATH

# Or add to /etc/ld.so.conf.d/azul.conf
echo "/usr/lib" | sudo tee /etc/ld.so.conf.d/azul.conf
sudo ldconfig
```

---

## 📜 License

ALinux is MIT licensed. Individual components may have different licenses:

- Linux Kernel: GPL-2.0
- BusyBox: GPL-2.0
- Azul Framework: LGPL-3.0 with static linking exception
- Rust applications: MIT (unless otherwise specified)

<p align="center">
  <strong>Built with 🦀 Rust and ❤️ by the ALinux community</strong>
</p>
