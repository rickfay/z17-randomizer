use std::collections::{HashMap, HashSet};

use albw::{course, Item};
use log::{debug, error};
use vec_drain_where::VecDrainWhereExt;

use crate::{
    graph::{Graph, Node},
    queue::Queue,
    regions::Subregion,
    state::State,
    Condition, ItemExt, Layout, Location, Pool, Settings,
};

#[derive(Clone)]
struct Filler<'a> {
    weights: &'a HashMap<Location, u32>,
    state: State<'a>,
    layout: Layout,
    subregions: HashSet<&'static Subregion>,
    locations: Queue<Location>,
    excluded: Queue<Location>,
    edges: Vec<(Condition, Node)>,
    restrict: Option<course::Id>,
}

impl<'a> Filler<'a> {
    fn new(weights: &'a HashMap<Location, u32>, state: State<'a>, layout: Layout) -> Self {
        Self {
            weights,
            state,
            layout,
            subregions: Default::default(),
            locations: Default::default(),
            excluded: Default::default(),
            edges: Default::default(),
            restrict: Default::default(),
        }
    }

    /// Adds a subregion node to the graph.
    fn add_subregion(&mut self, subregion: &'static Subregion) {
        self.subregions.insert(subregion);
        subregion.add(self);
        if let Some(quest) = subregion.quest() {
            self.state.add_quest_item(quest);
        }
    }

    /// Adds a location node to the graph.
    fn add_location(&mut self, location: &Location) {
        if self
            .restrict
            .map(|dungeon| dungeon == location.subregion.course())
            .unwrap_or(true)
        {
            if let Some(item) = self.layout.get(location) {
                self.state
                    .add_item_with_location(item, location.subregion.course());
            } else {
                if self.settings().is_excluded(location) {
                    &mut self.excluded
                } else {
                    &mut self.locations
                }
                .push(
                    *self
                        .weights
                        .get(location)
                        .expect("Locations were not initialized properly."),
                    location.clone(),
                );
            }
        }
    }

    fn add_item(&mut self, item: Item) {
        self.state.add_item(item);
        loop {
            let state = self.state.clone();
            let edges = self
                .edges
                .e_drain_where(|(edge, _)| edge(&state))
                .collect::<Vec<_>>();
            for (_, node) in edges {
                self.add(node);
            }
            if !state.is_different(&self.state) {
                break;
            }
        }
    }

    /// Sets a location's item and adds new reachable nodes.
    fn add_item_in_location(&mut self, item: Item, location: Location) {
        debug!(
            "placed {} in {}/{}",
            item.normalize().as_str(),
            location.subregion.name(),
            location.name
        );
        self.state
            .add_item_with_location(item, location.subregion.course());
        self.layout.set(location, item);
        loop {
            let state = self.state.clone();
            let edges = self
                .edges
                .e_drain_where(|(edge, _)| edge(&state))
                .collect::<Vec<_>>();
            for (_, node) in edges {
                self.add(node);
            }
            if !state.is_different(&self.state) {
                break;
            }
        }
    }

    fn can_progress(&self, item: Item) -> bool {
        let mut filler = self.clone();
        filler.add_item(item);
        if item.is_ore() {
            if filler.locations.len() > 1 {
                filler.add_item(item);
                filler.edges.is_empty() || filler.locations.len() > self.locations.len()
            } else {
                false
            }
        } else {
            filler.edges.is_empty() || filler.locations.len() > self.locations.len()
        }
    }

