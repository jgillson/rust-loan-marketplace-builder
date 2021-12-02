# rust-loan-marketplace-builder

> A proof-of-concept Rust application using the [cqrs-es](https://github.com/serverlesstechnology/cqrs) framework,
> [actix-web](https://github.com/actix/actix-web) framework and a [postgres-es](https://docs.rs/postgres-es/0.1.0/postgres_es/) implementation.

### Getting Started

1.  Install Rust and Cargo:

    https://doc.rust-lang.org/cargo/getting-started/installation.html


2.  Clone this repository:
 
    `git clone git@github.com:jgillson/rust-loan-marketplace-builder.git`


3.  Enter the project folder and start postgres

    `cd rust-loan-marketplace-builder`

    `docker-compose up -d`


4. Start the application

    `cargo run`


Call the API - the easiest way to do this is to import
[the provided postman collection](rust-loan-marketplace-builder.postman_collection.json) into your Postman client.

### Documentation

- Rust CQRS and Event Sourcing

  https://doc.rust-cqrs.org/

- Crate cqrs_es (a lightweight, opinionated CQRS and event sourcing framework)

  https://docs.rs/cqrs-es/0.1.0/cqrs_es/

- Actix Web (a powerful, pragmatic, and extremely fast web framework for Rust)

  https://github.com/actix/actix-web

- The Rust Programming Language
  
  https://doc.rust-lang.org/book/title-page.html

- The Cargo Book

  https://doc.rust-lang.org/cargo/index.html