<p align="center">
<img height="140" src="img/preview.png"/>

<div align="center">

[![Version](https://img.shields.io/github/v/release/dybdeskarphet/mcfetch?color=c6d0f5&label=Latest%20Release&style=for-the-badge&labelColor=a6d189)](https://github.com/dybdeskarphet/mcfetch/releases/latest)

<a href="https://github.com/dybdeskarphet/mcfetch#-installation"><kbd> <br>Install<br> </kbd></a>  <a href="https://github.com/dybdeskarphet/mcfetch#-usage"><kbd> <br>Usage<br> </kbd></a> <a href="https://github.com/harilvfs/mcfetch#-uninstallation"><kbd> <br>Uninstall<br> </kbd></a>  

</div>
  
<h1> mcfetch</h1>

_mcfetch (monochromatic fetch)_ is a lightweight system information fetching program, similar to ufetch but with color options.

</p>

## 🔧 Installation

### Using Installation Script

Quickly install `mcfetch` with a one-liner:

```bash
bash <(curl -L https://raw.githubusercontent.com/dybdeskarphet/mcfetch/main/install.sh)
```

### 📦 From crates.io

[![Crates.io Version](https://img.shields.io/crates/v/mcfetch?style=for-the-badge&color=e64553&labelColor=000000&logo=rust&logoColor=e64553)](https://crates.io/crates/mcfetch) 

If you have Rust and Cargo installed, you can install `mcfetch` directly from Crates.io:

```bash
cargo install mcfetch
```

### 🛠️ Manual Installation

Clone the repository, build the binary, and move it to your PATH:

```bash
git clone https://github.com/dybdeskarphet/mcfetch.git
cd mcfetch
cargo build --release
sudo mv target/release/mcfetch /usr/bin/
```

## 🚮 Uninstallation

If you installed mcfetch using the installation script, you can uninstall it easily:

```bash
bash <(curl -L https://raw.githubusercontent.com/dybdeskarphet/mcfetch/main/install.sh) --uninstall
```

This will remove the mcfetch binary from your system.

If you installed it manually or through cargo, remove it as follows:

- **Cargo installation:**

```bash
cargo uninstall mcfetch
```

- **Manual installation:**

```bash
sudo rm /usr/bin/mcfetch
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

## 📜 License

This project is licensed under the GNU General Public License v3.0.
See the [LICENSE](https://github.com/dybdeskarphet/mcfetch/blob/main/LICENSE) file for more details.
