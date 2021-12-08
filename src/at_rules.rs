use crate::data_types;

// TODO

pub enum AtRule {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#@annotation
    FontFeatureValues {
        swash: (),
        annotation: (),
        ornaments: (),
        stylistic: (),
        styleset: (),
        character_variant: (),
    },

    FontFace {
        ascent_override: FontFaceAscentOverride,
    },
}

pub enum FontFaceAscentOverride {
    Normal,
    Percentage(data_types::Percentage),
}
