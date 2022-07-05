use crate::owl::{Annotation, ClassConstructor};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DisjointClasses(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);

impl From<DisjointClasses> for ClassConstructor {
    fn from(c: DisjointClasses) -> Self {
        ClassConstructor::DisjointClasses(c)
    }
}
impl From<DisjointClasses> for Box<ClassConstructor> {
    fn from(c: DisjointClasses) -> Self {
        Box::new(ClassConstructor::DisjointClasses(c))
    }
}

impl ClassConstructor {
    pub fn disjoint_classes(&self) -> Option<&DisjointClasses> {
        match self {
            ClassConstructor::DisjointClasses(d) => Some(d),
            _ => None,
        }
    }
}
