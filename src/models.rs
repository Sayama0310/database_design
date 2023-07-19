use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalDependency {
    pub(crate) determinant: Vec<String>,
    pub(crate) resultant: Vec<String>,
}

impl fmt::Display for FunctionalDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let determinant = if self.determinant.len() == 1 {
            self.determinant[0].clone()
        } else {
            format!("{{{}}}", self.determinant.join(", "))
        };
        let resultant = if self.resultant.len() == 1 {
            self.resultant[0].clone()
        } else {
            format!("{{{}}}", self.resultant.join(", "))
        };
        write!(f, "{} → {}", determinant, resultant)
    }
}

pub struct Attributes(Vec<String>);

impl Attributes {
    pub fn new(attributes: Vec<String>) -> Self {
        Attributes(attributes)
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attributes = &self.0.join(", ");
        write!(f, "{}", attributes)
    }
}

pub struct FunctionalDependencies(Vec<FunctionalDependency>);

impl FunctionalDependencies {
    pub fn new(functional_dependencies: Vec<FunctionalDependency>) -> Self {
        FunctionalDependencies(functional_dependencies)
    }
}

impl fmt::Display for FunctionalDependencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dependencies = &self
            .0
            .iter()
            .map(|dependency| dependency.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", dependencies)
    }
}

pub struct RelationSchema {
    attributes: Attributes,
    functional_dependencies: FunctionalDependencies,
}

impl RelationSchema {
    pub fn new(attributes: Attributes, functional_dependencies: FunctionalDependencies) -> Self {
        // Check if all attributes in functional dependencies are in attributes
        for dependency in &functional_dependencies.0 {
            for attribute in &dependency.determinant {
                if !attributes.0.contains(attribute) {
                    panic!(
                        "Attribute {} in functional dependency {} is not in attributes {}",
                        attribute, dependency, attributes
                    );
                }
            }
            for attribute in &dependency.resultant {
                if !attributes.0.contains(attribute) {
                    panic!(
                        "Attribute {} in functional dependency {} is not in attributes {}",
                        attribute, dependency, attributes
                    );
                }
            }
        }
        RelationSchema {
            attributes,
            functional_dependencies,
        }
    }

    // pub fn calculate_closure(&self) -> Vec<FunctionalDependency> {
    //     // TODO implement
    //     return vec![];
    // }
    //
    // pub fn is_equivalent(&self, other: &RelationSchema) -> bool {
    //     let closure = self.calculate_closure();
    //     let other_closure = other.calculate_closure();
    //     closure == other_closure
    // }

    // pub fn find_minimal_cover(&self) -> Self {
    //     let mut minimal_cover = dependencies.clone();
    //
    //     // If there is a functional dependency in minimal_cover with a non-singleton right-hand side,
    //     // it will be transformed into a single-attribute right-hand side functional dependency.
    //     // This is done by replacing the functional dependency X → Y1, Y2, ..., Yn with the functional dependencies X → Y1, X → Y2, ..., X → Yn.
    //     for dependency in dependencies {
    //         if dependency.resultant.len() > 1 {
    //             for attribute in &dependency.resultant {
    //                 minimal_cover.push(FunctionalDependency {
    //                     determinant: dependency.determinant.clone(),
    //                     resultant: vec![attribute.to_string()],
    //                 });
    //             }
    //             minimal_cover.remove(minimal_cover.iter().position(|x| *x == dependency).unwrap());
    //         }
    //     }
    //
    //     // For each functional dependency X → A in minimal_cover, if for each attribute B that constitutes X,
    //     // minimal_cover - {X → A} ∪ {{X - B} → A} is equivalent to minimal_cover, then replace X → A in minimal_cover with {X - B} → A.
    //     let mut temp_minimal_cover = minimal_cover.clone();
    //     for dependency in &minimal_cover {
    //         for attribute in &dependency.determinant {
    //             let mut minimal_cover_without_dependency = minimal_cover.clone();
    //             minimal_cover_without_dependency.remove(
    //                 minimal_cover_without_dependency
    //                     .iter()
    //                     .position(|x| *x == dependency.clone())
    //                     .unwrap(),
    //             );
    //             minimal_cover_without_dependency.push(FunctionalDependency {
    //                 determinant: dependency
    //                     .determinant
    //                     .clone()
    //                     .into_iter()
    //                     .filter(|x| *x != attribute.clone())
    //                     .collect(),
    //                 resultant: dependency.resultant.clone(),
    //             });
    //             if is_equivalent(
    //                 minimal_cover.clone(),
    //                 minimal_cover_without_dependency.clone(),
    //             ) {
    //                 temp_minimal_cover = minimal_cover_without_dependency;
    //             }
    //         }
    //     }
    //     minimal_cover = temp_minimal_cover;
    //
    //     // For each functional dependency X → A in minimal_cover, if minimal_cover - {X → A} is equivalent to minimal_cover, then remove X → A from minimal_cover.
    //     for dependency in minimal_cover.clone() {
    //         let mut minimal_cover_without_dependency = minimal_cover.clone();
    //         minimal_cover_without_dependency.remove(
    //             minimal_cover_without_dependency
    //                 .iter()
    //                 .position(|x| *x == dependency)
    //                 .unwrap(),
    //         );
    //         if is_equivalent(
    //             minimal_cover.clone(),
    //             minimal_cover_without_dependency.clone(),
    //         ) {
    //             minimal_cover = minimal_cover_without_dependency;
    //         }
    //     }
    //
    //     return minimal_cover;
    // }
}

pub struct Relation {
    pub(crate) relation_name: String,
    pub(crate) schema: RelationSchema,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attributes = format!("{}", self.schema.attributes);
        let dependencies = format!("{}", self.schema.functional_dependencies);
        write!(
            f,
            "Relation: {}({}), FD: {{{}}}",
            self.relation_name, attributes, dependencies
        )
    }
}
