use personas::{Lvl, Persona, PersonaId};
use petgraph::dot::{Config, Dot};
use race::Race;
use recipe::Recipe;
use std::collections::{HashMap, HashSet, VecDeque};

mod personas;
mod race;
mod recipe;

#[derive(Debug, Clone)]
pub struct Personas {
    personas: Vec<Persona>,
    name_to_id: HashMap<String, PersonaId>,
    /// Vec is sorted by lvl
    normal_persona_race_index: HashMap<Race, Vec<(PersonaId, Lvl)>>,

    // cou be relplaced with a bitset
    buyable: HashSet<(PersonaId, u32)>,
}

impl Personas {
    pub fn new() -> Self {
        let personas: Vec<Persona> = serde_json::from_str(include_str!("personas.json")).unwrap();
        let name_to_id = personas
            .iter()
            .map(|p| (p.name.clone(), p.id))
            .collect::<HashMap<_, _>>();
        let mut race_index: HashMap<Race, Vec<(PersonaId, Lvl)>> = HashMap::new();

        for persona in personas.iter() {
            if !persona.is_special {
                race_index
                    .entry(persona.race)
                    .or_default()
                    .push((persona.id, persona.lvl));
            }
        }

        for race in race_index.values_mut() {
            race.sort_by_key(|(_, lvl)| *lvl);
            race.dedup();
        }

        let mut this = Self {
            personas,
            name_to_id,
            normal_persona_race_index: race_index,
            buyable: Default::default(),
        };

        this.calculate_recipes_for_non_special_personas();

        this
    }

    pub fn personas(&self) -> &[Persona] {
        &self.personas
    }

    pub fn recipe_calculator(&self) -> RecipeCalculator {
        let mut recipes: Vec<RecipeCounter> = Vec::new();
        let mut persona_recipe_info: Vec<PersonaRecipeInfo> = vec![
            PersonaRecipeInfo {
                buy_price: None,
                is_reachable: false,
                result_of: Vec::new(),
                ingredient_of: Vec::new(),

                cheapest_recipe: None,
                cheapest_price: None,
            };
            self.personas.len()
        ];

        for (id, price) in self.buyable.iter() {
            persona_recipe_info[id.0].buy_price = Some(*price);
            persona_recipe_info[id.0].cheapest_price = Some(*price);
            persona_recipe_info[id.0].is_reachable = true;
        }

        for persona in self.personas.iter() {
            for recipe in persona.recipes.iter() {
                let recipe_idx = recipes.len();
                recipes.push(RecipeCounter::new(persona.id, recipe.clone()));

                persona_recipe_info[persona.id.0].result_of.push(recipe_idx);
                match recipe {
                    Recipe::Normal(a, b) => {
                        persona_recipe_info[a.0].ingredient_of.push(recipe_idx);
                        persona_recipe_info[b.0].ingredient_of.push(recipe_idx);
                    }
                    Recipe::Special(ingredients) => {
                        for ingredient in ingredients {
                            persona_recipe_info[ingredient.0]
                                .ingredient_of
                                .push(recipe_idx);
                        }
                    }
                }
            }
        }

        RecipeCalculator::new(persona_recipe_info, recipes)
    }

    pub fn add_buyable_personas(&mut self, names: &[(&str, u32)]) {
        for (name, price) in names {
            self.add_buyable_persona(name, *price);
        }
    }

    pub fn add_buyable_persona(&mut self, name: &str, price: u32) {
        if let Some(id) = self.name_to_id.get(name) {
            self.buyable.insert((*id, price));
        } else {
            panic!("Persona not found: {}", name);
        }
    }

