# emoji crafter

a command line tool for automating emoji exports from svg, including animation

## emojiset files
### emoji.toml

the emojiset manifest file, used for defining what assets are used by the project, and what will be exported at build time.

```toml
[emojiset]
# human readable name for the project
name = "my emojis"
document = "emojiset.svg"
stylesheet = "emojiset.css"

[[theme]]
name = "my emojis"
prefix = ""
stylesheet = "themes/my emojis.css"

[[output]]
# not all platforms work well with
# emoji that aren't square, so the
# option to trim is disabled
trim = false
directory = "original"

[[output]]
# some platforms work best with the
# transparent parts cropped from
# the emoji
trim = true
directory = "trimmed"
```


### emojiset.svg

each emoji is a group that has a desc which contains some toml describing how that group should be exported. for a static image emoji, it looks like:

```toml
type = "image"
# name of the emoji, prefixed with
# a theme name on export
name = "bunne"
```

animations are much the same:

```toml
type = "animation"
name = "bunnehop"
```

however they also contain groups which make up the individual frames of the animation:

```toml
type = "frame"
# delay before the next frame in ms
delay = 60
# animation timeline position
position = 1
```


### emojiset.css

included directly into `emojiset.svg` for editing, it is not used at build time, instead the individual theme css is used. this separation allows for convienience styles while editing.