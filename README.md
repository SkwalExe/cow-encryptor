# Cow-encryptor

![](images/banner.png)

Encrypt your files in cow language 🐮

# Installation 📦

## Arch Linux 🐧

cow-encryptor is in the AUR

```bash
yay -S cow-encryptor
```

## Other 🪟🐧

### With make - Linux 🐧

Run make

```bash
# 📂 cow-encryptor/
make
```

### Build from source - Linux 🐧 & Windows 🪟

**Clone this repo**

```bash
git clone https://github.com/SkwalExe/cow-encryptor.git
```

build with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
# 📂 cow-encryptor/
cargo build --release
```

**[ LINUX ONLY ] :** Move the binary

```bash
# 📂 cow-encryptor/
sudo cp target/release/cow-encryptor /usr/bin/cow-encryptor
```

**On windows** the executable will be `target\release\cow-encryptor.exe` you can move it wherever you want.

# Usage 📝

![](images/usage.png)

## --overwrite

Folder content : 

![](images/1.png)

If you try 

```bash
cow-encryptor secret.txt
```

You will get the following error : 

![](images/2.png)

Because a destination file already exists.

Use the --overwrite flag to overwrite the destination file.

## --encrypt

Enter encryption mode, the specified file will be encrypted.

Original file : 

![](images/5.png)

```bash
cow-encryptor --encrypt secret.txt
```

![](images/6.png)

Result : 

![](images/7.png)

Encryption mode will be used by default if the file doens't have the `.cow` extension.

## --decrypt

Enter decription mode, the specified file will be decrypted.

Encrypted file : 

![](images/3.png)

```bash
cow-encryptor [--decrypt] secret.txt.cow
```

![](images/4.png)

The decryption mode will automatically be used if the file ends with `.cow`

## --print

With this option, the encrypted/decrypted file will be printed to the standard output instead if saving it to a file.

# Docker 🐳

## Run the latest version

```bash
docker run --rm -it ghcr.io/skwalexe/cow-encryptor:main
```

## Test your changes 🚧

### Build 🛠️

```bash
# 📂 cow-encryptor/
docker build -t cow-encryptor .
```

### Run 🏃

```bash
docker run --rm -it cow-encryptor [OPTIONS]
```

# Uninstall 🗑

## With make

Run make uninstall

```bash
# 📂 cow-encryptor/
make uninstall
```

## Or
Just remove the binary

```bash
sudo rm /usr/bin/cow-encryptor
```

# final

If you have any problem, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>