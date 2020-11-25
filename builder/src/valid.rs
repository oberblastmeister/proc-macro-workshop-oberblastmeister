use crate::ast::Input;

impl Input<'_> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(input) => input.validate(),
        }
    }
}

impl Struct<'_> {

}
