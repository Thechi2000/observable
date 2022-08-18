use crate::{Mutable, SoftEq};

fn simple_cmp<T: PartialEq + Clone>(old: &T, new: &T) -> Vec<(T, T)> {
    if old != new {
        vec![(old.clone(), new.clone())]
    } else {
        vec![]
    }
}

impl Mutable for i8 {
    type Mutation = (i8, i8);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for i16 {
    type Mutation = (i16, i16);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for i32 {
    type Mutation = (i32, i32);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for i64 {
    type Mutation = (i64, i64);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for i128 {
    type Mutation = (i128, i128);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for isize {
    type Mutation = (isize, isize);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for u8 {
    type Mutation = (u8, u8);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for u16 {
    type Mutation = (u16, u16);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for u32 {
    type Mutation = (u32, u32);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for u64 {
    type Mutation = (u64, u64);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for u128 {
    type Mutation = (u128, u128);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for usize {
    type Mutation = (usize, usize);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for bool {
    type Mutation = (bool, bool);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Mutable for () {
    type Mutation = ();

    fn cmp(&self, _: &Self) -> Vec<Self::Mutation> {
        vec![]
    }
}

impl Mutable for String {
    type Mutation = (String, String);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

#[derive(Debug, PartialEq)]
pub enum VecMutation<T: Mutable + SoftEq> {
    Insertion(T::Uid),
    Deletion(T::Uid),
    Update(T::Uid, T::Mutation),
}

impl<T: Mutable + SoftEq + Clone> Mutable for Vec<T> {
    type Mutation = VecMutation<T>;

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        let mut updates = Vec::new();

        for new_item in new.iter() {
            let curr = self.iter().find(|a| a.se(new_item));

            if let Some(curr) = curr {
                updates.append(&mut curr.cmp(new_item).into_iter().map(|m| VecMutation::Update(curr.uid(), m)).collect::<Vec<VecMutation<T>>>())
            } else {
                updates.push(VecMutation::Insertion(new_item.uid()));
            }
        }

        for old_item in self.iter() {
            if !new.iter().any(|n| old_item.se(n)) {
                updates.push(VecMutation::Deletion(old_item.uid()));
            }
        }

        updates
    }
}