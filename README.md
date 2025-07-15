# rQuery Builder (WIP)

**rQuery Builder** is a lightweight, modular SQL query builder designed to generate SQL statements dynamically and programmatically. It aims to simplify common query construction across various databases—without the heavy abstraction of traditional ORMs.

Unlike most ORMs, `rQuery Builder` respects the syntax and features unique to each supported database. It provides isolated modules and methods per database, ensuring full flexibility and alignment with native SQL dialects.

---

## ✨ Key Features (Planned)

### ✅ Multi-Database Support *(in progress)*

- Modular design with adapters for each database.
- Initial support for:
  - **PostgreSQL**
  - **MySQL**
  - **SurrealDB**

---

### 🐘 PostgreSQL Support

#### Operators
- [x] `=` Equal  
- [x] `!=` Not Equal  
- [x] `LIKE`  
- [x] `>` Greater Than  
- [x] `>=` Greater Than or Equal  
- [x] `<` Less Than  
- [x] `<=` Less Than or Equal  
- [x] `IN`  
- [x] `NOT IN`  
- [x] `IS NULL`  
- [x] `IS NOT NULL`  
- [ ] `BETWEEN`  

#### SELECT
- [ ] `DISTINCT`  
- [x] `ORDER BY`  
- [ ] `GROUP BY`  
- [x] `WHERE`  
- [x] Select specific columns  
- [x] `JOIN` (inner, left, etc.)  
- [x] Table aliasing  
- [x] Retrieve columns of a table  
- [ ] JSONB filtering  

#### INSERT
- [ ] Single row insert  
- [ ] Bulk insert  

#### UPDATE
- [ ] Single row update  
- [ ] Bulk update  

#### DELETE
- [ ] `DELETE` queries  

---

### 🐬 MySQL Support *(Planned)*

Targeting full feature parity with PostgreSQL support.

---

### 🧪 SurrealDB Support *(Experimental)*

Exploring support for SurrealDB's document-style query language and unique features.

---

## 🚀 Getting Started

> Coming soon: Installation, setup instructions, and usage examples.

---

## 🛠 Development Setup

To speed up builds, it's recommended to use [`sccache`](https://github.com/mozilla/sccache) for caching compiled dependencies.

### Installing `sccache`

#### Linux / macOS (via Cargo):
```bash
cargo install sccache
```

#### macOS (via Homebrew):
```bash
brew install sccache
```

#### Windows (via Chocolatey):
```powershell
choco install sccache
```

### Configure Cargo to Use `sccache`

Add the following to your `.cargo/config.toml`:

```toml
[build]
rustc-wrapper = "sccache"
```

Verify it's working:
```bash
sccache --show-stats
```

---

## 🤝 Contributing

All contributions are welcome—feature suggestions, bug reports, documentation updates, or pull requests. Feel free to [open an issue](https://github.com/your-repo/issues) or contribute directly.

---

## 📄 License

MIT License
