# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Plugin | Input Device | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-input-device">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-input-device) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-input-device/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides entity types which allows using raw input from keyboards.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-input-device/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-input-device/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-input-device">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-input-device">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-input-device">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-input-device)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-input-device">](https://github.com/aschaeffer/inexor-rgf-plugin-input-device/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Entity Types

| Name | Property | Data Type | Socket Type |
| --- | --- | --- | --- |
| InputDevice | name | string | output |
| | event | object | output |
| | physical_path | string | output |
| | driver_version | string | output |
| | vendor | number | output |
| | product | number | output |
| | version | number | output |
| InputDeviceKey | key | string | none |
| | keycode | number | none |
| | keydown | bool | output |

#### Relation Types

| Name | Outbound Entity Type | Inbound Entity Type |
| --- | --- | --- |
| KeyEvent | InputDevice | InputDeviceKey |

#### Entity Behaviours

| Name | Description |
| --- | --- |
| InputDevice | Streams input events from evdev and sets the entity instance property `event` |

#### Relation Behaviours

| Name | Description |
| --- | --- |
| KeyEvent | Propagate input events and filters by event type (key event) and keycode defined by the inbound entity instance |

### TODO

1. InputDeviceKey
   1. Add property keyhold (value == 2)
3. Different modes
   1. Autodetect (current behaviour, slows down the initialization)
   2. Flow with keymap (the flow contains a keymap which defines which key shall be created)
   3. TOML config file with input devices and which keys shall be created)
4. Handle LEDs
   1. Entity Type InputDeviceLED (on/off)
5. Mouse input
   1. Entity Type InputDeviceMouse (x, y, lbtn, rbtn, mbtn, wheel, ...)
6. Virtual Keyboard
   1. Create a virtual device (useful for embedded devices without a keyboard)
   2. Send key events via the virtual device

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

| | | |
| --- | --- | --- |
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
