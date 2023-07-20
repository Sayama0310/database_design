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

    pub fn find_minimal_cover(&self) -> Self {
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
                minimal_cover.remove(minimal_cover.iter().position(|x| *x == dependency).unwrap());
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
                if is_equivalent(
                    minimal_cover.clone(),
                    minimal_cover_without_dependency.clone(),
                ) {
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
            if is_equivalent(
                minimal_cover.clone(),
                minimal_cover_without_dependency.clone(),
            ) {
                minimal_cover = minimal_cover_without_dependency;
            }
        }

        return minimal_cover;
    }
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

impl Relation {
    pub(crate) fn synthetic_approach_decompose(&self) -> Vec<Relation> {
        // Step 1: Find a minimal cover for the set of functional dependencies.
        let minimal_cover = self.schema.find_minimal_cover();

        // Step 2: For all FD in the minimal cover that have X as the determinant, denoted as
        // X → B1,..., X → Bk, create a relation schema R(X, Y) where Y = {B1, ..., Bk}.
        // Perform this operation for all FD in the minimal cover. As a result, obtain a set of
        // relation schemas {R1(X1, Y1), ..., Rm(Xm, Ym)}.
        let mut relation_schemas = vec![];
        let all_determinants = minimal_cover
            .0
            .iter()
            .map(|dependency| dependency.determinant.clone())
            .collect::<Vec<Vec<String>>>()
            .dedup();
        for determinant in all_determinants {
            let mut resultant = vec![];
            for dependency in &minimal_cover.0 {
                if dependency.determinant == determinant {
                    for attribute in &dependency.resultant {
                        resultant.push(attribute.clone());
                    }
                }
            }
            relation_schemas.push(RelationSchema::new(
                Attributes::new(determinant),
                FunctionalDependencies::new(vec![FunctionalDependency {
                    determinant: determinant.clone(),
                    resultant,
                }]),
            ));
        }

        // Step 3: If the set of relation schemas obtained in Step 2, {R1, ..., Rm}, contains a
        // relation schema that includes a candidate key of R, then {R1, ..., Rm} is the desired
        // decomposition.
        // Otherwise, if there is no relation schema in {R1, ..., Rm} that includes a candidate key
        // of R, select one of R's candidate keys as K and create R'(K). The desired decomposition
        // is then {R'(K), R1, ..., Rm}.
        let mut candidate_keys = vec![];
        for dependency in minimal_cover.0 {
            if dependency.determinant.len() == self.schema.attributes.0.len() {
                candidate_keys.push(dependency.determinant.clone());
            }
        }
        if candidate_keys.len() == 0 {
            candidate_keys.push(minimal_cover.0[0].determinant.clone());
        }
        let mut candidate_key = candidate_keys[0].clone();
        for key in candidate_keys {
            if key.len() < candidate_key.len() {
                candidate_key = key;
            }
        }
        let mut relation_schemas_with_candidate_key = vec![];
        for schema in relation_schemas.clone() {
            for attribute in &candidate_key {
                if schema.attributes.0.contains(attribute) {
                    relation_schemas_with_candidate_key.push(schema.clone());
                }
            }
        }
        if relation_schemas_with_candidate_key.len() == 0 {
            relation_schemas_with_candidate_key.push(RelationSchema::new(
                Attributes::new(candidate_key.clone()),
                FunctionalDependencies::new(vec![]),
            ));
        }
        let mut decomposition = vec![];
        for schema in relation_schemas_with_candidate_key {
            decomposition.push(Relation {
                relation_name: self.relation_name.clone(),
                schema,
            });
        }
        return decomposition;
    }
}
