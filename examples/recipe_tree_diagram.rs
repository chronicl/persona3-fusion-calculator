use p3r_fusion_calculator::Personas;

fn main() {
    let mut table = Personas::new();

    table.add_buyable_personas(&[
        ("Orpheus", 2169),
        ("Pixie", 2225),
        ("Apsaras", 2225),
        ("Angel", 2441),
        ("Silky", 2576),
        ("Ara Mitama", 2676),
        ("Forneus", 2900),
        ("Lilim", 3089),
        ("Onmoraki", 3089),
        ("Jack Frost", 3156),
        ("Valkyrie", 3521),
        ("Chimera", 3764),
        ("Unicorn", 3936),
        ("Nigi Mitama", 4401),
        ("Gurulu", 4601),
        ("Zouchouten", 4809),
        ("Slime", 4704),
        ("Fortuna", 5721),
        ("Pisaca", 5721),
        ("Principality", 5364),
        ("Berith", 5249),
        ("Oberon", 6624),
        ("Leanan Sidhe", 8400),
    ]);

    let mut calc = table.recipe_calculator();
    calc.process();

    println!("{}", calc.recipe_tree_dot("Pale Rider", &table));
    println!(
        "\n paste this into https://dreampuf.github.io/GraphvizOnline/ to see the tree diagram"
    );
}
