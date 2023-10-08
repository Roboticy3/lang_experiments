so I'm trying to demo some ui designs to learn svelte, here's what I've sorted out so far:

- Connecting buttons to scripts in svelte files
- Loading custom images
- Generating custom UI elements on the fly with things like \#each tags and the like
- How to layout a page of contiguous panels using flexbox.
    - there are flex containers with flex items, and different commands can target either
    - the magic of css is that I'm not supposed to worry too much about how this works under the hood, and a lot of the labels are self-documenting
    - For example, "stretch" fits an item to its container
    - The behavior of center can depend of the flow of the relevant container.

Todo:
- Research how to make more modular code in svelte, instead of just cramming everything in one file.
    - Sending a list of gates to +page when it loads to populate the shelf.
    - Making designs persist for each level even when a user loads a different one
        - Sort out with the professor how he wants to ship this thing
            - if local, research shipping svelte as a local app and saving in that context
    - Saving named designs
    - Rendering the logic simulator in +page
- Make a menu for selecting levels
