use owlib::examples::family;
use owlib::owl::*;

#[test]
fn test() {
    let family = family();
    let iri = family.iri_builder();

    // check person
    {
        let person = family.class(&iri.new("Person")).unwrap();
        assert_eq!(
            person
                .annotations()
                .iter()
                .find(|a| a.iri().to_string().ends_with("comment"))
                .unwrap()
                .value(),
            &Value::from("Represents the set of all people")
        );

        assert_eq!(person.super_classes().len(), 0);
    }

    // check woman
    {
        let woman = family.class(&iri.new("Woman")).unwrap();
        assert_eq!(woman.super_classes().is_empty(), false);
        assert_eq!(
            woman.super_classes()[0].parent().iri().unwrap().to_string(),
            "https://example.com/family#Person"
        );
    }

    // check man
    {
        let man = family.class(&iri.new("Man")).unwrap();
        assert_eq!(man.super_classes().len(), 1);
        println!("{:?}", man.super_classes()[0]);
        assert_eq!(
            man.super_classes()[0]
                .annotations()
                .iter()
                .map(|a| (a.iri().as_iri(), a.value()))
                .collect::<Vec<(&IRI, &Value)>>(),
            vec![(
                well_known::rdfs_comment().as_iri(),
                &Value::from("States that every man is a person")
            )]
        )
    }

    // check mary
    {
        let mary = family.individual(&iri.new("Mary")).unwrap();
        assert_eq!(mary.classes().is_empty(), false);
        assert_eq!(
            mary.classes(),
            vec![&iri.class("Person").into(), &iri.class("Woman").into()]
        );
    }

    // check john
    {
        let john = family.individual(&iri.new("John")).unwrap();
        assert_eq!(john.classes().is_empty(), false);
        assert!(
            john.classes().contains(&&iri.class("Father").into()),
            "Expect to contain 'Father' but was: {:?}",
            john.classes()
        );
        let has_parent_complex_class: ClassConstructor = ObjectMaxCardinality(
            4,
            iri.object_prop("hasChild").into(),
            Some(iri.class("Parent")),
        )
        .into();
        assert!(
            john.classes().contains(&&has_parent_complex_class),
            "Expect to contain {:?} but was: {:?}",
            has_parent_complex_class,
            john.classes()
        );
    }
}
