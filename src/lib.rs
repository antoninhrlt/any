// This file is part of "any"
// Under The Unlicense

/// Trait to implement on *any* object in order to be able to convert it as a
/// [`std::any::Any`] value.
pub trait ToAny {
    /// Converts the `self` value into a [`std::any::Any`] value.
    ///
    /// Generally implemented as following:
    /// ```
    /// impl ToAny for Foo {
    ///     fn as_any(&self) -> &dyn std::any::Any {
    ///         self
    ///     }    
    /// }
    /// ```
    fn as_any(&self) -> &dyn std::any::Any;

    /// Converts the `self` value into a [`std::any::Any`] mutable value.
    ///
    /// Generally implemented as following:
    /// ```
    /// impl ToAny for Foo {
    ///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    ///         self
    ///     }    
    /// }
    /// ```
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
