use crate::mutation::{Mutator, i32geu_to_gtu_mutator};

pub(crate) static ALL_MUTATORS: [&'static (dyn Mutator + Send + Sync + 'static); 1] = [
    &i32geu_to_gtu_mutator::MUTATOR
];
