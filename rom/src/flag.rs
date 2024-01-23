#[derive(Debug, Copy, Clone)]
pub enum Flag {
    React(u16),   // 0 - Reactions with system objects, not persisted
    Session(u16), // 1 - Flag persists until game is reset (I think?)
    Two(u16),     // 2 - ???
    Course(u16),  // 3 - Course-specific, shared between scenes of the same course
    Event(u16),   // 4 - Global
}

impl Flag {
    pub fn get_type(self) -> u8 {
        match self {
            Flag::React(_) => 0,
            Flag::Session(_) => 1,
            Flag::Two(_) => 2,
            Flag::Course(_) => 3,
            Flag::Event(_) => 4,
        }
    }

    pub fn get_value(self) -> u16 {
        match self {
            Flag::React(flag) => flag,
            Flag::Session(flag) => flag,
            Flag::Two(flag) => flag,
            Flag::Course(flag) => flag,
            Flag::Event(flag) => flag,
        }
    }

    pub fn into_pair(self) -> (u8, u16) {
        match self {
            Flag::React(flag) => (0, flag),
            Flag::Session(flag) => (1, flag),
            Flag::Two(flag) => (2, flag),
            Flag::Course(flag) => (3, flag),
            Flag::Event(flag) => (4, flag),
        }
    }

    /// Gets the "true" flag using the game's internal lookup table
    pub fn get_true_flag(flag: u16) -> u16 {
        Self::TRUE_FLAG_LUT[flag as usize]
    }

