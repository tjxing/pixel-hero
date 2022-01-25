# pixel-hero
A web-based NES emulator in WebAssembly

## Note

**The core features are still under development, thanks for watching it.**

### Core features

- [x] CPU instrcutions
- [x] Rendering
- [ ] Audio
- [ ] Controller

### Mappers supported

- [ ] Mapper000
- [ ] Mapper001
- [ ] Mapper002
- [x] Mapper003

## Build & Test

The project is written in Rust and built by `wasm-pack`. Setup the deleopment tool chains according the link

https://rustwasm.github.io/docs/book/game-of-life/setup.html

And execute the command under project root folder to build it

``` shell
wasm-pack build
```

The output is a NPM package, which is located in `pkg` folder.

The unit tests is ran by standard `cargo` command

``` shell
cargo test
```

**Note:** `wasm-pack test` can also exeute the unit tests but will encounter some memory issue with the default configuration.

## Usage

The output is a standard NPM package, which can be imported to your Javascript project.
``` Javascript
import * as wasm from "pixel-hero";
```

Create an emulator object with the function
``` Typescript
function create_emulator(element: HTMLElement, conf: any): Emulator;
```

Parameter `element` is a `div` html element, may be got by `document.getElementById`. Anything in the div will be cleared and a `<canvas>` will be created in it to display the game video.

Parameter `conf` is a Javascript object for game configuration, the detail fields are descriped [here](#Configuration).

The returned value is called emulator object, which is in fact an Javascript containing two functions
``` Typescript
class Emulator {
  insert(cartridge: Uint8Array): void;
  stop(): void;
}
```

Function `insert` accepts the NES file data in the format of `Uint8Array`, and start the game.

Function `stop` can stop the running game.

## Configuration

The configuration object to create an emulator object is

``` Javascript
{
  // To be implemented
}
```

## Example

A simple example is under folder `example`. 

Considering the copyright issue, no game file is published. If you are willing to run the example, please find a NES file, put it under the `example` folder and rename it as `game.nes`.

The exampe is hosted by `webpack-dev-server`. Run the following commands to start it.

``` shell
npm install
npm start
```

Then access http://localhost:8080 via your browser to enjoy the game.
