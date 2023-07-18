use crate::models::{FunctionalDependency, Relation, RelationSchema};

mod models;

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
