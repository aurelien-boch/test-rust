use crate::domain::account::AccountId;
use crate::domain::services::id_generation::Id;

mod domain;
mod application;
mod infrastructure;
mod configuration;

#[cfg(test)]
mod test;


fn main() {
    let mut context = configuration::init_context();
    let create_params = application::use_cases::account::create::Params {
        name: "John Doe".to_string(),
        email: "AB@gmail.com".to_string(),
        password: "hi".to_string(),
    };
    let result = application::use_cases::account::create::handler(&mut context, create_params);
    let id = AccountId::from(Id {value: "hi".to_string()});

    match result {
        Ok(_) => {
            let update_email_params = application::use_cases::account::update_email::Params {
                id: id.clone(),
                new_email: "BC@gmail.com".to_string(),
            };
            let update_password_params = application::use_cases::account::update_password::Params {
                id: id.clone(),
                new_password: "hi".to_string(),
            };
            let update_name_params = application::use_cases::account::update_name::Params {
                id: id.clone(),
                new_name: "hi".to_string(),
            };
            let delete_params = application::use_cases::account::delete::Params {
                id: id.clone()
            };

            let result = application::use_cases::account::update_email::handler(&mut context, update_email_params)
                .and_then(|_| application::use_cases::account::update_password::handler(&mut context, update_password_params))
                .and_then(|_| application::use_cases::account::update_name::handler(&mut context, update_name_params))
                .and_then(|_| application::use_cases::account::delete::handler(&mut context, delete_params));

            match result {
                Ok(_) => println!("Success {}", id),
                Err(err) => println!("Error, Message: {}, Type: {:?}", err.message, err.error_type)
            }
        }
        Err(err) => println!("Error, Message: {}, Type: {:?}", err.message, err.error_type)
    };
}
