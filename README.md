# mkwii-distro-tool

`Alternative tool to create Pulsar-powered custom track distributions for Mario Kart Wii`

> mkwii-distro-tool is a tentitive name, and it will be changed in the future.

## Initial roadmap

- [ ] Deserialize config file
  - [x] Game settings
  - [x] Cup/Track definitions
  - [ ] BMG
    - [x] Deserialization (BMG -> Structure)
    - [ ] Serialization
      - [ ] Into JSON
        - [ ] Improve visibility for 0x1A Escape Sequence
      - [ ] Into BMG back
  - [ ] Track filenames for editor
- [ ] UI
  - [ ] Cup/Track definition
  - [ ] Cup icon
  - [ ] Ghosts
  - [ ] Drag and drop files
  - [ ] Pulsar feature toggles
  - [ ] Build options
- [ ] Build pack
  - [ ] Serialize config file
  - [ ] Generate assets
  - [ ] Copy tracks and ghosts
  - [ ] Compress

## Supported engine versions (Planned)

- [ ] v2.0.1 (build from [latest commit](https://github.com/MelgMKW/Pulsar/tree/820ad929c3c7141a0396692d8b0896d1546240fd))

## Prerequisite

- Requires Rust v1.92 or later

## Credits

- [MelgMKW](https://github.com/MelgMKW), for their engine [Pulsar](https://github.com/MelgMKW/Pulsar).
- [iced](https://github.com/iced-rs/iced), for UI.
- [Custom Mario Kart Wiiki](https://wiki.tockdom.com), for the documentation.

## License

This source code is licensed MIT, see [LICENSE](https://https://github.com/expiteRz/mkwii-distro-tool/blob/master/LICENSE).
