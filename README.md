# Palette Generator

Palette Generator is a small Rust application that extracts dominant colors from an image and produces CSS variables ready for web projects.

The demo works with sample image metadata, but the architecture is prepared for integration with real image libraries.

---

## Pipeline

Image
   │
   ▼
Scan
   │
   ▼
Color Extraction
   │
   ▼
Palette
   │
   ▼
CSS Variables

---

## Example

```
Loading image...

mountains.jpg

Detected colors

#4F6D7A
#D9A441
#2E3F4F
#F2F2F2

Generated CSS

:root {

--color-1:#4F6D7A;
--color-2:#D9A441;
--color-3:#2E3F4F;
--color-4:#F2F2F2;

}
```

Run

```bash
cargo run
```
