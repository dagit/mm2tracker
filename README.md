# Introduction

This is a tracker that can be used to display your progress when playing mega
man 2. In particular, I created this for displaying progress during a
randomized play through of the game. To randomize your mm2 ROM, take a look at
[Mega Man 2 Randomizer by Duckfist](https://github.com/duckfist/MM2Random).

# Usage

The easiest way to use the tracker is to visit the [version I
host](https://codersbase.com/mm2tracker). Since the tracker is currently just a
single html file and a set of images, you can also run the tracker by cloning
this repository and pointing your browser at the index.html file.

If you are streaming and want to display the tracker using OBS there are two
primary ways. If you have the tracker open in a browser window, then you can
tell OBS to capture that window. Doing it this way you'll also need to crop the
view to remove window decorations and that sort of thing. The other option is
to use OBS's built in browser source. If you go the latter route, you'll also
want to right click the browser source and select "Interact" so that you can
use the tracker. Note: The tracker's background is set to transparent. If you
capture it using a browser source this transparency will work correctly but if
you capture the browser window instead, you may get a black background instead.

All the images displayed by the tracker can be toggled by clicking on them.
Initially you will see the 8 robot masters and three items to the right.
Clicking on a robot master portrait will place a red X on that robot master.
Clicking an item will toggle it from black and white to the normal colors
(white and red). To reset the interface, either toggle each image or refresh
the browser page.

I usually check off robot masters by stage. For example, if you visit Airman on
the stage select menu and defeat Metal Man, I would still check off Airman in
the tracker because that is the stage I completed. Of course, how you use the
tracker is entirely up to your personal perferences.

# Future Work

In the future I would like to make a native app version. This is primarily
because I'm not fond of the way the OBS browser source interaction works. This
is a low-priority hobby project for me so I can't really give a good estimate
when/if that native version will land.

# Issues & Suggestions

If you have any issues, suggestions, then please create a ticket in the github
issue tracker. I'll probably see it right away, but I may be slow to respond so
if it's been a while without a response feel free to message me again.
