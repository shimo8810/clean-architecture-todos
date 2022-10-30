use todos::infra::establish_connection;
use todos::server::run;
use todos::usecase::task::UseCase;

fn main() -> std::io::Result<()> {
    let repo = establish_connection();
    let usecase = UseCase::new(repo);
    run(usecase)
}
