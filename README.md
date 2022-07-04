# Z17 Randomizer

A randomizer for The Legend of Zelda: A Link Between Worlds, built off the [original ALBW Randomizer](https://gitlab.com/marsolk/albw-randomizer).

<img src="cli/icon.ico" alt="icon" />

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
  - Note 1: Climbing Hyrule Castle logically requires the Bow or the Ice Rod to kill the Red Spear soldier mergeless on 3F. It can also be done with Nice Bombs (Hard Logic, see below), or the Nice Tornado Rod (not in any logic).
  - Note 2: If you need to return to Hyrule after reaching Lorule in this way, you need to death warp to return to Link's House. We're exploring more *elegant* ways to do this in the future, but this is what we have for now.
  - Note 3: Swamp Palace was really not designed for the player to not have merge, with three rooms capable of softlocking the player if they unwittingly entered mergeless. To combat this, 2 doors that normally shut behind the player have been removed. Additionally, the crystal switch in the B1 north room that would normally divert the river has been removed, preventing a softlock that could otherwise occur (the switch was never needed, players could always bypass the river by merging).


- New Logic Modes
  - The logic has been expanded into six (6) different modes. Hopefully there's something for everyone.
    - <u>Normal</u>: Standard gameplay, no tricky item use or glitches. If unsure, choose this.
    - <u>Hard</u>: Adds tricks that aren't technically glitches, lamp + net considered as weapons. No glitches.
    - <u>Glitched (Basic)</u>: Includes the above plus "basic", easy-to-learn glitches.
    - <u>Glitched (Advanced)</u>: Includes the above plus "advanced" glitches that may be a challenge to master.
    - <u>Glitched (Hell)</u>: Includes every known RTA-viable glitch, including the insane ones. DO NOT CHOOSE THIS.
    - <u>No Logic</u>: Items are placed with no logic at all. Seeds may not be completable.

  - A full breakdown of the new logic modes is located further down this page.


- Sanctuary, Rewired
  - The doors to the Sanctuary church are now closed at game start. They will open upon completing the Sanctuary dungeon, specifically when the left-side switch is pulled at the dungeon's end.
  - This change effectively locks the Lorule Graveyard behind completing the Sanctuary, meaning players have a reason to collect the key, fight the miniboss, and actually finish the dungeon now.

  
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
    - Items include: Bow, Bombs, Fire Rod, Ice Rod, Hammer, Sword (if not playing Swordless), or Lamp/Net (if playing on Hard Logic or higher)


- New Option: `bow_of_light_in_castle`
  - Guarantees Bow of Light will spawn on one of the 15 checks inside Lorule Castle.
    - Note: This includes Zelda, unless you manually exclude her.


- Change to *`_in_shop` settings:
  - When the Bell, Pouch, Boots, or an assured weapon is placed in Ravio's Shop, it will now be placed on a random slot (but never the Sand Rod Slot).


- Letter in a Bottle pickup text has been removed


- Great Rupee Fairy will now logically guarantee (at least) 3000 Rupees are available between placed Purples, Silvers, and Golds.


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

- Download: [Latest \[BETA\] Release (v0.1.0)](https://github.com/rickfay/z17-randomizer/releases/download/v0.1.0/z17r-0.1.0.zip)

- Download: [Latest \[STABLE\] Release (v0.0.4)](https://github.com/rickfay/z17-randomizer/releases/download/v0.0.4/z17r-0.0.4.zip)

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

## Logic Breakdown

### Normal Logic

This is the standard logic mode and is recommended for new and casual players.

- Includes:
  - Standard gameplay
  - No glitches
  - No obscure tricks

### Hard Logic

This mode is recommended for players who have played the game before and are familiar with its mechanics. This is still a glitch-free mode, but the logic may require players to perform actions or use items in obscure, non-obvious ways to reach checks.

- Includes:
  - Using the Lamp or Net as weapons (they deal 1/2 the damage of the Forgotten Sword)
  - Completing the Boots Dungeon with either Bombs, Boomerang, or Sword Beams
  - Entering Eastern Ruins with Power Glove
  - Entering the Vacant House rear with the Bomb Flower
  - Hyrule Hotfoot with Merge + Bell
  - Jumping into the Kakariko Well with a Cucco
  - Reaching the Misery Mire Ledge with Stamina Scroll + Bombs
  - Southern Ruins Treasure Dungeon
    - Flippers, Hookshot, and either Bombs or Master Sword to hit the Boomerang switch
  - Eastern Palace:
    - Using Sword Beams or thrown Pots to activate switches
    - Skipping a Small Key with either bombs or the Ice Rod
  - House of Gales:
    - Using the Tornado Rod to jump on moving blocks to reach 2F without merge
    - Deathwarping from NE room on 1F without merge
  - Swamp Palace:
    - Precise Bow Shot to bypass River Room without merge
    - Access 1F Big Chest without extinguishing flames
  - Desert Palace:
    - Run past Armos on 3F
  - Turtle Rock
    - Hit the B1 crystal Switch with a thrown skull
  - Lorule Castle
    - Play Tennis with Yuganon using the Net

### Glitched (Basic) Logic

This mode is intended for players who want to use some of the game's many glitches, but not the hard ones.

<u>Note</u>: You are expected to spend your first 10 Maiamai on Nice Bombs for all Glitched Logic seeds due to the many glitches they enable. Failure to do so could make your seed much harder or potentially even impossible to complete.

- Includes:
  - Armos Boost to reach:
    - Eastern Ruins merge chest
  - Bird Boost to reach Waterfall Cave
  - Fire Rod or Lemon Boosts to get onto small ledges
  - Enemy Clips
  - Reaching the Misery Mire Ledge with Nice Bombs or Fire Rod with either Boots or regular Bombs
  - Southern Ruins Treasure Dungeon Flipperless w/ Nice Bombs
  - House of Gales:
    - Skip Skip to do 2F in reverse
    - Skip Skip Skip to reach 3F
  - Swamp Palace:
    - Miniboss Skip with Pegasus Boots
  - Thieves' Hideout:
    - Jailbreak to activate switches and reach B2 without merge
  - Desert Palace
    - Defeat Zaganaga with just Bow or just Sword Beams

### Glitched (Advanced) Logic

This mode is for experienced players who are comfortable with this game's harder, more involved glitches. 

<u>Note</u>: You are expected to spend your first 10 Maiamai on Nice Bombs for all Glitched Logic seeds due to the many glitches they enable. Failure to do so could make your seed much harder or potentially even impossible to complete.

- Includes:
  - Entering Southern Ruins Treasure Dungeon with just Ice Rod
  - Desert Palace Skip
    - Skip Desert with Ice Rod + Tornado Rod
    - OoB with Fire Rod or Nice Bombs
    - Zaganaga Skip with Pegasus Boots
    - Reverse Desert Palace
  - Fake Flipper Tricks
  - Flipperless House of Gales with Hookshot/Ice Rod
  - Enemy clips to reach Lost Woods checks without escape items (must use Crows instead)
  - Reaching the Misery Mire Ledge with Regular Bombs + Vulture Boost
  - Shield Rod Clips
  - Tornado Rod Enemy Clip to reach Eastern Ruins Peg Circle
  - Eastern Palace
    - Armos Boost to skip the Boss Key and a Small Key
  - Tower of Hera
    - Bombrods
    - Mergeless strategy
  - Swamp Palace
    - Ice Rod Clipping to skip merge requirements and raise/lower water levels
    - Ice Rod + Tornado Rod to skip dungeon entirely
  - Thieves' Hideout
    - Flipperless Thieves strategies with Tornado Rod + either Bombs or Ice Rod
      - Note: B1 Behind the Wall chest and the B1 Big Chest are reachable but excluded from this, due to the repetitive nature of the trick
    - Reach miniboss and/or B2 jail cell without merge
  - Ice Ruins
    - Get Out of Bounds with Pegasus Boots to:
      - Skip entire dungeon
      - Scroll Skip
      - Small Key skip with Tornado Rod
    - Desert Palace
      - Reverse Desert Palace with Fire Rod or Nice Bombs
      - Armos Boost to skip West 2F

### Glitched (Hell) Logic

This mode logically includes every known RTA-viable glitch, including those that are wildly inconsistent and inconsiderate of a player's time. It is not recommended for anyone, other than those seeking a challenge.

<u>Note</u>: You are expected to spend your first 10 Maiamai on Nice Bombs for all Glitched Logic seeds due to the many glitches they enable. Failure to do so could make your seed potentially impossible to complete.

- Includes:
  - Bee Boost Tricks
  - Defeating Yuga 1 with just Sword Beams or just Ice Rod
  - Fake Flippers via Bee Boosting
  - Catching a natural Golden Bee before Lorule with Bottle + Net
  - Regular Bomb Boost to reach:
    - Eastern Ruins merge chest
  - Sand Rod Enemy Clip to reach:
    - Eastern Ruins Peg Circle
    - Rosso Cave
  - Desert Palace Skip via Portal Clipping with Hookshot/Boomerang + Tornado Rod

### No Logic

It's all in the name: no logic is used to place items at all. Dungeon items are in their respective dungeons due to technical limitations, but otherwise any item could show up anywhere. Seeds are not guaranteed to be completable.

## Known Issues

- Bow of Light crashes the game if used outside the final boss arena (on 3DS console)
- Some text boxes will overflow
- After turning in the Letter in a Bottle to the Milk Bar owner, the Letter in a Bottle will be available for collection again. It can be turned in again, but doing so can lead to some unexpected behavior (and is never part of logic).

## Special Thanks

- Tocapa for building the original ALBWR foundation this hack is built from
- My Beta Testers: Br00ty, j_im, flora, and Herreteman

## License

This program is licensed under the GNU General Public License v2.0.
