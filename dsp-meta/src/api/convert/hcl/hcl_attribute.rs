pub struct HclAttribute<'a>(pub &'a hcl::Attribute);
pub struct HclAttributes<'a>(pub Vec<&'a hcl::Attribute>);
