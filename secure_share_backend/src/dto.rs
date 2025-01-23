use core::str;
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::{ReceivedFileDetails, SentFileDetails, User};

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
  #[validate(length(min = 1, message = "Name is required"))]
  pub name: String,

  #[validate (
    length(min = 1, message = "Email is required"),
    email(message = "Email is invalid")
  )]
  pub email: String,

  #[validate(
    length(min = 1, message = "Password is required"),
    length(min = 8, message = "Password must be at least 8 characters")
  )]
  pub password: String,

  #[validate (
    length(min = 1, message = "Confirm Password is required"),
    must_match(other = "password", message = "password do not match")
  )]
  #[serde(rename = "passwordConfirm")]
  pub password_confirm:String,

}

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
  #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
  pub email: String,
  #[validate (
    length(min = 1, message = "Password is required"),
    length(min = 8, message = "Password must be at least 8 characters")
  )]
  pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
  #[validate(range(min = 1))]
  pub page: Option<usize>,
  #[validate(range(min = 1, max = 50))]
  pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
  pub id: String,
  pub name: String,
  pub email: String,
  pub  public_key: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,

}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
      FilterUserDto {
        id: user.id.to_string(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        public_key: user.public_key.to_owned(),
        created_at: user.created_at.unwrap(),
        updated_at: user.updated_at.unwrap(),
      }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
  pub user: FilterUserDto,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponseDto {
  pub status: String,
  pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
  pub file_id: String,
  pub file_name: String,
  pub recipient_email: String,
  pub expiration_date: DateTime<Utc>,
  pub created_at: DateTime<Utc>
}

impl UserSendFileDto {
  pub fn filter_send_user_file(file_data: &SentFileDetails) -> Self {
    UserSendFileDto {
      file_id: file_data.file_id.to_string(),
      file_name: file_data.file_name.to_owned(),
      recipient_email: file_data.recipient_email.to_owned(),
      expiration_date: file_data.expiration_date.unwrap(),
      created_at: file_data.created_at.unwrap(),
    }
  }
    pub fn filter_send_user_files(user: &[SentFileDetails]) -> Vec<UserSendFileDto> {
      user.iter().map(UserSendFileDto::filter_send_user_file).collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSendFileListResponseDto {
  pub status: String, 
  pub files: Vec<UserSendFileDto>,
  pub result: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserReceiveFileDto {
  pub file_id: String,
  pub file_name: String,
  pub sender_email: String,
  pub expiration_date: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl UserReceiveFileDto {
  pub fn filter_recieve_user_file(file_data: &ReceivedFileDetails) -> Self {
    UserReceiveFileDto {
      file_id: file_data.file_id.to_string(),
      file_name: file_data.file_name.to_string(),
      sender_email: file_data.sender_email.to_owned(),
      expiration_date:file_data.expiration_date.unwrap(),
      created_at: file_data.created_at.unwrap()
    }
  }

  pub fn filter_recieve_user_files(user: &[ReceivedFileDetails]) -> Vec<UserReceiveFileDto> {
    user.iter().map(UserReceiveFileDto::filter_recieve_user_file).collect()
  }
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct UserRecieveListResponseDto {
  pub status: String,
  pub files: Vec<UserReceiveFileDto>,
  pub results: i64,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginResponseDto {
  pub status: String,
  pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
  pub status: &'static str,
  pub message: String,
}

#[derive(Validate, Clone, Default, Debug, Deserialize, Serialize)]
pub struct  NameUpdateDto {
  #[validate(length(min = 1, message = "Name is required"))]
  pub name: String,
}

#[derive(Validate, Debug, Default, Clone, Deserialize, Serialize)]
pub struct UserPasswordUpdateDto {
  #[validate(
    length(min = 1, message = "New password is required"),
    length(min = 8, message = "New password must be at least 8 characters")
)]
pub new_password: String,

#[validate(
  length(min = 1, message = "New password confirm is rrquired"),
  length(min = 8, message = "New password confirm must be atleast 8 characters"),
  must_match(other = "new_password", message = "New password does not match")
)]
pub new_password_confirm: String,

#[validate(
  length(min = 1, message = "Old password is required"),
  length(min = 8, message = "Old password must be at least 8 characters")
)]
pub old_password: String,
}

#[derive(Validate, Debug, Default, Clone, Deserialize, Serialize)]
pub struct SearchQueryByEmailDto {
  #[validate(length(min = 1, message = "Query is Required"))]
  pub query: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterEmailDto {
  pub email: String,
}

impl FilterEmailDto {
  pub fn filter_email(user: &User) -> Self {
    FilterEmailDto {
      email: user.email.to_owned(),
    }
  }
  pub fn filter_emails(user: &[User]) -> Vec<FilterEmailDto> {
    user.iter().map(FilterEmailDto::filter_email).collect()
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailListResponse {
  pub status: String,
  pub emails: Vec<FilterEmailDto>
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, Validate)]
pub struct FileUploadDto {
  #[validate(email(message = "Invalid email format"))]
  pub recipient_email: String,

#[validate (
  length(min = 1, message = "New password is required"),
  length(min = 8, message = "New password must be atleast 8 characters long")

)]
pub password: String,

#[validate(custom = "validate_expiration_date")]
pub expiration_date: String,

}

fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
  if expiration_date.is_empty() {
    let mut error = ValidationError::new("expiration_date_required");
    error.message  = Some("Expiration date is required.".into());
    return Err(error);
  }

  let parsed_date = DateTime::parse_from_rfc3339(expiration_date)
  .map_err(|_| {
    let mut error = ValidationError::new("Invalid_date_format");
    error.message = Some("Invalid date format. Expected format is YYY-MM-DDTHH:MM:SS.ssssssZ.".into());
    error
  })?;

  let now = Utc::now();

  if parsed_date <= now {
    let mut error = ValidationError::new("expiration_date_future");
    error.message = Some("Expiration date must be in the future.".into());
    return Err(error);
  }
  Ok(())
}

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Default)]
pub struct RetrieveFileDto {
  #[validate(length(min = 1, message = "Shared id is required"))]
  pub shared_id: String,

  #[validate(
    length(min = 1, message = "Password is required."),
    length(min = 8, message = "Password should be atleast 8 characters")
  )]
  pub password: String,
}

