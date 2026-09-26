mod json_concepts;
use crate::concept::Concept;
use indexmap::IndexSet;

pub struct ConceptRegistry {
    concepts: IndexSet<Concept>,
}

impl ConceptRegistry {
    pub fn new() -> color_eyre::Result<Self> {
        let mut registry = Self { concepts: IndexSet::from(Concept::defaults()) };

        let concept_files = json_concepts::get_files()?;
        let parsed_concepts = json_concepts::parse_concepts(concept_files)?;

        registry.concepts.reserve(parsed_concepts.len());
        registry.concepts.extend(parsed_concepts);

        Ok(registry)
    }

    pub fn get(&self, id: &str) -> Option<&Concept> {
        self.concepts.get(id)
    }

    pub fn at_index(&self, index: usize) -> Option<&Concept> {
        self.concepts.get_index(index)
    }

    pub fn count(&self) -> usize {
        self.concepts.len()
    }

    pub fn all_ids(&self) -> Vec<&str> {
        self.concepts.iter().map(|concept| concept.id.as_str()).collect()
    }
}
