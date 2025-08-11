///
///
/// ```rust
/// use boss::User;
/// let user: User = User::new("John".to_string(), 30);
/// assert_eq!(user.name, "John");
/// assert_eq!(user.age, 30);
/// ```
pub struct User {
    pub name: String,
    pub age: u8,
}

/// EXTENDED RANGE
///
///
///
///
/// Impl Bloc User
/// ```rust
/// use boss::User;
/// let user: User = User::new("John".to_string(), 30);
/// assert_eq!(user.name, "John");
/// assert_eq!(user.age, 30);
/// ```
impl User {
    pub fn println() {
        println!("Hello, world!");
    }

    /// POwer -> This still gets run as cargo test --doc --package project-a -- User --show-output
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    /// On this line it run correct -> cargo test --doc --package project-a -- User::new --show-output
    /// THIS IS LINE 41 DOG
    ///
    ///
    /// Create a new User
    /// ```rust
    /// use boss::User;
    ///
    /// let user: User = User::new("John".to_string(), 30);
    /// assert_eq!(user.name, "John");
    /// assert_eq!(user.age, 29);
    ///
    /// ```
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    /// ECHO -> properly runs : cargo test --doc --package project-a -- User::echo --show-output
    /// Echo a greeting // RANGE OF DOC TEST STARTS HERE
    /// ```rust
    /// use boss::User;
    /// let user: User = User::new("John".to_string(), 30);
    /// assert_eq!(user.name, "John");
    /// assert_eq!(user.age, 30);
    /// ```
    pub fn echo(&self) {
        println!("Hello, world!");
    } // RANGER OF DOC TEST ENDS HERE
}
