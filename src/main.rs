use crate::models::{FunctionalDependency, Relation, RelationSchema};

mod models;

fn is_equivalent(dependencies: Vec<FunctionalDependency>, minimal_cover: Vec<FunctionalDependency>) -> bool {
    // TODO implement
    return false;
}

fn find_minimal_cover(dependencies: Vec<FunctionalDependency>) -> Vec<FunctionalDependency> {
    let mut minimal_cover = dependencies.clone();

    // If there is a functional dependency in minimal_cover with a non-singleton right-hand side,
    // it will be transformed into a single-attribute right-hand side functional dependency.
    // This is done by replacing the functional dependency X → Y1, Y2, ..., Yn with the functional dependencies X → Y1, X → Y2, ..., X → Yn.
    for dependency in dependencies {
        if dependency.resultant.len() > 1 {
            for attribute in &dependency.resultant {
                minimal_cover.push(FunctionalDependency {
                    determinant: dependency.determinant.clone(),
                    resultant: vec![attribute.to_string()],
                });
            }
            minimal_cover.remove(
                minimal_cover
                    .iter()
                    .position(|x| *x == dependency)
                    .unwrap(),
            );
        }
    }

    // For each functional dependency X → A in minimal_cover, if for each attribute B that constitutes X,
    // minimal_cover - {X → A} ∪ {{X - B} → A} is equivalent to minimal_cover, then replace X → A in minimal_cover with {X - B} → A.
    let mut temp_minimal_cover = minimal_cover.clone();
    for dependency in &minimal_cover {
        for attribute in &dependency.determinant {
            let mut minimal_cover_without_dependency = minimal_cover.clone();
            minimal_cover_without_dependency.remove(
                minimal_cover_without_dependency
                    .iter()
                    .position(|x| *x == dependency.clone())
                    .unwrap(),
            );
            minimal_cover_without_dependency.push(FunctionalDependency {
                determinant: dependency
                    .determinant
                    .clone()
                    .into_iter()
                    .filter(|x| *x != attribute.clone())
                    .collect(),
                resultant: dependency.resultant.clone(),
            });
            if is_equivalent(minimal_cover.clone(), minimal_cover_without_dependency.clone()) {
                temp_minimal_cover = minimal_cover_without_dependency;
            }
        }
    }
    minimal_cover = temp_minimal_cover;

    // For each functional dependency X → A in minimal_cover, if minimal_cover - {X → A} is equivalent to minimal_cover, then remove X → A from minimal_cover.
    for dependency in minimal_cover.clone() {
        let mut minimal_cover_without_dependency = minimal_cover.clone();
        minimal_cover_without_dependency.remove(
            minimal_cover_without_dependency
                .iter()
                .position(|x| *x == dependency)
                .unwrap(),
        );
        if is_equivalent(minimal_cover.clone(), minimal_cover_without_dependency.clone()) {
            minimal_cover = minimal_cover_without_dependency;
        }
    }

    return minimal_cover;
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
    for relation in &relations {
        println!("{}", relation);
    }

    // Find minimal cover
    let minimal_cover = find_minimal_cover(relations[0].schema.functional_dependencies.clone());
    println!("Minimal cover: {:?}", minimal_cover);
}
