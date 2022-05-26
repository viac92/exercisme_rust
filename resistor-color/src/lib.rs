use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;


#[repr(usize)]
#[derive(Debug, PartialEq, Copy, Clone, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    int_enum::IntEnum::int_value(_color)
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => "Black".to_string(),
        1 => "Brown".to_string(),
        2 => "Red".to_string(),
        3 => "Orange".to_string(),
        4 => "Yellow".to_string(),
        5 => "Green".to_string(),
        6 => "Blue".to_string(),
        7 => "Violet".to_string(),
        8 => "Grey".to_string(),
        9 => "White".to_string(),
        _ => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let iter = ResistorColor::into_enum_iter();
    let mut result: Vec<ResistorColor> = [].to_vec();

    for item in iter {
        result.push(item);
    }

    result
}
