# AGENTS.md

## Project overview

Mobile/desktop application for managing shopping lists, written in Rust with
[Dioxus 0.7](https://dioxuslabs.com/) and local persistence in SQLite. The UI
is internationalized in English and Spanish.

## Stack

- **UI:** Dioxus 0.7 (rsx, router, mobile/desktop)
- **Database:** SQLite via `rusqlite` + `rusqlite_migration`
- **i18n:** `dioxus-i18n` with Fluent files (`.ftl`)
- **Styles:** Tailwind CSS 4 (compiled to `assets/tailwind.css`)
- **Environment:** Nix flake with the Rust toolchain, Android SDK/NDK and Tailwind

## Commands

Set up the environment with `direnv allow` or `nix develop`.

```bash
cargo check                 # compile/verify
cargo clippy --all-targets  # lints
cargo fmt                   # format
cargo fmt --check           # check formatting

dx serve --platform desktop --features desktop  # desktop app
dx serve --platform android                     # Android app (default: mobile)

tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
# npm alternative: npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

There is no test suite in the repository. To validate changes use
`cargo check`, `cargo clippy` and manual testing of the app.

## Structure

```text
src/
  main.rs              # entry point, routes, i18n, thread_local DATABASE
  components/          # reusable components (Navbar, Toggle)
  model/
    database.rs        # DBConnector trait + SQLite implementation
    lista.rs           # Lista, Item, ItemForm structs
  views/
    home.rs            # list listing/creation/editing
    lista.rs           # detail of a list and its items
assets/
  i18n/{en,es}.ftl     # translations
  tailwind.css         # generated CSS (committed)
  shopping_list.db     # local database (git-ignored)
input.css              # Tailwind source
```

## Architecture and conventions

- **Data access:** everything goes through `DATABASE` (thread-local in
  `main.rs:22`), which holds a `Box<dyn model::DBConnector>`. Do not access
  `rusqlite` directly from views or components; add methods to the
  `DBConnector` trait (`src/model/database.rs`).
- **Migrations:** add new `M::up(...)` entries to the slice in
  `SQLiteConnector::new()` at `src/model/database.rs:32`. Never modify
  existing migrations; the database may already be migrated.
- **State and reactivity:** views use `use_signal` and `use_context`.
  `ListaView` provides `ListaViewState` with a `Signal<Lista>`; it reloads the
  list from the database after every mutation.
- **Components:** each file in `components/` or `views/` is declared in its
  `mod.rs` and re-exported (`pub use`). View components are `pub`; internal
  helpers may be private.
- **Naming:** the domain is in Spanish (`Lista`, `Item`, `nombre`,
  `cantidad_requerida`, `cantidad_comprada`, `precio`, `unidad`). Keep that
  convention in the model and the database.
- **Errors:** the trait returns `anyhow::Result`/`rusqlite::Result`, but
  several queries currently use `unwrap()`/`expect()`. Do not introduce new
  unnecessary `unwrap()` calls; propagate errors where possible.
- **Routes:** defined in the `Route` enum at `src/main.rs:12`. `Navbar` is the
  layout and renders `Outlet::<Route>`.
- **Styles:** Tailwind classes live in the `rsx!`. When adding new classes,
  regenerate `assets/tailwind.css`; the Tailwind build scans
  `./src/**/*.{rs,html,css}` according to `input.css`.
- **i18n:** do not write visible text directly in `rsx!` when a key exists.
  Use `tid!("key")` and add the string to **both** `assets/i18n/en.ftl` and
  `assets/i18n/es.ftl`.
- **No comments:** the code does not use explanatory comments; follow that
  style unless the comment documents a non-obvious decision.
- **Do not commit** `assets/shopping_list.db`, `target/`, `node_modules/` or
  `.direnv/` (already in `.gitignore`).

## Notes

- `Cargo.toml` sets `default = ["mobile"]`; for desktop use
  `--features desktop`.
- On Android, the database path is obtained via JNI
  (`src/model/database.rs:75`); on Linux it is `assets/`.
- The project uses Rust edition 2024.
