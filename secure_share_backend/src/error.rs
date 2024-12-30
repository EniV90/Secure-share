use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
  pub status: String,
  pub message: String,

}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<>) -> fmt::Result {
    write!(f, "{}", serde_json::to_string(&self).unwrap())
  }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
  EmptyPassword,
  ExceededMaxPasswordLength(usize),
  InvalidHashFormat,
  HashingError,
  InvalidToken,
  WrongCredentials,
  EmailExists,
  UserNoLongerExists,
  TokenNotProvided,
}

impl ToString for ErrorMessage {
  fn to_string(&self) -> String {
      match self {
        ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
        ErrorMessage::UserNoLongerExists => "User belonging to this token no longer exists".to_string(),
        ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
        ErrorMessage::EmailExists => "A user with this email already exists".to_string(),
        ErrorMessage::HashingError => "Error while hasing password".to_string(),
        ErrorMessage::InvalidHashFormat => "Invalid password hash".to_string(),
        ErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(),
        ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
        ErrorMessage::ExceededMaxPasswordLength(max_length) => format!("Password must not be more than {} characters", max_length)
      }
  }
}

#[derive(Debug, Clone)]
pub struct HttpError {
  pub message: String,
  pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
      HttpError {
        message: message.into(),
        status,
      }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
      HttpError {
        message: message.into(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
      HttpError {
        message: message.into(),
        status: StatusCode::BAD_REQUEST,
      }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
      HttpError {
        message: message.into(),
        status: StatusCode::CONFLICT
      }
    }

    pub fn into_http_response(self) -> Response {
      let json_response = Json(ErrorResponse {
        status: "fail".to_string(),
        message: self.message.clone(),
      });
      (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "HttpError: message: {}, status: {}",
      self.message, self.status
    )
  }
}

impl std::error::Error for  HttpError {}

impl IntoResponse for HttpError {
  fn into_response(self) -> Response {
    self.into_http_response()
  }

}

