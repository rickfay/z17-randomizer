crate::region! {
    course: FieldDark,
    name: "Turtle Rock Area",
    color: Name,
    lorule {
        locations: [
            "Lorule River Portal": None @Portal(29[53] RiverLorule),
            "Lorule Lake Portal": None @Portal(35[30] LoruleLake),
            "Lorule Coldfoot Portal": None @Portal(36[43] LoruleColdfoot),

            "Turtle Rock Weather Vane": None @WeatherVane(35[67] TurtleRockWV),

            "Lorule Lake Chest": RupeeSilver @Chest(28[53]),
            "Lorule Lakeside Item Shop (1)": Bee @None(),
            "Lorule Lakeside Item Shop (2)": GoldenBeeForSale @None(),
            "Lorule Lakeside Item Shop (3)": Fairy @None(),
            "Lorule Lakeside Item Shop (3)": Shield @None(),
            "[Mai] Lorule Lake Rock": Maiamai @Maiamai(35[65]),
            "[Mai] Lorule Lake SE Wall": Maiamai @Maiamai(40[15]),
            "[Mai] Lorule Lake Skull": Maiamai @Maiamai(36[46]),
            "[Mai] Lorule Lake Water": Maiamai @Maiamai(35[64]),
            "[Mai] Lorule Lake West Wall": Maiamai @Maiamai(35[63]),
        ],
    },
}
