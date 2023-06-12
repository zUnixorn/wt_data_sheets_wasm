Custom scripts for updating modules:

```sh
cargo
update;
git
submodule
update
--remote;
git
pull;
cargo
check
```

## How to install


## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
trunk serve
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
trunk build --release
```

## What does each file/directory do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file
  with your details (name, description, version, authors, categories)

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html`
  file which loads the `index.js` file.
