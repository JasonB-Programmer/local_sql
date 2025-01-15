mod sql_client;


#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_connect_through_port() {
        let result=sql_client::connect_through_port().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_connect_through_sql_browser() {
        let result= sql_client::connect_through_sql_browser().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_connect_to_named_instance() {
        let result= sql_client::connect_to_named_instance().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_connect_with_jdbc_connection_string() {
        let result= sql_client::connect_with_jdbc_connection_string().await;
        assert_eq!(result.is_ok(), true);
    }
}
