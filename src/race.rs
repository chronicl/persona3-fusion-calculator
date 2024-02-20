use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Race {
    Fool,
    Magician,
    Priestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Fortune,
    Strength,
    Hanged,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    Aeon,
}

impl Race {
    pub fn fuse(self, other: Self) -> Option<Self> {
        use Race::*;

        let result = match (self, other) {
            (Fool, Fool) => Fool,
            (Fool, Magician) => Hierophant,
            (Fool, Priestess) => Magician,
            (Fool, Empress) => Star,
            (Fool, Emperor) => Temperance,
            (Fool, Hierophant) => Hanged,
            (Fool, Lovers) => Justice,
            (Fool, Chariot) => Emperor,
            (Fool, Justice) => Lovers,
            (Fool, Hermit) => Priestess,
            (Fool, Fortune) => Strength,
            (Fool, Strength) => Death,
            (Fool, Hanged) => Devil,
            (Fool, Death) => Fortune,
            (Fool, Temperance) => Chariot,
            (Fool, Devil) => Hermit,
            (Fool, Tower) => Moon,
            (Fool, Star) => Devil,
            (Fool, Moon) => Empress,
            (Fool, Sun) => Judgement,
            (Fool, Judgement) => Aeon,
            (Fool, Aeon) => Death,
            (Magician, Magician) => Magician,
            (Magician, Priestess) => Justice,
            (Magician, Empress) => Hanged,
            (Magician, Emperor) => Lovers,
            (Magician, Hierophant) => Hermit,
            (Magician, Lovers) => Chariot,
            (Magician, Chariot) => Devil,
            (Magician, Justice) => Hierophant,
            (Magician, Hermit) => Moon,
            (Magician, Fortune) => Lovers,
            (Magician, Strength) => Emperor,
            (Magician, Hanged) => Fool,
            (Magician, Death) => Priestess,
            (Magician, Temperance) => Justice,
            (Magician, Devil) => Temperance,
            (Magician, Tower) => Chariot,
            (Magician, Star) => Strength,
            (Magician, Moon) => Strength,
            (Magician, Sun) => Empress,
            (Magician, Judgement) => Star,
            (Magician, Aeon) => Sun,
            (Priestess, Priestess) => Priestess,
            (Priestess, Empress) => Temperance,
            (Priestess, Emperor) => Justice,
            (Priestess, Hierophant) => Lovers,
            (Priestess, Lovers) => Magician,
            (Priestess, Chariot) => Fool,
            (Priestess, Justice) => Lovers,
            (Priestess, Hermit) => Strength,
            (Priestess, Fortune) => Hanged,
            (Priestess, Strength) => Moon,
            (Priestess, Hanged) => Hierophant,
            (Priestess, Death) => Justice,
            (Priestess, Temperance) => Fortune,
            (Priestess, Devil) => Emperor,
            (Priestess, Tower) => Empress,
            (Priestess, Star) => Emperor,
            (Priestess, Moon) => Star,
            (Priestess, Sun) => Hierophant,
            (Priestess, Judgement) => Hanged,
            (Priestess, Aeon) => Empress,
            (Empress, Empress) => Empress,
            (Empress, Emperor) => Chariot,
            (Empress, Hierophant) => Tower,
            (Empress, Lovers) => Moon,
            (Empress, Chariot) => Hermit,
            (Empress, Justice) => Emperor,
            (Empress, Hermit) => Sun,
            (Empress, Fortune) => Strength,
            (Empress, Strength) => Fool,
            (Empress, Hanged) => Star,
            (Empress, Death) => Lovers,
            (Empress, Temperance) => Hierophant,
            (Empress, Devil) => Tower,
            (Empress, Tower) => Devil,
            (Empress, Star) => Priestess,
            (Empress, Moon) => Aeon,
            (Empress, Sun) => Emperor,
            (Empress, Judgement) => Lovers,
            (Empress, Aeon) => Priestess,
            (Emperor, Emperor) => Emperor,
            (Emperor, Hierophant) => Strength,
            (Emperor, Lovers) => Chariot,
            (Emperor, Chariot) => Devil,
            (Emperor, Justice) => Hanged,
            (Emperor, Hermit) => Hierophant,
            (Emperor, Fortune) => Star,
            (Emperor, Strength) => Magician,
            (Emperor, Hanged) => Death,
            (Emperor, Death) => Hermit,
            (Emperor, Temperance) => Star,
            (Emperor, Devil) => Moon,
            (Emperor, Tower) => Strength,
            (Emperor, Star) => Hierophant,
            (Emperor, Moon) => Lovers,
            (Emperor, Sun) => Temperance,
            (Emperor, Judgement) => Sun,
            (Emperor, Aeon) => Fortune,
            (Hierophant, Hierophant) => Hierophant,
            (Hierophant, Lovers) => Magician,
            (Hierophant, Chariot) => Justice,
            (Hierophant, Justice) => Fool,
            (Hierophant, Hermit) => Chariot,
            (Hierophant, Fortune) => Moon,
            (Hierophant, Strength) => Fortune,
            (Hierophant, Hanged) => Strength,
            (Hierophant, Death) => Fortune,
            (Hierophant, Temperance) => Hermit,
            (Hierophant, Devil) => Priestess,
            (Hierophant, Tower) => Temperance,
            (Hierophant, Star) => Moon,
            (Hierophant, Moon) => Magician,
            (Hierophant, Sun) => Tower,
            (Hierophant, Judgement) => Emperor,
            (Hierophant, Aeon) => Sun,
            (Lovers, Lovers) => Lovers,
            (Lovers, Chariot) => Priestess,
            (Lovers, Justice) => Emperor,
            (Lovers, Hermit) => Fool,
            (Lovers, Fortune) => Temperance,
            (Lovers, Strength) => Hermit,
            (Lovers, Hanged) => Justice,
            (Lovers, Death) => Hanged,
            (Lovers, Temperance) => Death,
            (Lovers, Devil) => Star,
            (Lovers, Tower) => Sun,
            (Lovers, Star) => Death,
            (Lovers, Moon) => Empress,
            (Lovers, Sun) => Devil,
            (Lovers, Judgement) => Moon,
            (Lovers, Aeon) => Tower,
            (Chariot, Chariot) => Chariot,
            (Chariot, Justice) => Magician,
            (Chariot, Hermit) => Lovers,
            (Chariot, Fortune) => Priestess,
            (Chariot, Strength) => Temperance,
            (Chariot, Hanged) => Strength,
            (Chariot, Death) => Hierophant,
            (Chariot, Temperance) => Hermit,
            (Chariot, Devil) => Hanged,
            (Chariot, Tower) => Star,
            (Chariot, Star) => Fortune,
            (Chariot, Moon) => Temperance,
            (Chariot, Sun) => Strength,
            (Chariot, Judgement) => Empress,
            (Chariot, Aeon) => Hermit,
            (Justice, Justice) => Justice,
            (Justice, Hermit) => Magician,
            (Justice, Fortune) => Hanged,
            (Justice, Strength) => Star,
            (Justice, Hanged) => Priestess,
            (Justice, Death) => Hermit,
            (Justice, Temperance) => Moon,
            (Justice, Devil) => Temperance,
            (Justice, Tower) => Sun,
            (Justice, Star) => Hermit,
            (Justice, Moon) => Temperance,
            (Justice, Sun) => Magician,
            (Justice, Judgement) => Fool,
            (Justice, Aeon) => Judgement,
            (Hermit, Hermit) => Hermit,
            (Hermit, Fortune) => Justice,
            (Hermit, Strength) => Emperor,
            (Hermit, Hanged) => Temperance,
            (Hermit, Death) => Chariot,
            (Hermit, Temperance) => Magician,
            (Hermit, Devil) => Strength,
            (Hermit, Tower) => Emperor,
            (Hermit, Star) => Fool,
            (Hermit, Moon) => Hierophant,
            (Hermit, Sun) => Star,
            (Hermit, Judgement) => Temperance,
            (Hermit, Aeon) => Devil,
            (Fortune, Fortune) => Fortune,
            (Fortune, Strength) => Sun,
            (Fortune, Hanged) => Magician,
            (Fortune, Death) => Star,
            (Fortune, Temperance) => Tower,
            (Fortune, Devil) => Empress,
            (Fortune, Tower) => Aeon,
            (Fortune, Star) => Magician,
            (Fortune, Moon) => Death,
            (Fortune, Sun) => Judgement,
            (Fortune, Judgement) => Sun,
            (Fortune, Aeon) => Moon,
            (Strength, Strength) => Strength,
            (Strength, Hanged) => Chariot,
            (Strength, Death) => Empress,
            (Strength, Temperance) => Moon,
            (Strength, Devil) => Lovers,
            (Strength, Tower) => Hanged,
            (Strength, Star) => Priestess,
            (Strength, Moon) => Devil,
            (Strength, Sun) => Lovers,
            (Strength, Judgement) => Devil,
            (Strength, Aeon) => Fool,
            (Hanged, Hanged) => Hanged,
            (Hanged, Death) => Strength,
            (Hanged, Temperance) => Hierophant,
            (Hanged, Devil) => Priestess,
            (Hanged, Tower) => Death,
            (Hanged, Star) => Empress,
            (Hanged, Moon) => Chariot,
            (Hanged, Sun) => Aeon,
            (Hanged, Judgement) => Tower,
            (Hanged, Aeon) => Death,
            (Death, Death) => Death,
            (Death, Temperance) => Devil,
            (Death, Devil) => Tower,
            (Death, Tower) => Aeon,
            (Death, Star) => Sun,
            (Death, Moon) => Hanged,
            (Death, Sun) => Justice,
            (Death, Judgement) => Devil,
            (Temperance, Temperance) => Temperance,
            (Temperance, Devil) => Fool,
            (Temperance, Tower) => Devil,
            (Temperance, Star) => Fortune,
            (Temperance, Moon) => Priestess,
            (Temperance, Sun) => Chariot,
            (Temperance, Judgement) => Empress,
            (Temperance, Aeon) => Justice,
            (Devil, Devil) => Devil,
            (Devil, Tower) => Judgement,
            (Devil, Star) => Justice,
            (Devil, Moon) => Fool,
            (Devil, Sun) => Death,
            (Devil, Judgement) => Death,
            (Devil, Aeon) => Star,
            (Tower, Tower) => Tower,
            (Tower, Star) => Judgement,
            (Tower, Moon) => Fortune,
            (Tower, Sun) => Hierophant,
            (Tower, Judgement) => Aeon,
            (Tower, Aeon) => Sun,
            (Star, Star) => Star,
            (Star, Moon) => Sun,
            (Star, Sun) => Justice,
            (Star, Judgement) => Tower,
            (Star, Aeon) => Judgement,
            (Moon, Moon) => Moon,
            (Moon, Sun) => Tower,
            (Moon, Judgement) => Fortune,
            (Moon, Aeon) => Judgement,
            (Sun, Sun) => Sun,
            (Sun, Judgement) => Aeon,
            (Sun, Aeon) => Empress,
            (Judgement, Judgement) => Judgement,
            (Judgement, Aeon) => Fool,
            (Aeon, Aeon) => Aeon,
            _ => return None,
        };

        Some(result)
    }

