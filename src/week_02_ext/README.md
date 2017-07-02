
List of changes from the original tutorial done this far week.

1. Moved most of the global variables into the objects they described. (e.g. `MAP_WIDTH` into `map.width()`)
2. Split up the one big file into different files.
3. Changed the map from `vec<vec<tile>>` to `vec<tile>` and included some helper methods for that.
4. Added tile rendering. Not completely happy with this, but for a proof of concept it works for now.
    - Moved the rendering function to the render_all method and removed the draw() function from objects
    - Created 2 structs to define draw information and some static variables to define these constants


Thoughts on the tile rendering.
I want to think of the rendering to be set into a "mode" either tile or ascii. When its in one of these modes it would do a straight code path for that rendering. So just reading only the data needed for that rendering mode and little branching during the rendering call (no if statement per object or virtual dispatch). In addition it would be nice if we only had the current modes data stored on each object so we didn't waste space with the other renderer modes data. 

But all of that is extra work for something that might be thrown away later so this works for now.
