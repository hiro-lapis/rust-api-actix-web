### QA

Q. What benefit Rust gives us ?
A. Rust is the language can run fast and use right weight memory, which enables to cut down the cost in crowd environment.  Rust also has robust language features, such as ownership and strict type function. The robustness is enough to be used in business critical situation, like payment service.

Q. What is `into()`? 
A. into() is a trait that enables a struct to cast another type. If into is implemented, it implicitly enables from() as well.  

Q. Why main function fails when `cargo run`?  
A. `cargo run` simply execute rust file while `cargo make run` execute program following Makefile.toml which contains env info. Thus, `cargo make run` is dominant in practical application.  

Q. What is shared ?  
A. Common dependencies added by this project.  

Q. What is adapter?  
A. Adapter is the layer name that accesses persitance layers including repositories. repository, concreate implementation to connect DB and querying is one of the struct belongs adapter. `Adapter` means that the layer adapt the project api to external servers.  

Q. what api layer?
A. Api is the layer that receive input. handler mainly play the role in this functionality.  

Q. what api kernel?
A. Kernel is the layer that format and process input for following function. model handles domain logic in this layer. repository in this layer is interface for integration of  external services and regis. 

Q. Why repository exists in two layers, kernel and adapter?  
A. For testability, kernel/repository is helpful to mock functions that avoid executing external services.  

Q. What is workspace in rust?
A. Mudularization in rust is termed as workspace. Run `cargo new --lib`, writing workspace member in cargo.toml, and open crate `pub mod xxx` in lib.rs, enables create project's libraries.  

Q. What is `derive`? such as `#[derive(Clone)]` , `#[derive(new)]` , `#[derive(Debug)]` ?
A. These are one of the procedural macro. derive helpfully generate default implementaion.  
- `#[derive(Clone)]` enables to use `clone()`, copy method  
- `#[derive(new)]`  enables to 
- `#[derive(Debug)]` enables output a struct's information for debug.  

Below is example of `#[derive(Debug)]`  
```
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{:?}", person); // Debug 表示
    println!("{:#?}", person); // インデント付きで見やすく表示
}
```

Q. Whan run `docker compose up -d app --build`, this will fail and show errors shows env vars is not set. why?
A. Because env vars are defined in Makefile.toml, not .env. Therefore, docker build commands always have to be executed by `cargo make xxx`.  

Q. What is macro?  
A. Macro is grouped functionality that can be seen C, C++. Rust has couple of types of macro(procedual, declarative), both of them are code that can write other code known as meta programming.  
https://doc.rust-jp.rs/book-ja/ch19-06-macros.html#%E5%B1%9E%E6%80%A7%E3%81%8B%E3%82%89%E3%82%B3%E3%83%BC%E3%83%89%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B%E6%89%8B%E7%B6%9A%E3%81%8D%E7%9A%84%E3%83%9E%E3%82%AF%E3%83%AD  


Q. When I command `sqlx migrate add -r start --source adapter/migrations`, this failes in error `sqlx not found`. How can solve?  
A. Plz run make command that contains `install_crate = { crate_name = "sqlx-cli" ...`. If only do that, sqlx command line will be automatically installed and be able to make migration file.  

Q. What is `xxx impl` and `yyy impl for <xxx>`
A. [impl](https://doc.rust-jp.rs/book-ja/ch05-03-method-syntax.html?search=#%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89%E8%A8%98%E6%B3%95) is the grammer that defines a struct's method.  
`xxx impl for <yyy>`, precisely `<trait> impl for <struct>` is that struct defines function to implement trait. `self` refers the struct. It seems to `トレイトが構造体を実装する`, but actual means is contrary. `構造体がトレイトを実装する` is correct.  

```
trait Greet {
    fn greet(&self);
}

struct User {
    name: String,
}
// implement User for trait
impl Greet for User {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}
```

Q. Is there any difference between interface in other language and trait in rust?  
A. Trait in rust accepts default implementation.  

Q. Some structs have code like `self.0` implementation. What and why can do this?  
A. These structs are **tuple type** struct that has only one field which allows to access 0 index field.  

```
pub struct AuthorizedUserId(UserId); // tuple

impl AuthorizedUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}

```

Q. What is New type pattern ?
A. プリミティブな型に独自の型をつける機能。bookId, UserId, どちらも同じUUID型だとして、独自の型づけをすることで変数の渡し間違いを防ぐことができる

Q. Sometimes `type Error = AppError;` can be seen in structs' `impl` while cannot be seen the type defnition. What or why is the difference?
A. 

Q. arguments sometimes prefixed with `_`. Why?  
A. These are unused arguments. To avoid warning by compiler, prefix is used.  

Q. I sometimes see error, like `error communicating with database: Connection refused (os error 61)`. How to solove it?  
[image](./img/error-communicating-with-database.png)  

A. This error is made by rust-analyzer. Try Ctrl + Shift + P → Rust Analyzer: Restart Server on editor. If the error remain, try `rustup update`, `cargo clean` and restart.  

Q. On CI/CD environment, clippy-ci ends in failure of `error: error communicating with database: failed to lookup address information: Temporary failure in name resolution`, because the process cannot access database. How to solve it?

A. Use [off-line mode](error: error communicating with database: failed to lookup address information: Temporary failure in name resolution). First, add [`SQLX_OFFLINE=true`](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#force-building-in-offline-mode) in the environment, which makes sqlx run without accessing database. Then execute `cargo sqlx prepare`, which generate cache(`.sqlx/`) that contains database information. The cache can be used for clippy's check.

Q. What is `Arc<T>` ?
A. Arc is an abbreviation of Atomically Reference Counted, which enables to share pointer in multi thread process.  

Q. Rust has edition apart from version such as 2024 edition. What is edition and difference?  
A. Edition is destructive changes. While rust assure compatibility between previos version, some big changes are not compatible. Those changes are released by new edition. In this way, existing apps can update version safely and take time to include new edition's feature. See [here](https://doc.rust-jp.rs/edition-guide/editions/index.html).  

Q. What is the difference between `cargo add` and `cargo install`?  
A. `cargo install` command install binary tool that used for general purpose globally, which doesn't affect rust projects. `cargo add` add library in a project and the dependency is written down in cargo.toml.  

Q. In api container, I cannot find rust, such as when I command `rustc --version`, the command is not found. Why?  
A. This is because api container doesn't have rust tool chains. This enviroment applies multi stage build, which is a docker's method separate build environment and running environment. Thus, if I want to use rust dependencies in api container, I have to cease multi stage build. That being said, Most case doesn't need to run rust command in the container after build stage.  

Q. 
A. 

### Impression

- Rust has many features, some of them cannot be seen in other languages, such as ownership,  
While they are useful, but I'm afraid of missing using them and write ugly code due to my experience to date.  

- crud process written in this book as I do in current project, ZB. ZB is constructed with clean archtechture.  

### development task steps

1. create migration `cargo make migration`
2. define trait of repository and function in kernel
3. define struct of model that receive return value of repoisotry in kernel, and struct of event model that passes input in INSERT or UPDATE statement.  
4. define xxxRow trait in adapter
5. implement repository trait in adapter


* in each step, some layers require aditional dependencies. In this case, add dependencies in cargo.toml.  
* make sure exporting new modules.  
