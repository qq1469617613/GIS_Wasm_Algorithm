use std::collections::BTreeMap;

type Graph = BTreeMap<u32, BTreeMap<u32, f64>>;

//输入边描述
#[derive(Clone, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum InputDescription {
    // (from,to,weight)
    Tuple((u32, u32, f64)),
    Object { from: u32, to: u32, weight: f64 },
}

impl InputDescription {
    pub fn into_parts(self) -> (u32, u32, f64) {
        match self {
            InputDescription::Tuple((from, to, weight)) => (from, to, weight),
            InputDescription::Object { from, to, weight } => (from, to, weight),
        }
    }
}

pub fn build_graph(input_description: &[InputDescription], undirected: bool) -> Graph {
    let mut graph = Graph::new();

    let mut add_edge = |from: u32, to: u32, weight: f64| {
        graph
            .entry(from)
            .or_insert_with(BTreeMap::new)
            .insert(to, weight);
        graph.entry(to).or_insert_with(BTreeMap::new);
    };

    for e in input_description.iter().cloned() {
        let (from, to, weight) = e.into_parts();
        add_edge(from, to, weight);
        if undirected {
            add_edge(to, from, weight);
        }
    }
    graph
}
