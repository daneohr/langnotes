{
  description = "Langnotes - Tauri+Svelte";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
	system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          # Tools that run on your host to compile/build the app
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustc
            cargo
            nodejs
            pnpm

            # tauri dependencies
            atk
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1

          ];

          # System libraries that your app links against (Tauri dependencies)
          buildInputs = with pkgs; [
            gsettings-desktop-schemas
            glib-networking
          ];

          GSETTINGS_SCHEMA_DIR = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}/glib-2.0/schemas";

          #shellHook = ''
          #  # Ensures dynamic linker finds the GUI libraries at runtime
          #  export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
          #    pkgs.webkitgtk_4_1
          #    pkgs.gtk3
          #    pkgs.glib
          #    pkgs.openssl
          #    pkgs.libxkbcommon
          #  ]}:$LD_LIBRARY_PATH"

          #  echo "Tauri Dev Environment Loaded Successfully!"
          #'';

          shellHook = ''
        export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"
        export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/"
      '';
        };
      }
    );
}
