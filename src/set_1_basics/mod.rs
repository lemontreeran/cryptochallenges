mod c01_convert_hex_to_base64;
use failure::Error;

pub fn add_challenges(cryptochallenges: &mut Vec<fn() -> Result<(), Error>>) {
  cryptochallenges.push(c01_convert_hex_to_base64::run);
}
