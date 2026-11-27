# VIctory for the RX 9070 XT

A personal fork of [VIctory](https://github.com/Marvisak/VIctory), tuned for one setup: **Radeon RX 9070 XT, 32 GB of RAM or more, Adrenalin 26.11.2**.

**This repository contains no game files, code or assets.** You need your own copy of the game. It is not affiliated with or endorsed by the game's developer or publisher.

## Supported setup

| | |
|---|---|
| GPU | AMD Radeon RX 9070 XT (16 GB) |
| Driver | Adrenalin 26.11.2 |
| RAM | 32 GB or more |
| CPU tested | Ryzen 7 5800X |
| OS | Windows 11 |
| Game | 1.0.2.1, Series X profile |
| Ray tracing | off |

**Nothing else is supported.** There will be no fixes for NVIDIA, Intel, other AMD cards, less RAM or other drivers, so please do not ask for them. It may run elsewhere; that is untested and at your own risk. The code does not block other hardware, and you are free to fork it.

## Where it stands (2026-11-27)

Not a smooth experience yet. At the safehouse balcony (about 8,500 draws a frame): **18.5 fps median**, p10 17.5, 54 ms a frame. Other places differ.

- The game uses 26 to 29 GB of RAM. Close other programs: hitches start when free RAM drops under about 3 GB.
- Video memory peaks around 12 to 13 GB with these settings. With ray tracing on it may not fit.
- Shiny indoor floors and light specks remain.
- `VICTORY_FSR_4K=balanced` or `performance` may show dark patches on characters; `quality` does not.

## How to run it

Build:

```
cargo build --locked --release -p victory
```

Set `VICTORY_GAME_DIR`, `VICTORY_CACHE_DIR` and `VICTORY_DATA_DIR` to your own folders, set the switches listed in [PLAY.md](PLAY.md), then:

```
victory run <path to the game executable> --execute --console series-x
```

[PLAY.md](PLAY.md) also lists what is in this branch, the intro skip, and what was tried and dropped.

## Credits and license

VIctory is by [Marvisak](https://github.com/Marvisak/VIctory) and its contributors. This branch builds on the a-publier branch of [Belobug's fork](https://github.com/Belobug/VIctory-Belobug-AMD) and on work by others named in [PLAY.md](PLAY.md) and in the [base README](README-base.md). Licensed GPL-3.0-or-later, see [LICENSE](LICENSE).
