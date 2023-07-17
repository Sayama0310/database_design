use std::fmt;

#[derive(Debug)]
struct FunctionalDependency {
    determinant: Vec<String>,
    resultant: Vec<String>,
}

impl fmt::Display for FunctionalDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let determinant = self.determinant.join(", ");
        let resultant = self.resultant.join(", ");
        write!(f, "{} → {{{}}}", determinant, resultant)
    }
}

struct RelationSchema {
    attributes: Vec<String>,
    functional_dependencies: Vec<FunctionalDependency>,
}

struct Relation {
    relation_name: String,
    schema: RelationSchema,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attributes = self.schema.attributes.join(", ");
        let dependencies = self
            .schema
            .functional_dependencies
            .iter()
            .map(|dependency| format!("{}", dependency))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "Relation: {}({}) FD: {}", self.relation_name, attributes, dependencies)
    }
}


fn main() {
    // Create relations
    let relations = vec![
        Relation {
            relation_name: "employee".to_string(),
            schema: RelationSchema {
                attributes: vec![
                    "id".to_string(),
                    "name".to_string(),
                    "age".to_string(),
                    "address".to_string(),
                    "salary".to_string(),
                ],
                functional_dependencies: vec![
                    FunctionalDependency {
                        determinant: vec!["id".to_string()],
                        resultant: vec![
                            "name".to_string(),
                            "age".to_string(),
                            "address".to_string(),
                            "salary".to_string(),
                        ],
                    }
                ],
            },
        },
        Relation {
            relation_name: "project".to_string(),
            schema: RelationSchema {
                attributes: vec![
                    "id".to_string(),
                    "name".to_string(),
                    "location".to_string(),
                    "manager_id".to_string(),
                ],
                functional_dependencies: vec![],
            },
        },
    ];
    // Show relations
    for relation in relations {
        println!("{}", relation);
    }
}
