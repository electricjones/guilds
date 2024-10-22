pub mod player;

use crate::cards::Cards;

type CardStack = Vec<Cards>;

// TODO: Better name: Used to create card stack from various sources
//         - json
//         - manifests
//         - builders
trait CardsCanBePopulated {
    fn from() -> Self;
}

impl CardsCanBePopulated for CardStack {
    fn from() -> Self {
        todo!()
    }
}
