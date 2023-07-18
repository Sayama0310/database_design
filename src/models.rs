use std::fmt;

#[derive(Debug)]
pub struct FunctionalDependency {
    pub(crate) determinant: Vec<String>,
    pub(crate) resultant: Vec<String>,
}

impl fmt::Display for FunctionalDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let determinant = self.determinant.join(", ");
        let resultant = self.resultant.join(", ");
        write!(f, "{{{}}} → {{{}}}", determinant, resultant)
    }
}

pub struct RelationSchema {
    pub(crate) attributes: Vec<String>,
    pub(crate) functional_dependencies: Vec<FunctionalDependency>,
}

pub struct Relation {
    pub(crate) relation_name: String,
    pub(crate) schema: RelationSchema,
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