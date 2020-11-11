let
    pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
    buildInputs = with pkgs; [
        rustup llvmPackages.libclang
        gtk3 python38Packages.xdot
        graphviz
    ];

    shellHook = ''
        XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH:${pkgs.hicolor-icon-theme}/share:${pkgs.gnome3.adwaita-icon-theme}/share:${pkgs.python38Packages.xdot}/share
    '';

    LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
}
