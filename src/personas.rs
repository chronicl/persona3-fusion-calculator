use crate::recipe::Recipe;
use serde::{Deserialize, Serialize};

use crate::race::Race;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    pub id: PersonaId,
    pub name: String,
    pub lvl: Lvl,
    pub race: Race,
    pub recipes: Vec<Recipe>,

    pub is_special: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct PersonaId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Lvl(pub u32);

#[test]
fn get_personas() {
    use std::collections::HashMap;

    let client = ureq::agent();
    let response = client
        .get("https://aqiu384.github.io/megaten-fusion-tool/47.f742834950cb54aa.js")
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let mut response = response.as_str();

    const SEARCH_STR: &str = "JSON.parse('";
    const NORMAL_PERSONA: &str = "Oberon";
    const SPECIAL_PERSONA: &str = "Pale Rider";

    let mut persona_list_str = None;
    let mut special_persona_recipes_str = None;
    while let Some(start) = response.find(SEARCH_STR) {
        let end = find_not_escaped(&response[start + SEARCH_STR.len()..], '\'').unwrap();
        let json = &response[start + SEARCH_STR.len()..start + SEARCH_STR.len() + end];
        response = &response[start + SEARCH_STR.len() + end..];

        if json.contains(NORMAL_PERSONA) {
            persona_list_str = Some(json);
        }

        // collecting special recipes
        if json.contains(SPECIAL_PERSONA) {
            special_persona_recipes_str = Some(json);
        }
    }

    let mut persona_data = Vec::new();
    // unescape '
    let persona_list_str = persona_list_str
        .expect("persona list not found")
        .replace(r"\'", "'");
    let data: HashMap<String, PersonaData> = serde_json::from_str(&persona_list_str).unwrap();
    for (name, data) in data {
        persona_data.push((data.lvl, data.race, name.to_string()));
    }

    // lvl > race > name
    persona_data.sort_unstable();
    let mut personas = persona_data
        .into_iter()
        .enumerate()
        .map(|(id, (lvl, race, name))| Persona {
            id: PersonaId(id),
            name,
            lvl: Lvl(lvl),
            race,
            recipes: Vec::new(),
            is_special: false,
        })
        .collect::<Vec<_>>();
    let name_to_idx: HashMap<String, usize> = personas
        .iter()
        .map(|Persona { name, id, .. }| (name.to_owned(), id.0))
        .collect();

    // unescape '
    let special_persona_recipes_str = special_persona_recipes_str
        .expect("special personas not found")
        .replace(r"\'", "'");
    let data: HashMap<String, Vec<String>> =
        serde_json::from_str(&special_persona_recipes_str).unwrap();
    for (name, recipe) in data {
        let persona = name_to_idx[&name];

        personas[persona].is_special = true;

        let recipe = Recipe::Special(
            recipe
                .iter()
                .map(|name| PersonaId(name_to_idx[name]))
                .collect(),
        );
        personas[persona].recipes.push(recipe);
    }

    std::fs::write(
        "src/personas.json",
        serde_json::to_string(&personas).unwrap(),
    )
    .unwrap();

    fn find_not_escaped(s: &str, c: char) -> Option<usize> {
        if s.starts_with(c) {
            return Some(0);
        }

        for (i, (previous, current)) in s.chars().zip(s.chars().skip(1)).enumerate() {
            if current == c && previous != '\\' {
                return Some(i + 1);
            }
        }

        None
    }

    #[derive(Debug, Clone, Copy, Deserialize)]
    struct PersonaData {
        lvl: u32,
        race: Race,
    }
}
