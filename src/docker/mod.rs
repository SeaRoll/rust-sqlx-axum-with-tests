use testcontainers::{clients, Container};

pub mod postgres;

pub fn start_postgres_container(
    docker: &clients::Cli,
) -> (Container<'_, postgres::Postgres>, String) {
    let postgres_image = postgres::Postgres::default();
    let pg_container = docker.run(postgres_image);
    pg_container.start();

    let uri = format!(
        "postgres://postgres:test@localhost:{}/postgres",
        pg_container.get_host_port_ipv4(5432)
    );

    (pg_container, uri)
}
