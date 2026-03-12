# WARNING! This is the active dev branch, checkout the latest stable release: https://github.com/lnx-search/lnx/tree/refs/tags/0.9.0

<p align="center">
  <img width="30%" src="https://user-images.githubusercontent.com/57491488/156235904-5c0f956f-1bd7-4b7e-8cd0-fd344db7e632.png" alt="lnx Logo">
</p>


#
<p align="center">
  <a href="https://github.com/lnx-search/lnx/stargazers"><img src="https://img.shields.io/github/stars/lnx-search/lnx"/></a>
  <a href="hhttps://github.com/lnx-search/lnx/issues"><img src="https://img.shields.io/github/issues/lnx-search/lnx"/></a>
  <a href="https://github.com/lnx-search/lnx/blob/master/LICENSE"><img src="https://img.shields.io/github/license/lnx-search/lnx"/></a>
  <a href="https://docs.lnx.rs"><img src="https://img.shields.io/badge/Docs-alive-sucess"/></a>
</p>
<p align="center"><a href="https://lnx.rs">✨ Feature Rich | ⚡ Insanely Fast</a></p>
<p align="center">An ultra-fast, adaptable deployment of the tantivy search engine via REST.</p>

### 🌟 Standing On The Shoulders of Giants
lnx is built to not re-invent the wheel, it stands on top of the [**tokio-rs**](https://tokio.rs) work-stealing runtime, [**hyper**](https://hyper.rs/) web framework combined with the raw compute power of the [**tantivy search engine**](https://github.com/tantivy-search/tantivy).

Together this allows lnx to offer millisecond indexing on tens of thousands of document inserts at once (No more waiting around for things to get indexed!), Per index transactions and the ability to process searches like it's just another lookup on the hashtable 😲

### ✨ Features
lnx although very new offers a wide range of features thanks to the ecosystem it stands on.

- 🤓 **Complex Query Parser.**
- ❤️ **Typo tolerant fuzzy queries.**
- ⚡️ **Typo tolerant fast-fuzzy queries. (pre-computed spell correction)**
- 🔥 **More-Like-This queries.**
- Order by fields.
- *Fast* indexing.
- *Fast* Searching.
- Several Options for fine grain performance tuning.
- Multiple storage backends available for testing and developing.
- Permissions based authorization access tokens.

<p align="center">
  <img src="https://i.imgur.com/QovtWlc.gif" alt="Demo video"/>
</p>

*Here you can see lnx doing search as you type on a 27 million document dataset coming in at reasonable 18GB once indexed, ran on my i7-8700k using ~3GB of RAM with our fast-fuzzy system*
Got a bigger dataset for us to try? Open an issue!

### Performance
lnx can provide the ability to fine tune the system to your particular use case. You can customise the async runtime threads. The concurrency thread pool, threads per reader and writer threads, all per index.

This gives you the ability to control in detail where your computing resources are going. Got a large dataset but lower amount of concurrent reads? Bump the reader 
threads in exchange for lower max concurrency.

The bellow figures were taken by our `lnx-cli` on the small `movies.json` dataset, we didn't try any higher as Meilisearch takes an incredibly long time to index millions of docs although the new Meilisearch engine has improved this somewhat.

<p align="center">
<img width="45%" src="https://user-images.githubusercontent.com/57491488/149216271-6d30eae4-bb42-4121-a734-9fbd1bac2902.png"/>
<img width="45%" src="https://user-images.githubusercontent.com/57491488/149216285-705d4700-e10f-4ffe-a0f2-2fb325ba3004.png"/>
</p>

### 💔 Limitations
As much as lnx provides a wide range of features, it can not do it all being such a young system. Naturally, it has some limitations:

- lnx is not distributed (yet) so this really does just scale vertically.
- Simple but not too simple, lnx can't offer the same level of ease of use compared to MeiliSearch due to its schema-full nature and wide range of tuning options. With more tuning comes more settings, unfortunately.
- Metrics (yet)

# Building from scratch

## Requirements

- rustc v1.81+
- sccache
- linux os

## Development Workflow

This project includes a streamlined development workflow with Docker Compose for easy debugging and testing.

### Quick Start

1. **Start debug containers:**
   ```bash
   docker compose up lnx-debug lnx-dev -d
   ```

2. **Check compilation:**
   ```bash
   ./debug.sh check
   ```

3. **Build when ready:**
   ```bash
   ./debug.sh build
   ```

4. **Run the dev server:**
   ```bash
   ./debug.sh run
   ```

### Debug Script Commands

The `debug.sh` script provides easy access to common development tasks:

```bash
./debug.sh {check|build|run|shell|dev-shell}
```

- **check** - Check compilation errors without building
- **build** - Build the project 
- **run** - Run the development server on port 4203
- **shell** - Open pure debugging shell (lnx-debug container)
- **dev-shell** - Open development shell (lnx-dev container)

### How It Works - Step by Step

**1. Start the containers:**
```bash
docker compose up lnx-debug lnx-dev -d
```
This starts two containers:
- `lnx-debug`: Pure debugging environment (no server running)
- `lnx-dev`: Development environment with server capabilities

**2. Check compilation without building:**
```bash
./debug.sh check
```
This runs `cargo check --package lnx-server` inside the debug container, giving you fast compilation feedback without the overhead of a full build.

**3. Build when compilation is clean:**
```bash
./debug.sh build
```
This runs `cargo build --package lnx-server` to create the binary.

**4. Run the development server:**
```bash
./debug.sh run
```
This starts the lnx server in the `lnx-dev` container. The server will be accessible at `http://localhost:4203` because port 4203 is exposed in docker-compose.yml.

**Why this workflow?**
- **Fast iteration**: `check` is much faster than `build`
- **Separation of concerns**: Debug container for compilation, dev container for running
- **No interference**: Compilation testing doesn't affect the running server
- **Live mounting**: Source code changes are reflected immediately in both containers

### Container Services

- **lnx** - Production server (port 4202) - accessible at http://localhost:4202
- **lnx-dev** - Development environment with mounted source code (port 4203) - accessible at http://localhost:4203
- **lnx-debug** - Pure debugging container for compilation testing (no ports exposed)

### File Import & Persistence

**Known Issue**: The current storage implementation uses in-memory indexes (`RamDirectory`) which means data is not persisted between container restarts. This is being actively worked on.

For now, you can test the import functionality, but data will be lost when containers restart.

### Debugging Tips

- Use `./debug.sh check` after making code changes to quickly verify compilation
- The `lnx-debug` container is optimized for debugging with no server overhead
- Source code is mounted live, so changes are reflected immediately
- Use `./debug.sh shell` to access the debugging environment for manual testing


