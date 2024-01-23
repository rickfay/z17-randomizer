crate::region! {
    course: IndoorLight,
    name: "Ravio's Shop",
    color: Name,
    shop {
        locations: [
            "Ravio's Gift": RingHekiga @Event(FieldLight_2C_Rental[237]),
            "Ravio's Shop (1)": ItemRentalIceRod @Shop(Ravio(0)),
            "Ravio's Shop (2)": ItemRentalHookshot @Shop(Ravio(5)),
            "Ravio's Shop (3)": ItemRentalTornadeRod @Shop(Ravio(2)),
            "Ravio's Shop (4)": ItemRentalBomb @Shop(Ravio(3)),
            "Ravio's Shop (5)": ItemRentalBow @Shop(Ravio(8)),
            "Ravio's Shop (6)": ItemRentalSandRod @Shop(Ravio(1)),
            "Ravio's Shop (7)": ItemRentalHammer @Shop(Ravio(7)),
            "Ravio's Shop (8)": ItemRentalBoomerang @Shop(Ravio(6)),
            "Ravio's Shop (9)": ItemRentalFireRod @Shop(Ravio(4)),
        ],
    },
}
