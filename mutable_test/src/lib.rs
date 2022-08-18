#[cfg(test)]
mod test {
    use mutable::default_impl::VecMutation;
    use mutable::SoftEq;
    use mutable::Mutable;
    use mutable_derive::Mutable;
    use mutable_derive::SoftEq;

    #[derive(Mutable, Clone, Debug, PartialEq)]
    struct Simple {
        size: usize,
        string: String,
    }

    #[derive(Mutable, Clone, SoftEq, Debug, PartialEq)]
    struct Identifiable {
        #[softeq(uid)]
        id: String,
        value: u32,
    }

    #[derive(Mutable, Clone, SoftEq, Debug, PartialEq)]
    struct Complex{
        #[softeq(uid)]
        id: String,
        value: Simple,
    }

    #[test]
    fn simple_mutation() {
        let mut a0 = Simple{ size: 0, string: String::new()};
        let a1 = Simple{ size: 1, string: String::new()};
        let a2 = Simple{ size: 1, string: "str".to_string()};
        
        assert_eq!(a0.cmp(&a0), vec![]);
        assert_eq!(a1.cmp(&a1), vec![]);
        assert_eq!(a2.cmp(&a2), vec![]);
        
        assert_eq!(a0.cmp(&a1), vec![SimpleMutation::Size((0, 1))]);
        assert_eq!(a1.cmp(&a0), vec![SimpleMutation::Size((1, 0))]);

        assert_eq!(a1.cmp(&a2), vec![SimpleMutation::String(("".to_string(), "str".to_string()))]);
        assert_eq!(a2.cmp(&a1), vec![SimpleMutation::String(("str".to_string(), "".to_string()))]);

        assert_eq!(a0.cmp(&a2), vec![SimpleMutation::Size((0, 1)), SimpleMutation::String(("".to_string(), "str".to_string()))]);
        assert_eq!(a2.cmp(&a0), vec![SimpleMutation::Size((1, 0)), SimpleMutation::String(("str".to_string(), "".to_string()))]);

        assert_eq!(a0.update(a1.clone()), vec![SimpleMutation::Size((0, 1))]);
        assert_eq!(a0, a1);

        assert_eq!(a0.update(a2.clone()), vec![SimpleMutation::String(("".to_string(), "str".to_string()))]);
        assert_eq!(a0, a2);
    }

    #[test]
    fn simple_vec_mutation(){
        let a0 = Identifiable{id: "a".to_string(), value: 0};
        let a1 = Identifiable{id: "a".to_string(), value: 1};

        let b0 = Identifiable{id: "b".to_string(), value: 0};
        let b1 = Identifiable{id: "b".to_string(), value: 1};

        let mut v0 = vec![a0.clone()];
        let v1 = vec![a1.clone()];
        let v2 = vec![a0.clone(), b0.clone()];
        let v3 = vec![a0.clone(), b1.clone()];
        let v4 = vec![a1.clone(), b1.clone()];

        assert_eq!(v0.cmp(&v0), vec![]);
        assert_eq!(v1.cmp(&v1), vec![]);
        assert_eq!(v2.cmp(&v2), vec![]);
        assert_eq!(v3.cmp(&v3), vec![]);
        assert_eq!(v4.cmp(&v4), vec![]);

        assert_eq!(v0.cmp(&v1), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((0, 1)))]);
        assert_eq!(v1.cmp(&v0), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((1, 0)))]);

        assert_eq!(v0.cmp(&v2), vec![VecMutation::Insertion("b".to_string())]);
        assert_eq!(v2.cmp(&v0), vec![VecMutation::Deletion("b".to_string())]);

        assert_eq!(v2.cmp(&v3), vec![VecMutation::Update("b".to_string(),IdentifiableMutation::Value((0, 1)))]);
        assert_eq!(v3.cmp(&v2), vec![VecMutation::Update("b".to_string(),IdentifiableMutation::Value((1, 0)))]);

        assert_eq!(v0.cmp(&v4), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((0, 1))), VecMutation::Insertion("b".to_string())]);
        assert_eq!(v4.cmp(&v0), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((1, 0))), VecMutation::Deletion("b".to_string())]);

        assert_eq!(v0.update(v1.clone()), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((0, 1)))]);
        assert_eq!(v0, v1);

        assert_eq!(v0.update(v3.clone()), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((1, 0))), VecMutation::Insertion("b".to_string())]);
        assert_eq!(v0, v3);

        assert_eq!(v0.update(v4.clone()), vec![VecMutation::Update("a".to_string(),IdentifiableMutation::Value((0, 1)))]);
        assert_eq!(v0, v4);
    }

    #[test]
    fn complex_mutation(){
        let mut c0 = Complex{ id: "a".to_string(), value: Simple { size: 32, string: "str".to_string() } };
        let c1 = Complex{ id: "b".to_string(), value: Simple { size: 32, string: "str".to_string() } };
        let c2 = Complex{ id: "b".to_string(), value: Simple { size: 64, string: "str".to_string() } };

        assert_eq!(c0.cmp(&c0), vec![]);
        assert_eq!(c1.cmp(&c1), vec![]);
        assert_eq!(c2.cmp(&c2), vec![]);

        assert_eq!(c0.cmp(&c1), vec![ComplexMutation::Id(("a".to_string(), "b".to_string()))]);
        assert_eq!(c1.cmp(&c0), vec![ComplexMutation::Id(("b".to_string(), "a".to_string()))]);

        assert_eq!(c1.cmp(&c2), vec![ComplexMutation::Value(SimpleMutation::Size((32, 64)))]);
        assert_eq!(c2.cmp(&c1), vec![ComplexMutation::Value(SimpleMutation::Size((64, 32)))]);

        assert_eq!(c0.cmp(&c2), vec![ComplexMutation::Id(("a".to_string(), "b".to_string())), ComplexMutation::Value(SimpleMutation::Size((32, 64)))]);
        assert_eq!(c2.cmp(&c0), vec![ComplexMutation::Id(("b".to_string(), "a".to_string())), ComplexMutation::Value(SimpleMutation::Size((64, 32)))]);

        assert_eq!(c0.update(c2.clone()), vec![ComplexMutation::Id(("a".to_string(), "b".to_string())), ComplexMutation::Value(SimpleMutation::Size((32, 64)))]);
        assert_eq!(c0, c2);
    }
}