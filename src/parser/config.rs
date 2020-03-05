use super::profile::profiles;
use crate::Error;
use std::collections::HashMap;

pub(crate) fn config<'a>(
    profile_name: &'a str,
    input: &'a str,
) -> Result<Option<HashMap<&'a str, &'a str>>, Error> {
    Ok(
        if let Some(mut profile) = profiles(input).ok().map(|profile| profile.1) {
            profile.remove(profile_name)
        } else {
            None
        },
    )
}
