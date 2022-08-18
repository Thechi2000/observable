pub mod default_impl;

pub trait SoftEq where Self: Sized{
    type Uid;

    fn se(&self, _: &Self) -> bool;
    fn uid(&self) -> Self::Uid;
}

pub trait Observable where Self: Sized {
    type Mutation;

    fn update(&mut self, other: Self) -> Vec<Self::Mutation>{
        let res = self.cmp(&other);
        *self = other;
        res
    }
    fn cmp(&self, _: &Self) -> Vec<Self::Mutation>;
}