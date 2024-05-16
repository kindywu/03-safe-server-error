use error_code::ToErrorInfo;

// #[derive(Debug, thiserror::Error, ToErrorInfo)]
// #[error_info(app_type = "http::StatusCode", prefix = "01-")]
// pub enum MyError2 {
//     #[error("Default error")]
//     #[error_info(code = "DE", app_code = "100", client_msg = "default msg")]
//     DefaultError = 1,

//     #[error("Customer error")]
//     #[error_info(code = "CE", app_code = "200", client_msg = "customer msg")]
//     CustomerError = 2,
// }

// fn main() {
//     let err = MyError2::DefaultError;
//     let info = err.to_error_info();
//     println!("{:?}", info);
// }

#[derive(Debug, thiserror::Error, ToErrorInfo)]
#[error_info(app_type = "http::StatusCode", prefix = "01-")]
pub enum MyError {
    #[error("Invalid command: {0}")]
    #[error_info(code = "IC", app_code = "400")]
    InvalidCommand(String),

    #[error("Invalid argument: {0}")]
    #[error_info(code = "IA", app_code = "400", client_msg = "friendly msg")]
    InvalidArgument(String),

    #[error("{0}")]
    #[error_info(code = "RE", app_code = "500")]
    RespError(#[from] std::io::Error),

    #[error("Customer error: {name}")]
    #[error_info(code = "CE", app_code = "600", client_msg = "customer msg")]
    CustomError { name: String },

    #[error("Default error")]
    #[error_info(code = "DE", app_code = "100", client_msg = "default msg")]
    DefaultError,
}

fn main() {
    let err = MyError::InvalidArgument("cmd".to_string());
    let info = err.to_error_info();
    println!("client view {}", info);
    println!("server view {:?}", info);
}
