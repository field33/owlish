use crate::owl::{
    Annotation, ClassConstructor, ClassIRI, DataSomeValuesFrom, IndividualIRI,
    ObjectPropertyConstructor, ObjectPropertyIRI, Value,
};

mod sub_class_of;
pub use sub_class_of::*;

// TODO: Extract the other constructors to separate files.

#[derive(Debug)]
pub struct EquivalentClasses(
    pub(crate) ClassIRI,
    pub(crate) Box<ClassConstructor>,
    pub(crate) Vec<Annotation>,
);
#[derive(Debug)]
pub struct DisjointClasses(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);
#[derive(Debug)]
pub struct ObjectIntersectionOf(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);
#[derive(Debug)]
pub struct ObjectUnionOf(pub(crate) Vec<ClassConstructor>, pub(crate) Vec<Annotation>);
#[derive(Debug)]
pub struct ObjectComplementOf(pub(crate) Box<ClassConstructor>, pub(crate) Vec<Annotation>);

/// Class construction based on properties.
#[derive(Debug)]
pub struct ObjectSomeValuesFrom(
    pub(crate) ObjectPropertyConstructor,
    pub(crate) ClassIRI,
    pub(crate) Vec<Annotation>,
);
#[derive(Debug)]
pub struct ObjectMaxCardinality(
    pub(crate) u64,
    pub(crate) ObjectPropertyIRI,
    pub(crate) Option<ClassIRI>,
);
#[derive(Debug)]
pub struct ObjectMinCardinality(
    pub(crate) u64,
    pub(crate) ObjectPropertyIRI,
    pub(crate) Option<ClassIRI>,
);
#[derive(Debug)]
pub struct ObjectExactCardinality(
    pub(crate) u64,
    pub(crate) ObjectPropertyIRI,
    pub(crate) Option<ClassIRI>,
);
#[derive(Debug)]
pub struct ObjectAllValuesFrom(
    pub(crate) ObjectPropertyConstructor,
    pub(crate) ClassIRI,
    pub(crate) Vec<Annotation>,
);
#[derive(Debug)]
pub struct ObjectHasValue(
    pub(crate) ObjectPropertyConstructor,
    pub(crate) Value,
    pub(crate) Vec<Annotation>,
);
#[derive(Debug)]
pub struct ObjectHasSelf(
    pub(crate) ObjectPropertyConstructor,
    pub(crate) Vec<Annotation>,
);
/// Class construction based on instances
#[derive(Debug)]
pub struct ObjectOneOf(pub(crate) Vec<IndividualIRI>, pub(crate) Vec<Annotation>);

// into box
impl From<ClassIRI> for Box<ClassConstructor> {
    fn from(iri: ClassIRI) -> Self {
        Box::new(ClassConstructor::IRI(iri))
    }
}

impl From<DataSomeValuesFrom> for Box<ClassConstructor> {
    fn from(c: DataSomeValuesFrom) -> Self {
        Box::new(ClassConstructor::DataSomeValuesFrom(c))
    }
}
impl From<EquivalentClasses> for Box<ClassConstructor> {
    fn from(c: EquivalentClasses) -> Self {
        Box::new(ClassConstructor::EquivalentClasses(c))
    }
}
impl From<DisjointClasses> for Box<ClassConstructor> {
    fn from(c: DisjointClasses) -> Self {
        Box::new(ClassConstructor::DisjointClasses(c))
    }
}
impl From<ObjectIntersectionOf> for Box<ClassConstructor> {
    fn from(c: ObjectIntersectionOf) -> Self {
        Box::new(ClassConstructor::ObjectIntersectionOf(c))
    }
}
impl From<ObjectUnionOf> for Box<ClassConstructor> {
    fn from(c: ObjectUnionOf) -> Self {
        Box::new(ClassConstructor::ObjectUnionOf(c))
    }
}
impl From<ObjectComplementOf> for Box<ClassConstructor> {
    fn from(c: ObjectComplementOf) -> Self {
        Box::new(ClassConstructor::ObjectComplementOf(c))
    }
}
impl From<ObjectSomeValuesFrom> for Box<ClassConstructor> {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        Box::new(ClassConstructor::ObjectSomeValuesFrom(c))
    }
}
impl From<ObjectOneOf> for Box<ClassConstructor> {
    fn from(c: ObjectOneOf) -> Self {
        Box::new(ClassConstructor::ObjectOneOf(c))
    }
}
impl From<ObjectMaxCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMaxCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMaxCardinality(c))
    }
}
impl From<ObjectMinCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectMinCardinality) -> Self {
        Box::new(ClassConstructor::ObjectMinCardinality(c))
    }
}
impl From<ObjectExactCardinality> for Box<ClassConstructor> {
    fn from(c: ObjectExactCardinality) -> Self {
        Box::new(ClassConstructor::ObjectExactCardinality(c))
    }
}
impl From<ObjectAllValuesFrom> for Box<ClassConstructor> {
    fn from(c: ObjectAllValuesFrom) -> Self {
        Box::new(ClassConstructor::ObjectAllValuesFrom(c))
    }
}
impl From<ObjectHasValue> for Box<ClassConstructor> {
    fn from(c: ObjectHasValue) -> Self {
        Box::new(ClassConstructor::ObjectHasValue(c))
    }
}

