use p3r_fusion_calculator::Personas;

fn main() {
    let mut table = Personas::new();

    table.add_buyable_personas(&[
        ("Orpheus", 1000),
        ("Pixie", 1000),
        ("Apsaras", 1000),
        ("Angel", 1000),
        ("Silky", 1000),
        ("Ara Mitama", 1000),
        ("Forneus", 1000),
        ("Lilim", 1000),
        ("Onmoraki", 1000),
        ("Jack Frost", 1000),
        ("Valkyrie", 1000),
        ("Chimera", 1000),
        ("Unicorn", 1000),
        ("Nigi Mitama", 1000),
        ("Gurulu", 1000),
        ("Zouchouten", 1000),
        ("Slime", 1000),
        ("Fortuna", 1000),
        ("Pisaca", 1000),
        ("Principality", 1000),
        ("Berith", 1000),
        ("Oberon", 1000),
        ("Leanan Sidhe", 1000),
    ]);

    let mut calc = table.recipe_calculator();
    calc.process();

    for persona in table.personas().iter() {
        println!(
            "{}\t{}    {:?}",
            persona.name,
            calc.is_reachable(persona.id),
            calc.price(persona.id)
        );
    }
}
