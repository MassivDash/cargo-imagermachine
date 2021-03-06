<h1 align="center">Welcome to Cargo Imagermachine 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.2.0-blue.svg?cacheSeconds=2592000" />
  <a href="https://github.com/MassivDash/cargo-imagermachine" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
  </a>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/spaceoutPL" target="_blank">
    <img alt="Twitter: spaceoutPL" src="https://img.shields.io/twitter/follow/spaceoutPL.svg?style=social" />
  </a>
</p>

## Rust cli for batch optimizing images

Cargo-imagermachine was designed as a subtool of cargo package manger, so user can run the interactive cli from the command line inside folders that contain pictures. 

Imagermachine uses turbo-jpeg library for jpeg files optimization and oxi png for png images. You must install the turbo-jpeg yourself as the cli uses only the bindings to the turbo-jpeg library. Installation guide for respective systems below. 

Imagermachine can: 
- Optimize png and jpg images. 
- Resize and crop images. 
- Rename the transformed files based on provided_name_(index).ext
- Convert files to Webp
- todo: set optimalisation custom config 

---

### Dependencies  
So far the cli has been tested on unix systems (Debian, Mac).  
Before using cargo-imagermachine you must install 

#### Rust
Follow the official guideline for installing rust on your system
https://www.rust-lang.org/tools/install

#### Libturbojpeg

Offical webiste and documentation 👉 https://www.libjpeg-turbo.org/

*Debian* 

```bash 
sudo apt-get install libturbojpeg libturbojpeg0-dev
```

This should be enough to get started on Ubuntu

*Mac* 

```bash
brew install jpeg-turbo
```

! Remember to copy all the export paths that brew will print out after installation, I am using the following config in the cargo.toml file, pkg-config (also via brew) installation is needed for this setup to work. 

```yml
turbojpeg = {version = "0.4", features = ["image", "pkg-config"]}
```


If you have troubles linking the cli to your libturbo, you can try to edit the cargo.toml file and add feature flags into turbojpeg features array, for more reference follow this [link](https://github.com/honzasp/rust-turbojpeg#requirements). 

 

---



### Install
In order to install cargo-imagermachine on your computer, clone the project then run the following command from the project folder: 

```sh
cargo install --path . 
```

### Usage


If you followed the offical rust documentation and cargo / rustc is in scope, navigate to a input folder and run 

```sh
cargo imagermachine
```

Follow directions on the interactive cli to optimize, resize, rename the images. 


### Author

👤 **Lukasz Celitan**

* Website: https://spaceout.pl
* Twitter: [@spaceoutPL](https://twitter.com/spaceoutPL)
* Github: [@MassivDash](https://github.com/MassivDash)
* LinkedIn: [@luke-celitan-99920b168](https://linkedin.com/in/luke-celitan-99920b168)

## Show your support

Give a ⭐️ if this project helped you!

