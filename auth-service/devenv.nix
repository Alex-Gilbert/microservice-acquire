{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.DATABASE_URL = "postgres://auth_user:auth_password@localhost/auth_db";

  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.diesel-cli pkgs.postgresql];

  # https://devenv.sh/languages/
  languages.rust.enable = true;
}
