/// `Outcome` is similar to `anyhow::Result`, but provides better error reporting
pub type Outcome<T = ()> = helpful::Result<T>;
