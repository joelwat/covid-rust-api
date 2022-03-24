-- Replace with a real password.
CREATE USER 'covid-api-user' @'%' IDENTIFIED WITH mysql_native_password BY SECRET_PASSWORD;
GRANT SELECT,
  INSERT,
  UPDATE,
  DELETE ON covid_api.* TO 'covid-api-user' @'%';
FLUSH PRIVILEGES;
