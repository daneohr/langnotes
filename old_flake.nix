{
  description = "Langnotes - Svelte+Tauri dev env";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

	libs = with pkgs; [
	  webkitgtk_6_0
	  gtk3
	  cairo
	  gdk-pixbuf
	  glib
	  dbus
	  openssl
	  librsvg
	];
	packages = with pkgs; [
	  rustc
	  cargo
	  nodejs
	  pnpm
	  pkg-config
	  dbus
	  openssl
	  glib
	  gtk3
	  libsoup_3
	  webkitgtk_6_0
	  librsvg
	];
      in
      {
        devShell = pkgs.mkShell {
	  buildInputs = packages;
	  
	  shellHook = ''
	    # Ensures dynamic linker finds the GUI libraries at runtime
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.webkitgtk_6_0
              pkgs.gtk3
              pkgs.glib
              pkgs.openssl
              pkgs.libxkbcommon
            ]}:$LD_LIBRARY_PATH"
            echo "Done"
	  '';
	};
      }
    );
}
