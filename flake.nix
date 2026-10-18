{
  description = "Langnotes - Tauri+Svelte";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
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
          ];

          # System libraries that your app links against (Tauri dependencies)
          buildInputs = with pkgs; [
            webkitgtk_4_1
            gtk3
            cairo
            gdk-pixbuf
            glib
            dbus
            openssl
            librsvg
            libsoup_3
          ];

          shellHook = ''
            # Ensures dynamic linker finds the GUI libraries at runtime
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.webkitgtk_4_1
              pkgs.gtk3
              pkgs.glib
              pkgs.openssl
              pkgs.libxkbcommon
            ]}:$LD_LIBRARY_PATH"
            
            echo "Tauri Dev Environment Loaded Successfully!"
          '';
        };
      }
    );
}
