# rQuery Builder (Work In Progress)

**rQuery Builder** is a lightweight and modular SQL query builder designed to generate SQL statements dynamically and programmatically.

The goal of this project is to simplify the construction of common SQL queries across different databases, starting with PostgreSQL, MySQL, and SurrealDB. Unlike traditional ORMs, this project does not aim to abstract queries into a unified interface. Each database is treated independently, with its own isolated libraries and methods tailored to its specific syntax and features.

---

## Features (Planned)

### ✅ Cross-Database Support *(WIP)*

- Designed for adaptability with multiple databases.
- Initial focus: **PostgreSQL**, **MySQL**, and **SurrealDB**.

---

### 🚧 PostgreSQL Support

- [ ] `SELECT` *(Work in Progress)*
- [ ] `SELECT` with `JOIN`s
- [ ] `SELECT` using `JSONB` fields
- [ ] `INSERT`
- [ ] `INSERT BULK`
- [ ] `DELETE`
- [ ] `UPDATE`
- [ ] `UPDATE BULK`

---

### ⏳ MySQL Support

Planned feature parity with PostgreSQL.

---

### 🧪 SurrealDB Support

Exploring support for SurrealDB's query syntax and data model.

---

## Getting Started

> Coming soon: Installation instructions and usage examples.

---

## Development Setup

To improve compilation times, it's recommended to use [`sccache`](https://github.com/mozilla/sccache) as a compiler cache.

### 📦 Installing `sccache`

#### On Linux/macOS (via Cargo):

```bash
cargo install sccache
```

#### On macOS (via Homebrew):

```bash
brew install sccache
```

#### On Windows (via Chocolatey):

```powershell
choco install sccache
```

Then, configure Cargo to use `sccache` by adding the following to `.cargo/config.toml`:

```toml
[build]
rustc-wrapper = "sccache"
```

You can verify it's working with:

```bash
sccache --show-stats
```

---

## Contributing

Contributions are welcome! Whether it's a feature request, bug report, or code contribution, feel free to open an issue or submit a pull request.

---

## License

MIT License *(or specify your license here)*
