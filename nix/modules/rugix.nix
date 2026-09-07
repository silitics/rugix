{
  config,
  lib,
  pkgs,
  ...
}:

let
  inherit (lib)
    mkEnableOption
    mkIf
    mkOption
    ;
  cfg = config.services.rugix;
  toml = pkgs.formats.toml { };
in

{
  options.services.rugix = {
    enable = mkEnableOption "Rugix lifecycle management";

    package = lib.mkPackageOption pkgs "rugix-ctrl" { };

    settings = mkOption {
      inherit (toml) type;
      default = { };
      description = "Rugix Ctrl system configuration written to /etc/rugix/system.toml.";
    };

    apps.enable = mkEnableOption "Rugix Apps lifecycle and boot recovery";

    apps.dockerCompose.enable = mkEnableOption "Docker Compose Rugix Apps";

    daemon = {
      enable = mkEnableOption "the privileged Rugix Ctrl operation daemon";

      features = {
        factoryReset = mkEnableOption "factory-reset operations through the daemon";
        systemCommit = mkEnableOption "system-commit operations through the daemon";
        systemReboot = mkEnableOption "system-reboot operations through the daemon";
        appLifecycle = mkEnableOption "Rugix Apps operations through the daemon";
      };
    };
  };

  config = lib.mkMerge [
    (mkIf cfg.apps.dockerCompose.enable {
      services.rugix = {
        enable = true;
        apps.enable = true;
      };

      virtualisation.docker.enable = true;

      environment.etc."rugix/components/docker-compose.toml".source =
        toml.generate "rugix-docker-compose-component.toml"
          {
            id = "runtime.docker-compose";
            version = pkgs.docker-compose.version;
          };
    })

    (mkIf cfg.enable {
      environment.systemPackages = [ cfg.package ];
      environment.etc."rugix/system.toml".source = toml.generate "rugix-system.toml" cfg.settings;

      users.groups = mkIf cfg.daemon.enable {
        rugix-daemon = { };
      };

      environment.etc."rugix/daemon.toml" = mkIf cfg.daemon.enable {
        source = toml.generate "rugix-daemon.toml" {
          features = {
            factory-reset = cfg.daemon.features.factoryReset;
            system-commit = cfg.daemon.features.systemCommit;
            system-reboot = cfg.daemon.features.systemReboot;
            app-lifecycle = cfg.daemon.features.appLifecycle;
          };
        };
      };

      systemd.services = {
        rugix-ctrl-daemon = mkIf cfg.daemon.enable {
          description = "Privileged Rugix Ctrl Operation Daemon";
          wantedBy = [ "multi-user.target" ];
          after = [ "local-fs.target" ];
          serviceConfig = {
            Type = "simple";
            User = "root";
            Group = "rugix-daemon";
            UMask = "0117";
            ExecStart = "${cfg.package}/bin/rugix-ctrl daemon";
            Restart = "on-failure";
          };
        };

        rugix-apps-restore-units = mkIf cfg.apps.enable {
          description = "Restore Rugix App Units into systemd";
          wantedBy = [ "multi-user.target" ];
          after = [ "local-fs.target" ];
          unitConfig.DefaultDependencies = false;
          serviceConfig = {
            Type = "oneshot";
            ExecStart = "${cfg.package}/bin/rugix-ctrl apps service-manager systemd restore-units";
            RemainAfterExit = true;
          };
        };

        rugix-apps-recover = mkIf cfg.apps.enable {
          description = "Recover Interrupted Rugix App Transitions";
          wantedBy = [ "multi-user.target" ];
          wants = [ "rugix-apps-restore-units.service" ];
          requires = lib.optional cfg.apps.dockerCompose.enable "docker.service";
          after = [
            "multi-user.target"
            "rugix-apps-restore-units.service"
          ]
          ++ lib.optional cfg.apps.dockerCompose.enable "docker.service";
          serviceConfig = {
            Type = "oneshot";
            ExecStart = "${cfg.package}/bin/rugix-ctrl apps recover";
            RemainAfterExit = true;
          };
        };
      };
    })
  ];
}
