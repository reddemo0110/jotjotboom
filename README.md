# Jotjotboom

Retro-flavoured markdown notes for the COSMIC desktop

## Installation

The quick way — clone and run the one-shot installer (per-user, into
`~/.local`; it fetches Rust and build tools if the machine lacks them):

```sh
git clone https://github.com/reddemo0110/jotjotboom.git
cd jotjotboom && ./install.sh
```

Re-run it after a `git pull` to update. Your notes live in
`~/Documents/JotJotBoom` as plain markdown files.

A [justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

## Sync between machines

Notes can follow you between machines through a server you run yourself:
a single [PocketBase](https://pocketbase.io) binary. Set it up in five
minutes with [server/README.md](./server/README.md), then open
Options → Sync in the app, enter the address, an email and a password,
and press **Create account**. Every other machine signs in with the same
details. The server only ever sees sealed packages: a note's id, a
revision counter, a timestamp and which device wrote it — never its
text. If two machines change the same note between syncs, both versions
are kept and one is marked "(conflict, *machine*)".

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/jotjotboom prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer]. To improve compilation times, disable LTO in the release profile, install the [mold][mold] linker, and configure [sccache][sccache] for use with Rust. The [mold][mold] linker will only improve link times if LTO is disabled.

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[mold]: https://github.com/rui314/mold
[sccache]: https://github.com/mozilla/sccache

## Credits

Folder icons come from [Boxicons](https://github.com/box-icons/boxicons) (MIT), [Iconoir](https://iconoir.com) (MIT), [Solar](https://www.figma.com/community/file/1166831539721848736) by 480 Design (CC BY 4.0), [Myna UI Icons](https://github.com/praveenjuge/mynaui-icons) (MIT), [Majesticons](https://github.com/halfmage/majesticons) (MIT), [Pixelarticons](https://github.com/halfmage/pixelarticons) (MIT) and [Duoicons](https://github.com/fernandcf/duoicons) (MIT), bundled via Iconify. Bundled fonts are under the OFL/UFL, see `resources/fonts`.
