# Map Generation (placeholder name)

I wanted to make a simple game where NPCs are played by LLMs. Then I needed a map to put NPCs in. Then I needed a procedural map, biomes, procedural cities, procedural NPCs, procedural quests, procedural everything.

## Structure

- `game` is a world viewer powered by Ratatui
- `game_core` is the world generation at the moment
- `perlin_to_image` is a simple script to generate a perlin noise image
- `llm_backend` is the bridge to LLM APIs

## Checklist

- [x] Procedural terrain generation
- [ ] Temperature based on latitude
- [ ] Procedural biome generation (most likely based on voronoi diagrams)
- [ ] Procedural city generation (don't know how to place cities yet)
- [ ] LLM backed NPCs