    const TRUE_FLAG_LUT: [u16; 1018] = [
        0, 1, 2, 3, 4, 5, 6, 8, 12, 7, 13, 14, 15, 9, 16, 17, 18, 10, 19, 21, 23, 11, 25, 27, 28, 20, 29, 30, 31, 22,
        32, 33, 34, 24, 35, 36, 37, 26, 38, 39, 40, 55, 41, 42, 43, 57, 44, 45, 46, 84, 47, 48, 49, 104, 50, 51, 52,
        105, 53, 54, 56, 106, 58, 59, 60, 107, 61, 62, 63, 108, 64, 65, 66, 109, 67, 68, 69, 110, 70, 71, 72, 112, 73,
        74, 75, 210, 76, 77, 78, 211, 79, 80, 81, 222, 82, 83, 85, 223, 86, 87, 88, 224, 89, 90, 91, 225, 92, 93, 94,
        227, 95, 96, 97, 228, 98, 99, 100, 229, 101, 102, 103, 230, 111, 113, 114, 231, 115, 116, 117, 232, 118, 119,
        120, 234, 121, 122, 123, 238, 124, 125, 126, 239, 127, 128, 129, 240, 130, 131, 132, 242, 133, 134, 135, 243,
        136, 137, 138, 241, 139, 140, 141, 244, 142, 143, 144, 248, 145, 146, 147, 249, 148, 149, 150, 250, 151, 152,
        153, 251, 154, 155, 156, 246, 157, 158, 159, 308, 160, 161, 162, 309, 163, 164, 165, 310, 166, 167, 168, 320,
        169, 170, 171, 321, 172, 173, 174, 322, 175, 176, 177, 323, 178, 179, 180, 324, 181, 182, 183, 325, 184, 185,
        186, 233, 187, 188, 189, 315, 190, 191, 192, 235, 193, 194, 195, 236, 196, 197, 198, 330, 199, 200, 201, 340,
        202, 203, 204, 341, 205, 206, 207, 342, 208, 209, 212, 374, 213, 214, 215, 343, 216, 217, 218, 344, 219, 220,
        221, 345, 226, 237, 245, 348, 247, 252, 253, 350, 254, 255, 256, 360, 257, 258, 259, 370, 261, 262, 263, 371,
        264, 265, 266, 372, 267, 268, 269, 373, 270, 271, 272, 375, 273, 274, 275, 260, 276, 277, 278, 376, 279, 280,
        281, 377, 282, 283, 284, 380, 285, 286, 287, 381, 288, 289, 290, 382, 291, 292, 293, 383, 294, 295, 296, 384,
        297, 298, 299, 385, 300, 301, 302, 386, 303, 304, 305, 387, 306, 307, 311, 388, 312, 313, 314, 389, 316, 317,
        318, 390, 319, 326, 327, 391, 328, 329, 331, 396, 332, 333, 334, 410, 335, 336, 337, 415, 338, 339, 346, 420,
        347, 349, 351, 421, 352, 353, 354, 430, 355, 356, 357, 510, 358, 359, 361, 514, 362, 363, 364, 515, 365, 366,
        367, 516, 368, 369, 378, 517, 379, 392, 393, 518, 394, 395, 397, 519, 398, 399, 400, 520, 401, 402, 403, 522,
        404, 405, 406, 523, 407, 408, 409, 525, 411, 412, 413, 524, 414, 416, 417, 530, 418, 419, 422, 536, 423, 424,
        425, 537, 426, 427, 428, 538, 429, 431, 432, 540, 433, 434, 435, 542, 436, 437, 438, 543, 439, 440, 441, 544,
        442, 443, 444, 541, 445, 446, 447, 550, 448, 449, 450, 556, 451, 452, 453, 557, 454, 455, 456, 558, 457, 458,
        459, 560, 460, 461, 462, 562, 463, 464, 465, 570, 466, 467, 468, 572, 469, 470, 471, 573, 472, 473, 474, 576,
        475, 476, 477, 577, 478, 479, 480, 578, 481, 482, 483, 580, 484, 485, 486, 590, 487, 488, 489, 594, 490, 491,
        492, 596, 493, 494, 495, 597, 496, 497, 498, 598, 499, 500, 501, 599, 502, 503, 504, 600, 505, 506, 507, 602,
        508, 509, 511, 610, 512, 513, 521, 616, 526, 527, 528, 617, 529, 531, 532, 618, 533, 534, 535, 620, 539, 545,
        546, 622, 547, 548, 549, 630, 551, 552, 553, 636, 554, 555, 559, 637, 561, 563, 564, 638, 565, 566, 567, 640,
        568, 569, 571, 642, 574, 575, 579, 643, 581, 582, 583, 644, 584, 585, 586, 645, 587, 588, 589, 650, 591, 592,
        593, 656, 595, 601, 603, 657, 604, 605, 606, 658, 607, 608, 609, 670, 611, 612, 613, 706, 614, 615, 619, 708,
        621, 623, 624, 709, 625, 626, 627, 710, 628, 629, 631, 711, 632, 633, 634, 712, 635, 639, 641, 713, 646, 647,
        648, 714, 649, 651, 652, 715, 653, 654, 655, 716, 659, 660, 661, 717, 662, 663, 664, 718, 665, 666, 667, 722,
        668, 669, 671, 730, 672, 673, 674, 731, 675, 676, 677, 740, 678, 679, 680, 681, 682, 683, 684, 685, 686, 687,
        688, 689, 690, 691, 692, 693, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 707, 719, 720, 721,
        723, 724, 725, 726, 727, 728, 729, 732, 733, 734, 735, 736, 737, 738, 739, 741, 742, 743, 744, 745, 746, 747,
        748, 749, 750, 751, 752, 753, 754, 755, 756, 757, 758, 759, 760, 761, 762, 763, 764, 765, 766, 767, 768, 769,
        770, 771, 772, 773, 774, 775, 776, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 791,
        792, 793, 794, 795, 796, 797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812, 813,
        814, 815, 816, 817, 818, 819, 820, 821, 822, 823, 824, 825, 826, 827, 828, 829, 830, 831, 832, 833, 834, 835,
        836, 837, 838, 839, 840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 857,
        858, 859, 860, 861, 862, 863, 864, 865, 866, 867, 868, 869, 870, 871, 872, 873, 874, 875, 876, 877, 878, 879,
        880, 881, 882, 883, 884, 885, 886, 887, 888, 889, 890, 891, 892, 893, 894, 895, 896, 897, 898, 899, 900, 901,
        902, 903, 904, 905, 906, 907, 908, 909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920, 921, 922, 923,
        924, 925, 926, 927, 928, 929, 930, 931, 932, 933, 934, 935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945,
        946, 947, 948, 949, 950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960, 961, 962, 963, 964, 965, 966, 967,
        968, 969, 970, 971, 972, 973, 974, 975, 976, 977, 978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989,
        990, 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009,
        1010, 1011, 1012, 1013, 1014, 1015, 1016, 1017,
    ];
}

