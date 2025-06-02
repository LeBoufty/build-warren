![title](img/buildwarrentitle.png)

Unofficial API for Spawning Tool.

## Project summary

Build Warren aims to be an API endpoint for Spawning Tool, which lacks one. My end goal is to create an open source alternative to Spawning Tool's Build Advisor, as I don't like Overwolf's invasiveness.

Why "Build Warren" ? Because you can't build a Roach Warren without a Spawning Pool !

## Usage

The following commands get data from Spawning Tool and parses the pages for relevant information. Cloaked (i.e. private) builds therefore aren't accessible.

### Get the build order count

```Bash
cargo run build-count # Gives the highest valid build ID
```

### Get a specific build order

```Bash
cargo run fetch [BUILD_ID]
```
You can add `-o [OUTPUT_FILE]` to write the result in a file. For example :
```Bash
cargo run -- -o build.json fetch 193844 # Saves build 193844 to build.json
```

### Get the latest build order(s)

```Bash
cargo run fetch-latest # Fetch the latest public build order
cargo run fetch-latest 5 # Fetch the five latest public build orders
```
You can also specify an output file as with `fetch`.

```Bash
cargo run -- -o builds.json fetch-latest 20 # Fetches the twenty latest public builds and saves them to builds.json
```

### Get a batch of build orders

```Bash
cargo run fetch-segment 141 145 # Fetches build orders 141-145 (both included)
```

## Roadmap

### Without storage

* [x] Better CLI interface, with progress bars, styling and more detailed progress
* [x] Deployment with an actual API

### With storage

* [ ] Database integration (possibly MongoDB)
* [ ] Live updates -- checking if new builds have been posted

### Live overlay

* [ ] Build display
* [ ] Timer management
* [ ] Styling and icons
* [ ] Build browsing
