# belo-rx-v10: how it is played

Only for an RX 9070 XT with 32 GB of RAM or more on Adrenalin 26.11.2 (see [README.md](README.md)).

Measured on an RX 9070 XT (Adrenalin 26.11.2), Ryzen 7 5800X, 32 GB, Windows 11, Series X, RT off.
At the safehouse balcony (about 8,500 draws a frame): 18.5 fps median, p10 17.5, 54 ms a frame.
The same spot on a-publier `4eb6cfd` + #477 was 13.0 fps.

## What is in the branch

a-publier `4eb6cfd`, plus:

- notquiteog's BVH fixes (#385), #413, #414, #415, #394
- #477 with #476 (`VICTORY_WC_NO_BACK`), #512 (`VICTORY_TEXTURE_BUDGET_MB`), #374 (idle switches)
- #393 reverted: with it, the player character's head and neck skin did not draw on this card
- three ports, all off unless set:
  - `VICTORY_PORT_CONSTANTS_ORDER=1` (from Grimkaaah/Victory-Grimkaaah): fixes trees dropping out with `PORT_CONSTANTS`, but cost about 3 fps here
  - `VICTORY_REAPER=1` (from Grimkaaah, after mgrein): no measurable gain here
  - `VICTORY_AMBIENT_REFLECT=<multiple>` (from maren-halv/VIctory-Fast): bounds RT-off reflections indoors; rooms look less washed out, shiny floors stay

## Intro skip

`VICTORY_SKIP_MOVIES=size=84828076` (GTA VI 1.0.2.1, Series X) skips the artwork and Rockstar logo movie that plays after the start screen: loading art appears about 3.5 s after the button press instead of about 16 s (one run each). `VICTORY_LOG_MOVIES=1` logs the name of every movie the title opens. Also sent upstream: https://github.com/Marvisak/VIctory/pull/3550

## Build

```
cargo build --locked --release -p victory --features diagnostics
```

Leave out `--features diagnostics` for the build to play with.

## Settings

Paths (`VICTORY_GAME_DIR`, `VICTORY_CACHE_DIR`, `VICTORY_DATA_DIR`, `VICTORY_FIDELITYFX_DIR`) are your own. `VICTORY_FSR_4K` needs AMD's signed `amd_fidelityfx_upscaler_dx12.dll` in `VICTORY_FIDELITYFX_DIR`.

```
VICTORY_TEXTURE_BACK=1
VICTORY_MAP_THREADS=1
VICTORY_INDIRECT_BATCH=2
VICTORY_CARD_STEPS=2
VICTORY_AMBIENT=0.8
VICTORY_UPLOAD_AHEAD=1
VICTORY_JOIN_PARTS=8
VICTORY_RELAY=2
VICTORY_INDIRECT_WINDOW=1
VICTORY_CARD_REHASH_RUNS=1
VICTORY_SHOW_AFTER=1
VICTORY_JOIN_COPIES=1
VICTORY_TITLE_PRIORITIES=0
VICTORY_WIDE_BACK=1
VICTORY_RELAY_COPY=1
VICTORY_CARD_AWAKE=1
VICTORY_TEXTURE_NO_WAIT=1
VICTORY_THIN=1
VICTORY_SIZE_OWN_HANDLE=1
VICTORY_BVH_BUILDER=0
VICTORY_RESIDENCY=1
VICTORY_PIPELINE_BOOK=1
VICTORY_PANEL=1
VICTORY_PORT_JOURNAL=1
VICTORY_PORT_GUARD=1
VICTORY_PORT_CONSTANTS=1
VICTORY_WC_NO_BACK=1
VICTORY_FSR_4K=balanced
VICTORY_TEXTURE_BUDGET_MB=6000
VICTORY_STAGING_IDLE_MS=10000
VICTORY_WINDOW_IDLE_MS=10000
VICTORY_READ_KEEP_MB=32
VICTORY_WINDOWS_BAR_MB=2048
VICTORY_CARD_BUFFERS_MIN_KB=64
VICTORY_CARD_BUFFERS_MB=4096
VICTORY_AMBIENT_REFLECT=0.5
VICTORY_WIDE_SPIN=200
```

Run with `--console series-x`.

## Notes

- `CARD_BUFFERS_MIN_KB=64` + `CARD_BUFFERS_MB=4096` were the biggest win: +29% fps at the same spot (15.9 to 20.5), preparing 50 to 36 ms.
- VRAM peaks around 12-13 GB of a ~14 GB budget with these. With ray tracing on it may not fit; untested.
- RAM sits at 26-29 GB on a 32 GB PC. Hitches come back when free RAM drops under about 3 GB.
- Tried and dropped: `VICTORY_HEAP_SYNC_EARLY=1` (no change), `VICTORY_WINDOW_KB=1024` + `VICTORY_WINDOW_TAILS=1` (3x the windows, same memory, -3 fps).
- `VICTORY_FSR_4K=balanced` or `performance` may show dark patches on characters (#355); `quality` does not.
