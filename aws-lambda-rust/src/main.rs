use std::error::Error;
use log::*;
use lambda_runtime::lambda;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(myapi::handler);
    Ok(())
}

mod myapi {
    use lambda_runtime::error::HandlerError;
    use lambda_runtime::CustomOutput;
    fn handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
        if e.first_name == "" {
            error!("Empty first name in request {}", c.aws_request_id);
            bail!("Empty first name");
        }

        Ok(CustomOutput {
            message: format!("Hello, {}!", e.first_name),
        })
    }
}

