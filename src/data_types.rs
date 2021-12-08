//! CSS Data Types, as described in https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types

use std::string::String as StdString;

/// https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident
pub struct CustomIdent(StdString);

/// A [`CustomIdent`] prefixed by two dashes
pub struct DashedIdent(StdString);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/string
pub struct String(StdString);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/url()
pub struct Url(StdString); // TODO: use url::Url

pub enum Keyword {}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types#textual_data_types
pub enum Textual {
    CustomIdent(CustomIdent),
    DashedIdent(DashedIdent),

    String(String),
    Url(Url),

    Keyword(Keyword),
    // Initial,
    // Inherit,
    // Unset,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/integer
pub struct Integer(i32);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/number
pub struct Number(f32);
/// A number that cannot be negative // TODO: constructor
pub struct PositiveNumber(f32);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/dimension
pub struct Dimension(f32, Unit);
// TODO
pub enum Unit {
    Length(LengthUnit),
    Angle(AngleUnit),
    Time(TimeUnit),
}

pub trait IntoDimension {
    fn into_dimension(self) -> Dimension;
}
pub trait IntoUnit {
    fn into_unit(self) -> Unit;
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/percentage
pub struct Percentage(Number);
/// A ratio in the form (width, height)
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/ratio
pub struct Ratio(Number, Number);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/flex_value
pub struct Flex(Number);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types#numeric_data_types
pub enum Numeric {
    Integer(Integer),
    Number(Number),
    PositiveNumber(PositiveNumber),
    Dimension(Dimension),
    Percentage(Percentage),
    Ratio(Ratio),
    Flex(Flex),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/length
pub struct Length(f32, LengthUnit);
pub struct PositiveLength(f32, LengthUnit); // TODO: constructor
pub enum LengthUnit {
    //* Font Relative Lengths
    /// Represents the width, or more precisely the advance measure, of the
    /// glyph "0" (zero, the Unicode character U+0030) in the element's font.
    ///
    /// In the cases where it is impossible or impractical to determine the
    /// measure of the “0” glyph, it must be assumed to be 0.5em wide by 1em tall.
    Ch,

    /// Represents the calculated font-size of the element. If used on the
    /// font-size property itself, it represents the inherited font-size of the
    /// element.
    Em,

    /// Represents the x-height of the element's font. On fonts with the "x"
    /// letter, this is generally the height of lowercase letters in the font;
    // 1ex ≈ 0.5em in many fonts.
    Ex,

    /// Represents the font-size of the root element (typically <html>). When
    /// used within the root element font-size, it represents its initial value
    /// (a common browser default is 16px, but user-defined preferences may
    /// modify this).
    Rem,

    //* Viewport-Percentage Lengths
    /// Equal to 1% of the height of the viewport's initial containing block
    Vh,

    /// Equal to 1% of the width of the viewport's initial containing block.
    Vw,

    /// Equal to the smaller of Vw and Vh
    Vmin,

    /// Equal to the larger of Vw and Vh
    Vmax,

    //* Absolute Length Units
    /// One pixel. For screen displays, it traditionally represents one device
    /// pixel (dot). However, for printers and high-resolution screens, one CSS
    /// pixel implies multiple device pixels. 1px = 1/96th of 1in.
    Px,

    /// One centimeter. 1cm = 96px/2.54.
    Cm,

    /// One millimeter. 1mm = 1/10th of 1cm.
    Mm,

    /// One inch. 1in = 2.54cm = 96px.
    In,

    /// One pica. 1pc = 12pt = 1/6th of 1in.
    Pc,

