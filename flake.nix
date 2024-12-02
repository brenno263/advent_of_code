{
	description = "Repo for Advent of Code solutions";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/24.11-beta";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = {self, nixpkgs, flake-utils}:
		flake-utils.lib.eachDefaultSystem (system:
			let
				name = "advent-of-code";
				src = ./.;
				pkgs = nixpkgs.legacyPackages.${system};
			in
			{
				# packages.default = derivation {
				#   inherit system name src;
				#   builder = with pkgs; "${bash}/bin/bash";
				#   args = [ "-c" "echo foo > $out" ];
				# };
				devShells.default = pkgs.mkShell {
					buildInputs = with pkgs; [
						cargo
						rustc
						rustfmt
						rustPackages.clippy
					];
					RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
				};
			}
		);
}

		# packages =  (system: {
		#   sourcetorio-site = dream2nix.lib.evalModules {
		#     packageSets.nixpkgs = nixpkgs.legacyPackages.${system};
		#     modules = [
		#       ./sourcetorio-site.nix
		#       {
		#         paths.projectRoot = ./.;
		#         paths.projectRootFile = "flake.nix";
		#         paths.package = ./.;
		#       }
		#     ];
		#   };
		#   default = self.packages.${system}.sourcetorio-site;
		# });
