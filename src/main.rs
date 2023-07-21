use crate::models::{FunctionalDependency, Relation, RelationSchema};

mod models;

fn main() {
    // Create relations
    let relation = Relation {
        relation_name: "students".to_string(),
        schema: RelationSchema::new(
            vec![
                "id".to_string(),
                "name".to_string(),
                "academic_year".to_string(),
                "faculty".to_string(),
                "faculty_location".to_string(),
                "subject_name".to_string(),
                "grade".to_string(),
                "teacher".to_string(),
            ],
            vec![
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
        ),
    };
    // Show relations
    println!("{}", relation);

    // Decompose relation
    for relation in relation.synthetic_approach_decompose() {
        println!("{}", relation);
    }
}
