<p align="center">
<img height="140" src="img/preview.png"/><h1> mcfetch</h1>

_mcfetch (monochromatic fetch)_ is a lightweight system information fetching program, similar to ufetch but with color options.

</p>

## 🔧 Installation

### Script

```bash
bash <(curl -L https://raw.githubusercontent.com/dybdeskarphet/mcfetch/main/install.sh)
```

### From crates.io

You can install mcfetch directly from Crates.io if you have Rust and Cargo installed:

```bash
cargo install mcfetch
```

### Manually

```bash
git clone https://github.com/dybdeskarphet/mcfetch.git
cd mcfetch
cargo build --release
```

Move the binary to your path:

```bash
sudo mv target/release/mcfetch /usr/local/bin/
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

This project is licensed under the GPLv3.
