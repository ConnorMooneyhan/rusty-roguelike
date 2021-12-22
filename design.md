# Rusty Roguelike Design Document

## Short Description
A dungeon crawler with procedurally-generated levels, monsters of increasing difficulty, and turn-based movement.

## Story
Beneath the stormy sky of Gerlingtown lurks a strange and ominous horde. Silently they grow, amassing their army, lying in wait until the perfect time to strike. Nothing can stop them except the legendary Amulet of Paxus, which is said to be hidden deep within a long-abandoned dungeon -- the very same dungeon that the monsters have made their dwelling. Our hero, Aliana, must face this great danger in order to give her people a chance to survive. For the sake of Gerlingtown, she must make it out alive...

## Basic Game Loops
1. Enter dungeon level.
2. Explore, revealing the map.
3. Encounter enemies whom the player fights or flees from.
4. Find power-ups and use them to strengthen the player.
5. Locate the exit to the level - go to 1.

## Minimum Viable Product (MVP)
- [ ] Create a basic dungeon map.
- [ ] Place the player and let them walk around.
- [ ] Spawn monsters, draw them, and let the player kill them by walking into them.
- [ ] Add health and a combat system that uses it.
- [ ] Display a "game over" screen when the player dies.
- [ ] Add the Amulet of Paxus to the level and let the player win by reaching it.

## Stretch Goals
- [ ] Add Fields-of-View
- [ ] Add more interesting dungeon designs.
- [ ] Add some dungeon themes.
- [ ] Add healing potions.
- [ ] Add multiple layers to the dungeon, with the Amulet on the last one.
- [ ] Add varied weapons to the game.
- [ ] Move to a data-driven design for spawning enemies.
- [ ] Consider some visual effects to make combat more visceral.
- [ ] Consider keeping score.