    pub fn try_from_str(s: &str) -> Option<Self> {
        use Race::*;

        let first_4_chars = s
            .chars()
            .take(4)
            .flat_map(|c| c.to_lowercase())
            .collect::<String>();
        let this = match first_4_chars.as_str() {
            "fool" => Fool,
            "magi" => Magician,
            "prie" => Priestess,
            "empr" => Empress,
            "empe" => Emperor,
            "hier" => Hierophant,
            "love" => Lovers,
            "char" => Chariot,
            "just" => Justice,
            "herm" => Hermit,
            "fort" => Fortune,
            "stre" => Strength,
            "hang" => Hanged,
            "deat" => Death,
            "temp" => Temperance,
            "devi" => Devil,
            "towe" => Tower,
            "star" => Star,
            "moon" => Moon,
            "sun" => Sun,
            "judg" => Judgement,
            "aeon" => Aeon,
            _ => return None,
        };

        Some(this)
    }

    pub fn as_str(self) -> &'static str {
        use Race::*;

        match self {
            Fool => "Fool",
            Magician => "Magician",
            Priestess => "Priestess",
            Empress => "Empress",
            Emperor => "Emperor",
            Hierophant => "Hierophant",
            Lovers => "Lovers",
            Chariot => "Chariot",
            Justice => "Justice",
            Hermit => "Hermit",
            Fortune => "Fortune",
            Strength => "Strength",
            Hanged => "Hanged",
            Death => "Death",
            Temperance => "Temperance",
            Devil => "Devil",
            Tower => "Tower",
            Star => "Star",
            Moon => "Moon",
            Sun => "Sun",
            Judgement => "Judgement",
            Aeon => "Aeon",
        }
    }
}

