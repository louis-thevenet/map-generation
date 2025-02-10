# Map Generation (placeholder name)

I wanted to make a simple game where NPCs are played by LLMs. Then I needed a map to put NPCs in. Then I needed a procedural map, biomes, procedural cities, procedural NPCs, procedural quests, procedural everything.

## Structure

- `world_viewer` lets you generate and explore a world in a TUI

  Start with `cargo run -r -p world_viewer`

  Keys:

  - arrow keys to move
  - tab to switch between local and global view (view chunks or cells)
  - ctrl will speed up movement

  Chunks are generated as the player moves

- `world_gen` is the world generation crate
  It also provides a binary that generates images for different parameters of the biome generation as well as the biome map.

  In order:

  - Biome map
  - Temperature
  - Moisture
  - Continentalness
  - Erosion

  ![biome_map](https://github.com/user-attachments/assets/a00b0484-7f2e-4b1c-8846-5725c100dbba)
  ![temperature_map](https://github.com/user-attachments/assets/7f614520-7a04-44e3-a577-8d0038276083)
  ![moisture_map](https://github.com/user-attachments/assets/87f5e2c4-0b58-4585-9965-cc97255a1410)
  ![continentalness_map](https://github.com/user-attachments/assets/d26e7472-dce3-4b6c-a767-ae6a96f8cf26)
  ![erosion_map](https://github.com/user-attachments/assets/2fb3c164-c423-47a4-abac-9a95679ffcc4)

  Start with `cargo run -r -p world_gen`

- `game_core` is the core logic, provides the `Map` type.
- `llm_backend` is the bridge to LLM APIs

## Checklist

- [x] Procedural biome generation
  [![asciicast](https://asciinema.org/a/4OXnofqoeCJCmHfLWLziuXIij.svg)](https://asciinema.org/a/4OXnofqoeCJCmHfLWLziuXIij)
- [ ] Terrain generation
- [ ] Procedural city generation (don't know how to place cities yet)
- [ ] LLM backed NPCs
