# Z17 Randomizer

A randomizer for The Legend of Zelda: A Link Between Worlds, built off the [original ALBW Randomizer](https://gitlab.com/marsolk/albw-randomizer).

## New Features

- Highlights:
  - Portals to Lorule are open from game start
  - Ravio's Bracelet given out immediately (option)
  - Glitched Logic option
  - Ravio's Shop is fully open from game start
  - Maiamai are available to be collected from game start (still need bombs to turn them in)
  - Improved UI and auto-retry mechanism for seed generation


- Additional Changes:
  - All Hilda Text cutscenes have been skipped (except Dark Maze)
  - Skip Sahasrahla outside Link's House
  - Skip Ravio's Sign Trigger cutscene
  - Shady Guy Trigger is active from game start
  - Hyrule Hotfoot open from game start
  - Monster Guts/Horn/Tail initial pickup scenes skipped
  - Energy Potion initial pickup scene skipped
  - Fix Sahasrahla Softlock
  - Maiamai Map available by default
  - Map Swap icon available by default
  - Master Ore UI element present by default
  - Experimental option "Night Mode" changes Hyrule's lighting until you visit Lorule
  - Remove Standard Mode option

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

## Known Issues

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

## License

This program is licensed under the GNU General Public License v2.0.
