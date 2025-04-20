with import <nixpkgs> {};
{ pkgs ? import <nixpkgs> {} }:

stdenv.mkDerivation {
  name = "";
  src = ./.;

  buildInputs = with pkgs;  [ 
    # just for cargo 
    clang 
    llvm 
    lldb
  
    gnuplot
    cargo
    rustup
    rustc
    rust-analyzer
    cargo-show-asm
    cargo-profiler
    cargo-criterion
    cargo-flamegraph
    cargo-pgo
  ] ++ (lib.optionals pkgs.stdenv.isLinux ([
    # TODO show application
    cargo-valgrind  # does not exist on apple
    cargo-clone     # cannot be compiled on apple
  ]));
  nativeBuildInputs = with pkgs; [addOpenGLRunpath];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
