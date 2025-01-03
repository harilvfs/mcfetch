<p align="center">
<img height="140" src="img/preview.png"/>

<div align="center">

<a href="#-installation"><kbd> <br>Installation<br> </kbd></a> <a href="#-usage"><kbd> <br>Usage<br> </kbd></a> <a href="#-uninstallation"><kbd> <br>Uninstallation<br> </kbd></a>

[![Version](https://img.shields.io/github/v/release/dybdeskarphet/mcfetch?color=c6d0f5&label=Latest%20Release&style=for-the-badge&labelColor=a6d189)](https://github.com/dybdeskarphet/mcfetch/releases/latest) ![AUR Version](https://img.shields.io/aur/version/mcfetch-git?style=for-the-badge&color=c6d0f5&logo=arch-linux&label=%5BAUR%5D%20mcfetch-git&logocolor=85e185&labelColor=a6d189) [![Crates.io Version](https://img.shields.io/crates/v/mcfetch?style=for-the-badge&color=c6d0f5&labelColor=a6d189&logo=rust&logoColor=e64553)](https://crates.io/crates/mcfetch)

</div>

---

# 🌈 mcfetch

_mcfetch (monochromatic fetch)_ is a lightweight system information fetching program, similar to ufetch but with color options.

## 🔧 Installation

### Using Installation Script

Quickly install `mcfetch` with a one-liner:

```bash
bash <(curl -L https://raw.githubusercontent.com/dybdeskarphet/mcfetch/main/install.sh)
```

### From crates.io

If you have Rust and Cargo installed, you can install `mcfetch` directly from Crates.io:

```bash
cargo install mcfetch
```

### From the Arch User Repository (AUR)

For Arch-based Linux distributions, install [**mcfetch**](https://aur.archlinux.org/packages/mcfetch-git) from the AUR:

```bash
# For paru users
paru -S mcfetch-git
# For yay users
yay -S mcfetch-git
```

### 🛠️ Manual Installation

Clone the repository, build the binary, and move it to your PATH:

```bash
git clone https://github.com/dybdeskarphet/mcfetch.git
cd mcfetch
cargo build --release
sudo mv target/release/mcfetch /usr/bin/
```

## ✨ Usage

Run `mcfetch` with your preferred color:

```bash
mcfetch --color <COLOR>
```

Example:

```bash
mcfetch --color blue
```

## 🚮 Uninstallation

If you installed mcfetch using the **installation script**, you can uninstall it easily:

```bash
bash <(curl -L https://raw.githubusercontent.com/dybdeskarphet/mcfetch/main/install.sh) --uninstall
```

This will remove the mcfetch binary from your system.

If you installed it manually or through cargo, remove it as follows:

- **Cargo uninstallation:**

```bash
cargo uninstall mcfetch
```

- **Manual uninstallation:**

```bash
sudo rm /usr/bin/mcfetch
```

- **Aur installation:**

```bash
# For paru users
paru -R mcfetch-git
# For yay users
yay -R mcfetch-git
```

<br>

## 📜 License

This project is licensed under the GNU General Public License v3.0.
See the [LICENSE](https://github.com/dybdeskarphet/mcfetch/blob/main/LICENSE) file for more details.
