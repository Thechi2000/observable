pub trait Observable {
    type Item;
    type MutationEvent;

    fn update(&mut self, _: Self::Item) -> Vec<Self::MutationEvent>;
}
