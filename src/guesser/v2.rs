use crate::grammar::model::Model;


pub struct Generator {
    // queue: BinaryHeap<Node>,
    model: Model,
    base_idx: usize,
    var_idx: usize
    // pending: VecDeque<String>
}

impl Generator {
    pub fn new(model: Model) -> Self {
        // Generator { queue: BinaryHeap::new(), model }
        Generator { model, base_idx: 0, var_idx: 0 }
    }
}

impl Iterator for Generator {
    type Item = String;

    /// v2
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: base structures list can be iterator in model
        if let Some((size, base_structures_list)) = self.model.base_structures().get(self.base_idx) {
            for base in base_structures_list {
                println!("Expanding {:?} with size {} out of {} groups", base_structures_list, size, self.model.base_structures().iter().count());

                for var in base {
                    
                    if let Some(terminals) = self.model.terminals().get(var) {
                        for (occ, s) in terminals.iter().filter_map(|(occ, s)| if *occ > 16 { Some((occ, s)) } else { None }) {
                            println!("Length {occ}: {s:?}");
                        }
                    }
                }
            }

        }

        None
    }

    // /// v1
    // fn next(&self) -> Option<Self::Item> { None }
}