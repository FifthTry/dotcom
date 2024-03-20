pub(crate) fn json_field<T: serde::de::DeserializeOwned>(
    json_body: &ft_sdk::JsonBody,
    field_name: &str,
    error_var: &str,
) -> Result<T, ft_common::ActionError> {
    match json_body.field(field_name)? {
        Some(v) => Ok(v),
        None => Err(ft_common::ActionError::single_error(
            error_var,
            format!("{field_name} is missing").as_str(),
        )),
    }
}
