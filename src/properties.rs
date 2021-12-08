use crate::data_types;
use strum::EnumDiscriminants;

#[derive(EnumDiscriminants)]
pub enum Property {
    AccentColor(AccentColor),
    AlignContent {
        value: AlignContent,
        safety: Option<bool>,
    },
    AlignItems {
        value: AlignItems,
        safety: Option<bool>,
    },
    AlignSelf {
        value: AlignSelf,
        safety: Option<bool>,
    },
    All(shorthand::All),
    Animation(shorthand::Animation),
    AnimationDelay(AnimationDelay),
    AnimationDirection(AnimationDirection),
    AnimationDuration(AnimationDuration),
    AnimationFillMode(AnimationFillMode),
    AnimationName(AnimationName),
    AnimationPlayState(AnimationPlayState),
    AnimationTimingFunction(AnimationTimingFunction),
    Appearance(Appearance),
    AspectRatio(AspectRatio),
}

pub enum AccentColor {
    Auto,
    Color(data_types::Color),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-content
pub enum AlignContent {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Normal,
    Baseline,
    FirstBaseline,
    LastBaseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    // TODO: Safe/unsafe?
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
pub enum AlignItems {
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    Baseline,
    FirstBaseline,
    LastBaseline,
    Stretch,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-self
pub enum AlignSelf {
    Auto,
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    Baseline,
    FirstBaseline,
    LastBaseline,
    Stretch,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-delay
pub struct AnimationDelay(data_types::Time);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-direction
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-duration
pub struct AnimationDuration(data_types::PositiveTime);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-fill-mode
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-iteration-count
pub enum AnimationIterationCount {
    Infinite,
    Finite(data_types::PositiveNumber), // Default 1
}
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-name
pub enum AnimationName {
    None,
    Custom(data_types::CustomIdent),
}
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-play-state
pub enum AnimationPlayState {
    Running,
    Paused,
}
/// https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timing-function
pub enum AnimationTimingFunction {
    Easing(EasingFunction), // TODO
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(u32, EasingStepsJump),
}
pub enum EasingFunction {}
pub enum EasingStepsJump {
    // TODO: move to easings?
    Start,
    End,
    None,
    Both,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/appearance
pub enum Appearance {
    None,
    Auto,
    MenulistButton,
    Textfield,
    Button,
    Checkbox,
    Listbox,
    Menulist,
    Meter,
    ProgressBar,
    PushButton,
    Radio,
    Searchfield,
    SliderHorizontal,
    SquareButton,
    Textarea,
    // And a bunch more non-standard,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio
pub enum AspectRatio {
    Auto, // initial/inherit/rever/unset?
    Ratio(data_types::Ratio),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter
pub enum BackdropFilter {
    None,
    FilterFunctionList(Vec<FilterFunctionOrSvg>),
}
pub enum FilterFunctionOrSvg {
    Filter(data_types::FilterFunction),
    Svg(data_types::Url),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/backface-visibility
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-attachment
pub struct BackgroundAttachment(Vec<BackgroundLayerAttachment>);
pub enum BackgroundLayerAttachment {
    // TODO: maybe more global <attachment>
    Fixed,
    Local,
    Scroll,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-blend-mode
pub struct BackgroundBlendMode(Vec<BackgroundLayerBlendMode>);
pub struct BackgroundLayerBlendMode(data_types::BlendMode);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip
pub struct BackgroundClip(Vec<BackgroundLayerClip>);
pub enum BackgroundLayerClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-color
pub struct BackgroundColor(Vec<BackgroundLayerColor>);
pub enum BackgroundLayerColor {
    // TODO: or inherit/initial/revert/unset
    Color(data_types::Color),
    CurrentColor,
    Transparent,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-image
pub struct BackgroundImage(Vec<BackgroundLayerImage>);
pub enum BackgroundLayerImage {
    None,
    Image(data_types::Image),
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin
pub struct BackgroundOrigin(Vec<BackgroundLayerOrigin>);
pub enum BackgroundLayerOrigin {
    /// TODO: combine box types?
    BorderBox,
    PaddingBox,
    ContentBox,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
pub struct BackgroundPosition(Vec<BackgroundLayerPosition>);
pub struct BackgroundLayerPosition(data_types::Position);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-position-x
pub struct BackgroundPositionX(Vec<BackgroundLayerPositionX>);
pub struct BackgroundLayerPositionX(data_types::PositionX);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-position-y
pub struct BackgroundPositionY(Vec<BackgroundLayerPositionY>);
pub struct BackgroundLayerPositionY(data_types::PositionY);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat
pub struct BackgroundRepeat(Vec<BackgroundLayerRepeat>);
pub struct BackgroundLayerRepeat {
    pub x: BackgroundRepeatStyle,
    pub y: BackgroundRepeatStyle,
} // TODO: shorthands
pub enum BackgroundRepeatStyle {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/background-size
pub struct BackgroundSize(Vec<BackgroundLayerSize>);
pub enum BackgroundLayerSize {
    Contain,
    Cover,
    Width(data_types::LengthPercentage),
    Size {
        x: data_types::LengthPercentage,
        y: data_types::LengthPercentage,
    },
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/block-size
pub struct BlockSize(data_types::WidthValue);

//* Border Block
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-color
pub struct BorderBlockColor(data_types::Color);

//* Border Block Start
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-start-color
pub struct BorderBlockStartColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-start-style
pub struct BorderBlockStartStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-start-width
pub struct BorderBlockStartWidth(data_types::LineWidth);

//* Border Block End
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-end-color
pub struct BorderBlockEndColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-end-style
pub struct BorderBlockEndStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-end-width
pub struct BorderBlockEndWidth(data_types::LineWidth);

//* Border Bottom
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-color
pub struct BorderBottomColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-style
pub struct BorderBottomStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-width
pub struct BorderBottomWidth(data_types::LineWidth);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius
pub struct BorderBottomLeftRadius(data_types::RadiusCorner);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius
pub struct BorderBottomRightRadius(data_types::RadiusCorner);

//* Border Left
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-color
pub struct BorderLeftColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-style
pub struct BorderLeftStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-width
pub struct BorderLeftWidth(data_types::LineWidth);

//* Border Right
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-color
pub struct BorderRightColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-style
pub struct BorderRightStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-width
pub struct BorderRightWidth(data_types::LineWidth);

//* Border Top
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-color
pub struct BorderTopColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-style
pub struct BorderTopStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-width
pub struct BorderTopWidth(data_types::LineWidth);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius
pub struct BorderTopLeftRadius(data_types::RadiusCorner);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius
pub struct BorderTopRightRadius(data_types::RadiusCorner);

//* Border Start
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-start-start-radius
pub struct BorderStartStartRadius(data_types::RadiusCorner);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-start-end-radius
pub struct BorderStartEndRadius(data_types::RadiusCorner);

//* Border End
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-end-start-radius
pub struct BorderEndStartRadius(data_types::RadiusCorner);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-end-end-radius
pub struct BorderEndEndRadius(data_types::RadiusCorner);

//* Border Inline
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-width
pub struct BorderInlineWidth(data_types::LineWidth);

//* Border Inline Start
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-color
pub struct BorderInlineStartColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-style
pub struct BorderInlineStartStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-width
pub struct BorderInlineStartWidth(data_types::LineWidth);

//* Border Inline End
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-color
pub struct BorderInlineEndColor(data_types::Color);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-style
pub struct BorderInlineEndStyle(data_types::LineStyle);
/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-width
pub struct BorderInlineEndWidth(data_types::LineWidth);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-collapse
pub enum BorderCollapse {
    Collapse,
    Separate,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-spacing
pub struct BorderSpacing {
    // TODO: only when BorderCollpase is separate
    pub x: data_types::Length,
    pub y: data_types::Length,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image-outset
pub struct BorderImageOutset {
    pub top: data_types::LengthNumber,
    pub left: data_types::LengthNumber,
    pub bottom: data_types::LengthNumber,
    pub right: data_types::LengthNumber,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image-repeat
pub struct BorderImageRepeat {
    pub x: BorderImageRepeatValue,
    pub y: BorderImageRepeatValue,
}

pub enum BorderImageRepeatValue {
    Stretch,
    Repeat,
    Round,
    Space,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image-slice
pub struct BorderImageSlice {
    pub x1: data_types::NumberPercentage,
    pub x2: data_types::NumberPercentage,
    pub y1: data_types::NumberPercentage,
    pub y2: data_types::NumberPercentage,

