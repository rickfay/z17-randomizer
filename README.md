# Z17 Randomizer

A randomizer for The Legend of Zelda: A Link Between Worlds, built off the [original ALBW Randomizer](https://gitlab.com/marsolk/albw-randomizer).

## New Features

- Highlights:
  - Portals to Lorule are open from game start
  - Start with Ravio's Bracelet option
  - Glitched Logic option
  - Ravio's Shop is fully open from game start
  - Maiamai are available to be collected from game start (still need bombs to turn them in)
  - Improved UI and auto-retry mechanism for seed generation


- Additional Changes:
  - All Hilda Text cutscenes have been skipped (except Dark Maze)
  - Skip Sahasrahla outside Link's House
  - Skip Ravio's Sign Trigger cutscene
  - Shady Guy and Merchant's right-side item are available from game start
  - Hyrule Hotfoot open from game start
  - Monster Guts/Horn/Tail initial pickup scenes skipped
  - Energy Potion initial pickup scene skipped
  - Fix Sahasrahla Softlock
  - Maiamai Map available by default
  - Map Swap icon available by default
  - Master Ore UI element present by default
  - Remove Standard Mode option
  - Experimental option "Night Mode" changes Hyrule's lighting until you visit Lorule. There are some performance issues on console, so not advised for races.

## Setup

Download: [Latest Stable Release](https://github.com/rickfay/z17-randomizer/releases/download/alpha-0.0.10/z17-rando.zip)

1. Unzip the download to your directory of choice.
2. Move your A Link Between Worlds ROM into the same folder. Name it: `A Link Between Worlds (USA).3ds`
   - If your ROM is located elsewhere or if you'd like to name it something different, you can modify the `config.toml` file to point to the ROM instead

![fs-setup.png](docs/fs-setup.jpg?raw=true)

## Running the Randomizer

There are two ways you can run Z17R:

1. Double click `z17-randomizer.exe` to start the randomizer with basic settings. The randomizer will provide a simple interface for setting game options, after which it will attempt to generate a completable seed.

![cli-example.png](docs/cli-example.png)

2. Use a command line interface. If you take this approach you may also specify a preset and/or seed to use for seed generation.
   - Using a preset will give you some additional options and allow you to manually configure excluded checks.
     - See the example `presets/Standard.toml` for more information.
   - Note that the randomizer looks in the local `presets` directory now, and does NOT check `AppData`
   - Examples:
     - `$ ./z17-randomizer.exe --preset racerman`
     - `$ ./z17-randomizer.exe --seed 4057320268`

## Game Options

`start_with_bracelet`
- Causes Ravio to give out the Bracelet at the start of the game. More specifically, the Bracelet will be placed in the Bow Item's Slot in the shop, and Ravio will give you the item on that slot.

`glitched_logic`
- Enables items to be placed in locations that may require glitches/tricks to obtain. A complete list of potentially required glitches/tricks is below.

`dont_require_lamp_for_darkness`
- If enabled, the logic may place required items behind completely dark rooms without giving you the Lamp first.

`minigames_excluded`
- Excludes Cucco Rush, Hyrule Hotfoot, Treacherous Tower, Octoball Derby, and both Rupee Rush minigames from having progression.

`unsafe_key_placement`
- Randomizes the keys in an unsafe way that assumes the player will make the best possible routing choices. It's recommended that you don't enable this unless you know what you're doing.

## Glitched Logic Breakdown

The following tricks may potentially be required in a Glitched Logic seed:

- Rosso Cave with Boomerang or Hookshot (not TRod + Shield)
- Blacksmith Cave with Fire Rod or Nice Bombs
- Reach Death Mountain with Power Glove Skip
- Eastern Ruins Pegs with Boomerang or Hookshot or Tornado Rod (no Sand Rod)
- Flipperless Message in a Bottle with Fire Rod or Nice Bombs
- Access HoG with Fake Flippers or Ice Rod + Hookshot
- Lost Woods Chest with Boomerang or Hookshot and a means of escape (FRod, Bombs, or Bell. No Crow)
- Reverse Sanctuary (door is open after opening Gravestone, just need damage source to defeat Poes)
- Flippers Treasure Dungeon tricks. Need either Titan's Mitt or Ice Rod to enter. Then either just Hookshot + Flippers OR simply the Nice Bombs. Nice Ice Rod and Great Spin not considered.
- Zora's Waterfall Cave with the Crow Boost (FRod not needed)
- Pegasus Boots Dungeon with just Master Sword, Bombs, or Boomerang (Nice Bow and Nice Ice Rod not considered)
- Access Turtle Rock with Fake Flippers
- Misery Mire HP without Sand Rod. With just Bombs (regular works with Vulture boosting) or FRod + Boots
- Eastern Palace
  - Left entrance chest with a thrown Pot
  - 4 Switches Room with thrown Pots
  - TRod Armos Boost to get to Boss Key or Boss room without the appropriate key
- House of Gales
  - 2F Fire Ring Key using HoG Skip Skip to do 2F backwards if key missing (incredibly specific, unlikely)
  - Skip Skip Skip to reach 3F without keys
- Thieves' Hideout
  - Flipperless Thieves using TRod + a way to hit the switch (IRod or Bombs, no sword beams)
- Ice Ruins
  - Scroll Skip with Boots
- Desert Palace
  - Reverse DP means keys can show up anywhere in the dungeon if you have FRod or Nice Bombs
- Lorule Castle
  - Lamp Trial doesn't need the Lamp

  
Mergeless Tricks if `start_with_bracelet` is false:
- Behind Blacksmith with Fire Rod or Nice Bombs
- Death Mountain climb
- DM Fairy Cave with Fire Rod or Nice Bombs
- Bouldering Guy with TRod + Boots
- Access Eastern Ruins with just Power Glove
- Eastern Ruins Merge Chest with TRod, FRod, or Nice Bombs
- Lost Woods Alcove with Boomerang or Hookshot and a means of escape (FRod, Bombs, or Bell. No Crow)
- House of Gales TRod onto moving block in 1F East Room (if key logic allows access)
- Mergeless Hera with Bombs and Sword to bypass lobby. TRod needed for floors higher than 3F


Fake Flippers requires Pegasus Boots and either Fire Rod or Nice Bombs


Some notes about Nice Bombs:
  - Glitched logic *DOES* guarantee at least 10 Maiamai will be available for any checks requiring Nice Bombs
  - If you spend your Maiamai on anything other than Nice Bombs, you may potentially doom your seed. Regular Bomb boosts may save you, but if progression is on Southern Ruins Treasure Dungeon with Nice Bombs there's no salvaging it. YOU HAVE BEEN WARNED.

## Known Issues

- Bow of Light crashes the game if used outside the final boss arena (on console)
- After Ravio gives out the Bracelet from the Bow Item Slot in his shop, the model remains until you leave and reenter.
- Despite Maiamais being available for collection from game start, the Maiamai cave itself is still sealed shut and thus Bombs are still required to turn in any Maiamai
- Opening the Portals early has some side effects:
  - The Curtain over the Hyrule Castle Portal is not torn down, so if you attempt to enter the Portal right before the final boss fight, you'll notice some... odd behavior. This is NOT a softlock, but it is confusing. If you have the Lamp you can use it to burn down the Curtain from behind it. Otherwise you'll eventually void out and return to Lorule Castle.
  - The Yuga 2 Boss fight can be started but Yuga 2 cannot be defeated. If you do enter the boss arena, you'll find that Yuga gets stuck circling the room in his merged form, but that because the entrance door is opened he just stalls. Defeating Yuga 2 is no longer necessary as Portals are open, so you can just leave.

## Future Plans

Time and technical knowledge (or lack thereof) are the main barriers to future progress, but I will share my wishlist of features to eventually add:

- Keysanity
- Shuffled Pendants and Paintings
- Portal Randomizer
- Include Nice Items as Progressive Items
- Randomize Maiamais
- Randomize Maiamai Rewards for 10, 20, 30, etc.
- Great Spin
- Hyrule Hotfoot, preclear the first race
- Treacherous Tower, preclear the first round
- Fix existing Known Issues
- Other Stuff <sup>TM</sup>

![future.png](docs/future.png)

## Special Thanks

- Tocapa for building the ALBWR foundation this hack built from
- My Beta Testers: Br00ty, j_im, flora, and Herreteman

## License

This program is licensed under the GNU General Public License v2.0.