macro_rules! event_flags {
    (
        $(#[$attr:meta])*
        $($index:literal: $flag:ident,)+
    ) => {
        $(#[$attr])*
        $(pub const $flag: Flag = Flag::Event($index);)+
    }
}

impl Flag {
    event_flags! {
        0: ZERO,
        // ...
        251: EASTERN_COMPLETE,
        // ...
        342: GALES_COMPLETE,
        // ...
        372: HERA_COMPLETE,
        // ...
        410: HC_BARRIER,
        // ...
        510: EARTHQUAKE,
        // ...
        536: SAGE_GULLEY,
        // ...
        556: SAGE_OREN,
        // ...
        576: SAGE_SERES,
        // ...
        596: SAGE_OSFALA,
        // ...
        616: SAGE_ROSSO,
        // ...
        636: SAGE_IRENE,
        // ...
        656: SAGE_IMPA,
        // ...
        670: TRIFORCE_OF_COURAGE,
        // ...
        730: CREDITS,
        // ...
        801: PORTAL_STYLISH_WOMAN,
        802: PORTAL_YOUR_HOUSE,
        803: PORTAL_PARADOX_LOWER_HYRULE,
        804: PORTAL_PARADOX_UPPER_HYRULE,
        805: PORTAL_WATERFALL_HYRULE,
        806: PORTAL_EASTERN_RUINS_PILLAR,
        807: PORTAL_EASTERN_RUINS_SE,
        808: PORTAL_LOST_WOODS,
        809: PORTAL_SAHASRAHLAS_HOUSE,
        810: PORTAL_ROSSOS_HOUSE,
        811: PORTAL_TO_MISERY_MIRE,

        812: PORTAL_DESERT_RILLAR_RIGHT,
        813: PORTAL_DESERT_PILLAR_LEFT,
        814: PORTAL_DESERT_MIDDLE,
        815: PORTAL_DESERT_SW,
        816: PORTAL_TO_ZAGANAGA,
        817: PORTAL_DESERT_NORTH,

        818: PORTAL_DM_WEST_HYRULE,
        819: PORTAL_FLOATING_ISLAND_HYRULE,
        820: PORTAL_RIVER_HYRULE,
        821: PORTAL_LAKE_HYLIA,
        822: PORTAL_HYRULE_HOTFOOT,
        823: PORTAL_SANCTUARY,
        824: PORTAL_GRAVEYARD_LEDGE_HYRULE,
        825: PORTAL_ROSSOS_ORE_MINE_HYRULE,
        826: PORTAL_SWAMP_PILLAR_HYRULE,
        827: PORTAL_ZORAS_DOMAIN,
        // 828: ???,
        // 829: ???,
        // 830: ???,
        831: PORTAL_THIEVES_TOWN,
        832: PORTAL_VACANT_HOUSE,
        833: PORTAL_PARADOX_UPPER_LORULE,
        834: PORTAL_PARADOX_LOWER_LORULE,
        835: PORTAL_WATERFALL_LORULE,
        836: PORTAL_DARK_RUINS_PILLAR,
        837: PORTAL_DARK_MAZE_SE,
        838: PORTAL_SKULL_WOODS_PILLAR,
        839: PORTAL_N_SHAPED_HOUSE,
        840: PORTAL_DESTROYED_HOUSE,
        841: PORTAL_MISERY_MIRE_EXIT,
        842: PORTAL_MIRE_PILLAR_RIGHT,
        843: PORTAL_MIRE_PILLAR_LEFT,
        844: PORTAL_MIRE_MIDDLE,
        845: PORTAL_MIRE_SW,
        846: PORTAL_ZAGANAGA_EXIT,
        847: PORTAL_MIRE_NORTH,
        848: PORTAL_DM_WEST_LORULE,
        849: PORTAL_FLOATING_ISLAND_LORULE,
        850: PORTAL_RIVER_LORULE,
        851: PORTAL_LORULE_LAKE_WEST,
        852: PORTAL_LORULE_COLDFOOT,
        853: PORTAL_PHILOSOPHERS_CAVE,
        854: PORTAL_GRAVEYARD_LEDGE_LORULE,
        855: PORTAL_ROSSOS_ORE_MINE_LORULE,
        856: PORTAL_SWAMP_PILLAR_LORULE,
        857: PORTAL_KUS_DOMAIN,
        // ...
        920: WV_YOUR_HOUSE,
        921: WV_KAKARIKO_VILLAGE,
        922: WV_EASTERN_PALACE,
        923: WV_HOUSE_OF_GALES,
        924: WV_TOWER_OF_HERA,
        925: WV_WITCHS_HOUSE,
        926: WV_DEATH_MTN_HYRULE,
        927: WV_DESERT_PALACE,
        928: WV_SANCTUARY,
        // 929: ???,
        // 930: ???,
        // 931: ???,
        932: WV_SKULL_WOODS,
        933: WV_TREACHEROUS_TOWER,
        934: WV_ICE_RUINS,
        935: WV_LORULE_CASTLE,
        936: WV_GRAVEYARD,
        937: WV_THIEVES_TOWN,
        938: WV_DARK_PALACE,
        939: WV_BLACKSMITH,
        940: WV_VACANT_HOUSE,
        941: WV_MISERY_MIRE,
        942: WV_SWAMP_PALACE,
        943: WV_TURTLE_ROCK,
        944: WV_DEATH_MTN_LORULE,
        // ...
        // 1017: LAST_FLAG,
    }

    /// Flags of "Convenient" Weather Vanes, that don't affect logic but save time
    pub fn get_convenient_weather_vane_flags(portal_shuffle: bool) -> Option<Vec<Flag>> {
        let mut convenient_weather_vanes =
            vec![Flag::WV_YOUR_HOUSE, Flag::WV_KAKARIKO_VILLAGE, Flag::WV_WITCHS_HOUSE, Flag::WV_SANCTUARY];

        if !portal_shuffle {
            convenient_weather_vanes.extend(&[
                Flag::WV_SKULL_WOODS,
                Flag::WV_LORULE_CASTLE,
                Flag::WV_THIEVES_TOWN,
                Flag::WV_DARK_PALACE,
                Flag::WV_BLACKSMITH,
                Flag::WV_VACANT_HOUSE,
                Flag::WV_MISERY_MIRE,
            ]);
        }

        Some(convenient_weather_vanes)
    }

    pub fn get_hyrule_weather_vane_flags() -> Option<Vec<Flag>> {
        Some(vec![
            Flag::WV_YOUR_HOUSE,
            Flag::WV_KAKARIKO_VILLAGE,
            Flag::WV_EASTERN_PALACE,
            Flag::WV_HOUSE_OF_GALES,
            Flag::WV_TOWER_OF_HERA,
            Flag::WV_WITCHS_HOUSE,
            Flag::WV_DEATH_MTN_HYRULE,
            Flag::WV_DESERT_PALACE,
            Flag::WV_SANCTUARY,
        ])
    }

    pub fn get_lorule_weather_vane_flags() -> Option<Vec<Flag>> {
        Some(vec![
            Flag::WV_SKULL_WOODS,
            Flag::WV_TREACHEROUS_TOWER,
            Flag::WV_ICE_RUINS,
            Flag::WV_LORULE_CASTLE,
            Flag::WV_GRAVEYARD,
            Flag::WV_THIEVES_TOWN,
            Flag::WV_DARK_PALACE,
            Flag::WV_BLACKSMITH,
            Flag::WV_VACANT_HOUSE,
            Flag::WV_MISERY_MIRE,
            Flag::WV_SWAMP_PALACE,
            Flag::WV_TURTLE_ROCK,
            Flag::WV_DEATH_MTN_LORULE,
        ])
    }

    pub fn get_all_weather_vane_flags() -> Option<Vec<Flag>> {
        let mut flags = Vec::with_capacity(22);
        flags.extend(&mut Flag::get_hyrule_weather_vane_flags().iter().flatten());
        flags.extend(&mut Flag::get_lorule_weather_vane_flags().iter().flatten());
        Some(flags)
    }
}
