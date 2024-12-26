{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git ];


# PostgreSQL service configuration
  services.postgres = {
    enable = true;
    package = pkgs.postgresql_15;
    initialDatabases = [
      {
        name = "auth_db";
        user = "auth_user";
        pass = "auth_password";
      }
    ];
    port = 5432;  # Default port for PostgreSQL
    listen_addresses = "127.0.0.1";  # Restrict access to localhost
    settings = {
      log_connections = true;  # Log each connection
      log_statement = "all";  # Log all SQL statements
      max_connections = 100;  # Allow up to 100 concurrent connections
    };
  };
}
