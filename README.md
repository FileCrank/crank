# FileCrank

FileCrank is a WIP many-to-many file converter, written in Rust. 

In FileCrank, a conversion between one file format and another is expressed in terms of a graph, where each format is a node, and each conversion is an edge. The cost of an edge is defined based whether the conversion is lossy, whether the data retains it's format, and other salient factors. Expressing this as a graph allows multi-hop conversions - for example, if I implement a conversion between TXT and RTF, and one between RTF and DOCX, we can automatically convert from TXT to DOCX.

Goals: 
- [ ] Implement a number of basic conversions
- [ ] Publish the core library as a crate
- [ ] Build a web app with WASM
- [ ] Build a desktop app with Tauri
- [ ] Support large (multi-gigabyte) file conversions, and figure out a place besides memory to put the buffer we need for multi-hop
