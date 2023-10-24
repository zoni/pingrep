#[macro_export]
macro_rules! subcommands {
    ($($module:ident),*) => (
      paste::paste! {
        #[derive(Subcommand)]
        enum Commands {
            $(
              [<$module:camel>]($module::Args),
            )*
        }

        impl Commands {
            fn run(ctx: Context, args: Args) -> std::result::Result<(), snafu::Whatever> {
              match args.command {
                $(
                  Commands::[<$module:camel>](args) => $module::command(ctx, args),
                )*
              }
            }
        }
      }
    );
}
