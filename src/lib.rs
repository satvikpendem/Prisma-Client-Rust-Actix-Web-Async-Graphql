pub mod features {
    pub mod post {
        pub mod graphql {
            pub mod mutation;
            pub mod query;
            pub mod types;
        }
    }
    pub mod user {
        pub mod graphql {
            pub mod mutation;
            pub mod query;
            pub mod types;
        }
    }
    pub mod graphql;
}

pub mod prisma;