impl From<ObjectHasSelf> for Box<ClassConstructor> {
    fn from(c: ObjectHasSelf) -> Self {
        Box::new(ClassConstructor::ObjectHasSelf(c))
    }
}

// into constructor

impl From<ClassIRI> for ClassConstructor {
    fn from(iri: ClassIRI) -> Self {
        ClassConstructor::IRI(iri)
    }
}
impl From<DataSomeValuesFrom> for ClassConstructor {
    fn from(c: DataSomeValuesFrom) -> Self {
        ClassConstructor::DataSomeValuesFrom(c)
    }
}
impl From<EquivalentClasses> for ClassConstructor {
    fn from(c: EquivalentClasses) -> Self {
        ClassConstructor::EquivalentClasses(c)
    }
}
impl From<DisjointClasses> for ClassConstructor {
    fn from(c: DisjointClasses) -> Self {
        ClassConstructor::DisjointClasses(c)
    }
}
impl From<ObjectIntersectionOf> for ClassConstructor {
    fn from(c: ObjectIntersectionOf) -> Self {
        ClassConstructor::ObjectIntersectionOf(c)
    }
}
impl From<ObjectUnionOf> for ClassConstructor {
    fn from(c: ObjectUnionOf) -> Self {
        ClassConstructor::ObjectUnionOf(c)
    }
}
impl From<ObjectComplementOf> for ClassConstructor {
    fn from(c: ObjectComplementOf) -> Self {
        ClassConstructor::ObjectComplementOf(c)
    }
}
impl From<ObjectSomeValuesFrom> for ClassConstructor {
    fn from(c: ObjectSomeValuesFrom) -> Self {
        ClassConstructor::ObjectSomeValuesFrom(c)
    }
}
impl From<ObjectOneOf> for ClassConstructor {
    fn from(c: ObjectOneOf) -> Self {
        ClassConstructor::ObjectOneOf(c)
    }
}
impl From<ObjectMaxCardinality> for ClassConstructor {
    fn from(c: ObjectMaxCardinality) -> Self {
        ClassConstructor::ObjectMaxCardinality(c)
    }
}
impl From<ObjectMinCardinality> for ClassConstructor {
    fn from(c: ObjectMinCardinality) -> Self {
        ClassConstructor::ObjectMinCardinality(c)
    }
}
impl From<ObjectExactCardinality> for ClassConstructor {
    fn from(c: ObjectExactCardinality) -> Self {
        ClassConstructor::ObjectExactCardinality(c)
    }
}
impl From<ObjectAllValuesFrom> for ClassConstructor {
    fn from(c: ObjectAllValuesFrom) -> Self {
        ClassConstructor::ObjectAllValuesFrom(c)
    }
}
impl From<ObjectHasValue> for ClassConstructor {
    fn from(c: ObjectHasValue) -> Self {
        ClassConstructor::ObjectHasValue(c)
    }
}
impl From<ObjectHasSelf> for ClassConstructor {
    fn from(c: ObjectHasSelf) -> Self {
        ClassConstructor::ObjectHasSelf(c)
    }
}
