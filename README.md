<p align="center">
<img height="140" src="img/preview.png"/>

<div align="center">

[![Version](https://img.shields.io/github/v/release/dybdeskarphet/mcfetch?color=ca9ee6&label=Latest%20Release&style=for-the-badge&labelColor=ef9f76)](https://github.com/dybdeskarphet/mcfetch/releases/latest)

</div>
  
<h1> mcfetch</h1>

_mcfetch (monochromatic fetch)_ is a lightweight system information fetching program, similar to ufetch but with color options.

</p>

<div align="center">

[![Version](https://img.shields.io/github/v/release/dybdeskarphet/mcfetch?color=%230567ff&label=Latest%20Release&style=for-the-badge)](https://github.com/dybdeskarphet/mcfetch/releases/latest)

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
