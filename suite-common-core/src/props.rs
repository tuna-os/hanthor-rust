// props.rs — Typed property pool (SfxPoolItem/SfxItemSet pattern).
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Pattern: LibreOffice svl/poolitem.hxx, svl/itemset.hxx.
// A sparse key-value store for formatting properties shared across
// Tables (cell formatting), Letters (paragraph/character formatting),
// and Decks (shape fill/stroke, text formatting).

use std::collections::HashMap;
use std::fmt;

/// Property identifier — typed key for each property kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyId {
    FontFamily,
    FontSize,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    TextColor,
    BackgroundColor,
    Alignment,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
    BorderColor,
    NumberFormat,
    LineSpacing,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    FillColor,
    StrokeColor,
    StrokeWidth,
    Custom(u16), // Extension point for app-specific properties
}

/// Typed property value.
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Color { r: u8, g: u8, b: u8, a: u8 },
    Enum(u16),
    Point { x: f64, y: f64 },
    Rect { x: f64, y: f64, w: f64, h: f64 },
}

impl PropertyValue {
    pub fn as_bool(&self) -> Option<bool>          { if let Self::Bool(v) = self { Some(*v) } else { None } }
    pub fn as_float(&self) -> Option<f64>           { if let Self::Float(v) = self { Some(*v) } else { None } }
    pub fn as_string(&self) -> Option<&str>         { if let Self::String(v) = self { Some(v) } else { None } }
    pub fn as_color(&self) -> Option<(u8,u8,u8,u8)> { if let Self::Color{r,g,b,a}=self { Some((*r,*g,*b,*a)) } else { None } }
    pub fn as_int(&self) -> Option<i64>             { if let Self::Int(v) = self { Some(*v) } else { None } }
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Color { r, g, b, .. } => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
            Self::Enum(v) => write!(f, "enum({})", v),
            Self::Point { x, y } => write!(f, "({}, {})", x, y),
            Self::Rect { x, y, w, h } => write!(f, "({}, {}, {}, {})", x, y, w, h),
        }
    }
}

/// A sparse set of typed properties. Matches LO's SfxItemSet.
#[derive(Clone, Debug, Default)]
pub struct PropertySet {
    props: HashMap<PropertyId, PropertyValue>,
}

impl PropertySet {
    pub fn new() -> Self { PropertySet { props: HashMap::new() } }

    pub fn set(&mut self, id: PropertyId, value: PropertyValue) {
        self.props.insert(id, value);
    }

    pub fn get(&self, id: PropertyId) -> Option<&PropertyValue> {
        self.props.get(&id)
    }

    pub fn remove(&mut self, id: PropertyId) -> Option<PropertyValue> {
        self.props.remove(&id)
    }

    pub fn is_empty(&self) -> bool { self.props.is_empty() }
    pub fn len(&self) -> usize { self.props.len() }

    /// Merge another set into this one, overwriting conflicts.
    pub fn merge(&mut self, other: &PropertySet) {
        for (id, val) in &other.props {
            self.props.insert(*id, val.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_get_set() {
        let mut ps = PropertySet::new();
        ps.set(PropertyId::Bold, PropertyValue::Bool(true));
        assert_eq!(ps.get(PropertyId::Bold).unwrap().as_bool(), Some(true));
        assert!(ps.get(PropertyId::Italic).is_none());
    }

    #[test]
    fn test_remove() {
        let mut ps = PropertySet::new();
        ps.set(PropertyId::FontSize, PropertyValue::Float(14.0));
        assert!(ps.remove(PropertyId::FontSize).is_some());
        assert!(ps.get(PropertyId::FontSize).is_none());
    }

    #[test]
    fn test_merge_overwrites() {
        let mut a = PropertySet::new();
        a.set(PropertyId::Bold, PropertyValue::Bool(false));
        let mut b = PropertySet::new();
        b.set(PropertyId::Bold, PropertyValue::Bool(true));
        b.set(PropertyId::Italic, PropertyValue::Bool(true));
        a.merge(&b);
        assert_eq!(a.get(PropertyId::Bold).unwrap().as_bool(), Some(true));
        assert_eq!(a.get(PropertyId::Italic).unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_color_value() {
        let mut ps = PropertySet::new();
        ps.set(PropertyId::TextColor, PropertyValue::Color { r: 255, g: 128, b: 64, a: 255 });
        let c = ps.get(PropertyId::TextColor).unwrap().as_color().unwrap();
        assert_eq!(c, (255, 128, 64, 255));
    }

    #[test]
    fn test_string_value() {
        let mut ps = PropertySet::new();
        ps.set(PropertyId::FontFamily, PropertyValue::String("Sans".into()));
        assert_eq!(ps.get(PropertyId::FontFamily).unwrap().as_string(), Some("Sans"));
    }

    #[test]
    fn test_point_rect_values() {
        let mut ps = PropertySet::new();
        ps.set(PropertyId::Custom(1), PropertyValue::Point { x: 10.0, y: 20.0 });
        ps.set(PropertyId::Custom(2), PropertyValue::Rect { x: 0.0, y: 0.0, w: 100.0, h: 50.0 });
        assert!(ps.get(PropertyId::Custom(1)).is_some());
        assert!(ps.get(PropertyId::Custom(2)).is_some());
    }
}
