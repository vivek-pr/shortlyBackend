#[cfg(test)]
mod tests {
    use crate::models::Database;
    use super::*;

    #[test]
    fn test_database_new() {
        // Create a new instance of the Database
        let database = Database::new();

        // Assert that the client and collection are not null
        assert!(database.collection.name() == "urls");
    }
}
