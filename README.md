# Map Generation (placeholder name)

I wanted to make a simple game where NPCs are played by LLMs. Then I needed a map to put NPCs in. Then I needed a procedural map, biomes, procedural cities, procedural NPCs, procedural quests, procedural everything.

## Structure

- `game` is a world viewer powered by Ratatui

  Start with `cargo run -r -p game`

  Keys:

  - arrow keys to move
  - tab to switch between local and global view (view chunks or cells)
  - ctrl will speed up movement

  Chunks are generated as the player moves

- `game_core` is the world generation at the moment
- `perlin_to_image` is a simple script to generate a perlin noise image
- `llm_backend` is the bridge to LLM APIs

## Checklist

- [x] Procedural terrain generation
    ![image](https://github.com/user-attachments/assets/3b5ce26d-b6c2-4f03-8bd0-8db0a97d3bb0)

- [x] Temperature based on latitude (with smaller variation based on local height)
    ![image](https://github.com/user-attachments/assets/b2e9498e-01fe-4b07-88a4-9e8b7a7b8518)
    ![image](https://github.com/user-attachments/assets/15c77c63-5455-4681-8d19-ec372c76c55d)


- [ ] Procedural biome generation (most likely based on voronoi diagrams)
- [ ] Procedural city generation (don't know how to place cities yet)
- [ ] LLM backed NPCs
