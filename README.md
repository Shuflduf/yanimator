# Yanimator

Animation editor for the Rhythm Tengoku decompilation (WIP)

# To-do

### Exporting/Saving

- [x] Put animations in the `cells.bin` so it can be renamed something like `(project name).yan` for easier sharing
- [ ] Export animation cells to a `_anim_cels.c` file
- [ ] Export animations to a `_anim.c` file

### Animation/timeline editing

- [ ] Selecting keyframes in timeline (shift+click or click+drag for multiple)
- [ ] Moving keyframes in timeline
- [ ] Adding keyframes into timeline
- [ ] Deleting keyframes in timeline
- [ ] Keybinds for timeline (space to play, arrow keys to seek, del to delete, etc)
- [ ] Copy/paste keyframes in timeline

### AnimationCell/OAM editing

- [ ] Adding/removing animation cells
- [ ] Better visual for what OAM is selected (outline probably)
- [ ] Properly clamp Tile ID based on shape and size
- [ ] Ability to select Tile ID from spritesheet
- [ ] Ability to select multiple OAMs
- [ ] Move tool with snapping

### QoL stuff

- [ ] Preview of AnimationCells and OAMs in side panel, and maybe in timeline too (something like paint.net's layers window)
