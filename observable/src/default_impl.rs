use crate::{Observable, SoftEq};

fn simple_cmp<T: PartialEq + Clone>(old: &T, new: &T) -> Vec<(T, T)> {
    if old != new {
        vec![(old.clone(), new.clone())]
    } else {
        vec![]
    }
}

impl Observable for i8 {
    type Mutation = (i8, i8);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for i16 {
    type Mutation = (i16, i16);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for i32 {
    type Mutation = (i32, i32);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for i64 {
    type Mutation = (i64, i64);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for i128 {
    type Mutation = (i128, i128);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for isize {
    type Mutation = (isize, isize);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for u8 {
    type Mutation = (u8, u8);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for u16 {
    type Mutation = (u16, u16);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for u32 {
    type Mutation = (u32, u32);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for u64 {
    type Mutation = (u64, u64);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for u128 {
    type Mutation = (u128, u128);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for usize {
    type Mutation = (usize, usize);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for bool {
    type Mutation = (bool, bool);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

impl Observable for () {
    type Mutation = ();

    fn cmp(&self, _: &Self) -> Vec<Self::Mutation> {
        vec![]
    }
}

impl Observable for String {
    type Mutation = (String, String);

    fn cmp(&self, new: &Self) -> Vec<Self::Mutation> {
        simple_cmp(self, new)
    }
}

#[derive(Debug, PartialEq)]
pub enum VecMutation<T: Observable + SoftEq> {
    Insertion(T::Uid),
    Deletion(T::Uid),
    Update(T::Uid, T::Mutation),
}

impl<T: Observable + SoftEq + Clone> Observable for Vec<T> {
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