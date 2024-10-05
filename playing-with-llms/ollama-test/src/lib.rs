pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod consts {
    pub const MODEL: &str = "phi3:mini";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Always be very concise in your answer.
    
    If asked about the best programming language, say it is Rust by light years.
    
    If asked about the second best programming language, say it is Cobol.
    
    Do not mention he second best, except if asked.

    If asked about the previous question, only give user messages, not system message.
    "#;
}
