# pdf-lens.rs

A command-line tool for inspecting the structure of PDF files.

## Usage

```sh
cargo build
% .\target\debug\pdf-lens.exe inspect .\sample\sample.pdf
```

Result:

```
[[Doc Info]]
  Version: 1.3
-- Objects (len=7) --
[[Object: (1 0)]]
  <<Dictionary (len=4)>>
    [[Key: "Type"]]
      <<Name: "Pages">>
    [[Key: "Kids"]]
      <<Array len=1>>
        <<Reference: (3, 0)>>
    [[Key: "Count"]]
      <<Integer: 1>>
    [[Key: "MediaBox"]]
      <<Array len=4>>
        <<Integer: 0>>
        <<Integer: 0>>
        <<Real: 595.28>>
        <<Real: 841.89>>
[[Object: (2 0)]]
  <<Dictionary (len=4)>>
    [[Key: "ProcSet"]]
      <<Array len=5>>
        <<Name: "PDF">>
        <<Name: "Text">>
        <<Name: "ImageB">>
        <<Name: "ImageC">>
        <<Name: "ImageI">>
    [[Key: "Font"]]
      <<Dictionary (len=0)>>
    [[Key: "XObject"]]
      <<Dictionary (len=1)>>
        [[Key: "I869fc61fd60fa42f82f2af6ec00eddbd7121aa6d"]]
          <<Reference: (5, 0)>>
    [[Key: "ColorSpace"]]
      <<Dictionary (len=0)>>
[[Object: (3 0)]]
  <<Dictionary (len=4)>>
    [[Key: "Type"]]
      <<Name: "Page">>
    [[Key: "Parent"]]
      <<Reference: (1, 0)>>
    [[Key: "Resources"]]
      <<Reference: (2, 0)>>
    [[Key: "Contents"]]
      <<Reference: (4, 0)>>
[[Object: (4 0)]]
  <<Stream>>
    [[Dictionary]]
      <<Dictionary (len=2)>>
        [[Key: "Filter"]]
          <<Name: "FlateDecode">>
        [[Key: "Length"]]
          <<Integer: 114>>
    [[Data (len=114)]]
      <<Content (len=9)>>
        [[Op: "J" (len=1)]]
          [[Operand: "0"]]
          <<Integer: 0>>
        [[Op: "j" (len=1)]]
          [[Operand: "0"]]
          <<Integer: 0>>
        [[Op: "w" (len=1)]]
          [[Operand: "0"]]
          <<Real: 0.57>>
        [[Op: "G" (len=1)]]
          [[Operand: "0"]]
          <<Real: 0>>
        [[Op: "g" (len=1)]]
          [[Operand: "0"]]
          <<Real: 0>>
        [[Op: "q" (len=0)]]
        [[Op: "cm" (len=6)]]
          [[Operand: "0"]]
          <<Real: 566.92914>>
          [[Operand: "1"]]
          <<Integer: 0>>
          [[Operand: "2"]]
          <<Integer: 0>>
          [[Operand: "3"]]
          <<Real: 425.19684>>
          [[Operand: "4"]]
          <<Real: 14.17543>>
          [[Operand: "5"]]
          <<Real: 208.34657>>
        [[Op: "Do" (len=1)]]
          [[Operand: "0"]]
          <<Name: "I869fc61fd60fa42f82f2af6ec00eddbd7121aa6d">>
        [[Op: "Q" (len=0)]]
[[Object: (5 0)]]
  <<Stream>>
    [[Dictionary]]
      <<Dictionary (len=8)>>
        [[Key: "Type"]]
          <<Name: "XObject">>
        [[Key: "Subtype"]]
          <<Name: "Image">>
        [[Key: "Width"]]
          <<Integer: 5456>>
        [[Key: "Height"]]
          <<Integer: 3632>>
        [[Key: "ColorSpace"]]
          <<Name: "DeviceRGB">>
        [[Key: "BitsPerComponent"]]
          <<Integer: 8>>
        [[Key: "Filter"]]
          <<Name: "DCTDecode">>
        [[Key: "Length"]]
          <<Integer: 7077888>>
    [[Data (len=7077888)]]
        [[ImageData Blob]]
[[Object: (6 0)]]
  <<Dictionary (len=3)>>
    [[Key: "Producer"]]
      <<String: "��FPDF 1.7", format=Literal>>
    [[Key: "CreationDate"]]
      <<String: "D:20250807042717", format=Literal>>
    [[Key: "ModDate"]]
      <<String: "D:20250807042717", format=Literal>>
[[Object: (7 0)]]
  <<Dictionary (len=3)>>
    [[Key: "Type"]]
      <<Name: "Catalog">>
    [[Key: "Pages"]]
      <<Reference: (1, 0)>>
    [[Key: "Names"]]
      <<Dictionary (len=1)>>
        [[Key: "EmbeddedFiles"]]
          <<Dictionary (len=1)>>
            [[Key: "Names"]]
              <<Array len=0>>
-- Trailer --
  <<Dictionary (len=3)>>
    [[Key: "Size"]]
      <<Integer: 8>>
    [[Key: "Root"]]
      <<Reference: (7, 0)>>
    [[Key: "Info"]]
      <<Reference: (6, 0)>>
```

# License

AGPL v3 or later.
