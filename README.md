# About
FITU-16 is a free and open-source fantasy computer built for creating, playing, and sharing small retro-style games. It’s powered by [Rust](https://www.rust-lang.org/) and [Macroquad](https://github.com/not-fl3/macroquad/tree/master), and its core concept is uniquely self-referential: **the engine is, in essence, written by itself**.

That might sound confusing at first, but here’s what it means: the entire engine is fully moddable using Lua scripts. While the core functionality is written in Rust, you can customize, remove, or add new tools, editors, and behaviors entirely through Lua!

FITU-16 comes with a complete set of built-in development tools — including a file explorer, code editor, sprite, auto-tile, and map editors, sound and music editors, and a command-line interface — everything you need to create a mini retro game in a simple and streamlined way.

Games are packaged into cartridge files, making them easy to distribute and share. And since FITU-16 runs on all major platforms, your games can be played on virtually any device.

To keep the retro spirit alive, FITU-16 enforces a few creative constraints:

- 240x135 pixel resolution

- 64-color palette

- 256 8×8 pixel sprites

- 4-channel audio

- and more, encouraging you to build within fun limitations.

# State of The Engine: Proof of Concept
FITU-16 is currently in an early proof-of-concept phase. Many features are either incomplete, missing, or buggy. As of now, the engine supports basic Lua scripting and a simple background display.

But there's much more planned — check out the [Roadmap](#roadmap) below!

# Features (Implemented, Planned and In Progress)
- 🟢 = **Implemented**  🔵 = **In Progress**  🟣 = **Planned**
<br> <br />
- [Lua](https://www.lua.org) as the main scripting language. 🟢

- Modular engine logic: dynamically add or remove modules as needed. 🔵

- Input support for keyboard, mouse, and touch. 🔵

- File Explorer: browse, rename, delete, or create scripts and assets. 🔵

- Console interface for debugging games and the editor itself. 🟣

- Code Editor: edit code and data files with syntax highlighting. 🟣

- Sprite Editor: create and manage sprites for your game or tools. 🟣

- Auto-Tile Editor: design modular terrain with minimal effort. 🟣

- World Editor: build game maps using tiles and sprites you've created. 🟣

- Sound Editor: design sound effects for use in your games or tools. 🟣

- Music Editor: compose music to bring your retro games to life. 🟣

- All Platforms Support: mobile, consoles and pc. 🟣