#[derive(Debug)]
pub struct Node<Kind = NodeKind> {
    pub kind: Kind,
}

#[derive(Debug)]
pub enum NodeKind {
    /// A module declaration (`mod`).
    ///
    /// E.g. `mod foo;` or `mod foo { ... }`.
    Mod,
    /// A component declaration (`component`).
    ///
    /// E.g. `component Foo;` or `component Foo { ... }`.
    Component,
    /// A system declaration (`system`).
    ///
    /// E.g. `system HandleFoo { ... }`.
    System,
    /// A import declaration (`import`)
    ///
    /// E.g. `import foo.bar` or `import foo.baz.*`
    Import,
    /// A event declaration (`event`)
    ///
    /// E.g. `event Foo;` or `event Bar { ... }`
    Event,
    /// A const declaration (`const`)
    ///
    /// E.g. `const G: f64 = 9.8;`
    Const,
}