    /// Create a world layout by filling items in random locations.
    fn fill<F>(mut self, pool: Pool, start: &'static Subregion, f: F) -> Layout
    where
        F: Fn(&mut Queue<Item>, &Filler) -> Item,
    {
        let Pool {
            mut progression,
            mut rest,
        } = pool;
        if progression
            .peek()
            .expect("The item pool was not initialized properly.")
            .is_dungeon()
        {
            self.restrict = Some(start.course());
        }
        self.add_subregion(start);
        while !self.edges.is_empty() {
            let item = f(&mut progression, &self);
            let location = self.locations.pop().unwrap_or_else(|| unreachable!());
            self.add_item_in_location(item, location);
            if item.is_ore() {
                let location = self.locations.pop().unwrap_or_else(|| unreachable!());
                self.add_item_in_location(
                    progression
                        .remove(ItemExt::is_ore)
                        .unwrap_or_else(|| unreachable!()),
                    location,
                );
            }
        }
        if self.restrict.is_none() {
            let zelda = Location::new(&crate::regions::dungeons::castle::boss::SUBREGION, "Zelda");
            self.locations.push(
                *self
                    .weights
                    .get(&zelda)
                    .expect("Locations were not initialized."),
                zelda,
            );
            let bow_of_light = rest
                .remove(|item| *item == Item::ItemBowLight)
                .expect("Bow of Light was not properly set.");
            self.layout.set(
                self.locations
                    .pop()
                    .expect("Could not place Bow of Light, ran out of locations."),
                bow_of_light,
            );
        }
        rest.merge(progression);
        self.locations.merge(self.excluded);
        while let Some(item) = rest.pop() {
            self.layout
                .set(self.locations.pop().expect("Ran out of locations."), item);
        }
        self.layout
    }
}

impl<'a> Graph for Filler<'a> {
    fn settings(&self) -> &Settings {
        self.state.settings()
    }

    fn check(&self, predicate: fn(&State) -> bool) -> bool {
        predicate(&self.state)
    }

    fn add(&mut self, node: Node) {
        match node {
            Node::Location(location) => {
                self.add_location(&location);
            }
            Node::Path(path) => {
                if !self.subregions.contains(path) {
                    self.add_subregion(path);
                }
            }
        }
    }

    fn add_edge(&mut self, edge: Condition, node: Node) {
        self.edges.push((edge, node));
    }
}

fn fill_dungeon(
    settings: &Settings,
    weights: &HashMap<Location, u32>,
    layout: Layout,
    pool: Pool,
    start: &'static Subregion,
) -> Layout {
    Filler::new(weights, State::with_all_overworld_items(settings), layout).fill(
        pool,
        start,
        |pool, filler| {
            if filler.locations.len() == 1 && pool.peek() == Some(&Item::KeyBoss) {
                pool.remove(|item| *item == Item::KeySmall)
            } else {
                pool.pop()
            }
            .unwrap_or_else(|| {
                unreachable!("{:?}", pool)
                //filler.error(&pool)
            })
        },
    )
}

fn fill_world(
    settings: &Settings,
    weights: &HashMap<Location, u32>,
    layout: Layout,
    pool: Pool,
    start: &'static Subregion,
) -> Layout {
    Filler::new(weights, State::new(settings), layout).fill(pool, start, |pool, filler| {
        pool.remove(|&item| filler.can_progress(item))
            .unwrap_or_else(|| {
                error!("{:?}", filler.state);
                error!("{:?}", filler.layout);
                error!("{:?}", pool);
                for (_, edge) in &filler.edges {
                    error!("-> {:?}", edge);
                }
                panic!("Could not find an overworld progression item.")
            })
    })
}

macro_rules! fill_dungeons {
    (
        $settings:expr,
        $weights:expr,
        $layout:expr,
        $pool:expr,
        $($world:ident::$region:ident,)+
    ) => {{
        let layout = $layout;
        $(let layout = {
            use crate::regions::$world::$region as dungeon;
            fill_dungeon(
                $settings,
                $weights,
                layout,
                $pool.remove(&dungeon::COURSE).expect("Dungeon pool was not properly initialized."),
                dungeon::start(),
            )
        };)+
        layout
    }};
}

pub(crate) fn fill(
    settings: &Settings,
    weights: HashMap<Location, u32>,
    layout: Layout,
    pool: Pool,
    mut dungeons: HashMap<course::Id, Pool>,
) -> Layout {
    let layout = fill_dungeons!(
        settings,
        &weights,
        layout,
        dungeons,
        hyrule::sanctuary,
        dungeons::eastern,
        dungeons::house,
        dungeons::tower,
        dungeons::graveyard,
        dungeons::dark,
        dungeons::swamp,
        dungeons::skull,
        dungeons::thieves,
        dungeons::ice,
        dungeons::desert,
        dungeons::turtle,
        dungeons::castle,
    );
    fill_world(
        settings,
        &weights,
        layout,
        pool,
        crate::regions::hyrule::field::start(),
    )
}
