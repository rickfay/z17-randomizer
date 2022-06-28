# Z17 Randomizer

A randomizer for The Legend of Zelda: A Link Between Worlds, built off the [original ALBW Randomizer](https://gitlab.com/marsolk/albw-randomizer).

## Updates

<details open="open">
<summary>Version 0.1.0</summary>

- New Filler Algorithm:
  - This has been a long time coming :)
  - The filler has been completely rewritten using an **Assumed Fill** algorithm. The old algorithm had a tendency to front-load progression items in the Overworld (often just Hyrule's), and very rarely put anything noteworthy in dungeons. This new algorithm fixes those problems and produces interesting "logic chains" with a much higher frequency.
  - The vanilla Bow of Light issue has been solved once and for all! The new filler has no biases towards Zelda, and excluding her prevents her from having any progression at all.


- Hyrule Castle + "Yuga Seeds"
  - After a lot of remodelling, Hyrule Castle is open once again!
  - Defeating Yuga 2 awards a unique type of progression: Access to Lorule **without Ravio's Bracelet**. This makes it possible for the Bracelet or an item leading to Bracelet to spawn in the main area of Lorule, including the early portions of Thieves' Hideout and Swamp Palace.
  - To accomplish this, the north exit from the Yuga 2 boss fight has been redirected to drop Link into the Lorule Blacksmith. This door will remain open if the player needs to use it multiple times.
  - Note 1: Climbing Hyrule Castle logically requires the Bow or the Ice Rod to kill the Red Spear soldier mergeless on 3F. It can also be done with Nice Bombs or Nice Tornado Rod, but those are considered out of logic.
  - Note 2: If you need to return to Hyrule after reaching Lorule in this way, you need to death warp to return to Link's House. I'm exploring more *elegant* ways to do this in the future, but this is what we have for now.
  - Note 3: Swamp Palace was really not designed for the player to not have merge, with three rooms capable of softlocking the player if they unwittingly entered mergeless. To combat this, 2 doors that normally shut behind the player have been removed. Additionally, the crystal switch in the B1 north room that would normally divert the river has been removed, preventing a softlock that could otherwise occur (the switch was never needed, players could always bypass the river by merging).


- New Logic Modes
  - The logic has been expanded into six (6) different modes. Hopefully there's something for everyone.
    - Normal: Standard Gameplay, no tricky item use or glitches. If unsure, choose this.
    - Hard: Tricks that aren't technically glitches included, lamp + net considered as weapons. No glitches.
    - Glitched (Basic): Includes the above plus "basic", easy-to-learn glitches.
    - Glitched (Advanced): Includes the above plus "advanced" glitches that may be a challenge to master.
    - Glitched (Hell): Includes every known glitch, including the insane ones. Bee Badge not included. Do not choose this. DO NOT CHOOSE THIS.
    - No Logic: Items are placed with no logic at all. Dungeon items are in their respective dungeons. Seeds may not be completable.


- Ravio's Bracelet Changes
  - The second Bracelet has returned to the item pool, meaning you must find both before you can Merge.
  - The `start_with_bracelet` option has been deprecated. The option mostly existed to cover for the old filler algorithm's shortcomings, but now that those have been addressed the option is being retired.


- Pendant of Courage Changes
  - Rosso's House has been unlocked from game start and the check for smashing his pet rocks is logically available anytime after you've obtained the Power Glove.
  - The Irene check has been removed entirely. This check with its multiple invisible triggers caused a lot of confusion for players, so for now we've decided to remove it.
    - Don't worry, you'll still be able to see Irene on her broom after you've rescued her portrait.
  - This leaves the Haunted Grove Tree Stump as *the* Green Pendant-locked check.


- Swordless Mode Changes
  - The Hyrule Castle Barrier will now be removed when playing on Swordless Mode, granting access to 2 otherwise inaccessible chests and the Hyrule Castle dungeon.
  - Yuga 2 can still be fought in Swordless Mode to access Lorule without Bracelet, but note that he cannot be harmed with the Bow alone.


- New Option: `assured_weapon`
  - Guarantees at least one weapon will be placed in Ravio's Shop
    - Items include: Bow, Bombs, Fire Rod, Ice Rod, Hammer, or Sword (if not playing Swordless)


- New Option: `bow_of_light_in_castle`
  - Guarantees Bow of Light will spawn on one of the 15 checks inside Lorule Castle.
    - Note: This includes Zelda, unless you manually exclude her.


- Change to *`_in_shop` settings:
  - When the Bell, Pouch, Boots, or an assured weapon is placed in Ravio's Shop, it will now be placed on a random slot (but never the Sand Rod Slot).


- Letter in a Bottle pickup text has been removed


- The randomizer will no longer generate a `Standard.toml` preset that overwrites itself. Instead, an `Example.toml` preset is provided to demonstrate the correct format, and you may modify or delete it ~~at your own peril.~~


- **FIXED:** Vanilla Bow of Light
  - The new filler is not biased towards giving Zelda the Bow of Light.
  - Excluding Zelda is now *guaranteed* to prevent a vanilla Bow of Light placement.

  
- **FIXED:** Smooth Gem
  - The issue preventing players from giving Oren the Smooth Gem has been
    - (•\_•)
    - ( •\_•)>⌐■-■
    - _smoothed out_
    - (⌐■_■)
    - _YEAAAAHHHHHHHHHHH_


- **FIXED:** Hyrule Castle Barrier
  - The Barrier no longer mysteriously disappears after obtaining the Master Sword, instead you get to experience the joy tearing it down.
 

- **FIXED:** Skull Woods Softlock
  - It was technically possible to get softlocked in the hallway outside the Skull Woods boss if the player reached it without the Lamp, Fire Rod, Bombs, a Scoot Fruit, or the dungeon's Boss Key. There are no reports of this happening as it would require navigating Skull Woods completely in the dark while being unlucky enough to have none of those items, but it was technically possible. The fix prevents the door closing behind the player when they enter the hallway, allowing them a means of escape.


- **FIXED:** Duplicate/Wrong Enemies in Hyrule Field
  - The enemies present in Hyrule Field should now be those typically present in the vanilla game after beating Eastern Palace and opening the Portals to Lorule. The seemingly random duplicates in certain areas have been removed.


- **FIXED:** Kakariko now has the correct number of NPCs and Cuccos.


- **ALTERED:** The rock used to perform Trial's Skip in Lorule Castle will now respawn after leaving and re-entering the room. This is to keep Trial's Skip doable in the event the rock is accidentally destroyed.

</details>
<details>
<summary>Version 0.0.4</summary>

  - **FIX:** Prevent players missing the Big Key Chest in Thieves' Hideout.
    - Thief Girl will now remain in the dungeon even after completing it.
  - `skip_trials` option added. Removes the Trials Door in Lorule Castle if enabled.
  - `boots_in_shop` promoted to a UI option instead of requiring a preset to use.
  - Glitched logic now considers using the Ice Rod to reach the Eastern Palace Boss door.
  - Long Portal animation from Zelda's Study has been shortened.
  - The following cutscenes have been removed:
    - Zelda in the Sacred Realm
    - Thief Girl Cave
    - Kakariko Girl/Papa (if you didn't get anything from Ravio)
    - Blacksmith's Wife
  - The randomizer now calculates your ROM's SHA256 checksum for troubleshooting purposes.
  - Bye Seres
  - _True_ Double Ravio.
</details>
<details>
<summary>Version 0.0.3</summary>

- Highlights:
  - Super Lamp and Super Net inclusion option added. These are progressive updates to the base Lamp and Net, meaning there will be 2 of each in the item pool if the option is enabled.
  - Maiamai Cave is open by default, bombs are no longer necessary.
  - The Curtain bug in Zelda's Study is fixed.
  - The Weather Vanes by Link's House and the Vacant House are pre-activated.
  - Many small cutscenes have been removed, most notably in the Dark Maze. Sorry Hilda Hey fans.

- Additional Changes:
  - The Inside Hyrule Castle dungeon door is now sealed shut. Remember, there is no need to enter this dungeon in this version of the randomizer.
  - A boulder was removed from the Donkey Cave to fix a potential vanilla softlock
  - Minor UI tweaks
</details>
<details>
<summary>Version 0.0.2</summary>

- Highlights:
  - Introduction of Swordless Mode Option. I regret this already :^)
  - New options to place Bell, Pouch, and/or Pegasus Boots in Ravio's Shop
  - Expanded Glitched Logic to include more tricks in all dungeons
  - UI updates should provide more useful feedback when seeds fail to generate

- Additional Changes:
  - Smooth Gem is now locked to its original location in Kakariko (temporary fix for related bug)
  - Fix logic bug that could place Bracelet on Fire Cave Pillar in a Glitched No Bracelet seed
  - Removed `unsafe_key_logic` option. Please remove this from your presets.
</details>
<details>
<summary>Version 0.0.1</summary>

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
</details>

## Setup

Download: [Latest Stable Release (v0.1.0)](https://github.com/rickfay/z17-randomizer/releases/download/v0.1.0/z17r-0.1.0.zip)

1. Unzip the download to your directory of choice.
2. Move your A Link Between Worlds ROM into the same folder. Name it: `A Link Between Worlds (USA).3ds`
   - If your ROM is located elsewhere or if you'd like to name it something different, you can modify the `config.toml` file to point to the ROM instead

![fs-setup.png](docs/fs-setup.jpg?raw=true)

## Running the Randomizer

There are two ways you can run Z17R:

1. Double click `z17-randomizer.exe` to start the randomizer with basic settings. The randomizer will provide a simple interface for setting game options, after which it will attempt to generate a completable seed.
   - The randomizer may make multiple attempts to generate a completable seed. This is normal, and will happen automatically.

![cli-example.png](docs/cli-example.png)

2. Use a command line interface. If you take this approach you may also specify a preset and/or seed to use for seed generation.
   - Using a preset will give you some additional options and allow you to manually configure excluded checks.
     - See the example `presets/Standard.toml` for more information.
   - Note that the randomizer looks in the local `presets` directory now, and does NOT check `AppData`
   - Examples:
     - `$ ./z17-randomizer.exe --preset racerman`
     - `$ ./z17-randomizer.exe --seed 4057320268`

## Installing Seeds

After you've generated your seed in the above section, you'll need to install it in order to actually play the randomizer.

The randomizer will generate a folder called `00040000000EC300`. This folder is the patch you need to install to play your seed.

For 3DS hardware:
- Copy `00040000000EC300` to `/luma/titles` on your SD card.
- Ensure that `Enable game patching` is selected in Luma's config (this can be opened by holding `Select` when powering on the console).

For Citra (emulator):
- Copy `00040000000EC300` to `<Citra folder>/load/mods/`. You may need to create these folders.
- You can find the Citra folder by selecting `File > Open Citra folder...` in Citra.


## Game Options

`mode`
- Determines the Logic to use when generating the seed.
  - Options are: `Normal`, `Hard`, `GlitchBasic`, `GlitchAdvanced`, `GlitchHell`, or `NoLogic`

`assured_weapon`
- If enabled, guarantees that a weapon will be placed in Ravio's Shop
  - Potential weapons include: Sword, Bow, Hammer, Fire Rod, Ice Rod, or Bombs

`bell_in_shop`
- If enabled, guarantees the Bell will be placed in Ravio's Shop.

`pouch_in_shop`
- If enabled, guarantees the Pouch will be placed in Ravio's Shop.

`boots_in_shop`
- If enabled, guarantees the Pegasus Boots will be placed in Ravio's Shop.

`super_items`
- If enabled, includes the Super Lamp and Super Net in the shuffled item pool as progressive upgrades to the base Lamp and Net.

`minigames_excluded`
- Excludes Cucco Rush, Hyrule Hotfoot, Treacherous Tower, Octoball Derby, and both Rupee Rush minigames from having progression.

`skip_trials`
- If enabled, the Trials door in Lorule Castle will be removed.

`bow_of_light_in_castle`
- If enabled, guarantees the Bow of Light will be placed *somewhere* in Lorule Castle (including possibly Zelda)

`swordless_mode`
- Generates a seed with no Swords. Things you should know about Swordless:
  - The Hyrule Castle Barrier will be torn down from game start, providing access to its checks and the Inside Hyrule Castle dungeon.
  - You will need the [Net to play Tennis with Yuganon](https://www.twitch.tv/videos/1265170513). Good luck finding it!

## Glitched Logic Breakdown

The following tricks may potentially be required in a Glitched Logic seed:

- Lamp, Net, and Pegasus Boots are considered damage sources
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
  - Bombs or Ice Rod to activate switch opening path to Boss Door early
  - TRod Armos Boost to get to Boss Key or Boss room without the appropriate key
- House of Gales
  - 2F Fire Ring Key using HoG Skip Skip to do 2F backwards if key missing (incredibly specific, unlikely)
  - Skip Skip Skip to reach 3F without keys
- Tower of Hera
  - Bomb Rods can be used to climb the tower without any keys
- Swamp Palace
  - Early access to 1F SW/SE rooms with Ice Rod to raise water level
  - Miniboss Skips with Ice Rod/Tornado Rod or Boots
  - Early Big Chest with Ice Rod or Boots
  - Big Key Skip with Ice Rod and Tornado Rod
- Thieves' Hideout
  - Flipperless Thieves using TRod + a way to hit the switch (IRod or Bombs, no sword beams)
    - Softlock chest behind wall NOT in logic unless you can also get Bombs or Fire Rod (but really, bring a Scoot Fruit)
    - Big Key chest accessible, Boots made available
- Ice Ruins
  - All dungeon checks accessible without keys if you have Boots. The ones behind B1 locked doors also require Tornado Rod.
- Desert Palace
  - Reverse DP means keys can show up anywhere in the dungeon if you have Fire Rod or Nice Bombs
- Turtle Rock
  - Big Key Skip using a Bomb Rod
- Lorule Castle
  - Lamp Trial doesn't need the Lamp

  
Mergeless Tricks:
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
  - The logic guarantees Nice Bombs are obtainable for any Bomb Rods or Lemon Boosts
  - If you spend your Maiamai on anything other than Nice Bombs, you may potentially doom your seed. Regular Bomb boosts may save you, but if progression is on Southern Ruins Treasure Dungeon with Nice Bombs there's no salvaging it. YOU HAVE BEEN WARNED.

## Known Issues

- Bow of Light crashes the game if used outside the final boss arena (on 3DS console)
- Some text boxes will overflow
- After turning in the Letter in a Bottle to the Milk Bar owner, the Letter in a Bottle will be available for collection again. It can be turned in again, but doing so can lead to some unexpected behavior (and is never part of logic).

## Special Thanks

- Tocapa for building the original ALBWR foundation this hack is built from
- My Beta Testers: Br00ty, j_im, flora, and Herreteman

## License

This program is licensed under the GNU General Public License v2.0.
