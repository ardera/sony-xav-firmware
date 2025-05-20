# Sony XAV-AX5550D Firmware Reverse Engineering

### About
Sony XAV-AX5550D is an after-market big (2-DIN) Car Stereo that supports:
- FM, AM, and 3 DAB Tuners
- Bluetooth Audio playback
- Parking Camera
- Android Auto, Apple CarPlay
- Weblink Cast (Screensharing Phone to Car Stereo Display)

I use it in my personal Mazda2 from 2011 as a replacement for the original car stereo, which didn't support bluetooth nor Android Auto. It seemed like a good alternative to all the cheap Android-based car stereos, which use are slow and use super outdated android versions.

More Product information:
- https://www.sony.de/electronics/support/mobile-cd-players-digital-media-players-xav-series/xav-ax5550d

### Firmware
A firmware update to version 2.0 can be downloaded at: https://www.sony.de/electronics/support/mobile-cd-players-digital-media-players-xav-series/xav-ax5550d/software/00274154

A recovery firmware update, v2.0 can be downloaded at:
https://www.sony.de/electronics/support/mobile-cd-players-digital-media-players-xav-series/xav-ax5550d/software/00350379

The firmware update is a zip archive and contains 3 files, apparently meant for different components of the radio.
The recovery firmware website differentiates between MCU and CPU, and is meant to be used if updating the MCU firmware to v2.0 failed.

A bit puzzling is that the firmware update is only 8MB large. The radio supports android auto, carplay & bluetooth. Android Auto requires h264 decoding. If this was a full linux yocto image running on the board, I'd expect this to be larger.

#### `SHSO2001.FIR`: The largest firmware file

- The file size is oddly specific. It's exactly 128 bytes larger than 8MiB: `8388736 = 8 * 1024 * 1024 + 128`.
- The first 0x80/128 bytes look like header information.
- Why is the payload exactly 8MiB? Are they just flashing it as-is onto an EEPROM?
  That's very unusual for a linux device though.

##### Header: `0x00 .. 0x80`
```
+--------+-------------------------+-------------------------+--------+--------+
|00000000| 20 01 00 00 00 00 00 00 | 00 00 00 00 00 80 00 00 | .......|........|
|00000010| 4b 52 53 45 4c 45 43 4f | 00 00 00 00 00 00 00 00 |KRSELECO|........|
|00000020| 31 32 41 50 52 32 30 32 | 34 28 31 37 3a 34 35 29 |12APR202|4(17:45)|
|00000030| 53 4b 49 50 00 00 00 00 | 00 00 00 00 00 00 00 00 |SKIP....|........|
|00000040| 00 00 00 00 00 00 00 00 | 00 00 00 00 00 00 00 00 |........|........|
|*       |                         |                         |        |        |
|00000080| 76 ac 7e 28 c6 1a a4 f5 | 3d 48 ed f5 d0 1f 76 8a |v.~(....|=H....v.|
|00000090| e4 fb 64 5d 03 a0 93 53 | 34 82 de 66 f4 8a 90 f9 |..d]...S|4..f....|
|000000a0| 08 1e 13 5d 56 67 0d 8d | c9 2a bc 60 b6 22 d3 0f |...]Vg..|.*.`."..|
|000000b0| b1 3b b2 71 ef e8 b7 ce | 28 18 d2 a6 6c ff c5 19 |.;.q....|(...l...|
```

- `0x20 01` file header? firmware version? (2.01?)
- `0x80` at `0x0D`, maybe an offset? a flag?
- `KRSELECO` probably stands for KRS Electronics, a south-korean car stereo OEM.
- `12APR2024(17:45)` build date probably
- `SKIP` not sure

##### Payload

The payload looks mostly random.
There are large areas that are 16-byte sequences of `b7 72 10 03 00 8c 82 7e ┊ aa d1 83 58 23 ef 82 5c`

Maybe the areas are XORed, and the 16-byte sequences are where the zeroes are?

At the end of the file there's a large area of these "zeroes" potentially, followed by
`5*16` bytes of non-zero data.
Maybe archive (zip) header or signature?

End of file:
```
|00800000| b7 72 10 03 00 8c 82 7e | aa d1 83 58 23 ef 82 5c |.r.....~|...X#..\|
|00800010| b7 72 10 03 00 8c 82 7e | aa d1 83 58 23 ef 82 5c |.r.....~|...X#..\|
|00800020| b7 72 10 03 00 8c 82 7e | aa d1 83 58 23 ef 82 5c |.r.....~|...X#..\|
|00800030| 7d 86 a3 b5 33 0f 31 10 | 55 1b f0 16 67 27 07 47 |}...3.1.|U...g'.G|
|00800040| 64 02 a7 64 13 41 b0 8b | ca 59 3c c2 a7 64 24 93 |d..d.A..|.Y<..d$.|
|00800050| 9d 30 c8 08 56 09 02 e3 | 7b 90 36 0b 15 4f 41 84 |.0..V...|{.6..OA.|
|00800060| 30 1e 30 38 3b 7c 7b aa | bd 32 b7 c7 b9 81 8f 15 |0.08;|{.|.2......|
|00800070| e6 24 f6 9d cf 07 ef 87 | ec 55 75 30 c9 de f3 ad |.$......|.Uu0....|
+--------+-------------------------+-------------------------+--------+--------+
```

Code contained here un-xors (or re-xors) the areas with the potential key. (The 16-byte sequence above)