    /// One point. 1pt = 1/72nd of 1in.
    Pt,
}
impl IntoDimension for Length {
    fn into_dimension(self) -> Dimension {
        Dimension(self.0, Unit::Length(self.1))
    }
}
impl IntoUnit for LengthUnit {
    fn into_unit(self) -> Unit {
        Unit::Length(self)
    }
}

/// Clockwise Angle
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/angle
pub struct Angle(f32, AngleUnit); // TODO: Optional unit?
pub enum AngleUnit {
    /// Represents an angle in degrees. One full circle is 360deg.
    Deg,
    /// Represents an angle in gradians. One full circle is 400grad.
    Grad,
    /// Represents an angle in radians. One full circle is 2π radians which
    /// approximates to 6.2832rad. 1rad is 180/π degrees.
    Rad,
    /// Represents an angle in a number of turns. One full circle is 1turn.
    Turn,
}
impl IntoDimension for Angle {
    fn into_dimension(self) -> Dimension {
        Dimension(self.0, Unit::Angle(self.1))
    }
}
impl IntoUnit for AngleUnit {
    fn into_unit(self) -> Unit {
        Unit::Angle(self)
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/time
pub struct Time(f32, TimeUnit);
pub struct PositiveTime(f32, TimeUnit); // TODO: construct with only t >= 0

pub enum TimeUnit {
    /// Represents a time in seconds
    S,
    /// Represents a time in milliseconds
    Ms,
}
impl IntoDimension for Time {
    fn into_dimension(self) -> Dimension {
        Dimension(self.0, Unit::Time(self.1))
    }
}
impl IntoUnit for TimeUnit {
    fn into_unit(self) -> Unit {
        Unit::Time(self)
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/frequency
pub struct Frequency(f32, FrequencyUnit);
pub enum FrequencyUnit {
    /// Represents a frequency in hertz
    Hz,
    /// Represents a frequency in kilohertz
    KHz,
}

pub struct Resolution(f32, ResolutionUnit);
pub enum ResolutionUnit {
    /// Represents the number of dots per inch. Screens typically contains 72
    /// or 96 dots per inch, but the dpi for printed documents is usually much
    /// greater. As 1 inch is 2.54 cm, 1dpi ≈ 0.39dpcm.
    Dpi,

    /// Represents the number of dots per centimeter. As 1 inch is 2.54 cm,
    /// 1dpcm ≈ 2.54dpi.
    DpCm,

    /// Represents the number of dots per px unit. Due to the 1:96 fixed ratio
    /// of CSS in to CSS px, 1dppx is equivalent to 96dpi, which corresponds to
    /// the default resolution of images displayed in CSS as defined by
    /// image-resolution.
    ///
    /// Aliased by `x` unit
    DpPx,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types#quantities
pub enum Quantity {
    Length(Length),
    Angle(Angle),
    Time(Time),
    PositiveTime(PositiveTime),
    Frequency(Frequency),
    Resolution(Resolution),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/length-percentage
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

pub enum LengthNumber {
    Length(Length),
    Number(Number),
}

// pub enum FrequencyPercentage {
//     Frequency(Frequency),
//     Percentage(Percentage),
// }

/// https://developer.mozilla.org/en-US/docs/Web/CSS/angle-percentage
pub enum AnglePercentage {
    Angle(Angle),
    Percentage(Percentage),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/time-percentage
pub enum TimePercentage {
    Time(Time),
    Percentage(Percentage),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types#combinations_of_types
pub enum Combination {
    LengthPercentage(LengthPercentage),
    // FrequencyPercentage(FrequencyPercentage),
    AnglePercentage(AnglePercentage),
    TimePercentage(TimePercentage),
}

// TODO
/// https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
pub enum Color {
    // Keyword(ColorKeyword),
    Rgb,
    Hsl,
    Lch,
    Lab,
}

/// Specifies the transparency of a color. May be a <number>, in which case 0 is
/// fully transparent and 1 is fully opaque, or a <percentage>, in which case 0%
/// is fully transparent and 100% fully opaque.
pub enum Alpha {
    Number(Number), // Only 0 to 1
    Percentage(Percentage),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/image
pub enum Image {
    Url(Url),
    Gradient(Gradient),
    // Element,
    // Image,
    // CrossFade,
    // ImageSet,
}

// TODO: Gradient Types

/// https://developer.mozilla.org/en-US/docs/Web/CSS/gradient
pub enum Gradient {
    Linear {
        from: Color,
        to: Color,
        angle: Angle,
        repeat: Option<Length>,
    },
    Radial {
        from: Color,
        to: Color,

        shape: RadialGradientShape,
    },
}

pub enum RadialGradientShape {}

// pub enum GradientDirection {
//     ToTop,
//     ToLeft,
//     ToRight,
//     ToBottom, // Default
//     ToTopLeft,
//     ToTopRIght,
//     ToBottomLeft,
//     ToBottomRight,

//     Angle(Angle),
// }

// TODO: Move to data types
/// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function
pub enum FilterFunction {
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/blur()
    Blur { radius: Length },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/brightness()
    Brightness { amount: NumberPercentage },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/contrast()
    Constrast { amount: NumberPercentage },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/drop-shadow()
    DropShadow {
        offset_x: Length,
        offset_y: Length,
        blur_radius: Option<Length>,
        color: Option<Color>,
    },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/grayscale()
    Grayscale { amount: NumberPercentage },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/hue-rotate()
    HueRotate { angle: Angle },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/invert()
    Invert { amount: NumberPercentage },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/opacity()
    Opacity {
        amount: NumberPercentage, // TODO: 0-1 or 0-100%
    },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/saturate()
    Saturate {
        amount: NumberPercentage, // TODO: positive number
    },
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/sepia()
    Sepia { amount: NumberPercentage },
}
pub enum NumberPercentage {
    Number(Number),
    Percentage(Percentage),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/blend-mode
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/position_value
pub struct Position {
    pub x: PositionX,
    pub y: PositionY, // TODO: formal syntax vs BackgroundPosition syntax
}
/// X position (with optional offset)
pub enum PositionX {
    Left(Option<LengthPercentage>),
    Center,
    Right(Option<LengthPercentage>),
    Value(LengthPercentage),
}
/// Y position (with optional offset)
pub enum PositionY {
    Top(Option<LengthPercentage>),
    Center,
    Bottom(Option<LengthPercentage>),
    Value(LengthPercentage),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape
pub enum BasicShape {
    Inset(InsetRectange),
    Circle(Circle),
    Ellipse(Ellipse),
    Polygon(Polygon),
    Path(Path),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape/inset()
pub struct InsetRectange {
    top: LengthPercentage,
    left: LengthPercentage,
    bottom: LengthPercentage,
    right: LengthPercentage,
    border_radius: RadiusCorner,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape/circle()
pub struct Circle {
    radius: ShapeRadius,
    position: Option<Position>,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape/ellipse()
pub struct Ellipse {
    x_radius: ShapeRadius,
    y_radius: ShapeRadius,
    position: Option<Position>,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/basic-shape/polygon()
pub struct Polygon {
    fill_rule: FillRule,
    points: Vec<(LengthPercentage, LengthPercentage)>, // TODO: minimum 3
}

pub struct Path {
    // TODO: SvgPath?
    fill_rule: FillRule,
    svg_path: String,
}

pub enum ShapeRadius {
    // TODO: positive
    Radius(LengthPercentage),
    ClosestSide,
    FarthestSide,
}

pub enum FillRule {
    Nonzero,
    EvenOdd,
}

pub enum WidthValue {
    Length(Length),
    Percentage(Percentage),
    Auto,
    MaxContent,
    MinContent,
    FitContent(LengthPercentage),
}

pub enum LineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

pub enum LineWidth {
    Thin,
    Medium,
    Thick,
    Value(PositiveLength),
}

pub enum RadiusCorner {
    Circle(LengthPercentage),
    Ellipse {
        x: LengthPercentage,
        y: LengthPercentage,
    },
}

// TODO: finish
