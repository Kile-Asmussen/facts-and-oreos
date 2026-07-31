use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
};

#[derive(Clone)]
pub struct Path(pub Vec<String>);

#[derive(Clone)]
pub struct Collector<T> {
    path: Path,
    pub visited: Vec<(Path, T)>,
}

impl<T> Collector<T> {
    pub fn new() -> Self {
        Collector { path: Path(Vec::new()), visited: Vec::new() }
    }
}

pub trait Visitor {
    fn grab(&mut self, val: &dyn Any) -> bool;

    fn visit(&mut self, val: &dyn Visitable, entry: String);
}

impl<T: Clone + 'static> Visitor for Collector<T> {
    fn grab(&mut self, val: &dyn Any) -> bool {
        if let Some(v) = (val as &dyn Any).downcast_ref::<T>() {
            self.visited.push((self.path.clone(), v.clone()));
            return true;
        } else {
            return false;
        }
    }

    fn visit(&mut self, val: &dyn Visitable, entry: String) {
        self.path.0.push(entry);
        if !val.visit_leaf(self) {
            val.visit_node(self);
        }
        self.path.0.pop();
    }
}

pub trait Visitable: Any {
    #[allow(unused)]
    fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
        false
    }

    #[allow(unused)]
    fn visit_node(&self, visitor: &mut dyn Visitor) {}
}

#[macro_export]
macro_rules! impl_visitable_simple {
    ($($t:ty)*) => {
        $(
            impl Visitable for $t {
                fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
                    visitor.grab(self)
                }
            }
        )*
    };
}

impl_visitable_simple!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 bool f32 f64 String);

impl Visitable for serde_json::Value {}

impl<T: Visitable + 'static> Visitable for Box<T> {
    fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
        self.as_ref().visit_leaf(visitor)
    }
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        self.as_ref().visit_node(visitor);
    }
}

impl<T: Visitable + 'static> Visitable for [T] {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        for (sz, x) in self.iter().enumerate() {
            visitor.visit(x, sz.to_string());
        }
    }
}

impl<T: Visitable + 'static> Visitable for Vec<T> {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        for (sz, x) in self.iter().enumerate() {
            visitor.visit(x, sz.to_string());
        }
    }
}

impl<K: ToString + 'static, V: Visitable + 'static> Visitable for HashMap<K, V> {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        for (k, v) in self {
            visitor.visit(v, k.to_string());
        }
    }
}

impl<K: ToString + 'static, V: Visitable + 'static> Visitable for BTreeMap<K, V> {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        for (k, v) in self {
            visitor.visit(v, k.to_string());
        }
    }
}

impl<A: Visitable + 'static, B: Visitable + 'static> Visitable for (A, B) {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        visitor.visit(&self.0, "0".to_string());
        visitor.visit(&self.1, "1".to_string());
    }
}

impl<A: Visitable + 'static, B: Visitable + 'static, C: Visitable + 'static> Visitable
    for (A, B, C)
{
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        visitor.visit(&self.0, "0".to_string());
        visitor.visit(&self.1, "1".to_string());
        visitor.visit(&self.2, "2".to_string());
    }
}

impl<A: Visitable + 'static, B: Visitable + 'static, C: Visitable + 'static, D: Visitable + 'static>
    Visitable for (A, B, C, D)
{
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        visitor.visit(&self.0, "0".to_string());
        visitor.visit(&self.1, "1".to_string());
        visitor.visit(&self.2, "2".to_string());
        visitor.visit(&self.3, "3".to_string());
    }
}

impl<T: Visitable + 'static> Visitable for Option<T> {
    fn visit_node(&self, visitor: &mut dyn Visitor) {
        if let Some(v) = self {
            v.visit_node(visitor);
        }
    }

    fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
        match self {
            None => false,
            Some(v) => v.visit_leaf(visitor),
        }
    }
}

/*

    For every kind of concrete compound type now:

    impl Visitable for SomePrototype {

        fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
            visitor.grab(self)
        }

        fn visit_node(&self, visitor: &mut dyn Visitor) {
            visitor.visit(&self.property_a, "property_a".to_string());
            visitor.visit(&self.property_b, "property_b".to_string());
            ...
        }
    }

    and for enums:

        impl Visitable for SomePrototype {

        fn visit_leaf(&self, visitor: &mut dyn Visitor) -> bool {
            visitor.grab(&self)
        }

        fn visit_node(&self, visitor: &mut dyn Visitor) {
            match self {
                VariantA(a) => visitor.visit(a, "VariantA".to_string()),
                VariantB(b) => visitor.visit(b, "VariantB".to_string()),
                ...
            }
        }
    }

*/
