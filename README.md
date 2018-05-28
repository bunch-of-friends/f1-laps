# f1-laps

Eventually a companion app to Codemaster F1 games, which uses the game telemetry data to keep track of best lap times across the game (and eventually more!).

Current plan
- apps for Windows, MacOS, Linux, Android and iOS
- tracking best laps and sectors on each tyre compound, in each car
- integration with Phillips Hue and its EDK - revs lights, flags, notifications such as best sector time etc. and other (all configurable)

## Structure

- **core** - Main library, written in Rust, containing most of the logic - receiving the packets, processing them, storing them.
- **desktop** - An [Electron](https://github.com/electron/electron) app, currenntly in early stages.
- **js-bridge** -Mapping to the JS world (Node.js specifically) using [neon](https://crates.io/crates/neon).
- **hue** -A placeholder. There is nothing interesting there at the moment. But that's where the Hue EDK integration will be kept.
- **node-demo** - Just a small node app to execute some quick tests.

## Disclaimer

None of us working on this project are by far Rust experts. We are experienced software devs, just new to Rust. The code will improve over time, while we are happy to hear any feedback.

## Build

1. Make sure you have the latest stable [Rust installed](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
2. Clone the repo
3. Navigate to the `js-bridge` folder and `npm install` there
4. Go to the `desktop` folder, run `npm install` there and then `npm run start`
