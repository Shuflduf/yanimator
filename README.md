# Yanimator

Animation editor for the Rhythm Tengoku decompilation (WIP)

Yanimator currently technically has all the tools necessary to edit animations, but things are still very unstable and unpolished. It is not recommended to make a mod with this tool quite yet!

Instructions on how to use the program will be added when the first stable release is finished.

# To-do

### Exporting/Saving

- [x] Put animations in the `cells.bin` so it can be renamed something like `(project name).yan` for easier sharing
- [x] Export animation cells to a `_anim_cels.c` file
- [x] Export animations to a `_anim.c` file
- [ ] If .yan file is broken (missing `YAN` signature or animation offset is longer than file length), give an error popup instead of crashing
- [ ] ...maybe .yan should just be a json file, it would allow for backwards compatibility if .yan changes, and would be a lot easier to edit externally...

### Animation/timeline editing

- [x] Selecting keyframes in timeline (shift+click multiple)
- [x] Moving keyframes in timeline
- [x] Adding keyframes into timeline
- [x] Deleting keyframes in timeline
- [x] Keybinds for timeline (space to play, arrow keys to seek, del to delete, etc)
- [ ] Copy/paste keyframes in timeline
- [ ] Click and drag to select keyframes
- [x] Remove the "end" keyframe and make the total animation length a slider (the end keyframe hates me and causes a lot of issues)
- [ ] Make timeline zooming centered on where mouse cursor is and not at the beginning of the timeline
- [x] Adding animations (...idk how i missed this)
- [x] Removing animations
- [ ] Double click (or some other keybind) on a keyframe to open editor for it's AnimationCel
- [ ] Make keyframes opposite color when in light mode

### AnimationCel/OAM editing

- [x] Adding/removing animation cells
- [x] Sort AnimationCels properly in sidepanel
- [x] Put buttons next to AnimationCels in a right click menu instead
- [ ] Put AnimationCels used in selected animation at top of sidepanel
- [ ] Better visual for what OAM is selected (outline probably)
- [ ] Properly clamp Tile ID based on shape and size
- [ ] Ability to select Tile ID from spritesheet
- [ ] Ability to select multiple OAMs
- [ ] Move tool with snapping
- [ ] Arrow keys for more precise OAM movement

### QoL stuff

- [ ] Preview of AnimationCells and OAMs in side panel, and maybe in timeline too (something like paint.net's layers window)
- [ ] Undo/Redo

# Attributions

Icons used are from [FamFamFamSilkIcons](http://www.famfamfam.com/lab/icons/silk/) by Mark James, licensed under a [Creative Commons Attribution 2.5 License.](https://creativecommons.org/licenses/by/2.5/)


(...yea, the link is dead because this icon pack is ancient, i got the icons from [here](https://github.com/markjames/famfamfam-silk-icons))
