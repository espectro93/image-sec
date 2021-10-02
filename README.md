# image-sec

Encrypt bmp images to share them securely.

## Description

image-sec is a Proof-of-Concept cli tool that utilizes the AES Galois/Counter Mode for encrypting bmp images. 
It is not recommended to use this tool for real life purposes, as key and nonce management is hardcoded within the source.

## Getting Started

### Dependencies

* Install rust (https://doc.rust-lang.org/book/ch01-01-installation.html)

### Installing

* Checkout repository and build project

### Executing program

```
cargo run -decrypt <file_path_encrypted>.bmp
```
```
cargo run -encrypt <file_path>.bmp
```

### Further improvements
* bmp was chosen as the only supported image type, as the binary structure is rather simple. Other types could be implemented as well
* Key and Nonce Management is not implemented but should be done if considering this tool for a real world use case

## Authors

espectro93

