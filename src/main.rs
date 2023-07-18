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
        write!(f, "{{{}}} → {{{}}}", determinant, resultant)
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
            .join(", ");
        write!(f, "Relation: {}({}), FD: {{{}}}", self.relation_name, attributes, dependencies)
    }
}


fn main() {
    // Create relations
    let relations = vec![
        Relation {
            relation_name: "students".to_string(),
            schema: RelationSchema {
                attributes: vec![
                    "id".to_string(),
                    "name".to_string(),
                    "academic_year".to_string(),
                    "faculty".to_string(),
                    "faculty_location".to_string(),
                    "subject_name".to_string(),
                    "grade".to_string(),
                    "teacher".to_string(),
                ],
                functional_dependencies: vec![
                    FunctionalDependency {
                        determinant: vec!["id".to_string()],
                        resultant: vec![
                            "name".to_string(),
                            "academic_year".to_string(),
                            "faculty".to_string(),
                            "faculty_location".to_string(),
                            "subject_name".to_string(),
                            "grade".to_string(),
                            "teacher".to_string(),
                        ],
                    },
                    FunctionalDependency {
                        determinant: vec!["faculty".to_string()],
                        resultant: vec!["faculty_location".to_string()],
                    },
                    FunctionalDependency {
                        determinant: vec!["subject_name".to_string()],
                        resultant: vec!["teacher".to_string()],
                    },
                ],
            },
        }
    ];
    // Show relations
    for relation in relations {
        println!("{}", relation);
    }
}
