1. Generate the map SVG
2. Create a `resources` folder in `static` and put the map svg in there as `map.svg`
3. `cargo run`
4. Open `http://localhost:12345/` in a web browser
5. Start up the Project Zomboid server and connect to it
6. The map should automatically update

Open a JS console if you want to try some cli only features. (right click -> inspect -> console)

Commands: 
- `mainModule.renderer.players` - Lists players
- `mainModule.renderer.focus_player("PlayerName")` - Focuses on "PlayerName", must be a key in players object
- `mainModule.renderer.center_on({x: 10000.0, y: 9000.0})` - Moves the camera to the provided coordinates. Doesn't zoom.