    fn calculate_recipes_for_non_special_personas(&mut self) {
        let persona_ids: Vec<_> = self.personas.iter().map(|p| p.id).collect();
        for persona1 in persona_ids.iter() {
            'a: for persona2 in persona_ids.iter() {
                if persona1 == persona2 {
                    continue;
                }

                let race1 = self.personas[persona1.0].race;
                let race2 = self.personas[persona2.0].race;
                if let Some(resulting_race) = race1.fuse(race2) {
                    let personas_with_resulting_race =
                        &self.normal_persona_race_index[&resulting_race];

                    let average_lvl = (self.personas[persona1.0].lvl.0
                        + self.personas[persona2.0].lvl.0)
                        .div_ceil(2);

                    // If the races are the same, min_resulting_lvl is actually max_resulting_lvl
                    // and the resulting persona is neither of the two input personas.
                    let (resulting_persona, recipe) = if race1 == race2 {
                        // adding 1 to average lvl, in this case
                        let resulting_persona_idx = personas_with_resulting_race
                            .binary_search_by_key(&Lvl(average_lvl + 1), |(_, lvl)| *lvl);

                        let mut resulting_persona_idx = match resulting_persona_idx {
                            Ok(i) => i,
                            Err(i) => i.saturating_sub(1),
                        };

                        let is_one_of_the_input_personas =
                            |persona: PersonaId| persona == *persona1 || persona == *persona2;

                        while is_one_of_the_input_personas(
                            personas_with_resulting_race[resulting_persona_idx].0,
                        ) {
                            if resulting_persona_idx == 0 {
                                continue 'a;
                            }
                            resulting_persona_idx -= 1;
                        }

                        let resulting_persona =
                            personas_with_resulting_race[resulting_persona_idx].0;

                        (
                            resulting_persona,
                            Recipe::Normal(*persona1.min(persona2), *persona1.max(persona2)),
                        )
                    } else {
                        let resulting_persona_idx = personas_with_resulting_race
                            .binary_search_by_key(&Lvl(average_lvl), |(_, lvl)| *lvl);

                        let mut resulting_persona_idx = match resulting_persona_idx {
                            Ok(i) | Err(i) => i,
                        };

                        // If lvl is higher than all personas of the race, it becomes the highest lvl of the race
                        if resulting_persona_idx == personas_with_resulting_race.len() {
                            resulting_persona_idx -= 1;
                        }

                        let resulting_persona =
                            personas_with_resulting_race[resulting_persona_idx].0;

                        (resulting_persona, Recipe::Normal(*persona1, *persona2))
                    };

                    self.personas[resulting_persona.0].recipes.push(recipe);
                }
            }
        }

        for persona in self.personas.iter_mut() {
            persona.recipes.sort();
            persona.recipes.dedup();
        }
    }
}

pub struct RecipeCalculator {
    persona_recipe_info: Vec<PersonaRecipeInfo>,
    recipes: Vec<RecipeCounter>,

    processed: bool,
}

impl RecipeCalculator {
    fn new(persona_recipe_info: Vec<PersonaRecipeInfo>, recipes: Vec<RecipeCounter>) -> Self {
        Self {
            persona_recipe_info,
            recipes,
            processed: false,
        }
    }

