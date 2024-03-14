pub mod db;
pub mod binary;
mod user_db;

#[macro_export]
macro_rules! clean_string {
    ( $s:expr ) => (
        $s.trim_matches('"').to_string()
    );
}