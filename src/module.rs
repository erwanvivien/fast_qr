/// Module is a single pixel in the QR code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ModuleType {
    /// The module is part of the data              (Encoded data)
    Data = 0 << 1,
    /// The module is part of the finder pattern    (bigger cubes)
    FinderPattern = 1 << 1,
    /// The module is part of the alignment pattern (smaller cubes)
    Alignment = 2 << 1,
    /// The module is part of the timing pattern    (Line between finder patterns)
    Timing = 3 << 1,
    /// The module is part of the format information
    Format = 4 << 1,
    /// The module is part of the version information
    Version = 5 << 1,
    /// Dark module
    DarkModule = 6 << 1,
    /// Space between finder patterns
    Empty = 7 << 1,
}

impl From<u8> for ModuleType {
    fn from(value: u8) -> Self {
        match value {
            0 => ModuleType::Data,
            1 => ModuleType::FinderPattern,
            2 => ModuleType::Alignment,
            3 => ModuleType::Timing,
            4 => ModuleType::Format,
            5 => ModuleType::Version,
            6 => ModuleType::DarkModule,
            7 => ModuleType::Empty,
            _ => unreachable!(),
        }
    }
}

/// Module is a single pixel in the QR code.
/// Module uses u8 to store value and type.
#[derive(Copy, Clone, Debug)]
pub struct Module(pub u8);

impl Module {
    /// Represents a dark module, which is a black pixel.
    pub const DARK: bool = true;
    /// Represents a light module, which is a white pixel.
    pub const LIGHT: bool = false;

    /// Creates a new module with the given type and value.
    #[must_use]
    pub const fn new(value: bool, module_type: ModuleType) -> Self {
        let value = value as u8;
        Module(value | (module_type as u8))
    }

    /// Creates a new module with the given value with type data.
    #[must_use]
    pub const fn data(value: bool) -> Self {
        Module::new(value, ModuleType::Data)
    }

    /// Creates a new module with the given value with type finder pattern.
    #[must_use]
    pub const fn finder_pattern(value: bool) -> Self {
        Module::new(value, ModuleType::FinderPattern)
    }

    /// Creates a new module with the given value with type alignment.
    #[must_use]
    pub const fn alignment(value: bool) -> Self {
        Module::new(value, ModuleType::Alignment)
    }

    /// Creates a new module with the given value with type timing.
    #[must_use]
    pub const fn timing(value: bool) -> Self {
        Module::new(value, ModuleType::Timing)
    }

    /// Creates a new module with the given value with type format.
    #[must_use]
    pub const fn format(value: bool) -> Self {
        Module::new(value, ModuleType::Format)
    }

    /// Creates a new module with the given value with type version.
    #[must_use]
    pub const fn version(value: bool) -> Self {
        Module::new(value, ModuleType::Version)
    }

    /// Creates a new module with the given value with type dark module.
    #[must_use]
    pub const fn dark(value: bool) -> Self {
        Module::new(value, ModuleType::DarkModule)
    }

    /// Creates a new module with the given value with type empty.
    #[must_use]
    pub const fn empty(value: bool) -> Self {
        Module::new(value, ModuleType::Empty)
    }

    /// Returns the boolean value of the module.
    #[must_use]
    pub const fn value(self) -> bool {
        self.0 & 1 == 1
    }

    /// Returns the type of the module.
    #[must_use]
    pub fn module_type(self) -> ModuleType {
        ModuleType::from(self.0 >> 1)
    }

    /// Sets the boolean value of the module.
    pub fn set(&mut self, value: bool) {
        self.0 = if value { self.0 | 1 } else { self.0 & !1 };
    }

    /// Toggles the boolean value of the module.
    pub fn toggle(&mut self) {
        self.0 ^= 1;
    }
}

impl From<bool> for Module {
    fn from(value: bool) -> Self {
        Module::empty(value)
    }
}

impl PartialEq<bool> for Module {
    fn eq(&self, other: &bool) -> bool {
        self.value() == *other
    }
}

impl PartialEq<Self> for Module {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Module {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byte_size() {
        assert_eq!(std::mem::size_of::<Module>(), 1);
    }

    #[test]
    fn data() {
        let module = Module::data(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::Data);
    }

    #[test]
    fn finder_pattern() {
        let module = Module::finder_pattern(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::FinderPattern);
    }

    #[test]
    fn alignment() {
        let module = Module::alignment(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::Alignment);
    }

    #[test]
    fn timing() {
        let module = Module::timing(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::Timing);
    }

    #[test]
    fn format() {
        let module = Module::format(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::Format);
    }

    #[test]
    fn version() {
        let module = Module::version(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::Version);
    }

    #[test]
    fn dark() {
        let module = Module::dark(Module::LIGHT);
        assert_eq!(module.module_type(), ModuleType::DarkModule);
    }

    #[test]
    fn value_light() {
        let module = Module::data(Module::LIGHT);
        assert_eq!(module.value(), Module::LIGHT);
    }

    #[test]
    fn value_dark() {
        let module = Module::data(Module::DARK);
        assert_eq!(module.value(), Module::DARK);
    }

    #[test]
    fn set() {
        let mut module = Module::data(Module::LIGHT);
        module.set(Module::DARK);
        assert_eq!(module.value(), Module::DARK);
    }
}
