/// # Builder
///
/// ## Type
///
/// Creational pattern
///
/// ## Description
///
/// Separate the construction of a complex object from its representing
/// so that the same construction process can create different representations.
///
/// ## Addition
///
/// ### Complex object
///
/// There are might be several variants of coffee recipes:
///
/// 1. `coffee` + `milk` + `cocoa` + `sugar`
/// 1. `coffee` + `sugar`
/// 1. `coffee` + `sugar` + `sugar` + `milk`
pub mod builder;

/// # Singleton
///
/// ## Type
///
/// Creational
///
/// ## Description
///
/// Ensure a struct only has one instance and provide a global point of access to it.
pub mod singleton;

/// # Prototype
///
/// ## Type
///
/// Creational
///
/// ## Description
///
/// Specify the kinds of objects to create using a prototypical instance, and create new objects by copying this prototype.
pub mod prototype;
