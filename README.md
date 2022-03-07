<div align="center">
  <h2><code>paydept</code></h2>
  <h4>🙌 Shows every open-source dependency you use in your system that accept donations. 💝</h4>
  <img src="https://user-images.githubusercontent.com/26198477/156766025-d2a6a16a-8fd6-4d6f-b37f-1f9bedcb7a7f.png" height="500" width="700">
</div>

---

This utility traverses through your filesystem looking for open-source dependencies that are seeking donations by parsing `README.md` and `FUNDING.yml` files.

## Installation

```
$ curl -L https://github.com/mufeedvh/paydept/releases/download/v1.0.0/paydept_amd64 -o paydept
```

(`Linux AMD x86-64`)

**OR**

Download the executable from [**Releases**](https://github.com/mufeedvh/paydept/releases) for your OS.

**OR**

Install with `cargo`:

    $ cargo install --git https://github.com/mufeedvh/paydept.git
    
[Install Rust/Cargo](https://rust-lang.org/tools/install)

## Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/mufeedvh/paydept.git
$ cd paydept/
$ cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Usage

Run under your entire working directories:

    $ cd $HOME
    $ paydept
    
Export the results to a CSV file:

    $ paydept export
    
### Related Projects

- https://github.com/feross/thanks
    
## License

Licensed under the MIT License, see <a href="https://github.com/mufeedvh/pdfrip/blob/master/LICENSE">LICENSE</a> for more information.    
