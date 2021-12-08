use crate::data_types::Length;

pub enum Function {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/attr()
    Attr { name: String }, // Attr(name, unit?, fallback?)

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/blur()
    Blur { radius: Length },
}
