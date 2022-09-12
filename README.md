# RaspberyPiRustOCR

This repo contains some sample code for an ocr application.

For me cross-compiling did not work.
I than had to install rust on my raspberry pi zero w.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

As the application uses the [leptess](https://crates.io/crates/leptess) crate which uses tesseract also the following packages are necessary

```
sudo apt-get install libleptonica-dev libtesseract-dev clang -y
sudo apt-get install tesseract-ocr-eng -y
```

The application expects a file path in arg[1]. As easy test the repository has a [sample image](https://github.com/nampacx/RaspberyPiRustOCR/blob/main/src/OcrSample/di.png).

One way to execute the application is 

```
cargo run -- %FilePath%
```
