pub enum Ast {
    Component(fastn_js::Component),
    UDF(fastn_js::UDF), // user defined function
    StaticVariable(fastn_js::StaticVariable),
    MutableVariable(fastn_js::MutableVariable),
}
