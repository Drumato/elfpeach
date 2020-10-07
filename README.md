# elfpeach

![sample.gif](./sample.gif)

An TUI based elf analyzer

## TODO

- [x] ELF header
- [x] section header table
  - [x] each section information
    - ex. symbol table's relative string table
  - [ ] hexdump
- [x] program header table
- [x] symbols
- [x] dymanic information
  - [ ] each dynamic information
    - ex. shared library name
- [ ] relocation symbols
- [ ] Filter by attribute

## Usage

```
cargo run <file-path>
# or
./elfpeach <file-path>
```

|  key  |  description  |
| ---- | ---- |
|  `q/Esc`  |  quit  |
|  `←/→`  |  change attribute  |
|  `↑/↓`  |  change section/segment/symbol  |