    pub fn process(&mut self) {
        if self.processed {
            return;
        }

        let mut queue: VecDeque<usize> = self
            .persona_recipe_info
            .iter()
            .enumerate()
            .filter_map(|(i, info)| {
                if info.buy_price.is_some() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let Self {
            persona_recipe_info: persona_info,
            recipes,
            ..
        } = self;

        // calculate which ones
        let mut ingredient_buffer = Vec::<usize>::new();
        // TODO: change to front
        while let Some(persona_to_activate) = queue.pop_front() {
            assert!(
                persona_info[persona_to_activate].cheapest_price.is_some(),
                "Price is None {}",
                persona_to_activate
            );

            ingredient_buffer.extend(persona_info[persona_to_activate].ingredient_of.iter());
            for recipe_idx in ingredient_buffer.iter() {
                let recipe = &mut recipes[*recipe_idx];
                recipe.count -= 1;

                if recipe.count == 0 {
                    let resulting_persona_id = recipe.result.0;
                    let resulting_persona = &mut persona_info[resulting_persona_id];

                    // if the persona was not reachable before we add it to our queue
                    if !resulting_persona.is_reachable {
                        resulting_persona.is_reachable = true;
                        queue.push_back(resulting_persona_id);
                    }

                    // all personas in queue have a price, therefore this is not None

                    update_cheapest_price(persona_info, recipes, resulting_persona_id, *recipe_idx);
                    assert!(persona_info[resulting_persona_id].cheapest_price.is_some());

                    fn update_cheapest_price(
                        persona_info: &mut [PersonaRecipeInfo],
                        recipes: &[RecipeCounter],
                        persona_id: usize,
                        recipe_idx: usize,
                    ) {
                        let recipe_price = recipes[recipe_idx].price(&persona_info).unwrap();

                        let cheaper_price_found = match persona_info[persona_id].cheapest_price {
                            Some(cheapest_price) if recipe_price < cheapest_price => {
                                persona_info[persona_id].cheapest_price = Some(recipe_price);
                                persona_info[persona_id].cheapest_recipe = Some(recipe_idx);
                                true
                            }
                            None => {
                                persona_info[persona_id].cheapest_price = Some(recipe_price);
                                persona_info[persona_id].cheapest_recipe = Some(recipe_idx);
                                true
                            }
                            _ => false,
                        };

                        if cheaper_price_found {
                            let ingredients = persona_info[persona_id].ingredient_of.clone();
                            for recipe_idx in ingredients.iter() {
                                let recipe = &recipes[*recipe_idx];

                                if recipe.count == 0 {
                                    let resulting_persona_id = recipe.result.0;

                                    update_cheapest_price(
                                        persona_info,
                                        recipes,
                                        resulting_persona_id,
                                        *recipe_idx,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            ingredient_buffer.clear();
        }
    }

    pub fn is_reachable(&self, persona: PersonaId) -> bool {
        self.persona_recipe_info[persona.0].is_reachable
    }

    pub fn price(&self, persona: PersonaId) -> Option<u32> {
        self.persona_recipe_info[persona.0].cheapest_price
    }

    pub fn recipe_tree_dot(&self, wanted_persona: &str, personas: &Personas) -> String {
        let wanted_persona = personas.name_to_id[wanted_persona];

        let mut graph = petgraph::graph::Graph::new();
        let mut queue = VecDeque::new();
        queue.push_back(wanted_persona);

        let add_persona_to_graph =
            |graph: &mut petgraph::graph::Graph<String, ()>,
             persona_id: PersonaId,
             personas: &Personas,
             id_to_node: &mut [petgraph::graph::NodeIndex]| {
                let persona = &personas.personas[persona_id.0];
                let persona_node = graph.add_node(persona.name.clone());
                id_to_node[persona_id.0] = persona_node;
                persona_node
            };

        let mut id_to_node = vec![petgraph::graph::node_index(0); personas.personas().len()];

        add_persona_to_graph(&mut graph, wanted_persona, personas, &mut id_to_node);

        while let Some(persona_id) = queue.pop_front() {
            let persona_node = id_to_node[persona_id.0];

            if let Some(recipe_idx) = self.persona_recipe_info[persona_id.0].cheapest_recipe {
                let recipe = &self.recipes[recipe_idx];

                match recipe {
                    RecipeCounter {
                        recipe: Recipe::Normal(a, b),
                        ..
                    } => {
                        let a_node =
                            add_persona_to_graph(&mut graph, *a, personas, &mut id_to_node);
                        let b_node =
                            add_persona_to_graph(&mut graph, *b, personas, &mut id_to_node);
                        graph.add_edge(persona_node, a_node, ());
                        graph.add_edge(persona_node, b_node, ());
                        queue.push_back(*a);
                        queue.push_back(*b);
                    }
                    RecipeCounter {
                        recipe: Recipe::Special(ingredients),
                        ..
                    } => {
                        for ingredient in ingredients {
                            let ingredient_node = add_persona_to_graph(
                                &mut graph,
                                *ingredient,
                                personas,
                                &mut id_to_node,
                            );
                            graph.add_edge(persona_node, ingredient_node, ());
                            queue.push_back(*ingredient);
                        }
                    }
                }
            } else {
                // persona is buyable
                let buy_price = self.persona_recipe_info[persona_id.0].buy_price.unwrap();
                let buy_node = graph.add_node(format!("Buy for {}", buy_price));
                graph.add_edge(persona_node, buy_node, ());
            }
        }

        format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).replace("\\\"", "")
    }
}

#[derive(Debug, Clone)]
struct PersonaRecipeInfo {
    // TODO: maybe remove because cheapest price is None if not reachable anywa
    is_reachable: bool,
    buy_price: Option<u32>,

    result_of: Vec<usize>,
    ingredient_of: Vec<usize>,

    // Is None if the persona is not reachable
    cheapest_price: Option<u32>,
    // Is None if cheapest_price is None or if the cheapest price is the buy price
    cheapest_recipe: Option<usize>,
}

#[derive(Debug, Clone)]
struct RecipeCounter {
    result: PersonaId,
    recipe: Recipe,
    count: u32,
}

impl RecipeCounter {
    fn new(result: PersonaId, recipe: Recipe) -> Self {
        let count = match &recipe {
            Recipe::Normal(_, _) => 2,
            Recipe::Special(personas) => personas.len() as u32,
        };
        Self {
            result,
            recipe,
            count,
        }
    }

    /// Is None if one of the ingredients does not have a price
    fn price(&self, personas: &[PersonaRecipeInfo]) -> Option<u32> {
        match &self.recipe {
            Recipe::Normal(a, b) => {
                let price_a = personas[a.0].cheapest_price?;
                let price_b = personas[b.0].cheapest_price?;
                Some(price_a + price_b)
            }
            Recipe::Special(ingredients) => {
                let mut price = 0;
                for persona in ingredients {
                    price += personas[persona.0].cheapest_price?;
                }
                Some(price)
            }
        }
    }
}
