# Setup Backend
## Install and setup rustc from [here](https://www.rust-lang.org/tools/install)
## Open project folder and run the following commands
- `rustup toolchain install nightly`
- `cd backend`
- Open PgAdmin and create a new Database
- `echo DATABASE_URL=postgres://username:password@localhost/database_name > .env`\
Enter details as per your configurations
- Add `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin` to `Path` in environment variables. 
- `cargo install diesel_cli --no-default-features --features postgres` \
Doing this throws an error, go to `C:\Program Files\PostgreSQL\14\lib` copy `libpq.lib` and `libpq.dll` file into `C:\Users\{User Name}\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib` and also copy `libssl-1_1-x64.dll` from `C:\Users\tanis\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin` into the previously mentioned location \
Enter details as required
- This downloads diesel cli.
- Run `diesel migration run` and this does all the data feeding and querying.
- `cargo run` \
And you should have a running server at `localhost/127.0.0.1`

