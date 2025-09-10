echo "FP Hardinfo installation program"

apt update

if hash rustup 2>/dev/null; then
  echo "Rust found"
else
  echo "Rust not found, downloading..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  . "$HOME/.cargo/env"
  echo "Rust downloaded and installed"
fi

if hash neofetch 2>/dev/null; then
  echo "Neofetch found"
else
  echo "Neofetch not found, installing..."
  apt install neofetch
  echo "Neofetch installed"
fi

if hash chromium 2>/dev/null; then
  echo "Chromium found"
else
  echo "Chromium not found, installing..."
  apt install chromium
  echo "Chromium installed"
fi

if hash fp-hardinfo-nogui 2>/dev/null; then
  echo "Old version found"
  echo "Uninstalling old version..."
  apt remove ./target/debian/*.deb
  echo "Old version uninstalled"

  echo "Removing old build..."
  rm ./target/debian/*.deb
  echo "Old build removed"
else
  echo "Old version not found"
fi

echo "Building..."
cargo deb
echo "Builded"

echo "Installing..."
apt install ./target/debian/*.deb
echo "Installed"

echo "All done"