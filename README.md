# Keygen.fi

https://keygen.fi

## Description

I wanted a passphrase-generator with Finnish words, but there was no such product available. I also wanted something that can be accessed from the web and does all magic in client-side, so that nothing leaks to someone elses computer (cloud).

Alphabet is read from `src/wordlist.rs`. Feel free to fork the project and update this file to make your own password generator with customized wordlist.

## Development

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf -y | sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk build
```

After building, open `dist/index.html` in browser

## Credits

* Inspiration for generating randomness: https://github.com/TuningSweeper/keygen
* https://github.com/tanelikaivola
* https://github.com/Zokol
