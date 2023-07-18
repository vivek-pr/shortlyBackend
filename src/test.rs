#[cfg(test)]
mod tests {
    use crate::models::Database;
    use super::*;

    #[test]
    fn test_database_new() {
        // Create a new instance of the Database
        let database = Database::new("mongodb://localhost:27017", "url_shortener", "urls");

        // Check if the database is created
        assert!(database.is_ok());

    }
}
