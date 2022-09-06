mod sub_class_of;

pub use sub_class_of::*;

mod equivalent_classes;
pub use equivalent_classes::*;

mod disjoint_classes;
pub use disjoint_classes::*;

mod object_intersection_of;
pub use object_intersection_of::*;

mod object_union_of;
pub use object_union_of::*;

mod object_complement_of;
pub use object_complement_of::*;

mod object_some_values_from;
pub use object_some_values_from::*;

mod object_max_cardinality;
pub use object_max_cardinality::*;

mod object_min_cardinality;
pub use object_min_cardinality::*;

mod object_exact_cardinality;
pub use object_exact_cardinality::*;

mod object_all_values_from;
pub use object_all_values_from::*;

mod object_has_value;
pub use object_has_value::*;

mod object_has_self;
pub use object_has_self::*;

/// Class construction based on instances
mod object_one_of;
pub use object_one_of::*;

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// pub enum Constructor {
//     SubClassOf(SubClassOf),
//     EquivalendClasses(EquivalentClasses),
//     DisjointClasses(DisjointClasses),
//     ObjectIntersectionOf(ObjectIntersectionOf),
//     ObjectUnionOf(ObjectUnionOf),
//     ObjectComplementOf(ObjectComplementOf),
//     ObjectSomeValuesFrom(ObjectSomeValuesFrom),
//     ObjectMaxCardinality(ObjectMaxCardinality),
//     ObjectMinCardinality(ObjectMinCardinality),
//     ObjectExactCardinality(ObjectExactCardinality),
//     ObjectAllValuesFrom(ObjectAllValuesFrom),
//     ObjectHasValaue(ObjectHasValue),
//     ObjectHasSelf(ObjectHasSelf),
// }