#[test]
fn build_match_statement_from_table_str() {
    // source: https://aqiu384.github.io/megaten-fusion-tool/p3r/chart
    const FUSION_TABLE: &str = r#"
            Fool	Magi	Prie	Empr	Empe	Hier	Love	Char	Just	Herm	Fort	Stre	Hang	Deat	Temp	Devi	Towe	Star	Moon	Sun	    Judg	Aeon	
Fool	    Fool	Hier	Magi	Star	Temp	Hang	Just	Empe	Love	Prie	Stre	Deat	Devi	Fort	Char	Herm	Moon	Devi	Empr	Judg	Aeon	Deat
Magician	Hier	Magi	Just	Hang	Love	Herm	Char	Devi	Hier	Moon	Love	Empe	Fool	Prie	Just	Temp	Char	Stre	Stre	Empr	Star	Sun
Priestess	Magi	Just	Prie	Temp	Just	Love	Magi	Fool	Love	Stre	Hang	Moon	Hier	Just	Fort	Empe	Empr	Empe	Star	Hier	Hang	Empr
Empress	    Star	Hang	Temp	Empr	Char	Towe	Moon	Herm	Empe	Sun	    Stre	Fool	Star	Love	Hier	Towe	Devi	Prie	Aeon	Empe	Love	Prie
Emperor	    Temp	Love	Just	Char	Empe	Stre	Char	Devi	Hang	Hier	Star	Magi	Deat	Herm	Star	Moon	Stre	Hier	Love	Temp	Sun	    Fort
Hierophant	Hang	Herm	Love	Towe	Stre	Hier	Magi	Just	Fool	Char	Moon	Fort	Stre	Fort	Herm	Prie	Temp	Moon	Magi	Towe	Empe	Sun
Lovers	    Just	Char	Magi	Moon	Char	Magi	Love	Prie	Empe	Fool	Temp	Herm	Just	Hang	Deat	Star	Sun	    Deat	Empr	Devi	Moon	Towe
Chariot	    Empe	Devi	Fool	Herm	Devi	Just	Prie	Char	Magi	Love	Prie	Temp	Stre	Hier	Herm	Hang	Star	Fort	Temp	Stre	Empr	Herm
Justice 	Love	Hier	Love	Empe	Hang	Fool	Empe	Magi	Just	Magi	Hang	Star	Prie	Herm	Moon	Temp	Sun	    Herm	Temp	Magi	Fool	Judg
Hermit	    Prie	Moon	Stre	Sun	    Hier	Char	Fool	Love	Magi	Herm	Just	Empe	Temp	Char	Magi	Stre	Empe	Fool	Hier	Star	Temp	Devi
Fortune	    Stre	Love	Hang	Stre	Star	Moon	Temp	Prie	Hang	Just	Fort	Sun	    Magi	Star	Towe	Empr	Aeon	Magi	Deat	Judg	Sun 	Moon
Strength	Deat	Empe	Moon	Fool	Magi	Fort	Herm	Temp	Star	Empe	Sun	    Stre	Char	Empr	Moon	Love	Hang	Prie	Devi	Love	Devi	Fool
Hanged	    Devi	Fool	Hier	Star	Deat	Stre	Just	Stre	Prie	Temp	Magi	Char	Hang	Stre	Hier	Prie	Deat	Empr	Char	Aeon	Towe	Deat
Death	    Fort	Prie	Just	Love	Herm	Fort	Hang	Hier	Herm	Char	Star	Empr	Stre	Deat	Devi	Towe	Aeon	Sun	    Hang	Just	Devi	None
Temperance	Char	Just	Fort	Hier	Star	Herm	Deat	Herm	Moon	Magi	Towe	Moon	Hier	Devi	Temp	Fool	Devi	Fort	Prie	Char	Empr	Just
Devil	    Herm	Temp	Empe	Towe	Moon	Prie	Star	Hang	Temp	Stre	Empr	Love	Prie	Towe	Fool	Devi	Judg	Just	Fool	Deat	Deat	Star
Tower	    Moon	Char	Empr	Devi	Stre	Temp	Sun	    Star	Sun	    Empe	Aeon	Hang	Deat	Aeon	Devi	Judg	Towe	Judg	Fort	Hier	Aeon	Sun
Star	    Devi	Stre	Empe	Prie	Hier	Moon	Deat	Fort	Herm	Fool	Magi	Prie	Empr	Sun	    Fort	Just	Judg	Star	Sun 	Just	Towe	Judg
Moon	    Empr	Stre	Star	Aeon	Love	Magi	Empr	Temp	Temp	Hier	Deat	Devi	Char	Hang	Prie	Fool	Fort	Sun 	Moon	Towe	Fort	Judg
Sun	        Judg	Empr	Hier	Empe	Temp	Towe	Devi	Stre	Magi	Star	Judg	Love	Aeon	Just	Char	Deat	Hier	Just	Towe	Sun 	Aeon	Empr
Judgement	Aeon	Star	Hang	Love	Sun	    Empe	Moon	Empr	Fool	Temp	Sun	    Devi	Towe	Devi	Empr	Deat	Aeon	Towe	Fort	Aeon	Judg	Fool
Aeon	    Deat	Sun	    Empr	Prie	Fort	Sun	    Towe	Herm	Judg	Devi	Moon	Fool	Deat	None	Just	Star	Sun	    Judg	Judg	Empr	Fool    Aeon
"#;

    let mut table_rows = FUSION_TABLE.lines().skip(1);
    let columns: Vec<Race> = table_rows
        .next()
        .unwrap()
        .split_whitespace()
        .map(|r| Race::try_from_str(r).unwrap())
        .collect();
    println!("{:?}", columns);

    let mut i = 0;
    let mut table = String::new();
    for line in table_rows {
        println!("row");
        let mut row = line.split_whitespace().map(|r| Race::try_from_str(r));

        let race1 = row.next().unwrap().unwrap();
        // only taking the upper right half of the table
        for (race2, result) in columns.iter().skip(i).zip(row.skip(i)) {
            if let Some(result) = result {
                table.push_str(&format!(
                    "({}, {}) => {},\n",
                    race1.as_str(),
                    race2.as_str(),
                    result.as_str()
                ));
            }
        }
        i += 1;
    }

    println!("{table}");
}