    pub fill: bool,
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image-source
pub struct BorderImageSource(Option<data_types::Image>);

/// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image-width
pub struct BorderImageWidth {
    pub top: BorderImageWidthValue,
    pub left: BorderImageWidthValue,
    pub bottom: BorderImageWidthValue,
    pub right: BorderImageWidthValue,
}
pub enum BorderImageWidthValue {
    Auto,
    Length(data_types::Length),
    Percentage(data_types::Percentage),
    Number(data_types::Number),
}

pub mod shorthand {
    use super::*;

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/all
    pub enum All {
        Initial,
        Inherit,
        Unset,
        Revert,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/animation
    pub struct Animation {
        // TODO: all option?
        pub name: Option<AnimationName>,
        pub duration: Option<AnimationDuration>,
        pub timing_function: Option<AnimationTimingFunction>,
        pub delay: Option<AnimationDelay>,
        pub direction: Option<AnimationDirection>,
        pub iteration_count: Option<AnimationIterationCount>,
        pub fill_mode: Option<AnimationFillMode>,
        pub play_state: Option<AnimationPlayState>,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/background
    pub struct Background(Vec<BackgroundLayer>);
    pub struct BackgroundLayer {
        // TODO: at least one?
        pub attachment: Option<BackgroundLayerAttachment>,
        pub clip: Option<BackgroundLayerClip>,
        pub color: Option<BackgroundLayerColor>,
        pub image: Option<BackgroundLayerImage>,
        pub origin: Option<BackgroundLayerOrigin>,
        pub position: Option<BackgroundLayerPosition>,
        pub repeat: Option<BackgroundLayerRepeat>,
        pub size: Option<BackgroundLayerSize>,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border
    pub struct Border {
        pub color: Option<data_types::Color>,
        pub style: Option<data_types::LineStyle>,
        pub width: Option<data_types::LineWidth>,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-style
    pub struct BorderStyle {
        pub top: data_types::LineStyle,
        pub left: data_types::LineStyle,
        pub bottom: data_types::LineStyle,
        pub right: data_types::LineStyle,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-color
    pub struct BorderColor {
        pub top: data_types::Color,
        pub left: data_types::Color,
        pub bottom: data_types::Color,
        pub right: data_types::Color,
    }
    
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-width
    pub struct BorderWidth {
        pub top: data_types::LineWidth,
        pub left: data_types::LineWidth,
        pub bottom: data_types::LineWidth,
        pub right: data_types::LineWidth,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius
    pub struct BorderRadius {
        // TODO
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block
    pub struct BorderBlock(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-end
    pub struct BorderBlockEnd(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-start
    pub struct BorderBlockStart(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom
    pub struct BorderBottom(Border);
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-left
    pub struct BorderLeft(Border);
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-right
    pub struct BorderRight(Border);
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-top
    pub struct BorderTop(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-style
    pub struct BorderBlockStyle(BorderStyle);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-style
    pub struct BorderInlineStyle(BorderStyle);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline
    pub struct BorderInline(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end
    pub struct BorderInlineEnd(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start
    pub struct BorderInlineStart(Border);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-color
    pub struct BorderInlineColor(BorderColor);

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-width
    pub struct BorderBlockWidth {
        pub bottom: data_types::LineWidth,
        pub left: data_types::LineWidth,
        pub right: data_types::LineWidth,
        pub top: data_types::LineWidth,
    }

    /// https://developer.mozilla.org/en-US/docs/Web/CSS/border-image
    pub struct BorderImage {
        pub outset: BorderImageOutset,
        pub repeat: BorderImageRepeat,
        pub slice: BorderImageSlice,
        pub source: BorderImageSource,
        pub width: BorderImageWidth,
    }
}
